use std::{env, path::PathBuf};

fn main() {
    #[cfg(feature = "stream")]
    {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        tonic_build::configure()
            .file_descriptor_set_path(out_dir.join("stream_descriptor.bin"))
            .compile(&["proto/stream.proto"], &["proto"])
            .unwrap();
    }
}
