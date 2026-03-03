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
/// Generates a wrapper struct with a oneof field. When the `proto` feature is
/// enabled, prost attributes are emitted directly; otherwise plain Rust types
/// are generated.
///
/// With `proto` feature:
/// ```rust, ignore
/// #[derive(Clone, PartialEq, ::prost::Message)]
/// pub struct {ProgramName}Account {
///     #[prost(oneof = "account::Account", tags = "1, 2, 3")]
///     pub account: Option<account::Account>,
/// }
///
/// pub mod account {
///     #[derive(Clone, PartialEq, ::prost::Oneof)]
///     pub enum Account {
///         #[prost(message, tag = 1)]
///         AccountA(super::AccountA),
///         #[prost(message, tag = 2)]
///         AccountB(super::AccountB),
///     }
/// }
/// ```
///
/// Without `proto` feature:
/// ```rust, ignore
/// #[derive(Clone, Debug, PartialEq)]
/// pub struct {ProgramName}Account {
///     pub account: Option<account::Account>,
/// }
///
/// pub mod account {
///     #[derive(Clone, Debug, PartialEq)]
///     pub enum Account {
///         AccountA(super::AccountA),
///         AccountB(super::AccountB),
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

    let account_mod_ident = format_ident!("account");

    // prost oneof attribute requires a string literal like "account::Account"
    let oneof_path_lit = LitStr::new("account::Account", Span::call_site());

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

        if cfg!(feature = "proto") {
            quote! {
                #[prost(message, tag = #tag)]
                #account_ident(super::#account_ident)
            }
        } else {
            quote! {
                #account_ident(super::#account_ident)
            }
        }
    });

    let account_matches = accounts.iter().filter_map(|account| {
        let discriminator = match account.discriminators.first() {
            Some(d) => d,
            None => {
                return None;
            }
        };

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

                let account_name = account_ident.to_string();

                quote! {
                    if let Some(discriminator) = data.get(#offset) {
                        if discriminator == #value {
                            match <#account_ident as ::borsh::BorshDeserialize>::deserialize(&mut &data[..]) {
                                Ok(parsed) => {
                                    return Ok(#account_struct_ident {
                                        account: Some(#account_mod_ident::Account::#account_ident(parsed))
                                    });
                                }
                                Err(e) => {
                                    println!("[try_unpack] pubkey={}, {} deserialization FAILED: {}", pubkey_str, #account_name, e);
                                    return Err(ParseError::Other(e.into()));
                                }
                            }
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
                let field = match struct_node.fields.iter().find(|f| f.name == node.name) {
                    Some(f) => f,
                    None => {
                        return None;
                    }
                };

                // Skip if discriminator field isn't fixed-size bytes
                let TypeNode::FixedSize(fixed_size_node) = &field.r#type else {
                    return None;
                };

                let size = fixed_size_node.size;

                // Skip if no default value
                let default_value = match field.default_value.as_ref() {
                    Some(v) => v,
                    None => {
                        return None;
                    }
                };

                // Skip if default value isn't bytes
                let ValueNode::Bytes(bytes) = default_value else {
                    return None;
                };

                // Decode expected discriminator bytes
                let discriminator: Vec<u8> = match bytes.encoding {
                    codama_nodes::BytesEncoding::Base16 => {
                        let padded = crate::utils::pad_hex(&bytes.data);

                        hex::decode(&padded).expect("Failed to decode base16 (hex) bytes")
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
                let account_name = account_ident.to_string();

                quote! {
                    if let Some(slice) = data.get(#offset..#end) {
                        if slice == &[#(#discriminator),*] {
                            match <#account_ident as ::borsh::BorshDeserialize>::deserialize(&mut &data[#end..]) {
                                Ok(parsed) => {
                                    return Ok(#account_struct_ident {
                                        account: Some(#account_mod_ident::Account::#account_ident(parsed))
                                    });
                                }
                                Err(e) => {
                                    println!("[try_unpack] pubkey={}, {} deserialization FAILED: {}", pubkey_str, #account_name, e);
                                    return Err(ParseError::Other(e.into()));
                                }
                            }
                        }
                    }
                }
            },

            // Handle accounts based on size only (e.g the account is 558 Bytes long)
            DiscriminatorNode::Size(node) => {
                let size = node.size;

                let account_name = account_ident.to_string();

                quote! {
                    if data.len() == #size {
                        match <#account_ident as ::borsh::BorshDeserialize>::deserialize(&mut &data[..]) {
                            Ok(parsed) => {
                                return Ok(#account_struct_ident {
                                    account: Some(#account_mod_ident::Account::#account_ident(parsed))
                                });
                            }
                            Err(e) => {
                                println!("[try_unpack] pubkey={}, {} deserialization FAILED: {}", pubkey_str, #account_name, e);
                                return Err(ParseError::Other(e.into()));
                            }
                        }
                    }
                }
            },
        })
    });

    let struct_and_mod = if accounts.is_empty() {
        // When there are no accounts, prost cannot handle an empty `tags = ""` attribute,
        // so we emit a plain struct with no prost oneof.
        if cfg!(feature = "proto") {
            quote! {
                /// Wrapper struct for program accounts (no accounts defined).
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct #account_struct_ident {}
            }
        } else {
            quote! {
                /// Wrapper struct for program accounts (no accounts defined).
                #[derive(Clone, Debug, PartialEq)]
                pub struct #account_struct_ident {}
            }
        }
    } else if cfg!(feature = "proto") {
        quote! {
            /// Wrapper struct for program accounts.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct #account_struct_ident {
                #[prost(oneof = #oneof_path_lit, tags = #tags_lit)]
                pub account: ::core::option::Option<#account_mod_ident::Account>,
            }

            pub mod #account_mod_ident {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Account {
                    #(#oneof_variants),*
                }
            }
        }
    } else {
        quote! {
            /// Wrapper struct for program accounts.
            #[derive(Clone, Debug, PartialEq)]
            pub struct #account_struct_ident {
                pub account: ::core::option::Option<#account_mod_ident::Account>,
            }

            pub mod #account_mod_ident {
                #[derive(Clone, Debug, PartialEq)]
                pub enum Account {
                    #(#oneof_variants),*
                }
            }
        }
    };

    quote! {
        #struct_and_mod

        impl #account_struct_ident {
            pub fn try_unpack(data: &[u8]) -> ParseResult<Self> {
                Self::try_unpack_inner(data, None)
            }

            fn try_unpack_inner(data: &[u8], pubkey: Option<&[u8]>) -> ParseResult<Self> {
                let pubkey_str = pubkey
                    .filter(|p| p.len() == 32)
                    .map(|p| ::yellowstone_vixen_core::bs58::encode(p).into_string())
                    .unwrap_or_else(|| "<unknown>".to_string());

                let first_bytes: String = data.iter().take(16).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");

                #(#account_matches)*

                println!("[try_unpack] pubkey={}, no discriminator matched, returning error", pubkey_str);
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
                    .ok_or_else(|| {
                        println!("[account_parser] account ref is None!");
                        ParseError::from("Unable to unwrap account ref".to_owned())
                    })?;

                #account_struct_ident::try_unpack_inner(&inner.data, Some(&inner.pubkey))
            }
        }

        // Implement the trait for Mock
        impl ::yellowstone_vixen_core::ProgramParser for AccountParser {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::KeyBytes::<32> {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }
    }
}
