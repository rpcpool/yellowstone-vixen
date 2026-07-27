#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

//! Protobuf definitions used by the `shipstern` family of crates.

pub extern crate prost;
#[cfg(feature = "stream")]
pub extern crate prost_types;
#[cfg(feature = "stream")]
pub extern crate tonic;
#[cfg(feature = "stream")]
pub extern crate tonic_reflection;

mod shipstern {
    #[cfg(feature = "parser")]
    pub mod parser {
        #![allow(missing_docs)]

        pub mod token {
            #![allow(clippy::all)]
            include!(concat!(env!("OUT_DIR"), "/vixen.parser.token.rs"));

            pub const DESCRIPTOR_SET: &[u8] =
                include_bytes!(concat!(env!("OUT_DIR"), "/vixen.parser.token.bin"));

            /// Raw `.proto` schema text for the token parser.
            pub const PROTOBUF_SCHEMA: &str = include_str!("../proto/token.proto");

            ///
            /// IMPORTANT - If you update the .proto files, make sure to update the dispatch message indices below if needed
            ///

            /// 0-based index of the account dispatch message (`TokenProgramState`) in the proto file descriptor.
            pub const ACCOUNT_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(4);
            /// 0-based index of the instruction dispatch message (`TokenProgram`) in the proto file descriptor.
            pub const INSTRUCTION_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(71);
        }

        pub mod bpf_loader {
            #![allow(clippy::all)]
            include!(concat!(env!("OUT_DIR"), "/vixen.parser.bpf_loader.rs"));

            pub const DESCRIPTOR_SET: &[u8] =
                include_bytes!(concat!(env!("OUT_DIR"), "/vixen.parser.bpf_loader.bin"));

            /// Raw `.proto` schema text for the BPF loader parser.
            pub const PROTOBUF_SCHEMA: &str = include_str!("../proto/bpf_loader.proto");

            ///
            /// IMPORTANT - If you update the .proto files, make sure to update the dispatch message indices below if needed
            ///

            /// 0-based index of the account dispatch message (`BpfLoaderState`) in the proto file descriptor.
            pub const ACCOUNT_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(4);
            /// 0-based index of the instruction dispatch message (`BpfLoaderProgram`) in the proto file descriptor.
            pub const INSTRUCTION_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(24);
        }

        pub mod token_extensions {
            #![allow(clippy::all)]
            include!(concat!(
                env!("OUT_DIR"),
                "/vixen.parser.token_extensions.rs"
            ));

            pub const DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(
                env!("OUT_DIR"),
                "/vixen.parser.token_extensions.bin"
            ));

            /// Self-contained `.proto` schema text for the token extensions
            /// parser. Bundles the `token.proto` dependency inline so
            /// consumers don't need to resolve the import separately.
            pub const PROTOBUF_SCHEMA: &str = include_str!(concat!(
                env!("OUT_DIR"),
                "/token_extensions_full_schema.proto"
            ));

            ///
            /// IMPORTANT - If you update the .proto files, make sure to update the dispatch message indices below if needed
            ///

            /// 0-based index of the account dispatch message (`TokenExtensionState`) in the proto file descriptor.
            pub const ACCOUNT_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(6);
            /// 0-based index of the instruction dispatch message (`TokenExtensionProgram`) in the proto file descriptor.
            pub const INSTRUCTION_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(93);
        }
    }

    #[cfg(feature = "stream")]
    pub mod stream {
        #![allow(missing_docs)]

        //! Protobuf definitions for the `stream` feature of the
        //! `shipstern` crate.

        tonic::include_proto!("vixen.stream");

        /// Compiled protobuf file descriptor set for the `vixen.stream`
        /// package.
        pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("stream_descriptor");
    }
}

pub use shipstern::*;

