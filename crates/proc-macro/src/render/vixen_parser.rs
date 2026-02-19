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

    let proto_schema = if cfg!(feature = "proto") {
        let proto_str =
            crate::render::proto_schema_string(&schema_ir, &program_mod_ident.to_string());

        let proto_lit = syn::LitStr::new(&proto_str, proc_macro2::Span::call_site());

        quote! {
            /// Generated .proto schema for this program.
            pub const PROTOBUF_SCHEMA: &str = #proto_lit;
        }
    } else {
        quote! {}
    };

    quote! {
        pub mod #program_mod_ident {
            use yellowstone_vixen_parser::prelude::*;

            /// 32 bytes by convention.
            pub type PubkeyBytes = Vec<u8>;

            /// Borsh: deserialize a fixed 32-byte pubkey into `Vec<u8>`.
            /// On-chain pubkeys are always 32 bytes with no length prefix,
            /// but `Vec<u8>` borsh expects `u32 len + bytes`. This bridges the gap.
            fn borsh_deser_pubkey_bytes<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<PubkeyBytes, ::borsh::io::Error> {
                let mut buf = [0u8; 32];
                reader.read_exact(&mut buf)?;
                ::core::result::Result::Ok(buf.to_vec())
            }

            fn borsh_ser_pubkey_bytes<W: ::borsh::io::Write>(
                val: &PubkeyBytes,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                writer.write_all(val)
            }

            fn borsh_deser_opt_pubkey_bytes<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<::core::option::Option<PubkeyBytes>, ::borsh::io::Error> {
                let tag: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;
                match tag {
                    0 => ::core::result::Result::Ok(::core::option::Option::None),
                    1 => {
                        let mut buf = [0u8; 32];
                        reader.read_exact(&mut buf)?;
                        ::core::result::Result::Ok(::core::option::Option::Some(buf.to_vec()))
                    }
                    _ => ::core::result::Result::Err(::borsh::io::Error::new(
                        ::borsh::io::ErrorKind::InvalidData,
                        "invalid option tag for PubkeyBytes",
                    )),
                }
            }

            fn borsh_ser_opt_pubkey_bytes<W: ::borsh::io::Write>(
                val: &::core::option::Option<PubkeyBytes>,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match val {
                    ::core::option::Option::Some(v) => {
                        ::borsh::BorshSerialize::serialize(&1u8, writer)?;
                        writer.write_all(v)
                    }
                    ::core::option::Option::None => {
                        ::borsh::BorshSerialize::serialize(&0u8, writer)
                    }
                }
            }

            fn borsh_deser_vec_pubkey_bytes<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<PubkeyBytes>, ::borsh::io::Error> {
                let len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;
                let mut result = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    let mut buf = [0u8; 32];
                    reader.read_exact(&mut buf)?;
                    result.push(buf.to_vec());
                }
                ::core::result::Result::Ok(result)
            }

            fn borsh_ser_vec_pubkey_bytes<W: ::borsh::io::Write>(
                val: &[PubkeyBytes],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                ::borsh::BorshSerialize::serialize(&(val.len() as u32), writer)?;
                for v in val {
                    writer.write_all(v)?;
                }
                ::core::result::Result::Ok(())
            }

            pub const PROGRAM_ID: [u8; 32] = #program_pubkey;

            #proto_schema

            #schema_types
            #account_parser
            #instruction_parser
        }
    }
}
