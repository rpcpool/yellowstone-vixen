use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::LitStr;

use crate::intermediate_representation::{
    FieldIr, FieldTypeIr, LabelIr, OneofIr, OneofKindIr, ScalarIr, TypeIr, TypeKindIr,
};

///
/// Generate a manual `prost::Message` impl for a oneof wrapper struct.
///
/// The struct has a single non-Option field (`field_ident`) of type `mod_ident::oneof_ident`
/// which derives `prost::Oneof`. We implement Message by delegating to the Oneof methods
/// (calling Oneof generated methods via `self.field_ident`).
///
fn manual_prost_message_impl(
    parent_ident: &syn::Ident,
    field_ident: &syn::Ident,
    mod_ident: &syn::Ident,
    oneof_ident: &syn::Ident,
) -> TokenStream {
    quote! {
        impl ::core::fmt::Debug for #parent_ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#parent_ident))
                    .field(stringify!(#field_ident), &self.#field_ident)
                    .finish()
            }
        }

        impl ::prost::Message for #parent_ident {
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                self.#field_ident.encode(buf);
            }

            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                // Oneof::merge() requires `&mut Option<Self>`, so we wrap our non-Option
                // field into Some, call merge, then unwrap back.
                let mut opt = ::core::option::Option::Some(self.#field_ident.clone());

                #mod_ident::#oneof_ident::merge(&mut opt, tag, wire_type, buf, ctx)?;

                if let ::core::option::Option::Some(v) = opt {
                    self.#field_ident = v;
                }

                ::core::result::Result::Ok(())
            }

            fn encoded_len(&self) -> usize {
                self.#field_ident.encoded_len()
            }

            fn clear(&mut self) {}
        }
    }
}

