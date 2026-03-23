use base64::{engine::general_purpose::STANDARD, Engine};
use codama_nodes::{
    CamelCaseString, DiscriminatorNode, NestedTypeNode, Number, TypeNode, ValueNode,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

///
/// Build the *account parser* for a program.
///
/// Generates a wrapper struct with a non-Option oneof field and a manual
/// `prost::Message` impl when `proto` is enabled. prost derives require
/// `Option` for oneof fields, so we bypass them by implementing `Message`
/// manually.
///
/// Example output:
///
/// ```rust, ignore
/// // --- wrapper struct + enum (identical shape for proto and non-proto) ---
///
/// #[derive(Clone, PartialEq)]          // + Debug when non-proto
/// pub struct {ProgramName}Account {
///     pub account: account::Account,
/// }
///
/// pub mod account {
///     #[derive(Clone, PartialEq, ::prost::Oneof)]  // or Clone, Debug, PartialEq
///     pub enum Account {
///         AccountA(super::AccountA),
///         AccountB(super::AccountB),
///         // ...
///     }
/// }
///
/// // --- proto only: manual Debug + prost::Message impls ---
///
/// impl Debug for {ProgramName}Account { ... }
/// impl prost::Message for {ProgramName}Account { ... }
///
/// // --- try_unpack: discriminator-based deserialization ---
///
/// impl {ProgramName}Account {
///     pub fn try_unpack(data: &[u8]) -> ParseResult<Self> { ... }
/// }
///
/// // --- AccountParser + Parser impl + ProgramParser impl ---
///
/// pub struct AccountParser;
/// impl Parser for AccountParser { ... }
/// impl ProgramParser for AccountParser { ... }
/// ```
///
pub fn account_parser(
    program_name_camel: &CamelCaseString,
    accounts: &[codama_nodes::AccountNode],
) -> TokenStream {
    let program_name = crate::utils::to_pascal_case(program_name_camel);

    let account_struct_ident = format_ident!("{}Account", program_name);

    let account_mod_ident = format_ident!("account");

    let parser_id = format!("{}::AccountParser", program_name);

    let parser_error_msg = format!("Unknown account for program {}", program_name);

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
                                        account: #account_mod_ident::Account::#account_ident(parsed),
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
                                        account: #account_mod_ident::Account::#account_ident(parsed),
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

    let (struct_and_mod, proto_impls) = if accounts.is_empty() {
        let empty_struct = if cfg!(feature = "proto") {
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
        };

        (empty_struct, quote! {})
    } else {
        let enum_derive = if cfg!(feature = "proto") {
            quote! { #[derive(Clone, PartialEq, ::prost::Oneof)] }
        } else {
            quote! { #[derive(Clone, Debug, PartialEq)] }
        };

        let debug_derive = if cfg!(feature = "proto") {
            quote! {}
        } else {
            quote! { Debug, }
        };

        let s = quote! {
            /// Wrapper struct for program accounts.
            #[derive(Clone, #debug_derive PartialEq)]
            pub struct #account_struct_ident {
                pub account: #account_mod_ident::Account,
            }

            pub mod #account_mod_ident {
                #enum_derive
                pub enum Account {
                    #(#oneof_variants),*
                }
            }
        };

        let p = if cfg!(feature = "proto") {
            let account_field = format_ident!("account");
            let account_enum = format_ident!("Account");

            quote! {
                impl ::core::fmt::Debug for #account_struct_ident {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(stringify!(#account_struct_ident))
                            .field("account", &self.account)
                            .finish()
                    }
                }

                impl ::prost::Message for #account_struct_ident {
                    fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                        self.#account_field.encode(buf);
                    }

                    fn merge_field(
                        &mut self,
                        tag: u32,
                        wire_type: ::prost::encoding::WireType,
                        buf: &mut impl ::prost::bytes::Buf,
                        ctx: ::prost::encoding::DecodeContext,
                    ) -> ::core::result::Result<(), ::prost::DecodeError> {
                        // Oneof::merge() requires `&mut Option<Self>`, so we wrap our non-Option
                        // field into Some, call merge, then unwrap back.
                        let mut opt = ::core::option::Option::Some(self.#account_field.clone());

                        #account_mod_ident::#account_enum::merge(&mut opt, tag, wire_type, buf, ctx)?;

                        if let ::core::option::Option::Some(v) = opt {
                            self.#account_field = v;
                        }

                        ::core::result::Result::Ok(())
                    }

                    fn encoded_len(&self) -> usize {
                        self.#account_field.encoded_len()
                    }

                    fn clear(&mut self) {}
                }
            }
        } else {
            quote! {}
        };

        (s, p)
    };

    quote! {
        #struct_and_mod

        #proto_impls

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
            fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
                yellowstone_vixen_core::Pubkey::new(PROGRAM_ID)
            }
        }
    }
}
