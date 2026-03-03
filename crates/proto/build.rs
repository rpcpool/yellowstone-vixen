use std::{env, path::PathBuf};

fn main() {
    #[cfg(any(feature = "parser", feature = "stream"))]
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(any(feature = "parser", feature = "stream"))]
    unsafe {
        std::env::set_var("PROTOC", protobuf_src::protoc());
    }

    #[cfg(feature = "parser")]
    {
        tonic_prost_build::configure()
            .file_descriptor_set_path(out_dir.join("vixen.parser.token.bin"))
            .compile_protos(&["proto/token.proto"], &["proto"])
            .unwrap();

        tonic_prost_build::configure()
            .file_descriptor_set_path(out_dir.join("vixen.parser.token_extensions.bin"))
            .compile_protos(&["proto/token_extensions.proto"], &["proto"])
            .unwrap();

        // Generate a self-contained schema for token_extensions that bundles
        // the token.proto dependency inline, so consumers of PROTOBUF_SCHEMA
        // don't need to resolve the `import "token.proto"` themselves.
        let token_schema = std::fs::read_to_string("proto/token.proto").unwrap();
        let ext_schema = std::fs::read_to_string("proto/token_extensions.proto").unwrap();

        let combined = format!(
            "{}\n\n// ----- token_extensions.proto -----\n\n{}",
            token_schema, ext_schema,
        );
        std::fs::write(out_dir.join("token_extensions_full_schema.proto"), combined).unwrap();
    }

    #[cfg(feature = "stream")]
    {
        tonic_prost_build::configure()
            .file_descriptor_set_path(out_dir.join("stream_descriptor.bin"))
            .compile_protos(&["proto/stream.proto"], &["proto"])
            .unwrap();
    }
}
