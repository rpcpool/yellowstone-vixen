use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

use crate::intermediate_representation::{
    FieldIr, FieldTypeIr, LabelIr, OneofIr, OneofKindIr, ScalarIr, TypeIr,
};

pub fn rust_types_from_ir(schema_ir: &crate::intermediate_representation::SchemaIr) -> TokenStream {
    let mut out = TokenStream::new();

    let oneof_parents: HashSet<&str> = schema_ir
        .oneofs
        .iter()
        .map(|oneof_ir| oneof_ir.parent_message.as_str())
        .collect();

    // Render regular types (exclude oneof parents; we render those later with oneof field)
    for t in &schema_ir.types {
        if oneof_parents.contains(t.name.as_str()) {
            continue;
        }

        out.extend(render_type(t));
    }

    // Render oneof parent types + their modules/enums
    for oneof in &schema_ir.oneofs {
        out.extend(render_oneof_parent(oneof));
    }

    out
}

fn render_type(t: &TypeIr) -> TokenStream {
    let ident = format_ident!("{}", t.name);
    let fields: Vec<_> = t.fields.iter().map(render_field).collect();

    if cfg!(feature = "proto") {
        quote! {
            #[derive(Clone, PartialEq, ::borsh::BorshDeserialize, ::borsh::BorshSerialize, ::prost::Message)]
            pub struct #ident {
                #(#fields),*
            }
        }
    } else {
        quote! {
            #[derive(Clone, Debug, PartialEq, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
            pub struct #ident {
                #(#fields),*
            }
        }
    }
}

fn render_oneof_parent(oneof_ir: &OneofIr) -> TokenStream {
    match oneof_ir.kind {
        OneofKindIr::InstructionDispatch => render_instruction_dispatch(oneof_ir),
        OneofKindIr::Enum => render_enum_oneof(oneof_ir),
    }
}

/// Render instruction dispatch: flattened (no module wrapper).
///
/// The parent struct (`Instructions`) and the `Instruction` enum live at the same level.
fn render_instruction_dispatch(oneof_ir: &OneofIr) -> TokenStream {
    let parent_ident = format_ident!("{}", oneof_ir.parent_message); // "Instructions"
    let oneof_ident = format_ident!("Instruction");
    let field_ident = format_ident!("{}", oneof_ir.field_name);

    let variants = oneof_ir.variants.iter().map(|v| {
        let v_ident = format_ident!("{}", v.variant_name);
        let msg_ident = format_ident!("{}", v.message_type);
        let tag = v.tag;

        if cfg!(feature = "proto") {
            quote! {
                #[prost(message, tag = #tag)]
                #v_ident(#msg_ident)
            }
        } else {
            quote! {
                #v_ident(#msg_ident)
            }
        }
    });

    let borsh_serialize_arms = oneof_ir.variants.iter().enumerate().map(|(i, v)| {
        let disc = i as u8;
        let v_ident = format_ident!("{}", v.variant_name);

        quote! {
            ::core::option::Option::Some(#oneof_ident::#v_ident(v)) => {
                ::borsh::BorshSerialize::serialize(&#disc, writer)?;
                ::borsh::BorshSerialize::serialize(v, writer)
            }
        }
    });

    let borsh_deserialize_arms = oneof_ir.variants.iter().enumerate().map(|(i, v)| {
        let disc = i as u8;
        let v_ident = format_ident!("{}", v.variant_name);

        quote! {
            #disc => {
                let v = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                ::core::option::Option::Some(#oneof_ident::#v_ident(v))
            }
        }
    });

    let struct_and_enum = if cfg!(feature = "proto") {
        let tags_lit = {
            let tags_list = oneof_ir
                .variants
                .iter()
                .map(|v| v.tag.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            LitStr::new(&tags_list, Span::call_site())
        };

        let oneof_lit = LitStr::new("Instruction", Span::call_site());

        quote! {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct #parent_ident {
                #[prost(oneof = #oneof_lit, tags = #tags_lit)]
                pub #field_ident: ::core::option::Option<#oneof_ident>,
            }

            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum #oneof_ident {
                #(#variants),*
            }
        }
    } else {
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct #parent_ident {
                pub #field_ident: ::core::option::Option<#oneof_ident>,
            }

            #[derive(Clone, Debug, PartialEq)]
            pub enum #oneof_ident {
                #(#variants),*
            }
        }
    };

    quote! {
        #struct_and_enum

        impl ::borsh::BorshSerialize for #parent_ident {
            fn serialize<W: ::borsh::io::Write>(
                &self,
                writer: &mut W
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match &self.#field_ident {
                    #(#borsh_serialize_arms,)*

                    ::core::option::Option::None => {
                        ::core::result::Result::Err(::borsh::io::Error::new(
                            ::borsh::io::ErrorKind::InvalidData,
                            "oneof field is None"
                        ))
                    }
                }
            }
        }

        impl ::borsh::BorshDeserialize for #parent_ident {
            fn deserialize_reader<R: ::borsh::io::Read>(
                reader: &mut R
            ) -> ::core::result::Result<Self, ::borsh::io::Error> {
                let disc: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                let #field_ident = match disc {
                    #(#borsh_deserialize_arms,)*

                    _ => {
                        return ::core::result::Result::Err(::borsh::io::Error::new(
                            ::borsh::io::ErrorKind::InvalidData,
                            "invalid discriminant"
                        ));
                    }
                };
                ::core::result::Result::Ok(Self { #field_ident })
            }
        }
    }
}

