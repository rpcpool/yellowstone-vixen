use base64::{engine::general_purpose::STANDARD, Engine};
use codama_nodes::{
    CamelCaseString, DiscriminatorNode, InstructionInputValueNode, Number, TypeNode, ValueNode,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// A key that identifies a discriminator for collision detection.
/// Instructions with the same key will match the same discriminator check.
#[derive(PartialEq, Eq, Clone)]
enum DiscriminatorKey {
    Constant { offset: usize, value: u64 },
    Field { offset: usize, bytes: Vec<u8> },
    Size { size: usize },
}

/// Decode discriminator bytes from a codama [`BytesValueNode`](codama_nodes::BytesValueNode).
fn decode_discriminator_field_bytes(bytes: &codama_nodes::BytesValueNode) -> Vec<u8> {
    match bytes.encoding {
        codama_nodes::BytesEncoding::Base16 => {
            let padded = crate::utils::pad_hex(&bytes.data);
            hex::decode(&padded).expect("decode base16")
        },
        codama_nodes::BytesEncoding::Base58 => {
            bs58::decode(&bytes.data).into_vec().expect("decode base58")
        },
        codama_nodes::BytesEncoding::Base64 => STANDARD.decode(&bytes.data).expect("decode base64"),
        codama_nodes::BytesEncoding::Utf8 => bytes.data.as_bytes().to_vec(),
    }
}

/// Extract a comparable discriminator key from an instruction for collision detection.
fn extract_discriminator_key(
    instruction: &codama_nodes::InstructionNode,
) -> Option<DiscriminatorKey> {
    let discriminator = instruction.discriminators.first()?;

    match discriminator {
        DiscriminatorNode::Constant(node) => {
            let ValueNode::Number(nn) = node.constant.value.as_ref() else {
                return None;
            };
            let Number::UnsignedInteger(value) = nn.number else {
                return None;
            };
            Some(DiscriminatorKey::Constant {
                offset: node.offset,
                value,
            })
        },
        DiscriminatorNode::Field(node) => {
            let field = instruction.arguments.iter().find(|f| f.name == node.name)?;
            let TypeNode::FixedSize(_) = &field.r#type else {
                return None;
            };
            let default_value = field.default_value.as_ref()?;
            let InstructionInputValueNode::Bytes(bytes) = default_value else {
                return None;
            };
            let discriminator_bytes = decode_discriminator_field_bytes(bytes);
            Some(DiscriminatorKey::Field {
                offset: node.offset,
                bytes: discriminator_bytes,
            })
        },
        DiscriminatorNode::Size(node) => Some(DiscriminatorKey::Size { size: node.size }),
    }
}

/// Information extracted from a discriminator that's needed by both the match arm and helper fn.
struct DiscriminatorInfo {
    /// TokenStream for the args expression inside the helper fn body.
    args_expr: TokenStream,
    /// TokenStream for the discriminator check in the match arm.
    check: TokenStream,
}

