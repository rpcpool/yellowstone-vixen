fn main() {
    prost_build::Config::new()
        .enable_type_names()
        .compile_protos(&["proto/account.proto"], &["proto"])
        .unwrap();
}
