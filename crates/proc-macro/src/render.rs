use base64::{engine::general_purpose::STANDARD, Engine};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, DiscriminatorNode, EnumVariantTypeNode,
    InstructionInputValueNode, NestedTypeNode, NestedTypeNodeTrait, Number, NumberFormat, RootNode,
    TypeNode, ValueNode,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn render_vixen_parser(idl: &RootNode) -> TokenStream {
    let program_mod_ident = format_ident!("{}", to_snake_case(&idl.program.name));
    let program_public_key = &idl.program.public_key;

    let defined_types = render_defined_types(&idl.program.defined_types);
    let accounts = render_accounts(&idl.program.accounts);
    let instructions = render_instructions(&idl.program.instructions);
    let account_parser = render_account_parser(&idl.program.name, &idl.program.accounts);
    let instruction_parser =
        render_instruction_parser(&idl.program.name, &idl.program.instructions);

    quote! {
        mod #program_mod_ident {
            use yellowstone_vixen_parser::prelude::*;

            const ID: Pubkey = pubkey!(#program_public_key);

            #defined_types
            #accounts
            #instructions
            #account_parser
            #instruction_parser
        }
    }
}

fn render_defined_types(defined_types: &[codama_nodes::DefinedTypeNode]) -> TokenStream {
    let sections: Vec<TokenStream> = defined_types
        .iter()
        .map(|defined_type| {
            let defined_type_ident = format_ident!("{}", to_pascal_case(&defined_type.name));

            match &defined_type.r#type {
                TypeNode::Struct(struct_type) => {
                    let fields = quoted_fields(&struct_type.fields);
                    quote! {
                        #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                        pub struct #defined_type_ident {
                            #(#fields,)*
                        }
                    }
                },
                TypeNode::Enum(_) => {
                    let ty = quoted_type_node(&defined_type.r#type);
                    quote! {
                        #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                        pub enum #defined_type_ident #ty
                    }
                },
                _ => {
                    let ty = quoted_type_node(&defined_type.r#type);
                    quote! {
                        pub type #defined_type_ident = #ty;
                    }
                },
            }
        })
        .collect();

    quote! { #(#sections)* }
}

fn render_accounts(accounts: &[codama_nodes::AccountNode]) -> TokenStream {
    let sections: Vec<TokenStream> = accounts
        .iter()
        .map(|account| {
            let struct_ident = format_ident!("{}", to_pascal_case(&account.name));
            let fields = match &account.data {
                codama_nodes::NestedTypeNode::Value(struct_type) => {
                    quoted_fields(&struct_type.fields)
                },
                _ => vec![],
            };
            let len_const = account
                .size
                .map(|size| quote! { pub const LEN: usize = #size; });

            quote! {
                #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                pub struct #struct_ident {
                    #(#fields,)*
                }

                impl #struct_ident {
                    #len_const

                    #[inline(always)]
                    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
                        let mut data = data;
                        Self::deserialize(&mut data)
                    }
                }
            }
        })
        .collect();

    quote! { #(#sections)* }
}

fn render_instructions(instructions: &[codama_nodes::InstructionNode]) -> TokenStream {
    let sections: Vec<TokenStream> = instructions
        .iter()
        .map(|instruction| {
            let instruction_name_accounts_ident =
                format_ident!("{}Accounts", to_pascal_case(&instruction.name));
            let instruction_name_args_ident =
                format_ident!("{}Args", to_pascal_case(&instruction.name));

            let instruction_accounts_fields: Vec<TokenStream> = instruction
                .accounts
                .iter()
                .map(|account| {
                    let field_ident = format_ident!("{}", to_snake_case(&account.name));
                    quote! { pub #field_ident: KeyBytes<32> }
                })
                .collect();

            let instruction_args_fields: Vec<TokenStream> = instruction
                .arguments
                .iter()
                .filter_map(|argument| {
                    if let Some(DefaultValueStrategy::Omitted) = argument.default_value_strategy {
                        return None;
                    }

                    let field_name_ident = format_ident!("{}", to_snake_case(&argument.name));
                    let field_value_stream = quoted_type_node(&argument.r#type);

                    Some(quote! { pub #field_name_ident: #field_value_stream })
                })
                .collect();

            quote! {
                #[derive(Clone, Debug, Eq, PartialEq)]
                pub struct #instruction_name_accounts_ident {
                    #(#instruction_accounts_fields,)*
                }

                #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                pub struct #instruction_name_args_ident {
                    #(#instruction_args_fields,)*
                }

                impl #instruction_name_args_ident {
                    #[inline(always)]
                    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
                        let mut data = data;
                        Self::deserialize(&mut data)
                    }
                }
            }
        })
        .collect();

    quote! { #(#sections)* }
}

fn render_account_parser(
    program_name: &CamelCaseString,
    accounts: &[codama_nodes::AccountNode],
) -> TokenStream {
    let pascal_program_name = to_pascal_case(program_name);
    let account_state_ident = format_ident!("{}Account", pascal_program_name);
    let parser_id = format!("{}::AccountParser", pascal_program_name);
    let parser_error_msg = format!("Unknown account for program {}", **program_name);

    let account_enum_fields: Vec<TokenStream> = accounts
        .iter()
        .map(|account| {
            let account_ident = format_ident!("{}", to_pascal_case(&account.name));
            quote! { #account_ident(#account_ident) }
        })
        .collect();

    let account_matches: Vec<TokenStream> = accounts
        .iter()
        .filter_map(|account| {
            let discrimintator = match account.discriminators.first() {
                Some(d) => d,
                None => return None,
            };

            let account_ident = format_ident!("{}", to_pascal_case(&account.name));

            match discrimintator {
                DiscriminatorNode::Constant(node) => {
                    let offset = node.offset;
                    let value = match node.constant.value.as_ref() {
                        ValueNode::Number(node) => match node.number {
                            Number::UnsignedInteger(integer) => integer,
                            _ => {
                                return None;
                            }
                        },
                        _ => {
                            return None;
                        }
                    };

                    Some(quote! {
                        if let Some(discriminator) = data.get(#offset) {
                            if discriminator == #value {
                                return Ok(#account_state_ident::#account_ident(#account_ident::from_bytes(&data)?));
                            }
                        }
                    })
                }
                DiscriminatorNode::Field(node) => {
                    let offset = node.offset;
                    if let NestedTypeNode::Value(struct_node) = &account.data {
                        struct_node
                            .fields
                            .iter()
                            .find(|f| f.name == node.name)
                            .and_then(|field| {
                                let size = match &field.r#type {
                                    TypeNode::FixedSize(fixed_size_node) => fixed_size_node.size,
                                    _ => return None,
                                };
                                field.default_value.as_ref().and_then(|value| match value {
                                    ValueNode::Bytes(bytes) => {
                                        let discriminator = match bytes.encoding {
                                            codama_nodes::BytesEncoding::Base16 => {
                                                hex::decode(&bytes.data)
                                                    .expect("Failed to decode base16 (hex) bytes")
                                            }
                                            codama_nodes::BytesEncoding::Base58 => {
                                                bs58::decode(&bytes.data)
                                                    .into_vec()
                                                    .expect("Failed to decode base58 bytes")
                                            }
                                            codama_nodes::BytesEncoding::Base64 => STANDARD
                                                .decode(&bytes.data)
                                                .expect("Failed to decode base64 bytes"),
                                            codama_nodes::BytesEncoding::Utf8 => {
                                                bytes.data.as_bytes().to_vec()
                                            }
                                        };

                                        let end = offset + size;

                                        Some(quote! {
                                            if let Some(slice) = data.get(#offset..#end) {
                                                if slice == &[#(#discriminator),*] {
                                                    return Ok(#account_state_ident::#account_ident(#account_ident::from_bytes(&data[#end..])?));
                                                }
                                            }
                                        })
                                    }
                                    ValueNode::Constant(_constant) => None,
                                    _ => None,
                                })
                            })
                    } else {
                        None
                    }
                }
                DiscriminatorNode::Size(node) => {
                    let size = node.size;

                    Some(quote! {
                        if data.len() == #size {
                            return Ok(#account_state_ident::#account_ident(#account_ident::from_bytes(&data)?));
                        }
                    })
                }
            }
        })
        .collect();

    quote! {
        #[derive(Debug)]
        pub enum #account_state_ident {
            #(#account_enum_fields,)*
        }

        impl #account_state_ident {
            pub fn try_unpack(data: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
                #(#account_matches)*

                Err(yellowstone_vixen_core::ParseError::from(#parser_error_msg.to_owned()))
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct AccountParser;

        impl yellowstone_vixen_core::Parser for AccountParser {
            type Input = yellowstone_vixen_core::AccountUpdate;
            type Output = #account_state_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #parser_id.into()
            }

            fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
                yellowstone_vixen_core::Prefilter::builder()
                    .account_owners([ID])
                    .build()
                    .unwrap()
            }

            async fn parse(
                &self,
                acct: &yellowstone_vixen_core::AccountUpdate,
            ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
                let inner = acct
                    .account
                    .as_ref()
                    .ok_or(solana_program_error::ProgramError::InvalidArgument)?;

                #account_state_ident::try_unpack(&inner.data)
            }
        }
    }
}

fn render_instruction_parser(
    program_name: &CamelCaseString,
    instructions: &[codama_nodes::InstructionNode],
) -> TokenStream {
    let pascal_program_name = to_pascal_case(program_name);
    let program_instruction_ident = format_ident!("{}Instruction", pascal_program_name);
    let program_instruction_id = format!("{}::InstructionParser", pascal_program_name);

    let instruction_enum_fields: Vec<TokenStream> = instructions
        .iter()
        .map(|instruction| {
            let instruction_ident = format_ident!("{}", to_pascal_case(&instruction.name));
            let instruction_accounts_ident = format_ident!("{}Accounts", instruction_ident);
            let instruction_args_ident = format_ident!("{}Args", instruction_ident);

            quote! { #instruction_ident(#instruction_accounts_ident, #instruction_args_ident) }
        })
        .collect();

    let instruction_matches: Vec<TokenStream> = instructions
        .iter()
        .filter_map(|instruction| {
            let discrimintator = match instruction.discriminators.first() {
                Some(d) => d,
                None => return None,
            };

            let instruction_ident = format_ident!("{}", to_pascal_case(&instruction.name));
            let instruction_args_ident = format_ident!("{}Args", instruction_ident);
            let instruction_accounts_ident = format_ident!("{}Accounts", instruction_ident);

            let instruction_accounts_fields: Vec<TokenStream> = instruction
                .accounts
                .iter()
                .enumerate()
                .map(|(index, account)| {
                    let field = format_ident!("{}", to_snake_case(&account.name));
                    quote! { #field: accounts[#index] }
                })
                .collect();

            let instruction_accounts = quote! {
                #instruction_accounts_ident {
                    #(#instruction_accounts_fields,)*
                }
            };

            match discrimintator {
                DiscriminatorNode::Constant(node) => {
                    let offset = node.offset;
                    let value = match node.constant.value.as_ref() {
                        ValueNode::Number(node) => match node.number {
                            Number::UnsignedInteger(integer) => integer,
                            _ => {
                                return None;
                            }
                        },
                        _ => {
                            return None;
                        }
                    };

                    Some(quote! {
                        if let Some(discriminator) = data.get(#offset) {
                            if discriminator == #value {
                                return Ok(#program_instruction_ident::#instruction_ident(#instruction_accounts, #instruction_args_ident::from_bytes(&data)?));
                            }
                        }
                    })
                }
                DiscriminatorNode::Field(node) => {
                    let offset = node.offset;
                    instruction.arguments
                        .iter()
                        .find(|f| f.name == node.name)
                        .and_then(|field| {
                            let size = match &field.r#type {
                                TypeNode::FixedSize(fixed_size_node) => fixed_size_node.size,
                                _ => return None,
                            };
                            field.default_value.as_ref().and_then(|value| match value {
                                InstructionInputValueNode::Bytes(bytes) => {
                                    let discriminator = match bytes.encoding {
                                        codama_nodes::BytesEncoding::Base16 => {
                                            hex::decode(&bytes.data)
                                                .expect("Failed to decode base16 (hex) bytes")
                                        }
                                        codama_nodes::BytesEncoding::Base58 => {
                                            bs58::decode(&bytes.data)
                                                .into_vec()
                                                .expect("Failed to decode base58 bytes")
                                        }
                                        codama_nodes::BytesEncoding::Base64 => STANDARD
                                            .decode(&bytes.data)
                                            .expect("Failed to decode base64 bytes"),
                                        codama_nodes::BytesEncoding::Utf8 => {
                                            bytes.data.as_bytes().to_vec()
                                        }
                                    };
                                    let end = offset + size;

                                    Some(quote! {
                                        if let Some(slice) = data.get(#offset..#end) {
                                            if slice == &[#(#discriminator),*] {
                                                return Ok(#program_instruction_ident::#instruction_ident(#instruction_accounts, #instruction_args_ident::from_bytes(&data[#end..])?));
                                            }
                                        }
                                    })
                                }
                                InstructionInputValueNode::Constant(_constant) => None,
                                _ => None,
                            })
                        })
                }
                DiscriminatorNode::Size(node) => {
                    let size = node.size;

                    Some(quote! {
                        if data.len() == #size {
                            return Ok(#program_instruction_ident::#instruction_ident(#instruction_accounts, #instruction_args_ident::from_bytes(&data)?));
                        }
                    })
                }
            }
        })
        .collect();

    quote! {
        #[derive(Debug)]
        pub enum #program_instruction_ident {
            #(#instruction_enum_fields,)*
        }

        #[derive(Debug, Copy, Clone)]
        pub struct InstructionParser;

        impl yellowstone_vixen_core::Parser for InstructionParser {
            type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
            type Output = #program_instruction_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #program_instruction_id.into()
            }

            fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
                yellowstone_vixen_core::Prefilter::builder()
                    .transaction_accounts([ID])
                    .build()
                    .unwrap()
            }

            async fn parse(
                &self,
                ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
            ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
                let data = &ix_update.data;
                let accounts = &ix_update.accounts;

                #(#instruction_matches)*

                Err(yellowstone_vixen_core::ParseError::from(
                    "Invalid Instruction discriminator".to_owned(),
                ))
            }
        }
    }
}

fn quoted_type_node(type_node: &codama_nodes::TypeNode) -> TokenStream {
    use TypeNode::*;
    match type_node {
        String(_) => quote! { String },
        SizePrefix(_prefix) => {
            quote! { String }
        },
        Number(num) => quoted_number_type(num),
        PublicKey(_) => quote! { Pubkey },
        Boolean(_) => quote! { bool },
        Option(option_node) => {
            let inner_ty = quoted_type_node(&option_node.item);
            quote! { Option<#inner_ty> }
        },
        Tuple(node) => quoted_tuple_type(node),
        Enum(node) => quoted_enum_type(node),
        Link(node) => {
            let ident = format_ident!("{}", to_pascal_case(&node.name));
            quote! { #ident }
        },
        Array(node) => quoted_array_type(node),
        Map(node) => {
            let k = quoted_type_node(&node.key);
            let v = quoted_type_node(&node.value);
            quote! { HashMap<#k, #v> }
        },
        Set(node) => {
            let ty = quoted_type_node(&node.item);
            quote! { HashSet<#ty> }
        },
        Bytes(_) => {
            quote! { Vec<u8> }
        },
        FixedSize(node) => quoted_type_node(&node.r#type),
        RemainderOption(_) | ZeroableOption(_) => {
            quote! { Option<()> }
        },
        Struct(struct_type) => {
            let fields = quoted_fields(&struct_type.fields);
            quote! {
                {
                    #(#fields,)*
                }
            }
        },
        _ => todo!("Implement node type: {:?}", type_node),
    }
}

fn quoted_number_type(num: &codama_nodes::NumberTypeNode) -> TokenStream {
    match num.format {
        NumberFormat::U8 => quote! { u8 },
        NumberFormat::U16 => quote! { u16 },
        NumberFormat::U32 => quote! { u32 },
        NumberFormat::U64 => quote! { u64 },
        NumberFormat::U128 => quote! { u128 },
        NumberFormat::I8 => quote! { i8 },
        NumberFormat::I16 => quote! { i16 },
        NumberFormat::I32 => quote! { i32 },
        NumberFormat::I64 => quote! { i64 },
        NumberFormat::I128 => quote! { i128 },
        NumberFormat::F32 => quote! { f32 },
        NumberFormat::F64 => quote! { f64 },
        NumberFormat::ShortU16 => quote! { u16 },
    }
}

fn quoted_tuple_type(node: &codama_nodes::TupleTypeNode) -> TokenStream {
    let streams: Vec<_> = node.items.iter().map(quoted_type_node).collect();
    quote! { ( #(#streams),* ) }
}

fn quoted_enum_type(node: &codama_nodes::EnumTypeNode) -> TokenStream {
    let variants = node.variants.iter().map(|variant| match variant {
        EnumVariantTypeNode::Empty(v) => {
            let ident = format_ident!("{}", to_pascal_case(&v.name));
            quote! { #ident, }
        },
        EnumVariantTypeNode::Tuple(v) => {
            let ident = format_ident!("{}", to_pascal_case(&v.name));
            if let codama_nodes::NestedTypeNode::Value(tuple_type) = &v.tuple {
                let inner_types = tuple_type.items.iter().map(quoted_type_node);
                quote! { #ident( #(#inner_types),* ), }
            } else {
                panic!("Expected TupleTypeNode::Value in EnumTupleVariantTypeNode.tuple");
            }
        },
        EnumVariantTypeNode::Struct(v) => {
            let ident = format_ident!("{}", to_pascal_case(&v.name));
            if let codama_nodes::NestedTypeNode::Value(struct_type) = &v.r#struct {
                let fields = struct_type.fields.iter().map(|f| {
                    let field_ident = format_ident!("{}", to_snake_case(&f.name));
                    let ty = quoted_type_node(&f.r#type);
                    quote! { #field_ident: #ty }
                });
                quote! { #ident { #(#fields),* }, }
            } else {
                panic!("Expected StructTypeNode::Value in EnumStructVariantTypeNode.r#struct");
            }
        },
    });
    quote! { #(#variants)* }
}

fn quoted_array_type(node: &codama_nodes::ArrayTypeNode) -> TokenStream {
    let ty = quoted_type_node(&node.item);
    match &node.count {
        codama_nodes::CountNode::Fixed(fixed) => {
            let n = fixed.value;
            quote! { [#ty; #n] }
        },
        codama_nodes::CountNode::Remainder(_) => {
            quote! { Vec<#ty> }
        },
        codama_nodes::CountNode::Prefixed(prefix) => {
            let number_type = prefix.prefix.get_nested_type_node();
            match number_type.format {
                NumberFormat::U8
                | NumberFormat::U16
                | NumberFormat::U32
                | NumberFormat::U64
                | NumberFormat::ShortU16 => {
                    quote! { Vec<#ty> }
                },
                _ => {
                    quote! { Vec<#ty> }
                },
            }
        },
    }
}

fn quoted_fields(fields: &[codama_nodes::StructFieldTypeNode]) -> Vec<TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            if let Some(DefaultValueStrategy::Omitted) = field.default_value_strategy {
                return None;
            }

            let name = format_ident!("{}", to_snake_case(&field.name));
            let ty = quoted_type_node(&field.r#type);

            Some(quote! { pub #name: #ty })
        })
        .collect()
}

fn to_snake_case(s: &CamelCaseString) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i != 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn to_pascal_case(s: &CamelCaseString) -> String {
    let s = s.as_str();
    if let Some(first) = s.chars().next() {
        let mut result = String::with_capacity(s.len());
        result.push(first.to_ascii_uppercase());
        result.push_str(&s[1..]);
        result
    } else {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use codama_nodes::{
        AccountNode, NumberFormat::U64, NumberTypeNode, OptionTypeNode, PublicKeyTypeNode,
        RootNode, StringTypeNode, StructFieldTypeNode, StructTypeNode, U8,
    };

    use super::*;

    #[test]
    fn test_generate_vixen_parser() {
        let root = RootNode::new(codama_nodes::ProgramNode {
            name: "Test".into(),
            accounts: vec![AccountNode::new(
                "myAccount",
                StructTypeNode::new(vec![
                    StructFieldTypeNode::new("name", StringTypeNode::utf8()),
                    StructFieldTypeNode::new("age", NumberTypeNode::le(U8)),
                    StructFieldTypeNode::new("level", OptionTypeNode::new(NumberTypeNode::le(U64))),
                    StructFieldTypeNode::new("teammate", PublicKeyTypeNode::new()),
                ]),
            )],
            ..Default::default()
        });

        let tokens = render_vixen_parser(&root);

        let file: syn::File = syn::parse2(tokens).expect("Generated code should be valid Rust");
        let actual = prettyplease::unparse(&file);

        let expected_tokens: proc_macro2::TokenStream = quote::quote! {
            mod test {
                use yellowstone_vixen_parser::prelude::*;

                const ID: Pubkey = pubkey!("");
                #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                pub struct MyAccount {
                    pub name: String,
                    pub age: u8,
                    pub level: Option<u64>,
                    pub teammate: Pubkey,
                }
                impl MyAccount {
                    #[inline(always)]
                    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
                        let mut data = data;
                        Self::deserialize(&mut data)
                    }
                }
                #[derive(Debug)]
                pub enum TestAccount {
                    MyAccount(MyAccount),
                }
                impl TestAccount {
                    pub fn try_unpack(data: &[u8]) -> yellowstone_vixen_core::ParseResult<Self> {
                        Err(
                            yellowstone_vixen_core::ParseError::from(
                                "Unknown account for program test".to_owned(),
                            ),
                        )
                    }
                }
                #[derive(Debug, Copy, Clone)]
                pub struct AccountParser;
                impl yellowstone_vixen_core::Parser for AccountParser {
                    type Input = yellowstone_vixen_core::AccountUpdate;
                    type Output = TestAccount;
                    fn id(&self) -> std::borrow::Cow<'static, str> {
                        "Test::AccountParser".into()
                    }
                    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
                        yellowstone_vixen_core::Prefilter::builder()
                            .account_owners([ID])
                            .build()
                            .unwrap()
                    }
                    async fn parse(
                        &self,
                        acct: &yellowstone_vixen_core::AccountUpdate,
                    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
                        let inner = acct
                            .account
                            .as_ref()
                            .ok_or(solana_program_error::ProgramError::InvalidArgument)?;
                        TestAccount::try_unpack(&inner.data)
                    }
                }
                #[derive(Debug)]
                pub enum TestInstruction {}
                #[derive(Debug, Copy, Clone)]
                pub struct InstructionParser;
                impl yellowstone_vixen_core::Parser for InstructionParser {
                    type Input = yellowstone_vixen_core::instruction::InstructionUpdate;
                    type Output = TestInstruction;
                    fn id(&self) -> std::borrow::Cow<'static, str> {
                        "Test::InstructionParser".into()
                    }
                    fn prefilter(&self) -> yellowstone_vixen_core::Prefilter {
                        yellowstone_vixen_core::Prefilter::builder()
                            .transaction_accounts([ID])
                            .build()
                            .unwrap()
                    }
                    async fn parse(
                        &self,
                        ix_update: &yellowstone_vixen_core::instruction::InstructionUpdate,
                    ) -> yellowstone_vixen_core::ParseResult<Self::Output> {
                        let data = &ix_update.data;
                        let accounts = &ix_update.accounts;
                        Err(
                            yellowstone_vixen_core::ParseError::from(
                                "Invalid Instruction discriminator".to_owned(),
                            ),
                        )
                    }
                }
            }
        };

        let expected_file: syn::File = syn::parse2(expected_tokens).unwrap();
        let expected = prettyplease::unparse(&expected_file);

        assert_eq!(actual, expected);
    }
}
