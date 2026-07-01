extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, LitStr, Token,
};

mod intermediate_representation;
mod parse;
mod render;
mod utils;
mod vixen;

/// Attribute macro that auto-infers prost annotations from Rust types.
///
/// # Modes
///
/// - `#[vixen]` — struct with `prost::Message` (default)
/// - `#[vixen(oneof)]` — enum with `prost::Oneof`
/// - `#[vixen(enumeration)]` — enum with `prost::Enumeration`
///
/// Fields are auto-tagged starting at 1. Use `#[hint(...)]` on individual
/// fields when the type can't be auto-inferred.
#[proc_macro_attribute]
pub fn vixen(attr: TokenStream, item: TokenStream) -> TokenStream {
    vixen::expand(attr.into(), item.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn include_vixen_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as IncludeVixenParserInput);

    match input.parser_config() {
        Ok(config) => expand_include_vixen_parser(input.idl_path.value(), config),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Input for `include_vixen_parser!`.
///
/// The macro accepts a Codama IDL path, plus optional parser config:
///
/// ```ignore
/// include_vixen_parser!(
///     "../idls/custom_events.json",
///     cpi_event_discriminator = 0xfe,
///     cpi_event_payload_offset = 1,
/// );
/// ```
///
/// Supported config options:
/// - `cpi_event_discriminator = 0xfe`: hex bytes that identify self-CPI event instructions.
///   The string form (`"fe"`) is also accepted for multi-byte discriminators.
/// - `cpi_event_payload_offset = 1`: byte offset where event payload decoding starts.
///
/// When omitted, the parser uses Anchor's default self-CPI event envelope:
/// an 8-byte Anchor event instruction discriminator, followed by the event
/// discriminator and payload. Override these options for non-Anchor programs
/// that wrap events differently, such as Pinocchio programs that emit events
/// through a custom one-byte self-CPI instruction envelope.
struct IncludeVixenParserInput {
    idl_path: LitStr,
    cpi_event_discriminator: Option<HexBytesLiteral>,
    cpi_event_payload_offset: Option<LitInt>,
}

enum HexBytesLiteral {
    Str(LitStr),
    Int(LitInt),
}

impl Parse for HexBytesLiteral {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Str(input.parse()?))
        } else {
            Ok(Self::Int(input.parse()?))
        }
    }
}

impl Parse for IncludeVixenParserInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let idl_path = input.parse()?;
        let mut cpi_event_discriminator = None;
        let mut cpi_event_payload_offset = None;

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            if input.is_empty() {
                break;
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "cpi_event_discriminator" => {
                    cpi_event_discriminator = Some(input.parse()?);
                },
                "cpi_event_payload_offset" => {
                    cpi_event_payload_offset = Some(input.parse()?);
                },
                _ => {
                    return Err(syn::Error::new(
                        key.span(),
                        "unsupported include_vixen_parser option",
                    ));
                },
            }
        }

        Ok(Self {
            idl_path,
            cpi_event_discriminator,
            cpi_event_payload_offset,
        })
    }
}

impl IncludeVixenParserInput {
    fn parser_config(&self) -> syn::Result<crate::render::vixen_parser::ParserConfig> {
        let mut config = crate::render::vixen_parser::ParserConfig::default();

        if let Some(discriminator) = &self.cpi_event_discriminator {
            config.cpi_event.discriminator = decode_hex_bytes_literal(discriminator)?;
        }

        if let Some(offset) = &self.cpi_event_payload_offset {
            config.cpi_event.payload_offset = offset.base10_parse()?;
        } else if self.cpi_event_discriminator.is_some() {
            config.cpi_event.payload_offset = config.cpi_event.discriminator.len();
        }

        if config.cpi_event.discriminator.is_empty() {
            return Err(syn::Error::new(
                self.idl_path.span(),
                "cpi_event_discriminator must not be empty",
            ));
        }

        if config.cpi_event.payload_offset < config.cpi_event.discriminator.len() {
            return Err(syn::Error::new(
                self.cpi_event_payload_offset
                    .as_ref()
                    .map_or_else(|| self.idl_path.span(), LitInt::span),
                "cpi_event_payload_offset must be greater than or equal to \
                 cpi_event_discriminator length",
            ));
        }

        Ok(config)
    }
}

fn decode_hex_bytes_literal(lit: &HexBytesLiteral) -> syn::Result<Vec<u8>> {
    match lit {
        HexBytesLiteral::Str(lit) => crate::utils::decode_hex_text(&lit.value())
            .map_err(|err| invalid_cpi_event_discriminator_hex(lit.span(), err)),
        HexBytesLiteral::Int(lit) => decode_int_literal(lit),
    }
}

fn decode_int_literal(lit: &LitInt) -> syn::Result<Vec<u8>> {
    let value = lit.to_string();
    let trimmed = value.trim();

    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        return crate::utils::decode_hex_text(trimmed)
            .map_err(|err| invalid_cpi_event_discriminator_hex(lit.span(), err));
    }

    let value = lit.base10_parse::<u128>()?;
    if value == 0 {
        return Ok(vec![0]);
    }

    let bytes = value.to_be_bytes();
    let first_non_zero = bytes
        .iter()
        .position(|byte| *byte != 0)
        .unwrap_or(bytes.len() - 1);
    Ok(bytes[first_non_zero..].to_vec())
}

fn invalid_cpi_event_discriminator_hex(
    span: proc_macro2::Span,
    err: hex::FromHexError,
) -> syn::Error {
    syn::Error::new(
        span,
        format!("cpi_event_discriminator must be hex bytes: {err}"),
    )
}

fn expand_include_vixen_parser(
    idl_path: String,
    config: crate::render::vixen_parser::ParserConfig,
) -> TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let full_path = std::path::Path::new(&manifest_dir).join(&idl_path);

    match parse::load_codama_idl(&full_path) {
        Ok((idl, events)) => crate::render::vixen_parser(&idl, &events, &config).into(),
        Err(e) => {
            let error_msg = format!("Failed to load/parse IDL from {:?}: {}", full_path, e);

            quote::quote! {
                compile_error!(#error_msg);
            }
            .into()
        },
    }
}