/// Render user-defined enum oneofs: keeps the module wrapper pattern.
fn render_enum_oneof(oneof_ir: &OneofIr) -> TokenStream {
    let parent_ident = format_ident!("{}", oneof_ir.parent_message);
    let mod_ident = format_ident!("{}", crate::utils::to_snake_case(&oneof_ir.parent_message));
    let oneof_ident = format_ident!("Kind");
    let field_ident = format_ident!("{}", oneof_ir.field_name);

    let variants = oneof_ir.variants.iter().map(|v| {
        let v_ident = format_ident!("{}", v.variant_name);
        let msg_ident = format_ident!("{}", v.message_type);
        let tag = v.tag;

        if cfg!(feature = "proto") {
            quote! {
                #[prost(message, tag = #tag)]
                #v_ident(super::#msg_ident)
            }
        } else {
            quote! {
                #v_ident(super::#msg_ident)
            }
        }
    });

    let borsh_serialize_arms = oneof_ir.variants.iter().enumerate().map(|(i, v)| {
        let disc = i as u8;
        let v_ident = format_ident!("{}", v.variant_name);

        quote! {
            ::core::option::Option::Some(#mod_ident::#oneof_ident::#v_ident(v)) => {
                ::borsh::BorshSerialize::serialize(&#disc, writer)?;
                ::borsh::BorshSerialize::serialize(v, writer)
            }
        }
    });

    let borsh_deserialize_arms = oneof_ir.variants.iter().enumerate().map(|(i, v)| {
        let disc = i as u8;
        let v_ident = format_ident!("{}", v.variant_name);

        quote! {
            #disc => {
                let v = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                ::core::option::Option::Some(#mod_ident::#oneof_ident::#v_ident(v))
            }
        }
    });

    let struct_and_mod = if cfg!(feature = "proto") {
        let tags_lit = {
            let tags_list = oneof_ir
                .variants
                .iter()
                .map(|v| v.tag.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            LitStr::new(&tags_list, Span::call_site())
        };

        let oneof_lit = {
            let oneof_path = format!("{}::{}", mod_ident, oneof_ident);

            LitStr::new(&oneof_path, Span::call_site())
        };

        quote! {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct #parent_ident {
                #[prost(oneof = #oneof_lit, tags = #tags_lit)]
                pub #field_ident: ::core::option::Option<#mod_ident::#oneof_ident>,
            }

            pub mod #mod_ident {
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum #oneof_ident {
                    #(#variants),*
                }
            }
        }
    } else {
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct #parent_ident {
                pub #field_ident: ::core::option::Option<#mod_ident::#oneof_ident>,
            }

            pub mod #mod_ident {
                #[derive(Clone, Debug, PartialEq)]
                pub enum #oneof_ident {
                    #(#variants),*
                }
            }
        }
    };

    quote! {
        #struct_and_mod

        impl ::borsh::BorshSerialize for #parent_ident {
            fn serialize<W: ::borsh::io::Write>(
                &self,
                writer: &mut W
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match &self.#field_ident {
                    #(#borsh_serialize_arms,)*

                    ::core::option::Option::None => {
                        ::core::result::Result::Err(::borsh::io::Error::new(
                            ::borsh::io::ErrorKind::InvalidData,
                            "oneof field is None"
                        ))
                    }
                }
            }
        }

        impl ::borsh::BorshDeserialize for #parent_ident {
            fn deserialize_reader<R: ::borsh::io::Read>(
                reader: &mut R
            ) -> ::core::result::Result<Self, ::borsh::io::Error> {
                let disc: u8 = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                let #field_ident = match disc {
                    #(#borsh_deserialize_arms,)*

                    _ => {
                        return ::core::result::Result::Err(::borsh::io::Error::new(
                            ::borsh::io::ErrorKind::InvalidData,
                            "invalid discriminant"
                        ));
                    }
                };
                ::core::result::Result::Ok(Self { #field_ident })
            }
        }
    }
}

