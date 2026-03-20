use codama_nodes::CamelCaseString;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

///
/// Generate event support code: `resolve_event_default()`, `resolve_events_from_logs()`,
/// and `ProgramEventOutput` struct.
///
/// The event resolution functions are called by `InstructionParser` (when
/// `program-events` feature is active) to handle CPI and log-based events.
///
pub fn event_parser(
    _program_name_camel: &CamelCaseString,
    events: &[codama_nodes::InstructionNode],
) -> TokenStream {
    let wrapper_ident = format_ident!("Events");

    // Per-event parse helper functions
    let helper_fns: Vec<TokenStream> = events
        .iter()
        .filter_map(|ev| single_event_helper_fn(ev, &wrapper_ident))
        .collect();

    // Discriminator match arms
    let mut groups: Vec<(super::instruction_parser::DiscriminatorKey, Vec<&codama_nodes::InstructionNode>)> = Vec::new();

    for ev in events {
        if let Some(key) = super::instruction_parser::extract_discriminator_key(ev) {
            if let Some(group) = groups.iter_mut().find(|(k, _)| k == &key) {
                group.1.push(ev);
            } else {
                groups.push((key, vec![ev]));
            }
        }
    }

    let match_arms: Vec<TokenStream> = groups
        .iter()
        .filter_map(|(_, evs)| {
            if evs.len() == 1 {
                single_event_match_arm(evs[0])
            } else {
                // Events shouldn't have discriminator collisions, but handle it gracefully
                Some(super::instruction_parser::collision_group_match_arm(evs))
            }
        })
        .collect();

    let resolve_events_from_logs = quote! {
        ///
        /// Resolve events from `"Program data: "` transaction log lines.
        ///
        /// For each matching line, base64-decodes the payload, prepends the
        /// self-CPI event prefix (`EVENT_IX_TAG`), and runs it through
        /// the event discriminator matching.
        ///
        /// Returns successfully parsed events; lines that don't match any
        /// discriminator are silently skipped.
        ///
        pub fn resolve_events_from_logs(
            logs: &[String],
        ) -> Vec<#wrapper_ident> {
            const PREFIX: &str = "Program data: ";
            // First 8 bytes of sha256("anchor:event"), see anchor-lang event.rs
            const EVENT_IX_TAG: [u8; 8] = 0x1d9a_cb51_2ea5_45e4_u64.to_le_bytes();

            logs.iter()
                .filter_map(|line| {
                    let encoded = line.strip_prefix(PREFIX)?;

                    let decoded = yellowstone_vixen_parser::base64::Engine::decode(
                        &yellowstone_vixen_parser::base64::engine::general_purpose::STANDARD,
                        encoded.trim(),
                    ).ok()?;

                    let mut data = Vec::with_capacity(8 + decoded.len());

                    data.extend_from_slice(&EVENT_IX_TAG);
                    data.extend_from_slice(&decoded);

                    resolve_event_default(&[], &data).ok()
                })
                .collect()
        }
    };

    let program_event_output = generate_program_event_output();

    quote! {
        #(#helper_fns)*

        ///
        /// Default event resolution using discriminator matching.
        ///
        pub fn resolve_event_default(
            accounts: &[::yellowstone_vixen_core::Pubkey],
            data: &[u8],
        ) -> ParseResult<#wrapper_ident> {
            #(#match_arms)*

            Err(ParseError::Filtered)
        }

        #resolve_events_from_logs

        #program_event_output
    }
}

