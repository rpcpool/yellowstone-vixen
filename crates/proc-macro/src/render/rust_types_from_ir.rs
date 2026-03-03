use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

use crate::intermediate_representation::{
    FieldIr, FieldTypeIr, LabelIr, OneofIr, OneofKindIr, ScalarIr, TypeIr, TypeKindIr,
};

pub fn rust_types_from_ir(schema_ir: &crate::intermediate_representation::SchemaIr) -> TokenStream {
    let mut out = TokenStream::new();

    let oneof_parents: HashSet<&str> = schema_ir
        .oneofs
        .iter()
        .map(|oneof_ir| oneof_ir.parent_message.as_str())
        .collect();

    // Collect instruction-kind type names (these go inside the instruction module)
    let instruction_type_names: HashSet<&str> = schema_ir
        .types
        .iter()
        .filter(|t| t.kind == TypeKindIr::Instruction)
        .map(|t| t.name.as_str())
        .collect();

    // Render non-instruction types at top level (exclude oneof parents, rendered separately).
    // Use kind-based filtering (not name-based) so that defined types whose names
    // collide with instruction wrapper types are still rendered at the top level.
    for t in &schema_ir.types {
        if oneof_parents.contains(t.name.as_str()) {
            continue;
        }

        if t.kind == TypeKindIr::Instruction {
            continue;
        }

        out.extend(render_struct_type(t, None));
    }

    // Render oneof parent types + their modules/enums
    for oneof in &schema_ir.oneofs {
        match oneof.kind {
            OneofKindIr::InstructionDispatch => {
                let ix_types: Vec<&TypeIr> = schema_ir
                    .types
                    .iter()
                    .filter(|t| t.kind == TypeKindIr::Instruction)
                    .filter(|t| !oneof_parents.contains(t.name.as_str()))
                    .collect();

                out.extend(render_instruction_dispatch(
                    oneof,
                    &ix_types,
                    &instruction_type_names,
                ));
            },
            OneofKindIr::Enum => {
                out.extend(render_enum_oneof(oneof));
            },
        }
    }

    out
}

