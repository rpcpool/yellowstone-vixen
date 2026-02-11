use codama_nodes::RootNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn vixen_parser(idl: &RootNode) -> TokenStream {
    let program_mod_ident = format_ident!("{}", crate::utils::to_snake_case(&idl.program.name));

    let program_pubkey = crate::render::program_pubkey(&idl.program.public_key);

    let schema_ir = crate::intermediate_representation::build_schema_ir(idl);

    let schema_types = crate::render::rust_types_from_ir(&schema_ir);

    let account_parser = crate::render::account_parser(&idl.program.name, &idl.program.accounts);

    let instruction_parser =
        crate::render::instruction_parser(&idl.program.name, &idl.program.instructions);

    let proto_lit = {
        let proto_str =
            crate::render::proto_schema_string(&schema_ir, &program_mod_ident.to_string());

        syn::LitStr::new(&proto_str, proc_macro2::Span::call_site())
    };

    quote! {
        pub mod #program_mod_ident {
            use yellowstone_vixen_parser::prelude::*;

            /// 32 bytes by convention.
            pub type PubkeyBytes = ::prost::alloc::vec::Vec<u8>;

            pub const PROGRAM_ID: [u8; 32] = #program_pubkey;

            /// Generated .proto schema for this program.
            pub const PROTOBUF_SCHEMA: &str = #proto_lit;

            #schema_types
            #account_parser
            #instruction_parser
        }
    }
}