/// Generate the concrete `ProgramEventOutput` struct and optionally its
/// `prost::Message` impl (behind `proto` feature).
fn generate_program_event_output() -> TokenStream {
    let proto_impl = if cfg!(feature = "proto") {
        quote! {
            impl ::prost::Message for ProgramEventOutput {
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if let Some(ref ix) = self.instruction {
                        ::prost::encoding::message::encode(1, ix, buf);
                    }

                    for event in &self.program_events {
                        ::prost::encoding::message::encode(2, event, buf);
                    }
                }

                fn merge_field(
                    &mut self,
                    _tag: u32,
                    _wire_type: ::prost::encoding::WireType,
                    _buf: &mut impl ::prost::bytes::Buf,
                    _ctx: ::prost::encoding::DecodeContext,
                ) -> Result<(), ::prost::DecodeError> {
                    // ProgramEventOutput is encode-only. Decoding from proto is not supported
                    // because the inner types (Instructions) do not implement Default.
                    Err(::prost::DecodeError::new(
                        "ProgramEventOutput does not support proto decoding",
                    ))
                }

                fn encoded_len(&self) -> usize {
                    let mut len = 0;

                    if let Some(ref ix) = self.instruction {
                        len += ::prost::encoding::message::encoded_len(1, ix);
                    }

                    for event in &self.program_events {
                        len += ::prost::encoding::message::encoded_len(2, event);
                    }

                    len
                }

                fn clear(&mut self) {
                    self.instruction = None;
                    self.program_events.clear();
                }
            }

            impl Default for ProgramEventOutput {
                fn default() -> Self {
                    Self {
                        instruction: None,
                        program_events: Vec::new(),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    quote! {
        /// Combined output from instruction + event parsing.
        ///
        /// Generated per-program by the proc-macro with concrete `Instructions`
        /// and `Events` types.
        #[derive(Debug, PartialEq)]
        pub struct ProgramEventOutput {
            /// Parsed instruction (None if this was a CPI event or filtered).
            pub instruction: Option<Instructions>,
            /// Events parsed from logs and/or CPI self-invocations.
            pub program_events: Vec<Events>,
        }

        #proto_impl
    }
}

fn single_event_helper_fn(
    event: &codama_nodes::InstructionNode,
    wrapper_ident: &syn::Ident,
) -> Option<TokenStream> {
    let ev_name_pascal = crate::utils::to_pascal_case(&event.name);
    let ev_name_snake = crate::utils::to_snake_case(&event.name);

    let variant_ident = format_ident!("{}", ev_name_pascal);
    let accounts_ident = format_ident!("{}Accounts", ev_name_pascal);
    let args_ident = format_ident!("{}Args", ev_name_pascal);
    let fn_ident = format_ident!("parse_event_{}", ev_name_snake);

    let has_args = !event.arguments.is_empty();

    let info = super::instruction_parser::extract_discriminator_info(event, &args_ident, has_args)?;

    // Events have no accounts — only remaining_accounts (always empty vec)
    let accounts_value = quote! {
        event::#accounts_ident {
            remaining_accounts: vec![],
        }
    };

    let args_field = info.args_expr.as_ref().map(|expr| {
        // Remap the args expression to use `event::` module instead of `instruction::`
        let remapped = remap_module_prefix(expr, "instruction", "event");
        quote! { args: #remapped, }
    });

    Some(quote! {
        pub fn #fn_ident(
            accounts: &[::yellowstone_vixen_core::Pubkey],
            data: &[u8],
        ) -> ParseResult<#wrapper_ident> {
            Ok(#wrapper_ident {
                event: event::Event::#variant_ident {
                    accounts: #accounts_value,
                    #args_field
                },
            })
        }
    })
}

fn single_event_match_arm(
    event: &codama_nodes::InstructionNode,
) -> Option<TokenStream> {
    let ev_name_snake = crate::utils::to_snake_case(&event.name);
    let fn_ident = format_ident!("parse_event_{}", ev_name_snake);
    let args_ident = format_ident!("{}Args", crate::utils::to_pascal_case(&event.name));

    let has_args = !event.arguments.is_empty();

    let info = super::instruction_parser::extract_discriminator_info(event, &args_ident, has_args)?;

    let check = info.check;

    Some(quote! {
        if {
            #check
        } {
            return #fn_ident(accounts, data);
        }
    })
}

/// Remap `instruction::` references to `event::` in a TokenStream.
/// This is needed because `extract_discriminator_info` generates args deserialization
/// code that references `instruction::ArgsType`, but for events we need `event::ArgsType`.
fn remap_module_prefix(ts: &TokenStream, from: &str, to: &str) -> TokenStream {
    let s = ts.to_string();
    let remapped = s.replace(&format!("{from} ::"), &format!("{to} ::"));
    remapped.parse().unwrap_or_else(|_| ts.clone())
}