fn render_struct_type(t: &TypeIr, local_names: Option<&HashSet<&str>>) -> TokenStream {
    let ident = format_ident!("{}", t.name);

    let fields: Vec<_> = t
        .fields
        .iter()
        .map(|f| render_field(f, local_names))
        .collect();

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

///
/// Render instruction dispatch: module-wrapped.
///
/// Generates:
/// - `pub mod instruction { pub enum Instruction { ... } /* + payload types */ }`
/// - Wrapper struct `Instructions` with `instruction: Option<instruction::Instruction>`
/// - Custom Borsh impls for `Instructions`
///
fn render_instruction_dispatch(
    oneof_ir: &OneofIr,
    ix_types: &[&TypeIr],
    local_names: &HashSet<&str>,
) -> TokenStream {
    let parent_ident = format_ident!("{}", oneof_ir.parent_message); // "Instructions"
    let mod_ident = format_ident!("instruction");
    let oneof_ident = format_ident!("Instruction");
    let field_ident = format_ident!("{}", oneof_ir.field_name);

    // Render instruction types inside the module
    let module_types: Vec<TokenStream> = ix_types
        .iter()
        .map(|t| render_struct_type(t, Some(local_names)))
        .collect();

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

        let oneof_lit = LitStr::new("instruction::Instruction", Span::call_site());

        quote! {
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct #parent_ident {
                #[prost(oneof = #oneof_lit, tags = #tags_lit)]
                pub #field_ident: ::core::option::Option<#mod_ident::#oneof_ident>,
            }

            pub mod #mod_ident {
                #(#module_types)*

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
                #(#module_types)*

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

/// Render a single struct field.
///
/// `local_names`: when `Some`, we are inside a submodule. Message types not in the set
/// and borsh helper paths get `super::` prefixed.
pub fn render_field(f: &FieldIr, local_names: Option<&HashSet<&str>>) -> TokenStream {
    let name = format_ident!("{}", f.name);
    let tag = f.tag;
    let in_module = local_names.is_some();
    let path_prefix = if in_module { "super::" } else { "" };

    // PublicKey is rendered as a message type (PublicKey wrapper), not a scalar.
    let is_pubkey = matches!(&f.field_type, FieldTypeIr::Scalar(ScalarIr::PublicKey));

    // Custom borsh attrs for fields whose on-chain encoding differs from the Rust type
    let borsh_attr = {
        let fixed = fixed_bytes_borsh_attrs(&f.label, &f.field_type, path_prefix);

        if !fixed.is_empty() {
            fixed
        } else {
            let widen = widen_borsh_attrs(&f.label, &f.field_type, path_prefix);

            if !widen.is_empty() {
                widen
            } else {
                let float = float_borsh_attrs(&f.label, &f.field_type, path_prefix);

                if !float.is_empty() {
                    float
                } else {
                    // For FixedArray, we always need a custom borsh attr (no length prefix).
                    // If none of the specialized attrs matched, use the generic fixed-array helper.
                    fixed_array_default_borsh_attrs(&f.label, path_prefix)
                }
            }
        }
    };

    // Singular Message fields: on-chain the struct is required (no Option tag byte),
    // but prost requires `Option<T>` for message fields.
    // Note: PublicKey singular fields use their own borsh_deserialize_pubkey helper
    // which returns PublicKey directly (required), so they don't need required_msg_attr.
    let required_msg_attr = if matches!(
        (&f.label, &f.field_type),
        (LabelIr::Singular, FieldTypeIr::Message(_))
    ) {
        let deserialize_path = LitStr::new(
            &format!("{path_prefix}borsh_deserialize_required_msg"),
            Span::call_site(),
        );

        let serialize_path = LitStr::new(
            &format!("{path_prefix}borsh_serialize_required_msg"),
            Span::call_site(),
        );

        quote! {
            #[borsh(
                deserialize_with = #deserialize_path,
                serialize_with = #serialize_path
            )]
        }
    } else {
        quote! {}
    };

    // Resolve a Message type ident, adding `super::` when in a submodule and the type is external
    let resolve_msg = |msg: &str| -> TokenStream {
        let ident = format_ident!("{}", msg);

        if let Some(locals) = local_names {
            if locals.contains(msg) {
                quote!(#ident)
            } else {
                quote!(super::#ident)
            }
        } else {
            quote!(#ident)
        }
    };

    // Without proto, emit plain fields with no prost attributes
    if !cfg!(feature = "proto") {
        return match (&f.label, &f.field_type) {
            (LabelIr::Singular, FieldTypeIr::Message(msg)) => {
                let ty = resolve_msg(msg);

                quote! { #required_msg_attr pub #name: ::core::option::Option<#ty> }
            },
            (LabelIr::Singular, _) if is_pubkey => {
                let (_, rust_type) = map_ir_type_to_prost(&f.field_type, in_module);

                quote! { #borsh_attr pub #name: #rust_type }
            },
            (LabelIr::Singular, field_type) => {
                let (_, rust_type) = map_ir_type_to_prost(field_type, in_module);

                quote! { #borsh_attr pub #name: #rust_type }
            },
            (LabelIr::Optional, FieldTypeIr::Message(msg)) => {
                let ty = resolve_msg(msg);

                quote! { pub #name: ::core::option::Option<#ty> }
            },
            (LabelIr::Optional, field_type) => {
                let (_, rust_type) = map_ir_type_to_prost(field_type, in_module);

                quote! { #borsh_attr pub #name: ::core::option::Option<#rust_type> }
            },
            (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Message(msg)) => {
                let ty = resolve_msg(msg);

                quote! { #borsh_attr pub #name: Vec<#ty> }
            },
            (LabelIr::Repeated | LabelIr::FixedArray(_), field_type) => {
                let (_, rust_type) = map_ir_type_to_prost(field_type, in_module);

                quote! { #borsh_attr pub #name: Vec<#rust_type> }
            },
        };
    }

    match (&f.label, &f.field_type) {
        (LabelIr::Singular, FieldTypeIr::Message(msg)) => {
            let ty = resolve_msg(msg);

            quote! {
                #[prost(message, optional, tag = #tag)]
                #required_msg_attr
                pub #name: ::core::option::Option<#ty>
            }
        },

        (LabelIr::Singular, _) if is_pubkey => {
            let (_, rust_type) = map_ir_type_to_prost(&f.field_type, in_module);

            quote! {
                #[prost(message, required, tag = #tag)]
                #borsh_attr
                pub #name: #rust_type
            }
        },

        (LabelIr::Singular, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type, in_module);

            quote! {
                #[prost(#prost_type, tag = #tag)]
                #borsh_attr
                pub #name: #rust_type
            }
        },

        (LabelIr::Optional, FieldTypeIr::Message(msg)) => {
            let ty = resolve_msg(msg);

            quote! {
                #[prost(message, optional, tag = #tag)]
                pub #name: ::core::option::Option<#ty>
            }
        },

        (LabelIr::Optional, _) if is_pubkey => {
            let (_, rust_type) = map_ir_type_to_prost(&f.field_type, in_module);

            quote! {
                #[prost(message, optional, tag = #tag)]
                #borsh_attr
                pub #name: ::core::option::Option<#rust_type>
            }
        },

        (LabelIr::Optional, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type, in_module);

            quote! {
                #[prost(#prost_type, optional, tag = #tag)]
                #borsh_attr
                pub #name: ::core::option::Option<#rust_type>
            }
        },

        (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Message(msg)) => {
            let ty = resolve_msg(msg);

            quote! {
                #[prost(message, repeated, tag = #tag)]
                #borsh_attr
                pub #name: Vec<#ty>
            }
        },

        (LabelIr::Repeated | LabelIr::FixedArray(_), _) if is_pubkey => {
            let (_, rust_type) = map_ir_type_to_prost(&f.field_type, in_module);

            quote! {
                #[prost(message, repeated, tag = #tag)]
                #borsh_attr
                pub #name: Vec<#rust_type>
            }
        },

        (LabelIr::Repeated | LabelIr::FixedArray(_), field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type, in_module);

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
/// (Pubkey and FixedBytes), or an empty TokenStream for all other field types.
///
fn fixed_bytes_borsh_attrs(
    label: &LabelIr,
    field_type: &FieldTypeIr,
    path_prefix: &str,
) -> TokenStream {
    match field_type {
        FieldTypeIr::Scalar(ScalarIr::PublicKey) => {
            return pubkey_borsh_attrs(label, path_prefix);
        },
        FieldTypeIr::Scalar(ScalarIr::FixedBytes(_)) => {},
        _ => return quote! {},
    }

    let size = match field_type {
        FieldTypeIr::Scalar(ScalarIr::FixedBytes(n)) => *n,
        _ => unreachable!(),
    };

    let (deserialize_path, serialize_path) = (
        LitStr::new(
            &format!("{path_prefix}borsh_deserialize_fixed_bytes::<{size}, _>"),
            Span::call_site(),
        ),
        LitStr::new(
            &format!("{path_prefix}borsh_serialize_fixed_bytes::<{size}, _>"),
            Span::call_site(),
        ),
    );

    let (deserialize_opt_path, serialize_opt_path) = (
        LitStr::new(
            &format!("{path_prefix}borsh_deserialize_opt_fixed_bytes::<{size}, _>"),
            Span::call_site(),
        ),
        LitStr::new(
            &format!("{path_prefix}borsh_serialize_opt_fixed_bytes::<{size}, _>"),
            Span::call_site(),
        ),
    );

    let (deserialize_vec_path, serialize_vec_path) = (
        LitStr::new(
            &format!("{path_prefix}borsh_deserialize_vec_fixed_bytes::<{size}, _>"),
            Span::call_site(),
        ),
        LitStr::new(
            &format!("{path_prefix}borsh_serialize_vec_fixed_bytes::<{size}, _>"),
            Span::call_site(),
        ),
    );

    match label {
        LabelIr::Singular => quote! {
            #[borsh(
                deserialize_with = #deserialize_path,
                serialize_with = #serialize_path
            )]
        },
        LabelIr::Optional => quote! {
            #[borsh(
                deserialize_with = #deserialize_opt_path,
                serialize_with = #serialize_opt_path
            )]
        },
        LabelIr::Repeated => quote! {
            #[borsh(
                deserialize_with = #deserialize_vec_path,
                serialize_with = #serialize_vec_path
            )]
        },
        LabelIr::FixedArray(n) => {
            let d = LitStr::new(
                &format!(
                    "{path_prefix}borsh_deserialize_fixed_array_fixed_bytes::<{size}, {n}, _>"
                ),
                Span::call_site(),
            );

            let s = LitStr::new(
                &format!("{path_prefix}borsh_serialize_fixed_array_fixed_bytes::<{size}, {n}, _>"),
                Span::call_site(),
            );

            quote! { #[borsh(deserialize_with = #d, serialize_with = #s)] }
        },
    }
}

/// Returns borsh attrs for Pubkey fields, routing to Pubkey-wrapping helpers.
fn pubkey_borsh_attrs(label: &LabelIr, path_prefix: &str) -> TokenStream {
    let (d, s) = match label {
        LabelIr::Singular => (
            format!("{path_prefix}borsh_deserialize_pubkey"),
            format!("{path_prefix}borsh_serialize_pubkey"),
        ),
        LabelIr::Optional => (
            format!("{path_prefix}borsh_deserialize_opt_pubkey"),
            format!("{path_prefix}borsh_serialize_opt_pubkey"),
        ),
        LabelIr::Repeated => (
            format!("{path_prefix}borsh_deserialize_vec_pubkey"),
            format!("{path_prefix}borsh_serialize_vec_pubkey"),
        ),
        LabelIr::FixedArray(n) => (
            format!("{path_prefix}borsh_deserialize_fixed_array_pubkey::<{n}, _>"),
            format!("{path_prefix}borsh_serialize_fixed_array_pubkey::<{n}, _>"),
        ),
    };

    let d_lit = LitStr::new(&d, Span::call_site());
    let s_lit = LitStr::new(&s, Span::call_site());

    quote! {
        #[borsh(
            deserialize_with = #d_lit,
            serialize_with = #s_lit
        )]
    }
}

/// Returns `#[borsh(deserialize_with = "...", serialize_with = "...")]` for integer fields
/// that are widened from their on-chain size to a proto-compatible Rust type.
fn widen_borsh_attrs(label: &LabelIr, field_type: &FieldTypeIr, path_prefix: &str) -> TokenStream {
    let suffixes = match field_type {
        FieldTypeIr::Scalar(ScalarIr::U8) => ("u8_as_u32", "u32_as_u8"),
        FieldTypeIr::Scalar(ScalarIr::U16 | ScalarIr::ShortU16) => ("u16_as_u32", "u32_as_u16"),
        FieldTypeIr::Scalar(ScalarIr::I8) => ("i8_as_i32", "i32_as_i8"),
        FieldTypeIr::Scalar(ScalarIr::I16) => ("i16_as_i32", "i32_as_i16"),
        _ => return quote! {},
    };

    let (deserialize_fn_name, serialize_fn_name) = match label {
        LabelIr::Singular => (
            format!("{path_prefix}borsh_deserialize_{}", suffixes.0),
            format!("{path_prefix}borsh_serialize_{}", suffixes.1),
        ),
        LabelIr::Optional => (
            format!("{path_prefix}borsh_deserialize_opt_{}", suffixes.0),
            format!("{path_prefix}borsh_serialize_opt_{}", suffixes.1),
        ),
        LabelIr::Repeated => (
            format!("{path_prefix}borsh_deserialize_vec_{}", suffixes.0),
            format!("{path_prefix}borsh_serialize_vec_{}", suffixes.1),
        ),
        LabelIr::FixedArray(n) => (
            format!(
                "{path_prefix}borsh_deserialize_fixed_array_{}<{n}, _>",
                suffixes.0
            ),
            format!(
                "{path_prefix}borsh_serialize_fixed_array_{}<{n}, _>",
                suffixes.1
            ),
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

/// Returns `#[borsh(deserialize_with = "...", serialize_with = "...")]` for float fields
/// (f32, f64) that need permissive deserialization allowing NaN/Infinity values.
/// Standard borsh rejects NaN for portability, but on-chain data may contain them.
fn float_borsh_attrs(label: &LabelIr, field_type: &FieldTypeIr, path_prefix: &str) -> TokenStream {
    let suffix = match field_type {
        FieldTypeIr::Scalar(ScalarIr::Float) => "f32",
        FieldTypeIr::Scalar(ScalarIr::Double) => "f64",
        _ => return quote! {},
    };

    let (deserialize_fn_name, serialize_fn_name) = match label {
        LabelIr::Singular => (
            format!("{path_prefix}borsh_deserialize_{suffix}_permissive"),
            format!("{path_prefix}borsh_serialize_{suffix}_permissive"),
        ),
        LabelIr::Optional => (
            format!("{path_prefix}borsh_deserialize_opt_{suffix}_permissive"),
            format!("{path_prefix}borsh_serialize_opt_{suffix}_permissive"),
        ),
        LabelIr::Repeated => (
            format!("{path_prefix}borsh_deserialize_vec_{suffix}_permissive"),
            format!("{path_prefix}borsh_serialize_vec_{suffix}_permissive"),
        ),
        LabelIr::FixedArray(n) => (
            format!("{path_prefix}borsh_deserialize_fixed_array_{suffix}_permissive::<{n}, _>"),
            format!("{path_prefix}borsh_serialize_fixed_array_{suffix}_permissive::<{n}, _>"),
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

/// Returns borsh attrs for FixedArray fields whose element type has a standard BorshDeserialize
/// (i.e., not fixed-bytes, not widened, not floats). Returns empty for non-FixedArray labels.
fn fixed_array_default_borsh_attrs(label: &LabelIr, path_prefix: &str) -> TokenStream {
    let n = match label {
        LabelIr::FixedArray(n) => n,
        _ => return quote! {},
    };

    let deserialize_lit = LitStr::new(
        &format!("{path_prefix}borsh_deserialize_fixed_array::<_, {n}, _>"),
        Span::call_site(),
    );
    let serialize_lit = LitStr::new(
        &format!("{path_prefix}borsh_serialize_fixed_array::<_, {n}, _>"),
        Span::call_site(),
    );

    quote! {
        #[borsh(
            deserialize_with = #deserialize_lit,
            serialize_with = #serialize_lit
        )]
    }
}

/// Return (prost_type, rust_type). When `in_module`, `PublicKey` gets a `super::` prefix.
fn map_ir_type_to_prost(field_type: &FieldTypeIr, in_module: bool) -> (TokenStream, TokenStream) {
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
            ScalarIr::PublicKey => {
                if in_module {
                    (quote!(message), quote!(super::PublicKey))
                } else {
                    (quote!(message), quote!(PublicKey))
                }
            },
        },
        FieldTypeIr::Message(name) => {
            let ident = format_ident!("{}", name);
            (quote!(message), quote!(#ident))
        },
    }
}
