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

    let widen_helpers = widen_borsh_helpers();

    quote! {
        pub mod #program_mod_ident {
            use yellowstone_vixen_parser::prelude::*;

            /// 32 bytes by convention.
            pub type PubkeyBytes = Vec<u8>;

            /// Borsh: deserialize N fixed bytes into `Vec<u8>`.
            /// On-chain, fixed-size byte fields (pubkeys, u128, fixed arrays) have no
            /// length prefix, but `Vec<u8>` borsh expects `u32 len + bytes`.
            fn borsh_deserialize_fixed_bytes<const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<u8>, ::borsh::io::Error> {
                let mut buf = [0u8; N];

                reader.read_exact(&mut buf)?;

                ::core::result::Result::Ok(buf.to_vec())
            }

            fn borsh_serialize_fixed_bytes<const N: usize, W: ::borsh::io::Write>(
                val: &[u8],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                writer.write_all(val)
            }

            fn borsh_deserialize_opt_fixed_bytes<const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<::core::option::Option<Vec<u8>>, ::borsh::io::Error> {
                let tag: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                match tag {
                    0 => ::core::result::Result::Ok(::core::option::Option::None),
                    1 => {
                        let mut buf = [0u8; N];

                        reader.read_exact(&mut buf)?;

                        ::core::result::Result::Ok(::core::option::Option::Some(buf.to_vec()))
                    }
                    _ => ::core::result::Result::Err(::borsh::io::Error::new(
                        ::borsh::io::ErrorKind::InvalidData,
                        "invalid option tag for fixed bytes",
                    )),
                }
            }

            fn borsh_serialize_opt_fixed_bytes<const N: usize, W: ::borsh::io::Write>(
                val: &::core::option::Option<Vec<u8>>,
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

            fn borsh_deserialize_vec_fixed_bytes<const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<Vec<u8>>, ::borsh::io::Error> {
                let len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                let mut result = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    let mut buf = [0u8; N];

                    reader.read_exact(&mut buf)?;
                    result.push(buf.to_vec());
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_vec_fixed_bytes<const N: usize, W: ::borsh::io::Write>(
                val: &[Vec<u8>],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                ::borsh::BorshSerialize::serialize(&(val.len() as u32), writer)?;

                for v in val {
                    writer.write_all(v)?;
                }

                ::core::result::Result::Ok(())
            }

            /// Borsh: deserialize a required message field that prost wraps in Option.
            /// On-chain the struct is serialized directly (no Option tag byte),
            /// but the Rust field is `Option<T>` for prost compatibility.
            fn borsh_deserialize_required_msg<T: ::borsh::BorshDeserialize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<::core::option::Option<T>, ::borsh::io::Error> {
                let val = <T as ::borsh::BorshDeserialize>::deserialize_reader(reader)?;

                ::core::result::Result::Ok(::core::option::Option::Some(val))
            }

            fn borsh_serialize_required_msg<T: ::borsh::BorshSerialize, W: ::borsh::io::Write>(
                val: &::core::option::Option<T>,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match val {
                    ::core::option::Option::Some(v) => {
                        <T as ::borsh::BorshSerialize>::serialize(v, writer)
                    }

                    ::core::option::Option::None => {
                        ::core::result::Result::Err(::borsh::io::Error::new(
                            ::borsh::io::ErrorKind::InvalidData,
                            "required message field is None",
                        ))
                    }
                }
            }

            #widen_helpers

            pub const PROGRAM_ID: [u8; 32] = #program_pubkey;

            #proto_schema

            #schema_types
            #account_parser
            #instruction_parser
        }
    }
}

