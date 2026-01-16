use base64::{engine::general_purpose::STANDARD, Engine};
use codama_nodes::{
    CamelCaseString, DefaultValueStrategy, DiscriminatorNode, EnumVariantTypeNode,
    InstructionInputValueNode, NestedTypeNode, Number, NumberFormat, RootNode, TypeNode, ValueNode,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn render_vixen_parser(idl: &RootNode) -> TokenStream {
    let program_mod_ident = format_ident!("{}", to_snake_case(&idl.program.name));

    let program_pubkey = render_program_pubkey(&idl.program.public_key);
    let defined_types = render_defined_types(&idl.program.defined_types);
    let accounts = render_accounts(&idl.program.accounts);
    let instructions = render_instructions(&idl.program.instructions);
    let account_parser = render_account_parser(&idl.program.name, &idl.program.accounts);
    let instruction_parser =
        render_instruction_parser(&idl.program.name, &idl.program.instructions);

    quote! {
        pub mod #program_mod_ident {
            use borsh::{BorshDeserialize, BorshSerialize};
            use yellowstone_vixen_parser::prelude::*;

            pub const ID: yellowstone_vixen_parser::Pubkey = #program_pubkey;

            #defined_types
            #accounts
            #instructions
            #account_parser
            #instruction_parser
        }
    }
}

