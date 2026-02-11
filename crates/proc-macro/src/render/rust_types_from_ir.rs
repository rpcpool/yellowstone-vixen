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
    let fields = t.fields.iter().map(render_field);

    quote! {
        #[derive(Clone, PartialEq, ::prost::Message, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
        pub struct #ident {
            #(#fields),*
        }
    }
}

fn render_oneof_parent(oneof_ir: &OneofIr) -> TokenStream {
    let parent_ident = format_ident!("{}", oneof_ir.parent_message);

    let (mod_ident, oneof_ident) = match oneof_ir.kind {
        OneofKindIr::InstructionDispatch => (
            format_ident!("program_instruction_oneof"),
            format_ident!("Ix"),
        ),
        OneofKindIr::Enum => (
            format_ident!("{}", crate::utils::to_snake_case(&oneof_ir.parent_message)),
            format_ident!("Kind"),
        ),
    };

    // tags = "1, 2, 3"
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

    let field_ident = format_ident!("{}", oneof_ir.field_name);

    // Oneof variants (prost)
    let variants = oneof_ir.variants.iter().map(|v| {
        let v_ident = format_ident!("{}", v.variant_name);
        let msg_ident = format_ident!("{}", v.message_type);
        let tag = v.tag;

        quote! { #[prost(message, tag = #tag)] #v_ident(super::#msg_ident) }
    });

    // We encode discriminant as the VARIANT INDEX (0..n-1), not the proto tag.
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

    match (&f.label, &f.field_type) {
        // prost expects message fields as Option<T> unless repeated
        (LabelIr::Singular, FieldTypeIr::Message(msg)) => {
            let ident = format_ident!("{}", msg);

            quote! {
                #[prost(message, optional, tag = #tag)]
                pub #name: ::core::option::Option<#ident>
            }
        },

        (LabelIr::Singular, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type);

            quote! { #[prost(#prost_type, tag = #tag)] pub #name: #rust_type }
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
                pub #name: ::core::option::Option<#rust_type>
            }
        },

        (LabelIr::Repeated, FieldTypeIr::Message(msg)) => {
            let ident = format_ident!("{}", msg);

            quote! {
                #[prost(message, repeated, tag = #tag)]
                pub #name: ::prost::alloc::vec::Vec<#ident>
            }
        },

        (LabelIr::Repeated, field_type) => {
            let (prost_type, rust_type) = map_ir_type_to_prost(field_type);

            quote! {
                #[prost(#prost_type, repeated, tag = #tag)]
                pub #name: ::prost::alloc::vec::Vec<#rust_type>
            }
        },
    }
}

/// Return (prost_type, rust_type)
fn map_ir_type_to_prost(field_type: &FieldTypeIr) -> (TokenStream, TokenStream) {
    match field_type {
        FieldTypeIr::Scalar(s) => match s {
            ScalarIr::Bool => (quote!(bool), quote!(bool)),
            ScalarIr::Uint32 => (quote!(uint32), quote!(u32)),
            ScalarIr::Uint64 => (quote!(uint64), quote!(u64)),
            ScalarIr::Int32 => (quote!(int32), quote!(i32)),
            ScalarIr::Int64 => (quote!(int64), quote!(i64)),
            ScalarIr::Float => (quote!(float), quote!(f32)),
            ScalarIr::Double => (quote!(double), quote!(f64)),
            ScalarIr::String => (quote!(string), quote!(::prost::alloc::string::String)),
            ScalarIr::Bytes => (quote!(bytes = "vec"), quote!(::prost::alloc::vec::Vec<u8>)),
            ScalarIr::PubkeyBytes => (quote!(bytes = "vec"), quote!(PubkeyBytes)),
        },
        FieldTypeIr::Message(name) => {
            let ident = format_ident!("{}", name);

            (quote!(message), quote!(#ident))
        },
    }
}
