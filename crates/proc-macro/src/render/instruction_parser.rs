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
    /// `None` when the instruction has no arguments.
    args_expr: Option<TokenStream>,
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
                Some(quote! {
                    {
                        let mut slice: &[u8] = data.get(#args_start..).ok_or(ParseError::from("Missing args bytes"))?;

                        <instruction::#args_ident as ::borsh::BorshDeserialize>::deserialize_reader(&mut slice)
                            .map_err(|e| ParseError::Other(e.into()))?
                    }
                })
            } else {
                None
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
                Some(quote! {
                    {
                        let mut slice: &[u8] = data.get(#end..).ok_or(ParseError::from("Missing args bytes"))?;

                        <instruction::#args_ident as ::borsh::BorshDeserialize>::deserialize_reader(&mut slice)
                            .map_err(|e| ParseError::Other(e.into()))?
                    }
                })
            } else {
                None
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
                Some(quote! {
                    {
                        let mut slice: &[u8] = data;

                        <instruction::#args_ident as ::borsh::BorshDeserialize>::deserialize_reader(&mut slice)
                            .map_err(|e| ParseError::Other(e.into()))?
                    }
                })
            } else {
                None
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
///         instruction: instruction::Instruction::SwapBaseIn {
///             accounts: instruction::SwapBaseInAccounts { ... },
///             args: <instruction::SwapBaseInArgs as BorshDeserialize>::try_from_slice(...)?,
///         },
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

    let num_defined_accounts = instruction.accounts.len();

    let accounts_value = quote! {
        instruction::#accounts_ident {
            #(#accounts_fields,)*
            remaining_accounts: accounts
                .get(#num_defined_accounts..)
                .unwrap_or_default()
                .iter()
                .map(|a| ::yellowstone_vixen_core::PublicKey::new(a.to_vec()))
                .collect(),
        }
    };
    let args_field = info.args_expr.map(|expr| {
        quote! { args: #expr, }
    });

    Some(quote! {
        pub fn #fn_ident(
            accounts: &[::yellowstone_vixen_core::KeyBytes<32>],
            data: &[u8],
        ) -> ParseResult<#wrapper_ident> {
            Ok(#wrapper_ident {
                instruction: instruction::Instruction::#variant_ident {
                    accounts: #accounts_value,
                    #args_field
                },
                raw_logs: vec![],
                anchor_log_events: vec![],
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

///
/// Generate a match arm for resolving a single event from "Program data:" bytes.
///
/// In "Program data:" payloads the discriminator is always at offset 0,
/// regardless of the IDL offset (which describes the position within
/// instruction data for self-CPI events).
///
fn event_resolve_arm(ix: &codama_nodes::InstructionNode) -> Option<TokenStream> {
    let ix_name_pascal = crate::utils::to_pascal_case(&ix.name);
    let variant_ident = format_ident!("{}", ix_name_pascal);
    let args_ident = format_ident!("{}Args", ix_name_pascal);

    let discriminator = ix.discriminators.first()?;

    let DiscriminatorNode::Field(node) = discriminator else {
        return None;
    };

    let field = ix.arguments.iter().find(|f| f.name == node.name)?;

    let TypeNode::FixedSize(fixed_size_node) = &field.r#type else {
        return None;
    };

    let disc_size = fixed_size_node.size;

    let default_value = field.default_value.as_ref()?;

    let InstructionInputValueNode::Bytes(bytes) = default_value else {
        return None;
    };

    let discriminator_bytes = decode_discriminator_field_bytes(bytes);

    // Filter out the discriminator argument from args to deserialize.
    let has_args = ix.arguments.iter().any(|a| a.name != node.name);

    let args_expr = if has_args {
        quote! {
            {
                let mut slice: &[u8] = data.get(#disc_size..).unwrap_or(&[]);

                <instruction::#args_ident as ::borsh::BorshDeserialize>::deserialize_reader(&mut slice).ok()?
            }
        }
    } else {
        quote! { instruction::#args_ident {} }
    };

    Some(quote! {
        if data.get(0..#disc_size) == ::core::option::Option::Some(&[#(#discriminator_bytes),*]) {
            return ::core::option::Option::Some(
                AnchorLogEvent::#variant_ident(#args_expr)
            );
        }
    })
}

pub fn instruction_parser(
    program_name_camel: &CamelCaseString,
    instructions: &[codama_nodes::InstructionNode],
    event_names: &std::collections::HashSet<String>,
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

    // 3. Event resolver: match arms for Anchor log events ("Program data:" payloads).
    let event_arms: Vec<TokenStream> = instructions
        .iter()
        .filter(|ix| event_names.contains(ix.name.as_ref()))
        .filter_map(|ix| event_resolve_arm(ix))
        .collect();

    let resolve_event_fn = if event_arms.is_empty() {
        quote! {
            /// No Anchor log events in this program.
            pub fn resolve_event(_data: &[u8]) -> ::core::option::Option<AnchorLogEvent> {
                ::core::option::Option::None
            }
        }
    } else {
        quote! {
            ///
            /// Try to resolve an Anchor event from "Program data:" bytes.
            ///
            /// The input `data` is the base64-decoded payload from a
            /// `"Program data: <base64>"` log line.  Returns `None` if no
            /// known event discriminator matches.
            ///
            pub fn resolve_event(data: &[u8]) -> ::core::option::Option<AnchorLogEvent> {
                #(#event_arms)*

                ::core::option::Option::None
            }
        }
    };

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

        #resolve_event_fn

        /// Scan log messages for `"Program data: <base64>"` lines and resolve
        /// any matching Anchor events.
        pub fn resolve_events_from_logs(logs: &[String]) -> Vec<AnchorLogEvent> {
            const PREFIX: &str = "Program data: ";

            logs.iter()
                .filter_map(|line| {
                    let b64 = line.strip_prefix(PREFIX)?;

                    use ::yellowstone_vixen_core::base64::{engine::general_purpose::STANDARD, Engine};

                    let data = STANDARD.decode(b64).ok()?;

                    resolve_event(&data)
                })
                .collect()
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
        /// ```rust, ignore
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
        /// Instruction parser with a custom resolver.
        ///
        /// Raw logs are excluded by default; call with_raw_logs() to include them.
        ///
        #[derive(Debug, Copy, Clone)]
        pub struct CustomInstructionParser<R: InstructionResolver> {
            /// The resolver used to disambiguate instructions.
            pub resolver: R,
            include_raw_logs: bool,
        }

        impl<R: InstructionResolver> CustomInstructionParser<R> {
            /// Create a new parser with the given resolver.
            pub fn new(resolver: R) -> Self {
                Self { resolver, include_raw_logs: false }
            }

            /// Include raw logs in the parsed output.
            pub fn with_raw_logs(mut self) -> Self {
                self.include_raw_logs = true;
                self
            }
        }

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

                let mut result = self.resolver.resolve(&ix_update.accounts, &ix_update.data)?;

                if self.include_raw_logs {
                    result.raw_logs = ix_update.log_messages.clone();
                }

                result.anchor_log_events = resolve_events_from_logs(&ix_update.log_messages);

                Ok(result)
            }
        }

        impl<R: InstructionResolver> ::yellowstone_vixen_core::ProgramParser for CustomInstructionParser<R> {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::KeyBytes<32> {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct InstructionParser {
            include_raw_logs: bool,
        }

        #[allow(non_upper_case_globals)]
        pub const InstructionParser: InstructionParser = InstructionParser { include_raw_logs: false };

        impl InstructionParser {
            /// Include raw logs in the parsed output.
            pub fn with_raw_logs(mut self) -> Self {
                self.include_raw_logs = true;
                self
            }
        }

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

                let mut result = resolve_instruction_default(&ix_update.accounts, &ix_update.data)?;

                if self.include_raw_logs {
                    result.raw_logs = ix_update.log_messages.clone();
                }

                result.anchor_log_events = resolve_events_from_logs(&ix_update.log_messages);

                Ok(result)
            }
        }

        impl ::yellowstone_vixen_core::ProgramParser for InstructionParser {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::KeyBytes::<32> {
                yellowstone_vixen_core::KeyBytes::<32>(PROGRAM_ID)
            }
        }
    }
}