fn render_program_pubkey(pubkey: &String) -> TokenStream {
    let decoded = match bs58::decode(pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return quote! {
                compile_error!(concat!("Invalid base58 pubkey: ", #pubkey));
            };
        },
    };

    if decoded.len() != 32 {
        let len = decoded.len();
        return quote! {
            compile_error!(concat!(
                "Invalid pubkey length: expected 32 bytes, got ",
                stringify!(#len),
                " bytes for pubkey: ",
                #pubkey
            ));
        };
    }

    let bytes_tokens = decoded.iter().map(|&b| quote! { #b });

    quote! {
        {
            const BYTES: [u8; 32] = [#(#bytes_tokens),*];
            yellowstone_vixen_parser::prelude::KeyBytes::<32>(BYTES)
        }
    }
}

fn render_defined_types(defined_types: &[codama_nodes::DefinedTypeNode]) -> TokenStream {
    let sections = defined_types.iter().map(|defined_type| {
        let ident = format_ident!("{}", to_pascal_case(&defined_type.name));

        match &defined_type.r#type {
            TypeNode::Struct(struct_type) => {
                let fields = quoted_fields(&struct_type.fields);
                quote! {
                    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                    pub struct #ident {
                        #(#fields),*
                    }
                }
            },
            TypeNode::Enum(_) => {
                let ty = quoted_type_node(&defined_type.r#type);
                quote! {
                    #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                    pub enum #ident { #ty }
                }
            },
            _ => {
                let ty = quoted_type_node(&defined_type.r#type);
                quote! {
                    pub type #ident = #ty;
                }
            },
        }
    });

    quote! { #(#sections)* }
}

fn render_accounts(accounts: &[codama_nodes::AccountNode]) -> TokenStream {
    let sections = accounts.iter().map(|account| {
        let struct_ident = format_ident!("{}", to_pascal_case(&account.name));
        let fields = match &account.data {
            codama_nodes::NestedTypeNode::Value(struct_type) => quoted_fields(&struct_type.fields),
            _ => Vec::new(),
        };
        let len_const = account
            .size
            .map(|size| quote! { pub const LEN: usize = #size; });

        quote! {
            #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
            pub struct #struct_ident {
                #(#fields),*
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
    });

    quote! { #(#sections)* }
}

fn render_instructions(instructions: &[codama_nodes::InstructionNode]) -> TokenStream {
    let sections = instructions.iter().map(|instruction| {
        let accounts_ident = format_ident!("{}Accounts", to_pascal_case(&instruction.name));
        let args_ident = format_ident!("{}Args", to_pascal_case(&instruction.name));

        let accounts_fields = instruction.accounts.iter().map(|account| {
            let field_ident = format_ident!("{}", to_snake_case(&account.name));
            quote! { pub #field_ident: yellowstone_vixen_parser::Pubkey }
        });

        let args_fields = instruction.arguments.iter().filter_map(|argument| {
            if argument.default_value_strategy == Some(DefaultValueStrategy::Omitted) {
                return None;
            }
            let name_ident = format_ident!("{}", to_snake_case(&argument.name));
            let ty = quoted_type_node(&argument.r#type);

            Some(quote! { pub #name_ident: #ty })
        });

        quote! {
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct #accounts_ident {
                #(#accounts_fields),*
            }

            #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
            pub struct #args_ident {
                #(#args_fields),*
            }

            impl #args_ident {
                #[inline(always)]
                pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
                    let mut data = data;
                    Self::deserialize(&mut data)
                }
            }
        }
    });

    quote! { #(#sections)* }
}

fn render_account_parser(
    program_name: &CamelCaseString,
    accounts: &[codama_nodes::AccountNode],
) -> TokenStream {
    let pascal_program_name = to_pascal_case(program_name);
    let account_enum_ident = format_ident!("{}Account", pascal_program_name);
    let parser_id = format!("{}::AccountParser", pascal_program_name);
    let parser_error_msg = format!("Unknown account for program {}", **program_name);

    let account_enum_fields = accounts.iter().map(|account| {
        let account_ident = format_ident!("{}", to_pascal_case(&account.name));
        quote! { #account_ident(#account_ident) }
    });

    let account_matches = accounts.iter().filter_map(|account| {
        let discriminator = account.discriminators.first()?;

        let account_ident = format_ident!("{}", to_pascal_case(&account.name));

        Some(match discriminator {
            DiscriminatorNode::Constant(node) => {
                let offset = node.offset;
                let value = match node.constant.value.as_ref() {
                    ValueNode::Number(nn) => match nn.number {
                        Number::UnsignedInteger(integer) => integer,
                        _ => return None,
                    },
                    _ => return None,
                };

                quote! {
                    if let Some(discriminator) = data.get(#offset) {
                        if discriminator == #value {
                            return Ok(#account_enum_ident::#account_ident(#account_ident::from_bytes(&data)?));
                        }
                    }
                }
            }
            DiscriminatorNode::Field(node) => {
                let offset = node.offset;
                if let NestedTypeNode::Value(struct_node) = &account.data {
                    match struct_node
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
                                                return Ok(#account_enum_ident::#account_ident(#account_ident::from_bytes(&data[#end..])?));
                                            }
                                        }
                                    })
                                }
                                _ => None,
                            })
                        }) {
                        Some(token_stream) => token_stream,
                        None => return None,
                    }
                } else {
                    return None;
                }
            }
            DiscriminatorNode::Size(node) => {
                let size = node.size;
                quote! {
                    if data.len() == #size {
                        return Ok(#account_enum_ident::#account_ident(#account_ident::from_bytes(&data)?));
                    }
                }
            }
        })
    });

    quote! {
        #[derive(Debug)]
        pub enum #account_enum_ident {
            #(#account_enum_fields),*
        }

        impl #account_enum_ident {
            pub fn try_unpack(data: &[u8]) -> ParseResult<Self> {
                #(#account_matches)*

                Err(ParseError::from(#parser_error_msg.to_owned()))
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct AccountParser;

        impl Parser for AccountParser {
            type Input = AccountUpdate;
            type Output = #account_enum_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #parser_id.into()
            }

            fn prefilter(&self) -> Prefilter {
                Prefilter::builder()
                    .account_owners([ID])
                    .build()
                    .unwrap()
            }

            async fn parse(
                &self,
                acct: &AccountUpdate,
            ) -> ParseResult<Self::Output> {
                let inner = acct
                    .account
                    .as_ref()
                    .ok_or_else(|| ParseError::from("Unable to unwrap account ref".to_owned()))?;

                #account_enum_ident::try_unpack(&inner.data)
            }
        }
    }
}

fn render_instruction_parser(
    program_name: &CamelCaseString,
    instructions: &[codama_nodes::InstructionNode],
) -> TokenStream {
    let pascal_program_name = to_pascal_case(program_name);
    let instruction_enum_ident = format_ident!("{}Instruction", pascal_program_name);
    let instruction_parser_id = format!("{}::InstructionParser", pascal_program_name);

    let instruction_enum_fields = instructions.iter().map(|instruction| {
        let instr_ident = format_ident!("{}", to_pascal_case(&instruction.name));
        let instr_accounts_ident = format_ident!("{}Accounts", instr_ident);
        let instr_args_ident = format_ident!("{}Args", instr_ident);
        quote! {
            #instr_ident {
                accounts: #instr_accounts_ident,
                args: #instr_args_ident,
            }
        }
    });

    let instruction_matches = instructions.iter().filter_map(|instruction| {
        let discriminator = instruction.discriminators.first()?;
        let instr_ident = format_ident!("{}", to_pascal_case(&instruction.name));
        let instr_args_ident = format_ident!("{}Args", instr_ident);
        let instr_accounts_ident = format_ident!("{}Accounts", instr_ident);

        let instr_accounts_fields =
            instruction
                .accounts
                .iter()
                .enumerate()
                .map(|(idx, account)| {
                    let field = format_ident!("{}", to_snake_case(&account.name));
                    let error_msg = format!("Account does not exist at index {idx}");

                    quote! { #field: *accounts.get(#idx).ok_or(ParseError::from(#error_msg))? }
                });

        let instr_accounts = quote! {
            #instr_accounts_ident {
                #(#instr_accounts_fields),*
            }
        };

        Some(match discriminator {
            DiscriminatorNode::Constant(node) => {
                let offset = node.offset;
                let value = match node.constant.value.as_ref() {
                    ValueNode::Number(nn) => match nn.number {
                        Number::UnsignedInteger(integer) => integer,
                        _ => return None,
                    },
                    _ => return None,
                };

                quote! {
                    if let Some(discriminator) = data.get(#offset) {
                        if discriminator == #value {
                            return Ok(#instruction_enum_ident::#instr_ident {
                                accounts: #instr_accounts,
                                args: #instr_args_ident::from_bytes(&data)?,
                            });
                        }
                    }
                }
            },
            DiscriminatorNode::Field(node) => {
                let offset = node.offset;
                match instruction
                    .arguments
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
                                    codama_nodes::BytesEncoding::Base16 => hex::decode(&bytes.data)
                                        .expect("Failed to decode base16 (hex) bytes"),
                                    codama_nodes::BytesEncoding::Base58 => {
                                        bs58::decode(&bytes.data)
                                            .into_vec()
                                            .expect("Failed to decode base58 bytes")
                                    },
                                    codama_nodes::BytesEncoding::Base64 => STANDARD
                                        .decode(&bytes.data)
                                        .expect("Failed to decode base64 bytes"),
                                    codama_nodes::BytesEncoding::Utf8 => {
                                        bytes.data.as_bytes().to_vec()
                                    },
                                };
                                let end = offset + size;

                                Some(quote! {
                                    if let Some(slice) = data.get(#offset..#end) {
                                        if slice == &[#(#discriminator),*] {
                                            return Ok(#instruction_enum_ident::#instr_ident {
                                                accounts: #instr_accounts,
                                                args: #instr_args_ident::from_bytes(&data[#end..])?,
                                            });
                                        }
                                    }
                                })
                            },
                            _ => None,
                        })
                    }) {
                    Some(token_stream) => token_stream,
                    None => return None,
                }
            },
            DiscriminatorNode::Size(node) => {
                let size = node.size;

                quote! {
                    if data.len() == #size {
                        return Ok(#instruction_enum_ident::#instr_ident {
                            accounts: #instr_accounts,
                            args: #instr_args_ident::from_bytes(&data)?,
                        });
                    }
                }
            },
        })
    });

    quote! {
        #[derive(Debug)]
        pub enum #instruction_enum_ident {
            #(#instruction_enum_fields),*
        }

        #[derive(Debug, Copy, Clone)]
        pub struct InstructionParser;

        impl Parser for InstructionParser {
            type Input = instruction::InstructionUpdate;
            type Output = #instruction_enum_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #instruction_parser_id.into()
            }

            fn prefilter(&self) -> Prefilter {
                Prefilter::builder()
                    .transaction_accounts([ID])
                    .build()
                    .unwrap()
            }

            async fn parse(
                &self,
                ix_update: &instruction::InstructionUpdate,
            ) -> ParseResult<Self::Output> {
                let data = &ix_update.data;
                let accounts = &ix_update.accounts;

                #(#instruction_matches)*

                Err(ParseError::from(
                    "Invalid Instruction discriminator".to_owned(),
                ))
            }
        }
    }
}

