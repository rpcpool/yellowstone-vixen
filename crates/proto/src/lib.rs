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
            pub const INSTRUCTION_DISPATCH_MESSAGE_INDEX: Option<usize> = Some(64);
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
        //! `yellowstone-vixen` crate.

        tonic::include_proto!("vixen.stream");

        /// Compiled protobuf file descriptor set for the `vixen.stream`
        /// package.
        pub const DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("stream_descriptor");
    }
}

pub use vixen::*;

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
