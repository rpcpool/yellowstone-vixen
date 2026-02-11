use std::{env, path::PathBuf};

fn main() {
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
    }

    #[cfg(feature = "stream")]
    {
        tonic_prost_build::configure()
            .file_descriptor_set_path(out_dir.join("stream_descriptor.bin"))
            .compile_protos(&["proto/stream.proto"], &["proto"])
            .unwrap();
    }
}
