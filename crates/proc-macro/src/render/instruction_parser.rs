use base64::{engine::general_purpose::STANDARD, Engine};
use codama_nodes::{
    CamelCaseString, DiscriminatorNode, InstructionInputValueNode, Number, TypeNode, ValueNode,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn single_instruction_parser(
    instruction: &codama_nodes::InstructionNode,
    wrapper_ident: syn::Ident,
) -> Option<TokenStream> {
    let discriminator = instruction.discriminators.first()?;

    let ix_name_pascal = crate::utils::to_pascal_case(&instruction.name);

    let payload_ident: syn::Ident = format_ident!("{}Ix", ix_name_pascal);
    let accounts_ident = format_ident!("{}Accounts", ix_name_pascal);
    let args_ident = format_ident!("{}Args", ix_name_pascal);

    //
    // Example:
    //
    // CreateAccounts {
    //   seller: accounts.get(0).ok_or(...)?.to_vec(),
    //   token_account: accounts.get(1).ok_or(...)?.to_vec(),
    //   mint: accounts.get(2).ok_or(...)?.to_vec(),
    // }
    //
    let accounts_value = {
        let accounts_fields = instruction
        .accounts
        .iter()
        .enumerate()
        .map(|(idx, account)| {
            let field_name = format_ident!("{}", crate::utils::to_snake_case(&account.name));
            let error_msg = format!("Account does not exist at index {idx}");

            quote! { #field_name: accounts.get(#idx).ok_or(ParseError::from(#error_msg))?.to_vec() }
        });

        quote! { #accounts_ident { #(#accounts_fields),* } }
    };

    let has_args = !instruction.arguments.is_empty();

    let build_payload = |args_expr: TokenStream| {
        quote! {
            //
            // Example:
            //
            // ProgramInstruction {
            //  ix: Some(
            //      program_instruction_oneof::Ix::Create(
            //          CreateIx {
            //              accounts: Some(CreateAccounts { ... }),
            //              args: Some(CreateArgs { ... }),
            //          }
            //      )
            //  )
            // }
            //
            return Ok(#wrapper_ident {
                ix: ::core::option::Option::Some(
                    program_instruction_oneof::Ix::#payload_ident(
                        #payload_ident {
                            accounts: ::core::option::Option::Some(#accounts_value),
                            args: #args_expr,
                        }
                    )
                ),
            });
        }
    };

    Some(match discriminator {
        // 1-byte discriminator at offset (usually offset=0)
        DiscriminatorNode::Constant(node) => {
            let offset = node.offset;

            let ValueNode::Number(nn) = node.constant.value.as_ref() else {
                return None;
            };

            let Number::UnsignedInteger(value) = nn.number else {
                return None;
            };

            let args_expr = if has_args {
                quote! {
                    ::core::option::Option::Some(
                        <#args_ident as ::borsh::BorshDeserialize>::try_from_slice(
                            data.get((#offset + 1)..).ok_or(ParseError::from("Missing args bytes"))?
                        ).map_err(|e| ParseError::Other(e.into()))?
                    )
                }
            } else {
                quote! { ::core::option::Option::None }
            };

            let payload = build_payload(args_expr);

            quote! {
                if let Some(d) = data.get(#offset) {
                    if *d == (#value as u8) {
                        #payload
                    }
                }
            }
        },

        // Anchor-style discriminator in a field (usually 8 bytes) at offset
        DiscriminatorNode::Field(node) => {
            let offset = node.offset;

            let field = instruction.arguments.iter().find(|f| f.name == node.name)?;

            // Skip if not fixed-size type
            let TypeNode::FixedSize(fixed_size_node) = &field.r#type else {
                return None;
            };

            let size = fixed_size_node.size;

            let default_value = field.default_value.as_ref()?;

            let InstructionInputValueNode::Bytes(bytes) = default_value else {
                return None;
            };

            let discriminator: Vec<u8> = match bytes.encoding {
                codama_nodes::BytesEncoding::Base16 => {
                    hex::decode(&bytes.data).expect("decode base16")
                },
                codama_nodes::BytesEncoding::Base58 => {
                    bs58::decode(&bytes.data).into_vec().expect("decode base58")
                },
                codama_nodes::BytesEncoding::Base64 => {
                    STANDARD.decode(&bytes.data).expect("decode base64")
                },
                codama_nodes::BytesEncoding::Utf8 => bytes.data.as_bytes().to_vec(),
            };

            let end = offset + size;

            let args_expr = if has_args {
                quote! {
                    ::core::option::Option::Some(
                        <#args_ident as ::borsh::BorshDeserialize>::try_from_slice(
                            data.get(#end..).ok_or(ParseError::from("Missing args bytes"))?
                        ).map_err(|e| ParseError::Other(e.into()))?
                    )
                }
            } else {
                quote! { ::core::option::Option::None }
            };

            let payload = build_payload(args_expr);

            quote! {
                if let Some(slice) = data.get(#offset..#end) {
                    if slice == &[#(#discriminator),*] {
                        #payload
                    }
                }
            }
        },

        // Discriminator by total size only
        // In this case, args parsing is ambiguous (size match usually means "this instruction is fixed layout").
        // If the IDL says there are args, we parse the full buffer as args bytes; otherwise None.
        DiscriminatorNode::Size(node) => {
            let size = node.size;

            let args_expr = if has_args {
                quote! {
                    ::core::option::Option::Some(
                        <#args_ident as ::borsh::BorshDeserialize>::try_from_slice(data)
                            .map_err(|e| ParseError::Other(e.into()))?
                    )
                }
            } else {
                quote! { ::core::option::Option::None }
            };

            let payload = build_payload(args_expr);

            quote! {
                if data.len() == #size {
                    #payload
                }
            }
        },
    })
}

pub fn instruction_parser(
    program_name_camel: &CamelCaseString,
    instructions: &[codama_nodes::InstructionNode],
) -> TokenStream {
    let program_name = crate::utils::to_pascal_case(program_name_camel);

    let instruction_parser_id = format!("{}::InstructionParser", program_name);

    let wrapper_ident = format_ident!("ProgramInstruction");

    let instruction_matches = instructions
        .iter()
        .filter_map(|instruction| single_instruction_parser(instruction, wrapper_ident.clone()));

    quote! {
        #[derive(Debug, Copy, Clone)]
        pub struct InstructionParser;

        impl Parser for InstructionParser {
            type Input = instruction::InstructionUpdate;
            type Output = #wrapper_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #instruction_parser_id.into()
            }

            fn prefilter(&self) -> Prefilter {
                Prefilter::builder()
                    .transaction_accounts([PROGRAM_ID])
                    .build()
                    .unwrap()
            }

            async fn parse(
                &self,
                ix_update: &instruction::InstructionUpdate,
            ) -> ParseResult<Self::Output> {
                // TODO: this is a fix because everything gets parsed by the proc macro
                // Check program ID first to avoid parsing unrelated instructions
                if *ix_update.program != PROGRAM_ID {
                    return Err(ParseError::Filtered);
                }

                let data = &ix_update.data;
                let accounts = &ix_update.accounts;

                #(#instruction_matches)*

                Err(ParseError::Filtered)
            }
        }

        // Implement the trait for Mock
        impl ::yellowstone_vixen_core::ProgramParser for InstructionParser {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }
    }
}
