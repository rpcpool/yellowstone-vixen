use std::{env, path::PathBuf};

fn main() {
    #[cfg(feature = "parser")]
    {
        prost_build::Config::new()
            .enable_type_names()
            .compile_protos(&["proto/parser.proto"], &["proto"])
            .unwrap();
    }

    #[cfg(feature = "stream")]
    {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        tonic_build::configure()
            .file_descriptor_set_path(out_dir.join("stream_descriptor.bin"))
            .compile(&["proto/stream.proto"], &["proto"])
            .unwrap();
    }
}
