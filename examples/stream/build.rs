fn main() { prost_build::compile_protos(&["proto/account.proto"], &["proto"]).unwrap(); }
