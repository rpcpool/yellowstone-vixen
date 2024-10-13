use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "parser")]
    {
        prost_build::Config::new()
            .enable_type_names()
            .file_descriptor_set_path(out_dir.join("vixen.parser.bin"))
            .compile_protos(&["proto/parser.proto"], &["proto"])
            .unwrap();
    }

    #[cfg(feature = "stream")]
    {
        tonic_build::configure()
            .file_descriptor_set_path(out_dir.join("stream_descriptor.bin"))
            .compile(&["proto/stream.proto"], &["proto"])
            .unwrap();
    }
}