/// Extract discriminator info from an instruction node.
///
/// Returns None if the discriminator can't be processed (unsupported format).
fn extract_discriminator_info(
    instruction: &codama_nodes::InstructionNode,
    args_ident: &syn::Ident,
    has_args: bool,
) -> Option<DiscriminatorInfo> {
    let discriminator = instruction.discriminators.first()?;

    match discriminator {
        // 1-byte discriminator at offset (usually offset=0)
        DiscriminatorNode::Constant(node) => {
            let offset = node.offset;

            let ValueNode::Number(nn) = node.constant.value.as_ref() else {
                return None;
            };

            let Number::UnsignedInteger(value) = nn.number else {
                return None;
            };

            let args_start = offset + 1;

            let args_expr = if has_args {
                quote! {
                    ::core::option::Option::Some(
                        <instruction::#args_ident as ::borsh::BorshDeserialize>::try_from_slice(
                            data.get(#args_start..).ok_or(ParseError::from("Missing args bytes"))?
                        ).map_err(|e| ParseError::Other(e.into()))?
                    )
                }
            } else {
                quote! { ::core::option::Option::None }
            };

            let check = quote! {
                if let Some(d) = data.get(#offset) {
                    *d == (#value as u8)
                } else {
                    false
                }
            };

            Some(DiscriminatorInfo { args_expr, check })
        },

        // Anchor-style discriminator in a field (usually 8 bytes) at offset
        DiscriminatorNode::Field(node) => {
            let offset = node.offset;

            let field = instruction.arguments.iter().find(|f| f.name == node.name)?;

            let TypeNode::FixedSize(fixed_size_node) = &field.r#type else {
                return None;
            };

            let size = fixed_size_node.size;
            let end = offset + size;

            let default_value = field.default_value.as_ref()?;

            let InstructionInputValueNode::Bytes(bytes) = default_value else {
                return None;
            };

            let discriminator_bytes = decode_discriminator_field_bytes(bytes);

            let args_expr = if has_args {
                quote! {
                    ::core::option::Option::Some(
                        <instruction::#args_ident as ::borsh::BorshDeserialize>::try_from_slice(
                            data.get(#end..).ok_or(ParseError::from("Missing args bytes"))?
                        ).map_err(|e| ParseError::Other(e.into()))?
                    )
                }
            } else {
                quote! { ::core::option::Option::None }
            };

            let check = quote! {
                if let Some(slice) = data.get(#offset..#end) {
                    slice == &[#(#discriminator_bytes),*]
                } else {
                    false
                }
            };

            Some(DiscriminatorInfo { args_expr, check })
        },

        // Discriminator by total size only
        DiscriminatorNode::Size(node) => {
            let size = node.size;

            let args_expr = if has_args {
                quote! {
                    ::core::option::Option::Some(
                        <instruction::#args_ident as ::borsh::BorshDeserialize>::try_from_slice(data)
                            .map_err(|e| ParseError::Other(e.into()))?
                    )
                }
            } else {
                quote! { ::core::option::Option::None }
            };

            let check = quote! {
                data.len() == #size
            };

            Some(DiscriminatorInfo { args_expr, check })
        },
    }
}

///
/// Generate a public parse helper function for a single instruction.
///
/// Example output:
/// ```rust, ignore
/// pub fn parse_swap_base_in(
///     accounts: &[::yellowstone_vixen_core::KeyBytes<32>],
///     data: &[u8],
/// ) -> ParseResult<Instructions> {
///     Ok(Instructions {
///         instruction: Some(instruction::Instruction::SwapBaseIn(
///             instruction::SwapBaseIn {
///                 accounts: Some(instruction::SwapBaseInAccounts { ... }),
///                 args: Some(<instruction::SwapBaseInArgs as BorshDeserialize>::try_from_slice(...)?),
///             }
///         )),
///     })
/// }
/// ```
///
fn single_instruction_helper_fn(
    instruction: &codama_nodes::InstructionNode,
    wrapper_ident: &syn::Ident,
) -> Option<TokenStream> {
    let ix_name_pascal = crate::utils::to_pascal_case(&instruction.name);
    let ix_name_snake = crate::utils::to_snake_case(&instruction.name);

    let variant_ident = format_ident!("{}", ix_name_pascal);
    let payload_ident = format_ident!("{}", ix_name_pascal);
    let accounts_ident = format_ident!("{}Accounts", ix_name_pascal);
    let args_ident = format_ident!("{}Args", ix_name_pascal);
    let fn_ident = format_ident!("parse_{}", ix_name_snake);

    let has_args = !instruction.arguments.is_empty();

    let info = extract_discriminator_info(instruction, &args_ident, has_args)?;

    let accounts_fields = instruction
        .accounts
        .iter()
        .enumerate()
        .map(|(idx, account)| {
            let field_name = format_ident!("{}", crate::utils::to_snake_case(&account.name));
            let error_msg = format!("Account does not exist at index {idx}");

            quote! { #field_name: ::yellowstone_vixen_core::PublicKey::new(accounts.get(#idx).ok_or(ParseError::from(#error_msg))?.to_vec()) }
        });

    let accounts_value = quote! { instruction::#accounts_ident { #(#accounts_fields),* } };
    let args_expr = info.args_expr;

    Some(quote! {
        pub fn #fn_ident(
            accounts: &[::yellowstone_vixen_core::KeyBytes<32>],
            data: &[u8],
        ) -> ParseResult<#wrapper_ident> {
            Ok(#wrapper_ident {
                instruction: ::core::option::Option::Some(
                    instruction::Instruction::#variant_ident(
                        instruction::#payload_ident {
                            accounts: ::core::option::Option::Some(#accounts_value),
                            args: #args_expr,
                        }
                    )
                ),
            })
        }
    })
}