///
/// Non-regression tests to ensure that if token.proto or token_extensions.proto are updated, the dispatch message indices are also updated accordingly.
/// It's not 100% foul proof, but the idea is for the tests to fail if someone adds a new top-level message in the proto file without updating the dispatch indices
///
#[cfg(all(test, feature = "parser"))]
mod dispatch_index_tests {
    /// Extract top-level message names from a `.proto` file (skips nested messages).
    fn top_level_message_names(proto_text: &str) -> Vec<String> {
        let mut names = Vec::new();
        let mut depth: i32 = 0;

        for line in proto_text.lines() {
            let trimmed = line.trim();

            if depth == 0 && trimmed.starts_with("message ") {
                let name = trimmed
                    .strip_prefix("message ")
                    .unwrap()
                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                    .next()
                    .unwrap();
                names.push(name.to_string());
            }

            depth += trimmed.chars().filter(|&c| c == '{').count() as i32;
            depth -= trimmed.chars().filter(|&c| c == '}').count() as i32;
        }

        names
    }

    #[test]
    fn token_dispatch_indices_match_proto() {
        let proto = include_str!("../proto/token.proto");
        let messages = top_level_message_names(proto);

        let account_idx = crate::parser::token::ACCOUNT_DISPATCH_MESSAGE_INDEX
            .expect("ACCOUNT_DISPATCH_MESSAGE_INDEX should be Some for token");
        let instruction_idx = crate::parser::token::INSTRUCTION_DISPATCH_MESSAGE_INDEX
            .expect("INSTRUCTION_DISPATCH_MESSAGE_INDEX should be Some for token");

        assert_eq!(
            messages[account_idx], "TokenProgramState",
            "ACCOUNT_DISPATCH_MESSAGE_INDEX ({account_idx}) should point to TokenProgramState, \
             found {}",
            messages[account_idx],
        );
        assert_eq!(
            messages[instruction_idx], "TokenProgram",
            "INSTRUCTION_DISPATCH_MESSAGE_INDEX ({instruction_idx}) should point to TokenProgram, \
             found {}",
            messages[instruction_idx],
        );
    }

    #[test]
    fn bpf_loader_dispatch_indices_match_proto() {
        let proto = include_str!("../proto/bpf_loader.proto");
        let messages = top_level_message_names(proto);

        let account_idx = crate::parser::bpf_loader::ACCOUNT_DISPATCH_MESSAGE_INDEX
            .expect("ACCOUNT_DISPATCH_MESSAGE_INDEX should be Some for bpf_loader");
        let instruction_idx = crate::parser::bpf_loader::INSTRUCTION_DISPATCH_MESSAGE_INDEX
            .expect("INSTRUCTION_DISPATCH_MESSAGE_INDEX should be Some for bpf_loader");

        assert_eq!(
            messages[account_idx], "BpfLoaderState",
            "ACCOUNT_DISPATCH_MESSAGE_INDEX ({account_idx}) should point to BpfLoaderState, found \
             {}",
            messages[account_idx],
        );
        assert_eq!(
            messages[instruction_idx], "BpfLoaderProgram",
            "INSTRUCTION_DISPATCH_MESSAGE_INDEX ({instruction_idx}) should point to \
             BpfLoaderProgram, found {}",
            messages[instruction_idx],
        );
    }

    #[test]
    fn token_extensions_dispatch_indices_match_proto() {
        let proto = include_str!("../proto/token_extensions.proto");
        let messages = top_level_message_names(proto);

        let account_idx = crate::parser::token_extensions::ACCOUNT_DISPATCH_MESSAGE_INDEX
            .expect("ACCOUNT_DISPATCH_MESSAGE_INDEX should be Some for token_extensions");
        let instruction_idx = crate::parser::token_extensions::INSTRUCTION_DISPATCH_MESSAGE_INDEX
            .expect("INSTRUCTION_DISPATCH_MESSAGE_INDEX should be Some for token_extensions");

        assert_eq!(
            messages[account_idx], "TokenExtensionState",
            "ACCOUNT_DISPATCH_MESSAGE_INDEX ({account_idx}) should point to TokenExtensionState, \
             found {}",
            messages[account_idx],
        );
        assert_eq!(
            messages[instruction_idx], "TokenExtensionProgram",
            "INSTRUCTION_DISPATCH_MESSAGE_INDEX ({instruction_idx}) should point to \
             TokenExtensionProgram, found {}",
            messages[instruction_idx],
        );
    }
}

