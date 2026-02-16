extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, LitStr};

mod intermediate_representation;
mod parse;
mod render;
mod utils;
mod vixen_proto;

/// Attribute macro that auto-infers prost annotations from Rust types.
///
/// # Modes
///
/// - `#[vixen_proto]` — struct with `prost::Message` (default)
/// - `#[vixen_proto(oneof)]` — enum with `prost::Oneof`
/// - `#[vixen_proto(enumeration)]` — enum with `prost::Enumeration`
///
/// Fields are auto-tagged starting at 1. Use `#[vixen_proto_hint(...)]` on individual
/// fields when the type can't be auto-inferred.
#[proc_macro_attribute]
pub fn vixen_proto(attr: TokenStream, item: TokenStream) -> TokenStream {
    vixen_proto::expand(attr.into(), item.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro]
pub fn include_vixen_parser(input: TokenStream) -> TokenStream {
    let idl_path_lit = parse_macro_input!(input as LitStr);
    let idl_path = idl_path_lit.value();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let full_path = std::path::Path::new(&manifest_dir).join(&idl_path);

    match parse::load_codama_idl(&full_path) {
        Ok(idl) => crate::render::vixen_parser(&idl).into(),
        Err(e) => {
            let error_msg = format!("Failed to load/parse IDL from {:?}: {}", full_path, e);

            quote::quote! {
                compile_error!(#error_msg);
            }
            .into()
        },
    }
}