///
/// Generate a discriminator match arm that delegates to the helper function.
///
/// Example output:
/// ```rust, ignore
/// if let Some(d) = data.get(0) {
///     if *d == (9 as u8) {
///         return parse_swap_base_in(accounts, data);
///     }
/// }
/// ```
///
fn single_instruction_match_arm(
    instruction: &codama_nodes::InstructionNode,
) -> Option<TokenStream> {
    let ix_name_snake = crate::utils::to_snake_case(&instruction.name);
    let fn_ident = format_ident!("parse_{}", ix_name_snake);
    let args_ident = format_ident!("{}Args", crate::utils::to_pascal_case(&instruction.name));

    let has_args = !instruction.arguments.is_empty();

    let info = extract_discriminator_info(instruction, &args_ident, has_args)?;

    let check = info.check;

    Some(quote! {
        if {
            #check
        } {
            return #fn_ident(accounts, data);
        }
    })
}

///
/// Generate a match arm for a group of instructions sharing the same discriminator.
///
/// Disambiguates by account count: instructions with unique account counts are resolved
/// automatically (highest count first). Instructions sharing both discriminator and account
/// count produce a runtime error directing the user to [`CustomInstructionParser`].
///
fn collision_group_match_arm(instructions: &[&codama_nodes::InstructionNode]) -> TokenStream {
    // Use the first instruction to get the shared discriminator check.
    let first = instructions[0];

    let args_ident = format_ident!("{}Args", crate::utils::to_pascal_case(&first.name));

    let has_args = !first.arguments.is_empty();

    let info = extract_discriminator_info(first, &args_ident, has_args)
        .expect("collision group should have valid discriminator");

    let check = info.check;

    // Group by account count (BTreeMap gives us sorted keys).
    let mut by_count: std::collections::BTreeMap<usize, Vec<&codama_nodes::InstructionNode>> =
        std::collections::BTreeMap::new();

    for ix in instructions {
        by_count.entry(ix.accounts.len()).or_default().push(ix);
    }

    let mut inner_arms = Vec::new();
    let mut ambiguous: Vec<String> = Vec::new();

    // Iterate from highest to lowest account count.
    for (&count, ixs) in by_count.iter().rev() {
        if ixs.len() == 1 {
            let ix_name_snake = crate::utils::to_snake_case(&ixs[0].name);

            let fn_ident = format_ident!("parse_{}", ix_name_snake);

            inner_arms.push(quote! {
                if accounts.len() >= #count {
                    return #fn_ident(accounts, data);
                }
            });
        } else {
            for ix in ixs {
                ambiguous.push(ix.name.to_string());
            }
        }
    }

    let fallback = if !ambiguous.is_empty() {
        let names = ambiguous.join(", ");

        let msg = format!(
            "Ambiguous instruction: variants [{names}] share the same discriminator and account \
             count. Use CustomInstructionParser to disambiguate."
        );

        quote! {
            return Err(ParseError::from(#msg));
        }
    } else {
        quote! {}
    };

    quote! {
        if {
            #check
        } {
            #(#inner_arms)*
            #fallback
        }
    }
}

pub fn instruction_parser(
    program_name_camel: &CamelCaseString,
    instructions: &[codama_nodes::InstructionNode],
) -> TokenStream {
    let program_name = crate::utils::to_pascal_case(program_name_camel);

    let instruction_parser_id = format!("{}::InstructionParser", program_name);

    let wrapper_ident = format_ident!("Instructions");

    // 1. Per-instruction parse helper functions
    let helper_fns: Vec<TokenStream> = instructions
        .iter()
        .filter_map(|ix| single_instruction_helper_fn(ix, &wrapper_ident))
        .collect();

    // 2. Group instructions by discriminator for collision detection,
    //    then generate match arms per group.
    let mut groups: Vec<(DiscriminatorKey, Vec<&codama_nodes::InstructionNode>)> = Vec::new();

    for ix in instructions {
        if let Some(key) = extract_discriminator_key(ix) {
            if let Some(group) = groups.iter_mut().find(|(k, _)| k == &key) {
                group.1.push(ix);
            } else {
                groups.push((key, vec![ix]));
            }
        }
    }

    let match_arms: Vec<TokenStream> = groups
        .iter()
        .filter_map(|(_, ixs)| {
            if ixs.len() == 1 {
                single_instruction_match_arm(ixs[0])
            } else {
                Some(collision_group_match_arm(ixs))
            }
        })
        .collect();

    quote! {
        //
        // Per-instruction parse helper functions.
        // Each parses a specific instruction variant from raw accounts and data,
        // without checking the discriminator.
        //

        #(#helper_fns)*

        ///
        /// Default instruction resolution using discriminator matching.
        ///
        /// Tries each instruction's discriminator in order and delegates to
        /// the corresponding `parse_*()` helper function.
        ///
        /// Call this from a custom [`InstructionResolver`] to handle
        /// non-ambiguous instructions while overriding specific ones.
        ///
        pub fn resolve_instruction_default(
            accounts: &[::yellowstone_vixen_core::KeyBytes<32>],
            data: &[u8],
        ) -> ParseResult<#wrapper_ident> {
            #(#match_arms)*

            Err(ParseError::Filtered)
        }

        ///
        ///  Trait for customizing instruction resolution logic.
        ///
        /// Implement this trait to handle programs where multiple instruction
        /// variants share the same discriminator and need runtime disambiguation
        /// (e.g. by account count or specific account values).
        ///
        /// Use with [`CustomInstructionParser`] to plug your resolver into the
        /// Vixen parser pipeline.
        ///
        pub trait InstructionResolver: Send + Sync + std::fmt::Debug + Copy + 'static {
            fn resolve(
                &self,
                accounts: &[::yellowstone_vixen_core::KeyBytes<32>],
                data: &[u8],
            ) -> ParseResult<#wrapper_ident>;
        }

        ///
        /// Instruction parser with a custom resolver for ambiguous discriminators.
        ///
        /// Use this instead of [`InstructionParser`] when you need to override
        /// how instructions with shared discriminators are resolved.
        ///
        /// # Example
        ///
        /// ```rust,ignore
        /// #[derive(Debug, Copy, Clone)]
        /// struct MyResolver;
        ///
        /// impl program::InstructionResolver for MyResolver {
        ///     fn resolve(
        ///         &self,
        ///         accounts: &[yellowstone_vixen_core::KeyBytes<32>],
        ///         data: &[u8],
        ///     ) -> ParseResult<program::Instructions> {
        ///         // Custom disambiguation logic here
        ///         program::resolve_instruction_default(accounts, data)
        ///     }
        /// }
        ///
        /// let parser = program::CustomInstructionParser(MyResolver);
        /// ```
        ///
        #[derive(Debug, Copy, Clone)]
        pub struct CustomInstructionParser<R: InstructionResolver>(pub R);

        impl<R: InstructionResolver> Parser for CustomInstructionParser<R> {
            type Input = ::yellowstone_vixen_core::instruction::InstructionUpdate;
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
                ix_update: &::yellowstone_vixen_core::instruction::InstructionUpdate,
            ) -> ParseResult<Self::Output> {
                if *ix_update.program != PROGRAM_ID {
                    return Err(ParseError::Filtered);
                }

                self.0.resolve(&ix_update.accounts, &ix_update.data)
            }
        }

        impl<R: InstructionResolver> ::yellowstone_vixen_core::ProgramParser for CustomInstructionParser<R> {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::KeyBytes<32> {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct InstructionParser;

        impl Parser for InstructionParser {
            type Input = ::yellowstone_vixen_core::instruction::InstructionUpdate;
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
                ix_update: &::yellowstone_vixen_core::instruction::InstructionUpdate,
            ) -> ParseResult<Self::Output> {
                if *ix_update.program != PROGRAM_ID {
                    return Err(ParseError::Filtered);
                }

                resolve_instruction_default(&ix_update.accounts, &ix_update.data)
            }
        }

        // Implement the trait for Mock
        impl ::yellowstone_vixen_core::ProgramParser for InstructionParser {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::KeyBytes::<32> {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }
    }
}
