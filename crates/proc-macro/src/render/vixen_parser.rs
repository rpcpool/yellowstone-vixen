use codama_nodes::RootNode;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn vixen_parser(idl: &RootNode, events: &[codama_nodes::InstructionNode]) -> TokenStream {
    let program_mod_ident = format_ident!("{}", crate::utils::to_snake_case(&idl.program.name));

    let program_pubkey = crate::render::program_pubkey(&idl.program.public_key);

    let schema_ir = crate::intermediate_representation::build_schema_ir(idl, events);

    let schema_types = crate::render::rust_types_from_ir(&schema_ir);

    let account_parser = crate::render::account_parser(&idl.program.name, &idl.program.accounts);

    let instruction_parser =
        crate::render::instruction_parser(&idl.program.name, &idl.program.instructions);

    let event_parser = if cfg!(feature = "program-events") && !events.is_empty() {
        crate::render::event_parser(&idl.program.name, events)
    } else {
        quote! {}
    };

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

        let program_event_output_const = match output.program_event_output_index {
            Some(idx) => quote! {
                /// 0-based index of the ProgramEventOutput wrapper message in the proto file descriptor.
                pub const PROGRAM_EVENT_OUTPUT_MESSAGE_INDEX: Option<usize> = Some(#idx);
            },
            None => quote! {
                /// 0-based index of the ProgramEventOutput wrapper message in the proto file descriptor.
                pub const PROGRAM_EVENT_OUTPUT_MESSAGE_INDEX: Option<usize> = None;
            },
        };

        quote! {
            /// Generated .proto schema for this program.
            pub const PROTOBUF_SCHEMA: &str = #proto_lit;

            #account_dispatch_const
            #instruction_dispatch_const
            #program_event_output_const
        }
    } else {
        quote! {}
    };

    quote! {
        pub mod #program_mod_ident {
            use yellowstone_vixen_parser::prelude::*;

            pub use yellowstone_vixen_core::Pubkey;

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
            #event_parser
        }
    }
}