fn quoted_type_node(type_node: &codama_nodes::TypeNode) -> TokenStream {
    use TypeNode::*;
    match type_node {
        String(_) | SizePrefix(_) => quote! { String },
        Number(num) => quoted_number_type(num),
        PublicKey(_) => quote! { yellowstone_vixen_parser::Pubkey },
        Boolean(_) => quote! { bool },
        Option(option_node) => {
            let item = quoted_type_node(&option_node.item);
            quote! { Option<#item> }
        },
        Tuple(tuple_node) => quoted_tuple_type(tuple_node),
        Enum(enum_node) => quoted_enum_type(enum_node),
        Link(node) => {
            let ident = format_ident!("{}", to_pascal_case(&node.name));
            quote! { #ident }
        },
        Array(array_node) => quoted_array_type(array_node),
        Map(map_node) => {
            let k = quoted_type_node(&map_node.key);
            let v = quoted_type_node(&map_node.value);
            quote! { std::collections::HashMap<#k, #v> }
        },
        Set(set_node) => {
            let ty = quoted_type_node(&set_node.item);
            quote! { std::collections::HashSet<#ty> }
        },
        Bytes(_) => quote! { Vec<u8> },
        FixedSize(node) => {
            let size = node.size;

            match *node.r#type {
                Bytes(_) | String(_) => quote! { [u8; #size] },
                _ => {
                    todo!("Implement fixed node size: {:?}", node)
                },
            }
        },
        RemainderOption(_) | ZeroableOption(_) => quote! { Option<()> },
        Struct(struct_type) => {
            let fields = quoted_fields(&struct_type.fields);
            quote! {
                {
                    #(#fields),*
                }
            }
        },
        _ => todo!("Implement node type: {:?}", type_node),
    }
}

fn quoted_number_type(num: &codama_nodes::NumberTypeNode) -> TokenStream {
    use NumberFormat::*;
    match num.format {
        U8 => quote! { u8 },
        U16 => quote! { u16 },
        U32 => quote! { u32 },
        U64 => quote! { u64 },
        U128 => quote! { u128 },
        I8 => quote! { i8 },
        I16 => quote! { i16 },
        I32 => quote! { i32 },
        I64 => quote! { i64 },
        I128 => quote! { i128 },
        F32 => quote! { f32 },
        F64 => quote! { f64 },
        ShortU16 => quote! { u16 },
    }
}

fn quoted_tuple_type(node: &codama_nodes::TupleTypeNode) -> TokenStream {
    let types = node.items.iter().map(quoted_type_node);
    quote! { ( #(#types),* ) }
}

fn quoted_enum_type(node: &codama_nodes::EnumTypeNode) -> TokenStream {
    let variants = node.variants.iter().map(|variant| match variant {
        EnumVariantTypeNode::Empty(v) => {
            let ident = format_ident!("{}", to_pascal_case(&v.name));
            quote! { #ident, }
        },
        EnumVariantTypeNode::Tuple(v) => {
            let ident = format_ident!("{}", to_pascal_case(&v.name));
            match &v.tuple {
                codama_nodes::NestedTypeNode::Value(tuple_type) => {
                    let inner_types = tuple_type.items.iter().map(quoted_type_node);

                    quote! { #ident( #(#inner_types),* ), }
                },
                _ => panic!("Expected TupleTypeNode::Value in EnumTupleVariantTypeNode.tuple"),
            }
        },
        EnumVariantTypeNode::Struct(v) => {
            let ident = format_ident!("{}", to_pascal_case(&v.name));
            match &v.r#struct {
                codama_nodes::NestedTypeNode::Value(struct_type) => {
                    let fields = struct_type.fields.iter().map(|f| {
                        let field_ident = format_ident!("{}", to_snake_case(&f.name));
                        let ty = quoted_type_node(&f.r#type);
                        quote! { #field_ident: #ty }
                    });
                    quote! { #ident { #(#fields),* }, }
                },
                _ => panic!("Expected StructTypeNode::Value in EnumStructVariantTypeNode.r#struct"),
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
        codama_nodes::CountNode::Remainder(_) | codama_nodes::CountNode::Prefixed(_) => {
            quote! { Vec<#ty> }
        },
    }
}

fn quoted_fields(fields: &[codama_nodes::StructFieldTypeNode]) -> Vec<TokenStream> {
    fields
        .iter()
        .filter_map(|field| {
            if field.default_value_strategy == Some(DefaultValueStrategy::Omitted) {
                None
            } else {
                let name = format_ident!("{}", to_snake_case(&field.name));
                let ty = quoted_type_node(&field.r#type);
                Some(quote! { pub #name: #ty })
            }
        })
        .collect()
}

fn to_snake_case(s: &CamelCaseString) -> String {
    let mut out = String::with_capacity(s.len());
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
    if let Some((first, rest)) = s.chars().next().map(|first| (first, &s[1..])) {
        let mut result = String::with_capacity(s.len());
        result.push(first.to_ascii_uppercase());
        result.push_str(rest);
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
                use borsh::{BorshDeserialize, BorshSerialize};
                use yellowstone_vixen_parser::prelude::*;

                const ID: yellowstone_vixen_parser::Pubkey = {
                    const BYTES: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                    yellowstone_vixen_parser::Pubkey::from(BYTES)
                };

                #[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
                pub struct MyAccount {
                    pub name: String,
                    pub age: u8,
                    pub level: Option<u64>,
                    pub teammate: yellowstone_vixen_parser::Pubkey,
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
                    pub fn try_unpack(data: &[u8]) -> ParseResult<Self> {
                        Err(
                            ParseError::from(
                                "Unknown account for program test".to_owned(),
                            ),
                        )
                    }
                }
                #[derive(Debug, Copy, Clone)]
                pub struct AccountParser;
                impl Parser for AccountParser {
                    type Input = AccountUpdate;
                    type Output = TestAccount;
                    fn id(&self) -> std::borrow::Cow<'static, str> {
                        "Test::AccountParser".into()
                    }
                    fn prefilter(&self) -> Prefilter {
                        Prefilter::builder()
                            .account_owners([ID])
                            .build()
                            .unwrap()
                    }
                    async fn parse(
                        &self,
                        acct: &AccountUpdate,
                    ) -> ParseResult<Self::Output> {
                        let inner = acct
                            .account
                            .as_ref()
                            .ok_or(ParseError::from("Unable to unwrap account".to_into()))?;
                        TestAccount::try_unpack(&inner.data)
                    }
                }
                #[derive(Debug)]
                pub enum TestInstruction {}
                #[derive(Debug, Copy, Clone)]
                pub struct InstructionParser;
                impl Parser for InstructionParser {
                    type Input = instruction::InstructionUpdate;
                    type Output = TestInstruction;
                    fn id(&self) -> std::borrow::Cow<'static, str> {
                        "Test::InstructionParser".into()
                    }
                    fn prefilter(&self) -> Prefilter {
                        Prefilter::builder()
                            .transaction_accounts([ID])
                            .build()
                            .unwrap()
                    }
                    async fn parse(
                        &self,
                        ix_update: &instruction::InstructionUpdate,
                    ) -> ParseResult<Self::Output> {
                        let data = &ix_update.data;
                        let accounts = &ix_update.accounts;
                        Err(
                            ParseError::from(
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
