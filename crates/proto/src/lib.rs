#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

//! Protobuf definitions used by the `yellowstone-vixen` family of crates.

pub extern crate prost;
#[cfg(feature = "stream")]
pub extern crate prost_types;
#[cfg(feature = "stream")]
pub extern crate tonic;
#[cfg(feature = "stream")]
pub extern crate tonic_reflection;

mod vixen {
    #[cfg(feature = "parser")]
    pub mod parser {
        #![allow(missing_docs)]

        //! Protobuf definitions for the `yellowstone-vixen-parser` crate.

        include!(concat!(env!("OUT_DIR"), "/vixen.parser.rs"));

        pub const DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vixen.parser.bin"));
    }

    #[cfg(feature = "stream")]
    pub mod stream {
        #![allow(missing_docs)]

        //! Protobuf definitions for the `stream` feature of the
        //! `yellowstone-vixen` crate.

        tonic::include_proto!("vixen.stream");

        /// Compiled protobuf file descriptor set for the `vixen.stream`
        /// package.
        pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("stream_descriptor");
    }
}

pub use vixen::*;
