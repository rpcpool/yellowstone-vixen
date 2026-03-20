use codama_nodes::CamelCaseString;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

///
/// Generate event parser code: `resolve_event_default()`, `resolve_events_from_logs()`,
/// and `EventParser` struct.
///
/// Mirrors `instruction_parser()` but for events. Events use the same discriminator
/// matching logic — the only difference is they live in the `event` module and use
/// the `Events` wrapper type.
///
pub fn event_parser(
    program_name_camel: &CamelCaseString,
    events: &[codama_nodes::InstructionNode],
) -> TokenStream {
    let program_name = crate::utils::to_pascal_case(program_name_camel);
    let event_parser_id = format!("{}::EventParser", program_name);
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

        #[derive(Debug, Copy, Clone)]
        pub struct EventParser;

        impl Parser for EventParser {
            type Input = ::yellowstone_vixen_core::instruction::InstructionUpdate;
            type Output = #wrapper_ident;

            fn id(&self) -> std::borrow::Cow<'static, str> {
                #event_parser_id.into()
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

                resolve_event_default(&ix_update.accounts, &ix_update.data)
            }
        }

        impl ::yellowstone_vixen_core::ProgramParser for EventParser {
            #[inline]
            fn program_id(&self) -> yellowstone_vixen_core::Pubkey {
                yellowstone_vixen_core::Pubkey::new(PROGRAM_ID)
            }
        }

        /// Create a combined instruction + event parser.
        ///
        /// Returns a `ProgramEventParser` that wraps both `InstructionParser`
        /// and `EventParser`, automatically detecting CPI events and log events.
        pub fn program_event_parser(
        ) -> yellowstone_vixen_parser::ProgramEventParser<InstructionParser, EventParser>
        {
            yellowstone_vixen_parser::ProgramEventParser::new(
                InstructionParser,
                EventParser,
                PROGRAM_ID,
            )
        }
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
