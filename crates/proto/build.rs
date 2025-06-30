use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "parser")]
    {
        // token
        prost_build::Config::new()
            .enable_type_names()
            // .protoc_arg("--experimental_allow_proto3_optional")
            .file_descriptor_set_path(out_dir.join("vixen.parser.token.bin"))
            .compile_protos(&["proto/token.proto"], &["proto"])
            .unwrap();

        // token extensions
        prost_build::Config::new()
            .enable_type_names()
            // .protoc_arg("--experimental_allow_proto3_optional")
            .file_descriptor_set_path(out_dir.join("vixen.parser.token_extensions.bin"))
            .compile_protos(&["proto/token_extensions.proto"], &["proto"])
            .unwrap();
    }

    #[cfg(feature = "stream")]
    {
        tonic_build::configure()
            .file_descriptor_set_path(out_dir.join("stream_descriptor.bin"))
            .compile_protos(&["proto/stream.proto"], &["proto"])
            .unwrap();
    }
}