pub fn render_field(f: &FieldIr) -> TokenStream {
    let name = format_ident!("{}", f.name);
    let tag = f.tag;

    // Custom borsh attrs for fields whose on-chain encoding differs from the Rust type:
    // - FixedBytes/PubkeyBytes: on-chain fixed N bytes, Rust Vec<u8> (borsh reads length prefix)
    // - Widened integers: on-chain u8/u16/i8/i16, Rust u32/i32 (borsh reads wrong byte count)
    let borsh_attr = {
        let fixed = fixed_bytes_borsh_attrs(&f.label, &f.field_type);

        if fixed.is_empty() {
            widen_borsh_attrs(&f.label, &f.field_type)
        } else {
            fixed
        }
    };

    // Singular Message fields: on-chain the struct is required (no Option tag byte),
    // but prost requires `Option<T>` for message fields. We override borsh to read
    // the struct directly and wrap in Some.
    let required_msg_attr = if matches!(
        (&f.label, &f.field_type),
        (LabelIr::Singular, FieldTypeIr::Message(_))
    ) {
        quote! {
            #[borsh(
                deserialize_with = "borsh_deserialize_required_msg",
                serialize_with = "borsh_serialize_required_msg"
            )]
        }
    } else {
        quote! {}
    };

    // Without proto, emit plain fields with no prost attributes
    if !cfg!(feature = "proto") {
        return match (&f.label, &f.field_type) {
            (LabelIr::Singular, FieldTypeIr::Message(msg)) => {
                let ident = format_ident!("{}", msg);

                quote! { #required_msg_attr pub #name: ::core::option::Option<#ident> }
            },
            (LabelIr::Singular, field_type) => {
                let (_, rust_type) = map_ir_type_to_prost(field_type);

                quote! { #borsh_attr pub #name: #rust_type }
            },
            (LabelIr::Optional, FieldTypeIr::Message(msg)) => {
                let ident = format_ident!("{}", msg);

                quote! { pub #name: ::core::option::Option<#ident> }
            },
            (LabelIr::Optional, field_type) => {
                let (_, rust_type) = map_ir_type_to_prost(field_type);

                quote! { #borsh_attr pub #name: ::core::option::Option<#rust_type> }
            },
            (LabelIr::Repeated, FieldTypeIr::Message(msg)) => {
                let ident = format_ident!("{}", msg);

                quote! { pub #name: Vec<#ident> }
            },
            (LabelIr::Repeated, field_type) => {
                let (_, rust_type) = map_ir_type_to_prost(field_type);

                quote! { #borsh_attr pub #name: Vec<#rust_type> }
            },
        };
    }

    match (&f.label, &f.field_type) {
        (LabelIr::Singular, FieldTypeIr::Message(msg)) => {
            let ident = format_ident!("{}", msg);

            quote! {
                #[prost(message, optional, tag = #tag)]
                #required_msg_attr
                pub #name: ::core::option::Option<#ident>
            }
        },

        (LabelIr::Singular, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type);

            quote! {
                #[prost(#prost_type, tag = #tag)]
                #borsh_attr
                pub #name: #rust_type
            }
        },

        (LabelIr::Optional, FieldTypeIr::Message(msg)) => {
            let ident = format_ident!("{}", msg);

            quote! {
                #[prost(message, optional, tag = #tag)]
                pub #name: ::core::option::Option<#ident>
            }
        },

        (LabelIr::Optional, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type);

            quote! {
                #[prost(#prost_type, optional, tag = #tag)]
                #borsh_attr
                pub #name: ::core::option::Option<#rust_type>
            }
        },

        (LabelIr::Repeated, FieldTypeIr::Message(msg)) => {
            let ident = format_ident!("{}", msg);

            quote! {
                #[prost(message, repeated, tag = #tag)]
                pub #name: Vec<#ident>
            }
        },

        (LabelIr::Repeated, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type);

            quote! {
                #[prost(#prost_type, repeated, tag = #tag)]
                #borsh_attr
                pub #name: Vec<#rust_type>
            }
        },
    }
}

///
/// Returns `#[borsh(deserialize_with = "...", serialize_with = "...")]` for fixed-size byte fields
/// (PubkeyBytes and FixedBytes), or an empty TokenStream for all other field types.
///
/// Uses const-generic helpers like `borsh_deserialize_fixed_bytes::<32>` so a single set of functions
/// handles any fixed byte size.
///
fn fixed_bytes_borsh_attrs(label: &LabelIr, field_type: &FieldTypeIr) -> TokenStream {
    let size: usize = match field_type {
        FieldTypeIr::Scalar(ScalarIr::PubkeyBytes) => 32,
        FieldTypeIr::Scalar(ScalarIr::FixedBytes(n)) => *n,
        _ => return quote! {},
    };

    let deser_path = LitStr::new(
        &format!("borsh_deserialize_fixed_bytes::<{size}, _>"),
        Span::call_site(),
    );
    let ser_path = LitStr::new(
        &format!("borsh_serialize_fixed_bytes::<{size}, _>"),
        Span::call_site(),
    );
    let deser_opt_path = LitStr::new(
        &format!("borsh_deserialize_opt_fixed_bytes::<{size}, _>"),
        Span::call_site(),
    );
    let ser_opt_path = LitStr::new(
        &format!("borsh_serialize_opt_fixed_bytes::<{size}, _>"),
        Span::call_site(),
    );
    let deser_vec_path = LitStr::new(
        &format!("borsh_deserialize_vec_fixed_bytes::<{size}, _>"),
        Span::call_site(),
    );
    let ser_vec_path = LitStr::new(
        &format!("borsh_serialize_vec_fixed_bytes::<{size}, _>"),
        Span::call_site(),
    );

    match label {
        LabelIr::Singular => quote! {
            #[borsh(
                deserialize_with = #deser_path,
                serialize_with = #ser_path
            )]
        },
        LabelIr::Optional => quote! {
            #[borsh(
                deserialize_with = #deser_opt_path,
                serialize_with = #ser_opt_path
            )]
        },
        LabelIr::Repeated => quote! {
            #[borsh(
                deserialize_with = #deser_vec_path,
                serialize_with = #ser_vec_path
            )]
        },
    }
}