///
/// Generate a manual `prost::Oneof` impl for the `Instruction` enum with
/// struct variants (`Swap { accounts, args }` instead of `Swap(Swap)`).
///
/// Each variant encodes/decodes as a nested message with `accounts` at tag 1
/// and `args` at tag 2 — identical wire format to the wrapper struct approach.
///
fn manual_prost_oneof_impl(
    oneof_ir: &OneofIr,
    mod_ident: &syn::Ident,
    oneof_ident: &syn::Ident,
) -> TokenStream {
    let encode_arms: Vec<TokenStream> = oneof_ir
        .variants
        .iter()
        .map(|v| {
            let v_ident = format_ident!("{}", v.variant_name);
            let tag = v.tag;

            quote! {
                #mod_ident::#oneof_ident::#v_ident { accounts, args } => {
                    let body_len =
                        ::prost::encoding::message::encoded_len(1, accounts)
                        + ::prost::encoding::message::encoded_len(2, args);

                    ::prost::encoding::encode_key(
                        #tag,
                        ::prost::encoding::WireType::LengthDelimited,
                        buf,
                    );
                    ::prost::encoding::encode_varint(body_len as u64, buf);
                    ::prost::encoding::message::encode(1, accounts, buf);
                    ::prost::encoding::message::encode(2, args, buf);
                }
            }
        })
        .collect();

    let merge_arms: Vec<TokenStream> = oneof_ir
        .variants
        .iter()
        .map(|v| {
            let v_ident = format_ident!("{}", v.variant_name);
            let msg_ident = format_ident!("{}", v.message_type);
            let tag = v.tag;

            quote! {
                #tag => {
                    let mut wrapper: #mod_ident::#msg_ident = match field.take() {
                        ::core::option::Option::Some(
                            #mod_ident::#oneof_ident::#v_ident { accounts, args }
                        ) => #mod_ident::#msg_ident { accounts, args },

                        _ => ::core::default::Default::default(),
                    };

                    ::prost::encoding::message::merge(wire_type, &mut wrapper, buf, ctx)?;

                    *field = ::core::option::Option::Some(
                        #mod_ident::#oneof_ident::#v_ident {
                            accounts: wrapper.accounts,
                            args: wrapper.args,
                        }
                    );

                    ::core::result::Result::Ok(())
                }
            }
        })
        .collect();

    let encoded_len_arms: Vec<TokenStream> = oneof_ir
        .variants
        .iter()
        .map(|v| {
            let v_ident = format_ident!("{}", v.variant_name);
            let tag = v.tag;

            quote! {
                #mod_ident::#oneof_ident::#v_ident { accounts, args } => {
                    let body_len =
                        ::prost::encoding::message::encoded_len(1, accounts)
                        + ::prost::encoding::message::encoded_len(2, args);

                    ::prost::encoding::key_len(#tag)
                        + ::prost::encoding::encoded_len_varint(body_len as u64)
                        + body_len
                }
            }
        })
        .collect();

    quote! {
        impl #mod_ident::#oneof_ident {
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match self {
                    #(#encode_arms,)*
                }
            }

            pub fn merge(
                field: &mut ::core::option::Option<Self>,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    #(#merge_arms,)*
                    _ => unreachable!(concat!("invalid ", stringify!(#oneof_ident), " tag: {}"), tag),
                }
            }

            #[inline]
            pub fn encoded_len(&self) -> usize {
                match self {
                    #(#encoded_len_arms,)*
                }
            }
        }
    }
}

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
        let prost_impl = manual_prost_struct_impl(t, local_names);

        quote! {
            #[derive(Clone, PartialEq, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
            pub struct #ident {
                #(#fields),*
            }

            #prost_impl
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
/// - Wrapper struct `Instructions` with `instruction: instruction::Instruction` (non-Option)
/// - Custom Borsh impls for `Instructions`
/// - When proto: manual `prost::Message` impl (no prost derive on the wrapper)
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

    // Render instruction types inside the module (wrapper structs still exist
    // for proto merge decoding and for users who want them).
    let module_types: Vec<TokenStream> = ix_types
        .iter()
        .map(|t| render_struct_type(t, Some(local_names)))
        .collect();

    // Struct variants: `Swap { accounts: SwapAccounts, args: SwapArgs }`
    let variants = oneof_ir.variants.iter().map(|v| {
        let v_ident = format_ident!("{}", v.variant_name);

        let accounts_ident = format_ident!("{}Accounts", v.message_type);
        let args_ident = format_ident!("{}Args", v.message_type);

        quote! {
            #v_ident { accounts: #accounts_ident, args: #args_ident }
        }
    });

    let borsh_serialize_arms = oneof_ir.variants.iter().enumerate().map(|(i, v)| {
        let disc = i as u8;

        let v_ident = format_ident!("{}", v.variant_name);
        let msg_ident = format_ident!("{}", v.message_type);

        quote! {
            #mod_ident::#oneof_ident::#v_ident { accounts, args } => {
                ::borsh::BorshSerialize::serialize(&#disc, writer)?;
                ::borsh::BorshSerialize::serialize(&(#mod_ident::#msg_ident { accounts: accounts.clone(), args: args.clone() }), writer)
            }
        }
    });

    let borsh_deserialize_arms = oneof_ir.variants.iter().enumerate().map(|(i, v)| {
        let disc = i as u8;
        let v_ident = format_ident!("{}", v.variant_name);
        let msg_ident = format_ident!("{}", v.message_type);

        quote! {
            #disc => {
                let v: #mod_ident::#msg_ident = ::borsh::BorshDeserialize::deserialize_reader(reader)?;

                #mod_ident::#oneof_ident::#v_ident { accounts: v.accounts, args: v.args }
            }
        }
    });

    let proto_impls = if cfg!(feature = "proto") {
        let oneof_impl = manual_prost_oneof_impl(oneof_ir, &mod_ident, &oneof_ident);

        let message_impl =
            manual_prost_message_impl(&parent_ident, &field_ident, &mod_ident, &oneof_ident);

        quote! { #oneof_impl #message_impl }
    } else {
        quote! {}
    };

    // For the parent `Instructions` struct, Debug is manual in proto mode
    // (provided by manual_prost_message_impl). For the `Instruction` enum
    // we always derive Debug since struct variants don't use prost::Oneof.
    let parent_debug_derive = if cfg!(feature = "proto") {
        quote! {}
    } else {
        quote! { Debug, }
    };

    quote! {
        #[derive(Clone, #parent_debug_derive PartialEq)]
        pub struct #parent_ident {
            pub #field_ident: #mod_ident::#oneof_ident,
        }

        pub mod #mod_ident {
            #(#module_types)*

            #[derive(Clone, Debug, PartialEq)]
            pub enum #oneof_ident {
                #(#variants),*
            }
        }

        #proto_impls

        impl ::borsh::BorshSerialize for #parent_ident {
            fn serialize<W: ::borsh::io::Write>(
                &self,
                writer: &mut W
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match &self.#field_ident {
                    #(#borsh_serialize_arms,)*
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
            #mod_ident::#oneof_ident::#v_ident(v) => {
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

                #mod_ident::#oneof_ident::#v_ident(v)
            }
        }
    });

    let enum_derive = if cfg!(feature = "proto") {
        quote! { #[derive(Clone, PartialEq, ::prost::Oneof)] }
    } else {
        quote! { #[derive(Clone, Debug, PartialEq)] }
    };

    let proto_impls = if cfg!(feature = "proto") {
        let first_variant = &oneof_ir.variants[0];

        let first_variant_ident = format_ident!("{}", first_variant.variant_name);
        let first_variant_msg = format_ident!("{}", first_variant.message_type);

        let message_impl =
            manual_prost_message_impl(&parent_ident, &field_ident, &mod_ident, &oneof_ident);

        quote! {
            #message_impl

            impl ::core::default::Default for #parent_ident {
                fn default() -> Self {
                    Self {
                        #field_ident: #mod_ident::#oneof_ident::#first_variant_ident(
                            <#first_variant_msg as ::core::default::Default>::default(),
                        ),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let debug_derive = if cfg!(feature = "proto") {
        quote! {}
    } else {
        quote! { Debug, }
    };

    quote! {
        #[derive(Clone, #debug_derive PartialEq)]
        pub struct #parent_ident {
            pub #field_ident: #mod_ident::#oneof_ident,
        }

        pub mod #mod_ident {
            #enum_derive
            pub enum #oneof_ident {
                #(#variants),*
            }
        }

        #proto_impls

        impl ::borsh::BorshSerialize for #parent_ident {
            fn serialize<W: ::borsh::io::Write>(
                &self,
                writer: &mut W
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match &self.#field_ident {
                    #(#borsh_serialize_arms,)*
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

///
/// Render a single struct field.
///
/// `local_names`: when `Some`, we are inside a submodule. Message types not in the set
/// and borsh helper paths get `super::` prefixed.
///
pub fn render_field(f: &FieldIr, local_names: Option<&HashSet<&str>>) -> TokenStream {
    let name = format_ident!("{}", f.name);
    let in_module = local_names.is_some();
    let path_prefix = if in_module { "super::" } else { "" };

    // Custom borsh attrs for fields whose on-chain encoding differs from the Rust type.
    // With native types, we no longer need widening borsh attrs — only fixed-bytes,
    // float, and fixed-array helpers remain.
    let borsh_attr = {
        let fixed = fixed_bytes_borsh_attrs(&f.label, &f.field_type, path_prefix);

        if !fixed.is_empty() {
            fixed
        } else {
            let float = float_borsh_attrs(&f.label, &f.field_type, path_prefix);

            if !float.is_empty() {
                float
            } else {
                // For FixedArray, we always need a custom borsh attr (no length prefix).
                fixed_array_default_borsh_attrs(&f.label, path_prefix)
            }
        }
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

    // Both proto and non-proto paths now use native types and no prost attributes.
    // The manual prost::Message impl (generated by manual_prost_struct_impl) handles encoding.
    match (&f.label, &f.field_type) {
        (LabelIr::Singular, FieldTypeIr::Message(msg)) => {
            let ty = resolve_msg(msg);

            quote! { #borsh_attr pub #name: #ty }
        },
        (LabelIr::Singular, field_type) => {
            let rust_type = map_ir_type_to_native(field_type, in_module);

            quote! { #borsh_attr pub #name: #rust_type }
        },
        (LabelIr::Optional, FieldTypeIr::Message(msg)) => {
            let ty = resolve_msg(msg);

            quote! { pub #name: ::core::option::Option<#ty> }
        },
        (LabelIr::Optional, field_type) => {
            let rust_type = map_ir_type_to_native(field_type, in_module);

            quote! { #borsh_attr pub #name: ::core::option::Option<#rust_type> }
        },
        (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Message(msg)) => {
            let ty = resolve_msg(msg);

            quote! { #borsh_attr pub #name: Vec<#ty> }
        },
        (LabelIr::Repeated | LabelIr::FixedArray(_), field_type) => {
            let rust_type = map_ir_type_to_native(field_type, in_module);

            quote! { #borsh_attr pub #name: Vec<#rust_type> }
        },
    }
}

/// Returns `#[borsh(deserialize_with = "...", serialize_with = "...")]` for fixed-size byte fields
/// (FixedBytes), or an empty TokenStream for all other field types.
fn fixed_bytes_borsh_attrs(
    label: &LabelIr,
    field_type: &FieldTypeIr,
    path_prefix: &str,
) -> TokenStream {
    match field_type {
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

///
/// Returns `#[borsh(deserialize_with = "...", serialize_with = "...")]` for float fields
/// (f32, f64) that need permissive deserialization allowing NaN/Infinity values.
/// Standard borsh rejects NaN for portability, but on-chain data may contain them.
///
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

/// Return the native Rust type for a field, preserving on-chain precision.
/// Does NOT widen u8→u32 etc. — manual prost impl handles the casting.
fn map_ir_type_to_native(field_type: &FieldTypeIr, in_module: bool) -> TokenStream {
    match field_type {
        FieldTypeIr::Scalar(s) => match s {
            ScalarIr::Bool => quote!(bool),
            ScalarIr::U8 => quote!(u8),
            ScalarIr::U16 | ScalarIr::ShortU16 => quote!(u16),
            ScalarIr::Uint32 => quote!(u32),
            ScalarIr::Uint64 => quote!(u64),
            ScalarIr::I8 => quote!(i8),
            ScalarIr::I16 => quote!(i16),
            ScalarIr::Int32 => quote!(i32),
            ScalarIr::Int64 => quote!(i64),
            ScalarIr::Float => quote!(f32),
            ScalarIr::Double => quote!(f64),
            ScalarIr::U128 => quote!(u128),
            ScalarIr::I128 => quote!(i128),
            ScalarIr::String => quote!(String),
            ScalarIr::Bytes | ScalarIr::FixedBytes(_) => quote!(Vec<u8>),
            ScalarIr::PublicKey => {
                if in_module {
                    quote!(super::Pubkey)
                } else {
                    quote!(Pubkey)
                }
            },
        },
        FieldTypeIr::Message(name) => {
            let ident = format_ident!("{}", name);
            quote!(#ident)
        },
    }
}

/// Whether a scalar requires widening for proto encoding (on-chain type differs from proto type).
fn needs_widening(scalar: &ScalarIr) -> bool {
    matches!(
        scalar,
        ScalarIr::U8 | ScalarIr::U16 | ScalarIr::ShortU16 | ScalarIr::I8 | ScalarIr::I16
    )
}

/// The proto-compatible widened type for a scalar that needs widening.
fn widened_type(scalar: &ScalarIr) -> TokenStream {
    match scalar {
        ScalarIr::U8 | ScalarIr::U16 | ScalarIr::ShortU16 => quote!(u32),
        ScalarIr::I8 | ScalarIr::I16 => quote!(i32),
        _ => unreachable!("widened_type called on non-widened scalar"),
    }
}

/// Whether a scalar needs bytes-to-native-int conversion for proto encoding (u128/i128).
fn needs_bytes_conversion(scalar: &ScalarIr) -> bool {
    matches!(scalar, ScalarIr::U128 | ScalarIr::I128)
}

/// The native integer type for a scalar that needs bytes conversion.
fn bytes_native_type(scalar: &ScalarIr) -> TokenStream {
    match scalar {
        ScalarIr::U128 => quote!(u128),
        ScalarIr::I128 => quote!(i128),
        _ => unreachable!("bytes_native_type called on non-bytes-conversion scalar"),
    }
}

/// Whether a scalar is a public key that needs Pubkey ↔ PublicKeyProtoWrapper conversion for proto.
fn is_pubkey_scalar(scalar: &ScalarIr) -> bool { matches!(scalar, ScalarIr::PublicKey) }

/// Return the `prost::encoding` module path for a scalar type.
fn prost_encoding_mod(scalar: &ScalarIr) -> TokenStream {
    match scalar {
        ScalarIr::Bool => quote!(::prost::encoding::bool),
        ScalarIr::U8 | ScalarIr::U16 | ScalarIr::ShortU16 | ScalarIr::Uint32 => {
            quote!(::prost::encoding::uint32)
        },
        ScalarIr::Uint64 => quote!(::prost::encoding::uint64),
        ScalarIr::I8 | ScalarIr::I16 | ScalarIr::Int32 => quote!(::prost::encoding::int32),
        ScalarIr::Int64 => quote!(::prost::encoding::int64),
        ScalarIr::Float => quote!(::prost::encoding::float),
        ScalarIr::Double => quote!(::prost::encoding::double),
        ScalarIr::String => quote!(::prost::encoding::string),
        ScalarIr::Bytes | ScalarIr::FixedBytes(_) | ScalarIr::U128 | ScalarIr::I128 => {
            quote!(::prost::encoding::bytes)
        },
        ScalarIr::PublicKey => quote!(::prost::encoding::message),
    }
}

/// Generate a manual `prost::Message` impl for a regular struct (not oneof wrappers).
///
/// This replaces `derive(prost::Message)` so that struct fields can use native Rust types
/// (e.g. `u8` instead of `u32`) while encoding/decoding as the widened proto type on the wire.
fn manual_prost_struct_impl(t: &TypeIr, local_names: Option<&HashSet<&str>>) -> TokenStream {
    let ident = format_ident!("{}", t.name);
    let struct_name = &t.name;
    let in_module = local_names.is_some();

    let resolve_msg = |msg: &str| -> TokenStream {
        let msg_ident = format_ident!("{}", msg);

        if let Some(locals) = local_names {
            if locals.contains(msg) {
                quote!(#msg_ident)
            } else {
                quote!(super::#msg_ident)
            }
        } else {
            quote!(#msg_ident)
        }
    };

    let mut encode_stmts = Vec::new();
    let mut merge_arms = Vec::new();
    let mut encoded_len_stmts = Vec::new();
    let mut clear_stmts = Vec::new();
    let mut default_stmts = Vec::new();

    for f in &t.fields {
        let fname = format_ident!("{}", f.name);
        let field_name_str = &f.name;
        let tag = f.tag;
        let is_message = matches!(&f.field_type, FieldTypeIr::Message(_));

        match (&f.label, &f.field_type) {
            // --- Singular message ---
            (LabelIr::Singular, _) if is_message => {
                encode_stmts.push(quote! {
                    ::prost::encoding::message::encode(#tag, &self.#fname, buf);
                });

                merge_arms.push(quote! {
                    #tag => {
                        ::prost::encoding::message::merge(wire_type, &mut self.#fname, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })
                    }
                });

                encoded_len_stmts.push(quote! {
                    + ::prost::encoding::message::encoded_len(#tag, &self.#fname)
                });

                clear_stmts.push(quote! { self.#fname.clear(); });

                let msg_ty = match &f.field_type {
                    FieldTypeIr::Message(msg) => resolve_msg(msg),
                    _ => unreachable!(),
                };

                default_stmts.push(quote! {
                    #fname: <#msg_ty as ::core::default::Default>::default()
                });
            },

            // --- Singular pubkey (Pubkey ↔ PublicKeyProtoWrapper conversion) ---
            (LabelIr::Singular, FieldTypeIr::Scalar(s)) if is_pubkey_scalar(s) => {
                let pubkey_ty = map_ir_type_to_native(&f.field_type, in_module);
                let wrapper_ty = quote!(yellowstone_vixen_core::PublicKeyProtoWrapper);

                encode_stmts.push(quote! {
                    {
                        let wrapper = #wrapper_ty::new(self.#fname.0.to_vec());

                        ::prost::encoding::message::encode(#tag, &wrapper, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut wrapper = #wrapper_ty::default();

                        ::prost::encoding::message::merge(wire_type, &mut wrapper, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        let arr: [u8; 32] = wrapper.value.try_into().map_err(|_|
                            ::prost::DecodeError::new("expected exactly 32 bytes for Pubkey")
                        )?;

                        self.#fname = #pubkey_ty::new(arr);

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + {
                        let wrapper = #wrapper_ty::new(self.#fname.0.to_vec());

                        ::prost::encoding::message::encoded_len(#tag, &wrapper)
                    }
                });

                clear_stmts.push(
                    quote! { self.#fname = <#pubkey_ty as ::core::default::Default>::default(); },
                );
                default_stmts
                    .push(quote! { #fname: <#pubkey_ty as ::core::default::Default>::default() });
            },

            // --- Singular scalar (widened) ---
            (LabelIr::Singular, FieldTypeIr::Scalar(s)) if needs_widening(s) => {
                let enc_mod = prost_encoding_mod(s);
                let wide_ty = widened_type(s);
                let native_ty = map_ir_type_to_native(&f.field_type, in_module);

                encode_stmts.push(quote! {
                    if self.#fname != (0 as #native_ty) {
                        let tmp = self.#fname as #wide_ty;
                        #enc_mod::encode(#tag, &tmp, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut tmp = self.#fname as #wide_ty;

                        #enc_mod::merge(wire_type, &mut tmp, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        self.#fname = tmp as #native_ty;

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + if self.#fname != (0 as #native_ty) {
                        #enc_mod::encoded_len(#tag, &(self.#fname as #wide_ty))
                    } else { 0 }
                });

                clear_stmts.push(quote! { self.#fname = 0; });
                default_stmts.push(quote! { #fname: 0 });
            },

            // --- Singular scalar (bytes conversion: u128/i128) ---
            (LabelIr::Singular, FieldTypeIr::Scalar(s)) if needs_bytes_conversion(s) => {
                let native_ty = bytes_native_type(s);

                encode_stmts.push(quote! {
                    if self.#fname != (0 as #native_ty) {
                        let tmp: Vec<u8> = self.#fname.to_le_bytes().to_vec();

                        ::prost::encoding::bytes::encode(#tag, &tmp, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut tmp: Vec<u8> = Vec::new();

                        ::prost::encoding::bytes::merge(wire_type, &mut tmp, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        let arr: [u8; 16] = tmp.try_into().map_err(|_|
                            ::prost::DecodeError::new(
                                concat!("expected exactly 16 bytes for ", stringify!(#native_ty))
                            )
                        )?;

                        self.#fname = #native_ty::from_le_bytes(arr);

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + if self.#fname != (0 as #native_ty) {
                        ::prost::encoding::bytes::encoded_len(#tag, &self.#fname.to_le_bytes().to_vec())
                    } else { 0 }
                });

                clear_stmts.push(quote! { self.#fname = 0; });
                default_stmts.push(quote! { #fname: 0 });
            },

            // --- Singular scalar (no widening) ---
            (LabelIr::Singular, FieldTypeIr::Scalar(s)) => {
                let enc_mod = prost_encoding_mod(s);
                let native_ty = map_ir_type_to_native(&f.field_type, in_module);

                let default_check = match s {
                    ScalarIr::Bool => quote!(self.#fname != false),
                    ScalarIr::String => quote!(!self.#fname.is_empty()),
                    ScalarIr::Bytes | ScalarIr::FixedBytes(_) => quote!(!self.#fname.is_empty()),
                    _ => quote!(self.#fname != (0 as #native_ty)),
                };

                let default_val = match s {
                    ScalarIr::Bool => quote!(false),
                    ScalarIr::String => quote!(String::new()),
                    ScalarIr::Bytes | ScalarIr::FixedBytes(_) => quote!(Vec::new()),
                    ScalarIr::Float => quote!(0f32),
                    ScalarIr::Double => quote!(0f64),
                    _ => quote!(0),
                };

                let clear_val = match s {
                    ScalarIr::Bool => quote!(self.#fname = false;),
                    ScalarIr::String => quote!(self.#fname.clear();),
                    ScalarIr::Bytes | ScalarIr::FixedBytes(_) => quote!(self.#fname.clear();),
                    ScalarIr::Float => quote!(self.#fname = 0f32;),
                    ScalarIr::Double => quote!(self.#fname = 0f64;),
                    _ => quote!(self.#fname = 0;),
                };

                encode_stmts.push(quote! {
                    if #default_check {
                        #enc_mod::encode(#tag, &self.#fname, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        #enc_mod::merge(wire_type, &mut self.#fname, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })
                    }
                });

                encoded_len_stmts.push(quote! {
                    + if #default_check {
                        #enc_mod::encoded_len(#tag, &self.#fname)
                    } else { 0 }
                });

                clear_stmts.push(clear_val);
                default_stmts.push(quote! { #fname: #default_val });
            },

            // --- Optional message ---
            (LabelIr::Optional, _) if is_message => {
                encode_stmts.push(quote! {
                    if let ::core::option::Option::Some(ref msg) = self.#fname {
                        ::prost::encoding::message::encode(#tag, msg, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        ::prost::encoding::message::merge(
                            wire_type,
                            self.#fname.get_or_insert_with(::core::default::Default::default),
                            buf, ctx,
                        ).map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.as_ref().map_or(0, |m|
                        ::prost::encoding::message::encoded_len(#tag, m)
                    )
                });

                clear_stmts.push(quote! { self.#fname = ::core::option::Option::None; });
                default_stmts.push(quote! { #fname: ::core::option::Option::None });
            },

            // --- Optional pubkey (Pubkey ↔ PublicKeyProtoWrapper conversion) ---
            (LabelIr::Optional, FieldTypeIr::Scalar(s)) if is_pubkey_scalar(s) => {
                let pubkey_ty = map_ir_type_to_native(&f.field_type, in_module);
                let wrapper_ty = quote!(yellowstone_vixen_core::PublicKeyProtoWrapper);

                encode_stmts.push(quote! {
                    if let ::core::option::Option::Some(ref pk) = self.#fname {
                        let wrapper = #wrapper_ty::new(pk.0.to_vec());

                        ::prost::encoding::message::encode(#tag, &wrapper, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut wrapper = #wrapper_ty::default();

                        ::prost::encoding::message::merge(wire_type, &mut wrapper, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        let arr: [u8; 32] = wrapper.value.try_into().map_err(|_|
                            ::prost::DecodeError::new("expected exactly 32 bytes for Pubkey")
                        )?;

                        self.#fname = ::core::option::Option::Some(#pubkey_ty::new(arr));

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.as_ref().map_or(0, |pk| {
                        let wrapper = #wrapper_ty::new(pk.0.to_vec());

                        ::prost::encoding::message::encoded_len(#tag, &wrapper)
                    })
                });

                clear_stmts.push(quote! { self.#fname = ::core::option::Option::None; });
                default_stmts.push(quote! { #fname: ::core::option::Option::None });
            },

            // --- Optional scalar (widened) ---
            (LabelIr::Optional, FieldTypeIr::Scalar(s)) if needs_widening(s) => {
                let enc_mod = prost_encoding_mod(s);
                let wide_ty = widened_type(s);
                let native_ty = map_ir_type_to_native(&f.field_type, in_module);

                encode_stmts.push(quote! {
                    if let ::core::option::Option::Some(v) = self.#fname {
                        let tmp = v as #wide_ty;

                        #enc_mod::encode(#tag, &tmp, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut tmp: #wide_ty = self.#fname.unwrap_or_default() as #wide_ty;

                        #enc_mod::merge(wire_type, &mut tmp, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        self.#fname = ::core::option::Option::Some(tmp as #native_ty);

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.map_or(0, |v|
                        #enc_mod::encoded_len(#tag, &(v as #wide_ty))
                    )
                });

                clear_stmts.push(quote! { self.#fname = ::core::option::Option::None; });
                default_stmts.push(quote! { #fname: ::core::option::Option::None });
            },

            // --- Optional scalar (bytes conversion: u128/i128) ---
            (LabelIr::Optional, FieldTypeIr::Scalar(s)) if needs_bytes_conversion(s) => {
                let native_ty = bytes_native_type(s);

                encode_stmts.push(quote! {
                    if let ::core::option::Option::Some(v) = self.#fname {
                        let tmp: Vec<u8> = v.to_le_bytes().to_vec();

                        ::prost::encoding::bytes::encode(#tag, &tmp, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut tmp: Vec<u8> = Vec::new();

                        ::prost::encoding::bytes::merge(wire_type, &mut tmp, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        let arr: [u8; 16] = tmp.try_into().map_err(|_|
                            ::prost::DecodeError::new(
                                concat!("expected exactly 16 bytes for ", stringify!(#native_ty))
                            )
                        )?;

                        self.#fname = ::core::option::Option::Some(#native_ty::from_le_bytes(arr));

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.map_or(0, |v|
                        ::prost::encoding::bytes::encoded_len(#tag, &v.to_le_bytes().to_vec())
                    )
                });

                clear_stmts.push(quote! { self.#fname = ::core::option::Option::None; });
                default_stmts.push(quote! { #fname: ::core::option::Option::None });
            },

            // --- Optional scalar (no widening) ---
            (LabelIr::Optional, FieldTypeIr::Scalar(s)) => {
                let enc_mod = prost_encoding_mod(s);

                encode_stmts.push(quote! {
                    if let ::core::option::Option::Some(ref value) = self.#fname {
                        #enc_mod::encode(#tag, value, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        #enc_mod::merge(
                            wire_type,
                            self.#fname.get_or_insert_with(::core::default::Default::default),
                            buf, ctx,
                        ).map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.as_ref().map_or(0, |v|
                        #enc_mod::encoded_len(#tag, v)
                    )
                });

                clear_stmts.push(quote! { self.#fname = ::core::option::Option::None; });
                default_stmts.push(quote! { #fname: ::core::option::Option::None });
            },

            // --- Repeated/FixedArray message ---
            (LabelIr::Repeated | LabelIr::FixedArray(_), _) if is_message => {
                encode_stmts.push(quote! {
                    for msg in &self.#fname {
                        ::prost::encoding::message::encode(#tag, msg, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        ::prost::encoding::message::merge_repeated(wire_type, &mut self.#fname, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })
                    }
                });

                encoded_len_stmts.push(quote! {
                    + ::prost::encoding::message::encoded_len_repeated(#tag, &self.#fname)
                });

                clear_stmts.push(quote! { self.#fname.clear(); });
                default_stmts.push(quote! { #fname: Vec::new() });
            },

            // --- Repeated/FixedArray pubkey (Pubkey ↔ PublicKeyProtoWrapper conversion) ---
            (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Scalar(s))
                if is_pubkey_scalar(s) =>
            {
                let pubkey_ty = map_ir_type_to_native(&f.field_type, in_module);
                let wrapper_ty = quote!(yellowstone_vixen_core::PublicKeyProtoWrapper);

                encode_stmts.push(quote! {
                    for pk in &self.#fname {
                        let wrapper = #wrapper_ty::new(pk.0.to_vec());

                        ::prost::encoding::message::encode(#tag, &wrapper, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut wrapper = #wrapper_ty::default();

                        ::prost::encoding::message::merge(wire_type, &mut wrapper, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        let arr: [u8; 32] = wrapper.value.try_into().map_err(|_|
                            ::prost::DecodeError::new("expected exactly 32 bytes for Pubkey")
                        )?;

                        self.#fname.push(#pubkey_ty::new(arr));

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.iter().map(|pk| {
                        let wrapper = #wrapper_ty::new(pk.0.to_vec());

                        ::prost::encoding::message::encoded_len(#tag, &wrapper)
                    }).sum::<usize>()
                });

                clear_stmts.push(quote! { self.#fname.clear(); });
                default_stmts.push(quote! { #fname: Vec::new() });
            },

            // --- Repeated/FixedArray scalar (widened) ---
            (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Scalar(s))
                if needs_widening(s) =>
            {
                let enc_mod = prost_encoding_mod(s);
                let wide_ty = widened_type(s);
                let native_ty = map_ir_type_to_native(&f.field_type, in_module);

                encode_stmts.push(quote! {
                    {
                        let tmp: Vec<#wide_ty> = self.#fname.iter().map(|&v| v as #wide_ty).collect();

                        #enc_mod::encode_packed(#tag, &tmp, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut tmp: Vec<#wide_ty> = self.#fname.iter().map(|&v| v as #wide_ty).collect();

                        #enc_mod::merge_repeated(wire_type, &mut tmp, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        self.#fname = tmp.into_iter().map(|v| v as #native_ty).collect();

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + {
                        let tmp: Vec<#wide_ty> = self.#fname.iter().map(|&v| v as #wide_ty).collect();

                        #enc_mod::encoded_len_packed(#tag, &tmp)
                    }
                });

                clear_stmts.push(quote! { self.#fname.clear(); });
                default_stmts.push(quote! { #fname: Vec::new() });
            },

            // --- Repeated/FixedArray scalar (bytes conversion: u128/i128) ---
            (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Scalar(s))
                if needs_bytes_conversion(s) =>
            {
                let native_ty = bytes_native_type(s);

                encode_stmts.push(quote! {
                    for v in &self.#fname {
                        let tmp: Vec<u8> = v.to_le_bytes().to_vec();

                        ::prost::encoding::bytes::encode(#tag, &tmp, buf);
                    }
                });

                merge_arms.push(quote! {
                    #tag => {
                        let mut tmp: Vec<u8> = Vec::new();

                        ::prost::encoding::bytes::merge(wire_type, &mut tmp, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })?;

                        let arr: [u8; 16] = tmp.try_into().map_err(|_|
                            ::prost::DecodeError::new(
                                concat!("expected exactly 16 bytes for ", stringify!(#native_ty))
                            )
                        )?;

                        self.#fname.push(#native_ty::from_le_bytes(arr));

                        ::core::result::Result::Ok(())
                    }
                });

                encoded_len_stmts.push(quote! {
                    + self.#fname.iter().map(|v|
                        ::prost::encoding::bytes::encoded_len(#tag, &v.to_le_bytes().to_vec())
                    ).sum::<usize>()
                });

                clear_stmts.push(quote! { self.#fname.clear(); });
                default_stmts.push(quote! { #fname: Vec::new() });
            },

            // --- Repeated/FixedArray scalar (no widening) ---
            (LabelIr::Repeated | LabelIr::FixedArray(_), FieldTypeIr::Scalar(s)) => {
                let enc_mod = prost_encoding_mod(s);

                // String and bytes use length-delimited, not packed varint.
                let is_length_delimited = matches!(
                    s,
                    ScalarIr::String | ScalarIr::Bytes | ScalarIr::FixedBytes(_)
                );

                if is_length_delimited {
                    encode_stmts.push(quote! {
                        for v in &self.#fname {
                            #enc_mod::encode(#tag, v, buf);
                        }
                    });

                    encoded_len_stmts.push(quote! {
                        + #enc_mod::encoded_len_repeated(#tag, &self.#fname)
                    });
                } else {
                    encode_stmts.push(quote! {
                        #enc_mod::encode_packed(#tag, &self.#fname, buf);
                    });

                    encoded_len_stmts.push(quote! {
                        + #enc_mod::encoded_len_packed(#tag, &self.#fname)
                    });
                }

                merge_arms.push(quote! {
                    #tag => {
                        #enc_mod::merge_repeated(wire_type, &mut self.#fname, buf, ctx)
                            .map_err(|mut error| { error.push(STRUCT_NAME, #field_name_str); error })
                    }
                });

                clear_stmts.push(quote! { self.#fname.clear(); });
                default_stmts.push(quote! { #fname: Vec::new() });
            },

            // All Message variants are handled by the `if is_message` guards above.
            _ => unreachable!("unhandled field combo: {:?} / {:?}", f.label, f.field_type),
        }
    }

    let debug_field_names: Vec<_> = t
        .fields
        .iter()
        .map(|f| format_ident!("{}", f.name))
        .collect();

    quote! {
        impl ::core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut s = f.debug_struct(stringify!(#ident));

                #(s.field(stringify!(#debug_field_names), &self.#debug_field_names);)*

                s.finish()
            }
        }

        impl ::core::default::Default for #ident {
            fn default() -> Self {
                Self {
                    #(#default_stmts),*
                }
            }
        }

        impl ::prost::Message for #ident {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                #(#encode_stmts)*
            }

            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &str = #struct_name;

                match tag {
                    #(#merge_arms,)*
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }

            fn encoded_len(&self) -> usize {
                0 #(#encoded_len_stmts)*
            }

            fn clear(&mut self) {
                #(#clear_stmts)*
            }
        }
    }
}