///
/// Wire-compatibility guards for the protobuf `package` declarations.
///
/// These strings are part of the public wire contract, not an internal naming
/// detail. The package determines the gRPC method path
/// (`/vixen.stream.ProgramStreams/Subscribe`) and the `google.protobuf.Any`
/// type URLs carried in `SubscribeUpdate.parsed`
/// (`type.googleapis.com/vixen.parser.token.TokenAccount`).
///
/// Renaming a package breaks every deployed consumer: the gRPC path starts
/// returning `UNIMPLEMENTED`, and `Any` type URLs stop matching *without*
/// erroring, so the failure is silent. A project-wide find-and-replace will
/// happily rewrite these, and nothing else in the test suite notices.
///
/// Example output when a rename slips through:
///
/// ```rust, ignore
/// assertion `left == right` failed: token.proto must declare `package vixen.parser.token;`
///   left: "shipstern.parser.token"
///  right: "vixen.parser.token"
/// ```
///
#[cfg(all(test, feature = "parser"))]
mod wire_compat_tests {
    /// Extract the `package` declaration from `.proto` source text.
    fn package_decl(proto_text: &str) -> Option<&str> {
        for line in proto_text.lines() {
            let Some(rest) = line.trim().strip_prefix("package ") else {
                continue;
            };

            return rest.split(';').next().map(str::trim);
        }

        None
    }

    #[test]
    fn parser_proto_packages_are_stable() {
        let cases = [
            (
                "token.proto",
                include_str!("../proto/token.proto"),
                "vixen.parser.token",
            ),
            (
                "bpf_loader.proto",
                include_str!("../proto/bpf_loader.proto"),
                "vixen.parser.bpf_loader",
            ),
            (
                "token_extensions.proto",
                include_str!("../proto/token_extensions.proto"),
                "vixen.parser.token_extensions",
            ),
        ];

        for (file, text, expected) in cases {
            let found =
                package_decl(text).unwrap_or_else(|| panic!("{file} has no `package` declaration"));

            assert_eq!(
                found, expected,
                "{file} must declare `package {expected};` — this is the public wire contract \
                 (gRPC method paths and `Any` type URLs), so renaming it silently breaks deployed \
                 consumers",
            );
        }
    }
}

///
/// Wire-compatibility guard for the `stream` gRPC service path.
///
/// Decodes the compiled descriptor set rather than the `.proto` text, so this
/// asserts what the generated server actually serves. See
/// [`wire_compat_tests`] for why the package string is load-bearing.
///
#[cfg(all(test, feature = "stream"))]
mod stream_wire_compat_tests {
    use crate::prost::Message as _;

    #[test]
    fn stream_service_path_is_stable() {
        let set = crate::prost_types::FileDescriptorSet::decode(crate::stream::DESCRIPTOR_SET)
            .expect("stream DESCRIPTOR_SET should decode as a FileDescriptorSet");

        let file = set
            .file
            .iter()
            .find(|f| f.package() == "vixen.stream")
            .expect(
                "descriptor set must contain a file with `package vixen.stream` — the gRPC method \
                 path depends on it",
            );

        let service_names: Vec<&str> = file.service.iter().map(|s| s.name()).collect();

        assert!(
            service_names.contains(&"ProgramStreams"),
            "vixen.stream must expose the `ProgramStreams` service (wire path \
             /vixen.stream.ProgramStreams/Subscribe), found {service_names:?}",
        );
    }
}