/// Returns `#[borsh(deserialize_with = "...", serialize_with = "...")]` for integer fields
/// that are widened from their on-chain size to a proto-compatible Rust type
/// (e.g. u8 → u32, i16 → i32), or an empty TokenStream if no widening is needed.
fn widen_borsh_attrs(label: &LabelIr, field_type: &FieldTypeIr) -> TokenStream {
    let suffixes = match field_type {
        FieldTypeIr::Scalar(ScalarIr::U8) => ("u8_as_u32", "u32_as_u8"),
        FieldTypeIr::Scalar(ScalarIr::U16 | ScalarIr::ShortU16) => ("u16_as_u32", "u32_as_u16"),
        FieldTypeIr::Scalar(ScalarIr::I8) => ("i8_as_i32", "i32_as_i8"),
        FieldTypeIr::Scalar(ScalarIr::I16) => ("i16_as_i32", "i32_as_i16"),
        _ => return quote! {},
    };

    let (deserialize_fn_name, serialize_fn_name) = match label {
        LabelIr::Singular => (
            format!("borsh_deserialize_{}", suffixes.0),
            format!("borsh_serialize_{}", suffixes.1),
        ),
        LabelIr::Optional => (
            format!("borsh_deserialize_opt_{}", suffixes.0),
            format!("borsh_serialize_opt_{}", suffixes.1),
        ),
        LabelIr::Repeated => (
            format!("borsh_deserialize_vec_{}", suffixes.0),
            format!("borsh_serialize_vec_{}", suffixes.1),
        ),
    };

    let deserialize_lit = LitStr::new(&deserialize_fn_name, Span::call_site());
    let serialize_lit = LitStr::new(&serialize_fn_name, Span::call_site());

    quote! {
        #[borsh(
            deserialize_with = #deserialize_lit,
            serialize_with = #serialize_lit
        )]
    }
}

/// Return (prost_type, rust_type)
fn map_ir_type_to_prost(field_type: &FieldTypeIr) -> (TokenStream, TokenStream) {
    match field_type {
        FieldTypeIr::Scalar(s) => match s {
            ScalarIr::Bool => (quote!(bool), quote!(bool)),
            ScalarIr::U8 | ScalarIr::U16 | ScalarIr::ShortU16 | ScalarIr::Uint32 => {
                (quote!(uint32), quote!(u32))
            },
            ScalarIr::Uint64 => (quote!(uint64), quote!(u64)),
            ScalarIr::I8 | ScalarIr::I16 | ScalarIr::Int32 => (quote!(int32), quote!(i32)),
            ScalarIr::Int64 => (quote!(int64), quote!(i64)),
            ScalarIr::Float => (quote!(float), quote!(f32)),
            ScalarIr::Double => (quote!(double), quote!(f64)),
            ScalarIr::String => (quote!(string), quote!(String)),
            ScalarIr::Bytes => (quote!(bytes = "vec"), quote!(Vec<u8>)),
            ScalarIr::FixedBytes(_) => (quote!(bytes = "vec"), quote!(Vec<u8>)),
            ScalarIr::PubkeyBytes => (quote!(bytes = "vec"), quote!(PubkeyBytes)),
        },
        FieldTypeIr::Message(name) => {
            let ident = format_ident!("{}", name);

            (quote!(message), quote!(#ident))
        },
    }
}
