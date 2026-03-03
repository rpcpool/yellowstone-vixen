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

    let program_name_pascal = crate::utils::to_pascal_case(&idl.program.name);

    let proto_schema = if cfg!(feature = "proto") {
        let output = crate::render::proto_schema_string(
            &schema_ir,
            &program_mod_ident.to_string(),
            &program_name_pascal,
        );

        let proto_lit = syn::LitStr::new(&output.schema, proc_macro2::Span::call_site());

        let account_dispatch_const = match output.account_dispatch_index {
            Some(account_idx) => quote! {
                /// 0-based index of the account dispatch message in the proto file descriptor.
                pub const ACCOUNT_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(#account_idx);
            },
            None => quote! {
                /// 0-based index of the account dispatch message in the proto file descriptor.
                pub const ACCOUNT_DISPATCH_MESSAGE_INDEX: Option<usize> = None;
            },
        };

        let instruction_dispatch_const = match output.instruction_dispatch_index {
            Some(instruction_idx) => quote! {
                /// 0-based index of the instruction dispatch message in the proto file descriptor.
                pub const INSTRUCTION_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(#instruction_idx);
            },
            None => quote! {
                /// 0-based index of the instruction dispatch message in the proto file descriptor.
                pub const INSTRUCTION_DISPATCH_MESSAGE_INDEX: Option<usize> = None;
            },
        };

        quote! {
            /// Generated .proto schema for this program.
            pub const PROTOBUF_SCHEMA: &str = #proto_lit;

            #account_dispatch_const
            #instruction_dispatch_const
        }
    } else {
        quote! {}
    };

    let widen_helpers = widen_borsh_helpers();
    let fixed_array_widen_helpers = fixed_array_widen_borsh_helpers();

    quote! {
        pub mod #program_mod_ident {
            use yellowstone_vixen_parser::prelude::*;

            pub use yellowstone_vixen_core::PublicKey;

            /// Borsh: deserialize 32 bytes into a PublicKey wrapper (singular required field).
            fn borsh_deserialize_pubkey<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<PublicKey, ::borsh::io::Error> {
                let mut buf = [0u8; 32];

                reader.read_exact(&mut buf)?;

                ::core::result::Result::Ok(PublicKey::new(buf))
            }

            fn borsh_serialize_pubkey<W: ::borsh::io::Write>(
                val: &PublicKey,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                writer.write_all(&val.value)
            }

            fn borsh_deserialize_opt_pubkey<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<::core::option::Option<PublicKey>, ::borsh::io::Error> {
                let tag: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                match tag {
                    0 => ::core::result::Result::Ok(::core::option::Option::None),
                    1 => {
                        let mut buf = [0u8; 32];

                        reader.read_exact(&mut buf)?;

                        ::core::result::Result::Ok(::core::option::Option::Some(PublicKey::new(buf)))
                    }
                    _ => ::core::result::Result::Err(::borsh::io::Error::new(
                        ::borsh::io::ErrorKind::InvalidData,
                        "invalid option tag for pubkey",
                    )),
                }
            }

            fn borsh_serialize_opt_pubkey<W: ::borsh::io::Write>(
                val: &::core::option::Option<PublicKey>,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match val {
                    ::core::option::Option::Some(pk) => {
                        ::borsh::BorshSerialize::serialize(&1u8, writer)?;

                        writer.write_all(&pk.value)
                    }
                    ::core::option::Option::None => {
                        ::borsh::BorshSerialize::serialize(&0u8, writer)
                    }
                }
            }

            fn borsh_deserialize_vec_pubkey<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<PublicKey>, ::borsh::io::Error> {
                let len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                let mut result = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    let mut buf = [0u8; 32];

                    reader.read_exact(&mut buf)?;
                    result.push(PublicKey::new(buf));
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_vec_pubkey<W: ::borsh::io::Write>(
                val: &[PublicKey],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                ::borsh::BorshSerialize::serialize(&(val.len() as u32), writer)?;

                for pk in val {
                    writer.write_all(&pk.value)?;
                }

                ::core::result::Result::Ok(())
            }

            fn borsh_deserialize_fixed_array_pubkey<const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<PublicKey>, ::borsh::io::Error> {
                let mut result = Vec::with_capacity(N);

                for _ in 0..N {
                    let mut buf = [0u8; 32];

                    reader.read_exact(&mut buf)?;
                    result.push(PublicKey::new(buf));
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_fixed_array_pubkey<const N: usize, W: ::borsh::io::Write>(
                val: &[PublicKey],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                for pk in val {
                    writer.write_all(&pk.value)?;
                }

                ::core::result::Result::Ok(())
            }

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

            /// Borsh: deserialize a fixed-count array (no length prefix on-chain).
            /// Uses standard BorshDeserialize for each element.
            fn borsh_deserialize_fixed_array<T: ::borsh::BorshDeserialize, const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<T>, ::borsh::io::Error> {
                let mut result = Vec::with_capacity(N);

                for _ in 0..N {
                    result.push(<T as ::borsh::BorshDeserialize>::deserialize_reader(reader)?);
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_fixed_array<T: ::borsh::BorshSerialize, const N: usize, W: ::borsh::io::Write>(
                val: &[T],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                for v in val {
                    <T as ::borsh::BorshSerialize>::serialize(v, writer)?;
                }

                ::core::result::Result::Ok(())
            }

            /// Borsh: deserialize a fixed-count array of fixed-size byte fields (no length prefix).
            fn borsh_deserialize_fixed_array_fixed_bytes<const BYTE_SIZE: usize, const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<Vec<u8>>, ::borsh::io::Error> {
                let mut result = Vec::with_capacity(N);

                for _ in 0..N {
                    let mut buf = [0u8; BYTE_SIZE];

                    reader.read_exact(&mut buf)?;

                    result.push(buf.to_vec());
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_fixed_array_fixed_bytes<const BYTE_SIZE: usize, const N: usize, W: ::borsh::io::Write>(
                val: &[Vec<u8>],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                for v in val {
                    writer.write_all(v)?;
                }

                ::core::result::Result::Ok(())
            }

            #(#fixed_array_widen_helpers)*

            /// Borsh: deserialize a fixed-count array of f32 permissively (no length prefix, allows NaN).
            fn borsh_deserialize_fixed_array_f32_permissive<const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<f32>, ::borsh::io::Error> {
                let mut result = Vec::with_capacity(N);

                for _ in 0..N {
                    let mut buf = [0u8; 4];

                    reader.read_exact(&mut buf)?;

                    result.push(f32::from_le_bytes(buf));
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_fixed_array_f32_permissive<const N: usize, W: ::borsh::io::Write>(
                val: &[f32],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                for v in val {
                    writer.write_all(&v.to_le_bytes())?;
                }

                ::core::result::Result::Ok(())
            }

            fn borsh_deserialize_fixed_array_f64_permissive<const N: usize, R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<f64>, ::borsh::io::Error> {
                let mut result = Vec::with_capacity(N);

                for _ in 0..N {
                    let mut buf = [0u8; 8];

                    reader.read_exact(&mut buf)?;

                    result.push(f64::from_le_bytes(buf));
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_fixed_array_f64_permissive<const N: usize, W: ::borsh::io::Write>(
                val: &[f64],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                for v in val {
                    writer.write_all(&v.to_le_bytes())?;
                }

                ::core::result::Result::Ok(())
            }

            /// Borsh: deserialize f32 permissively (allows NaN/Infinity).
            /// Standard borsh rejects NaN for portability, but on-chain data may contain them.
            fn borsh_deserialize_f32_permissive<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<f32, ::borsh::io::Error> {
                let mut buf = [0u8; 4];

                reader.read_exact(&mut buf)?;

                ::core::result::Result::Ok(f32::from_le_bytes(buf))
            }

            fn borsh_serialize_f32_permissive<W: ::borsh::io::Write>(
                val: &f32,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                writer.write_all(&val.to_le_bytes())
            }

            fn borsh_deserialize_opt_f32_permissive<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<::core::option::Option<f32>, ::borsh::io::Error> {
                let tag: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                match tag {
                    0 => ::core::result::Result::Ok(::core::option::Option::None),
                    1 => {
                        let mut buf = [0u8; 4];

                        reader.read_exact(&mut buf)?;

                        ::core::result::Result::Ok(::core::option::Option::Some(f32::from_le_bytes(buf)))
                    }
                    _ => ::core::result::Result::Err(::borsh::io::Error::new(
                        ::borsh::io::ErrorKind::InvalidData,
                        "invalid option tag for f32",
                    )),
                }
            }

            fn borsh_serialize_opt_f32_permissive<W: ::borsh::io::Write>(
                val: &::core::option::Option<f32>,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match val {
                    ::core::option::Option::Some(v) => {
                        ::borsh::BorshSerialize::serialize(&1u8, writer)?;
                        writer.write_all(&v.to_le_bytes())
                    }
                    ::core::option::Option::None => {
                        ::borsh::BorshSerialize::serialize(&0u8, writer)
                    }
                }
            }

            fn borsh_deserialize_vec_f32_permissive<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<f32>, ::borsh::io::Error> {
                let len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                let mut result = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    let mut buf = [0u8; 4];

                    reader.read_exact(&mut buf)?;

                    result.push(f32::from_le_bytes(buf));
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_vec_f32_permissive<W: ::borsh::io::Write>(
                val: &[f32],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                ::borsh::BorshSerialize::serialize(&(val.len() as u32), writer)?;

                for v in val {
                    writer.write_all(&v.to_le_bytes())?;
                }

                ::core::result::Result::Ok(())
            }

            /// Borsh: deserialize f64 permissively (allows NaN/Infinity).
            fn borsh_deserialize_f64_permissive<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<f64, ::borsh::io::Error> {
                let mut buf = [0u8; 8];

                reader.read_exact(&mut buf)?;

                ::core::result::Result::Ok(f64::from_le_bytes(buf))
            }

            fn borsh_serialize_f64_permissive<W: ::borsh::io::Write>(
                val: &f64,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                writer.write_all(&val.to_le_bytes())
            }

            fn borsh_deserialize_opt_f64_permissive<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<::core::option::Option<f64>, ::borsh::io::Error> {
                let tag: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                match tag {
                    0 => ::core::result::Result::Ok(::core::option::Option::None),
                    1 => {
                        let mut buf = [0u8; 8];

                        reader.read_exact(&mut buf)?;

                        ::core::result::Result::Ok(::core::option::Option::Some(f64::from_le_bytes(buf)))
                    }
                    _ => ::core::result::Result::Err(::borsh::io::Error::new(
                        ::borsh::io::ErrorKind::InvalidData,
                        "invalid option tag for f64",
                    )),
                }
            }

            fn borsh_serialize_opt_f64_permissive<W: ::borsh::io::Write>(
                val: &::core::option::Option<f64>,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match val {
                    ::core::option::Option::Some(v) => {
                        ::borsh::BorshSerialize::serialize(&1u8, writer)?;

                        writer.write_all(&v.to_le_bytes())
                    }
                    ::core::option::Option::None => {
                        ::borsh::BorshSerialize::serialize(&0u8, writer)
                    }
                }
            }

            fn borsh_deserialize_vec_f64_permissive<R: ::borsh::io::Read>(
                reader: &mut R,
            ) -> ::core::result::Result<Vec<f64>, ::borsh::io::Error> {
                let len: u32 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                let mut result = Vec::with_capacity(len as usize);

                for _ in 0..len {
                    let mut buf = [0u8; 8];

                    reader.read_exact(&mut buf)?;
                    result.push(f64::from_le_bytes(buf));
                }

                ::core::result::Result::Ok(result)
            }

            fn borsh_serialize_vec_f64_permissive<W: ::borsh::io::Write>(
                val: &[f64],
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                ::borsh::BorshSerialize::serialize(&(val.len() as u32), writer)?;

                for v in val {
                    writer.write_all(&v.to_le_bytes())?;
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

/// Generate borsh helpers for fixed-count arrays of widened integer types.
/// These read exactly N narrow values without a length prefix, widening each to the proto type.
fn fixed_array_widen_borsh_helpers() -> Vec<TokenStream> {
    let type_pairs: &[(&str, &str, &str, &str)] = &[
        ("u8", "u32", "u8_as_u32", "u32_as_u8"),
        ("u16", "u32", "u16_as_u32", "u32_as_u16"),
        ("i8", "i32", "i8_as_i32", "i32_as_i8"),
        ("i16", "i32", "i16_as_i32", "i32_as_i16"),
    ];

    type_pairs
        .iter()
        .map(
            |(on_chain_type, rust_type, deserialize_suffix, serialize_suffix)| {
                let on_chain_ty: syn::Type = syn::parse_str(on_chain_type).unwrap();
                let rust_ty: syn::Type = syn::parse_str(rust_type).unwrap();

                let (deserialize_fn, serialize_fn) = (
                    format_ident!("borsh_deserialize_fixed_array_{}", deserialize_suffix),
                    format_ident!("borsh_serialize_fixed_array_{}", serialize_suffix),
                );

                quote! {
                    fn #deserialize_fn<const N: usize, R: ::borsh::io::Read>(
                        reader: &mut R,
                    ) -> ::core::result::Result<Vec<#rust_ty>, ::borsh::io::Error> {
                        let mut result = Vec::with_capacity(N);

                        for _ in 0..N {
                            let val: #on_chain_ty = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                            result.push(val as #rust_ty);
                        }

                        ::core::result::Result::Ok(result)
                    }

                    fn #serialize_fn<const N: usize, W: ::borsh::io::Write>(
                        val: &[#rust_ty],
                        writer: &mut W,
                    ) -> ::core::result::Result<(), ::borsh::io::Error> {
                        for &v in val {
                            ::borsh::BorshSerialize::serialize(&(v as #on_chain_ty), writer)?;
                        }

                        ::core::result::Result::Ok(())
                    }
                }
            },
        )
        .collect()
}
