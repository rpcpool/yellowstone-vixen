use base64::{engine::general_purpose::STANDARD, Engine};
use codama_nodes::{
    CamelCaseString, DiscriminatorNode, NestedTypeNode, Number, TypeNode, ValueNode,
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

/// 
/// Build the *account parser* for a program.
///
/// Generates a prost-compatible wrapper struct with a oneof field:
///
/// ```rust, ignore
/// #[derive(Clone, PartialEq, ::prost::Message)]
/// pub struct {ProgramName}Account {
///     #[prost(oneof = "{program_name}_account::Kind", tags = "1, 2, 3")]
///     pub kind: Option<{program_name}_account::Kind>,
/// }
///
/// pub mod {program_name}_account {
///     #[derive(Clone, PartialEq, ::prost::Oneof)]
///     pub enum Kind {
///         #[prost(bytes, tag = 1)]
///         AccountA(Vec<u8>),
///         #[prost(bytes, tag = 2)]
///         AccountB(Vec<u8>),
///     }
/// }
/// ```
/// 
pub fn account_parser(
    program_name_camel: &CamelCaseString,
    accounts: &[codama_nodes::AccountNode],
) -> TokenStream {
    let program_name = crate::utils::to_pascal_case(program_name_camel);

    let account_struct_ident = format_ident!("{}Account", program_name);

    let account_mod_name = format!(
        "{}_account",
        crate::utils::to_snake_case(program_name_camel)
    );

    let account_mod_ident = format_ident!("{}", account_mod_name);

    // prost oneof attribute requires a string literal like "module::Kind"
    let oneof_path_lit = LitStr::new(&format!("{}::Kind", account_mod_name), Span::call_site());

    let parser_id = format!("{}::AccountParser", program_name);

    let parser_error_msg = format!("Unknown account for program {}", program_name);

    // Generate tags list string for prost oneof attribute
    let tags_lit = {
        let tags_list = (1..=accounts.len())
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        LitStr::new(&tags_list, Span::call_site())
    };

    let oneof_variants = accounts.iter().enumerate().map(|(i, account)| {
        let tag = (i + 1) as u32;
        let account_ident = format_ident!("{}", crate::utils::to_pascal_case(&account.name));

        quote! {
            #[prost(bytes, tag = #tag)]
            #account_ident(::prost::alloc::vec::Vec<u8>)
        }
    });

    let account_matches = accounts.iter().filter_map(|account| {
        let discriminator = account.discriminators.first()?;
        
        let account_ident = format_ident!("{}", crate::utils::to_pascal_case(&account.name));

        Some(match discriminator {
            // Handle 1 byte discriminators (simple programs like SPL Token)
            DiscriminatorNode::Constant(node) => {
                let offset = node.offset;

                // Skip if not a number
                let ValueNode::Number(nn) = node.constant.value.as_ref() else {
                    return None;
                };

                // Skip if not an unsigned integer
                let Number::UnsignedInteger(value) = nn.number else {
                    return None;
                };

                quote! {
                    if let Some(discriminator) = data.get(#offset) {
                        if discriminator == #value {
                            return Ok(#account_struct_ident {
                                kind: Some(#account_mod_ident::Kind::#account_ident(data.to_vec()))
                            });
                        }
                    }
                }
            },

            // Handle multi-byte discriminators (like Anchor's 8 byte discriminators)
            DiscriminatorNode::Field(node) => {
                let offset = node.offset;

                // Skip if not a struct
                let NestedTypeNode::Value(struct_node) = &account.data else {
                    return None;
                };

                // Find the discriminator field by name
                let field = struct_node.fields.iter().find(|f| f.name == node.name)?;

                // Skip if discriminator field isn't fixed-size bytes
                let TypeNode::FixedSize(fixed_size_node) = &field.r#type else {
                    return None;
                };

                let size = fixed_size_node.size;

                // Skip if no default value
                let default_value = field.default_value.as_ref()?;

                // Skip if default value isn't bytes
                let ValueNode::Bytes(bytes) = default_value else {
                    return None;
                };

                // Decode expected discriminator bytes
                let discriminator: Vec<u8> = match bytes.encoding {
                    codama_nodes::BytesEncoding::Base16 => {
                        hex::decode(&bytes.data).expect("Failed to decode base16 (hex) bytes")
                    },
                    codama_nodes::BytesEncoding::Base58 => bs58::decode(&bytes.data)
                        .into_vec()
                        .expect("Failed to decode base58 bytes"),
                    codama_nodes::BytesEncoding::Base64 => STANDARD
                        .decode(&bytes.data)
                        .expect("Failed to decode base64 bytes"),
                    codama_nodes::BytesEncoding::Utf8 => bytes.data.as_bytes().to_vec(),
                };

                let end = offset + size;

                quote! {
                    if let Some(slice) = data.get(#offset..#end) {
                        if slice == &[#(#discriminator),*] {
                            return Ok(#account_struct_ident {
                                // Remove discriminator bytes from account data before returning (#end..)
                                kind: Some(#account_mod_ident::Kind::#account_ident(data[#end..].to_vec()))
                            });
                        }
                    }
                }
            },

            // Handle accounts based on size only (e.g the account is 558 Bytes long)
            DiscriminatorNode::Size(node) => {
                let size = node.size;

                quote! {
                    if data.len() == #size {
                        return Ok(#account_struct_ident {
                            kind: Some(#account_mod_ident::Kind::#account_ident(data.to_vec()))
                        });
                    }
                }
            },
        })
    });

    quote! {
        /// Prost-compatible wrapper struct for program accounts.
        /// Uses the oneof pattern to represent account variants.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct #account_struct_ident {
            #[prost(oneof = #oneof_path_lit, tags = #tags_lit)]
            pub kind: ::core::option::Option<#account_mod_ident::Kind>,
        }

        pub mod #account_mod_ident {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Kind {
                #(#oneof_variants),*
            }
        }

        impl #account_struct_ident {
            pub fn try_unpack(data: &[u8]) -> ParseResult<Self> {
                #(#account_matches)*
                Err(ParseError::from(#parser_error_msg.to_owned()))
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct AccountParser;

        impl Parser for AccountParser {
            type Input = AccountUpdate;
            type Output = #account_struct_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #parser_id.into()
            }

            fn prefilter(&self) -> Prefilter {
                Prefilter::builder()
                    .account_owners([PROGRAM_ID])
                    .build()
                    .unwrap()
            }

            async fn parse(&self, acct: &AccountUpdate) -> ParseResult<Self::Output> {
                let inner = acct
                    .account
                    .as_ref()
                    .ok_or_else(|| ParseError::from("Unable to unwrap account ref".to_owned()))?;

                #account_struct_ident::try_unpack(&inner.data)
            }
        }

        // Implement the trait for Mock
        impl ::yellowstone_vixen_core::ProgramParser for AccountParser {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }
    }
}