///
/// Generate borsh helper functions that bridge the gap between on-chain narrow
/// integer types (u8, u16, i8, i16) and their widened Rust/proto counterparts
/// (u32, i32).
///
/// Protobuf has no u8/u16/i8/i16 — those on-chain types are stored as u32/i32
/// in the generated Rust structs (for prost compatibility). But borsh must still
/// read/write the original narrow size (e.g. 1 byte for u8, not 4 bytes for u32).
///
/// For each type pair we generate 6 functions (deserialize + serialize × 3 labels):
///   - `borsh_deserialize_u8_as_u32`     / `borsh_serialize_u32_as_u8`       — singular
///   - `borsh_deserialize_opt_u8_as_u32`  / `borsh_serialize_opt_u32_as_u8`  — optional
///   - `borsh_deserialize_vec_u8_as_u32`  / `borsh_serialize_vec_u32_as_u8`  — repeated
///
fn widen_borsh_helpers() -> TokenStream {
    // (on_chain_type, rust_type, deserialize_suffix, serialize_suffix)
    let type_pairs: &[(&str, &str, &str, &str)] = &[
        ("u8", "u32", "u8_as_u32", "u32_as_u8"),
        ("u16", "u32", "u16_as_u32", "u32_as_u16"),
        ("i8", "i32", "i8_as_i32", "i32_as_i8"),
        ("i16", "i32", "i16_as_i32", "i32_as_i16"),
    ];

    let helper_fns: Vec<TokenStream> = type_pairs
        .iter()
        .map(
            |(on_chain_type, rust_type, deserialize_suffix, serialize_suffix)| {
                let on_chain_ty: syn::Type = syn::parse_str(on_chain_type).unwrap();
                let rust_ty: syn::Type = syn::parse_str(rust_type).unwrap();

                let (deserialize_singular, serialize_singular) = (
                    format_ident!("borsh_deserialize_{}", deserialize_suffix),
                    format_ident!("borsh_serialize_{}", serialize_suffix)
                );

                let (deserialize_optional, serialize_optional) = (
                    format_ident!("borsh_deserialize_opt_{}", deserialize_suffix),
                    format_ident!("borsh_serialize_opt_{}", serialize_suffix)
                );

                let (deserialize_repeated, serialize_repeated) = (
                    format_ident!("borsh_deserialize_vec_{}", deserialize_suffix),
                    format_ident!("borsh_serialize_vec_{}", serialize_suffix)
                );

                quote! {
                    fn #deserialize_singular<R: ::borsh::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<#rust_ty, ::borsh::io::Error> {
                        let val: #on_chain_ty = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                        ::core::result::Result::Ok(val as #rust_ty)
                    }

                    fn #serialize_singular<W: ::borsh::io::Write>(
                        val: &#rust_ty,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), ::borsh::io::Error> {
                        ::borsh::BorshSerialize::serialize(&(*val as #on_chain_ty), writer)
                    }

                    fn #deserialize_optional<R: ::borsh::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<::core::option::Option<#rust_ty>, ::borsh::io::Error> {
                        let opt: ::core::option::Option<#on_chain_ty> =
                            ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                        ::core::result::Result::Ok(opt.map(|v| v as #rust_ty))
                    }

                    fn #serialize_optional<W: ::borsh::io::Write>(
                        val: &::core::option::Option<#rust_ty>,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), ::borsh::io::Error> {
                        let narrowed = val.map(|v| v as #on_chain_ty);

                        ::borsh::BorshSerialize::serialize(&narrowed, writer)
                    }

                    fn #deserialize_repeated<R: ::borsh::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Vec<#rust_ty>, ::borsh::io::Error> {
                        let vec: Vec<#on_chain_ty> = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                        ::core::result::Result::Ok(vec.into_iter().map(|v| v as #rust_ty).collect())
                    }

                    fn #serialize_repeated<W: ::borsh::io::Write>(
                        val: &[#rust_ty],
                        writer: &mut W,
                    ) -> ::core::result::Result<(), ::borsh::io::Error> {
                        let narrowed: Vec<#on_chain_ty> = val.iter().map(|&v| v as #on_chain_ty).collect();

                        ::borsh::BorshSerialize::serialize(&narrowed, writer)
                    }
                }
            },
        )
        .collect();

    quote! { #(#helper_fns)* }
}
