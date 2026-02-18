use std::{io::Write, process::Command};

/// Validate that a protobuf schema string is syntactically correct by running `protoc`.
///
/// Panics if `protoc` is not installed or the schema is invalid.
pub fn check_protobuf_format(schema: &str) {
    let dir = std::env::temp_dir().join("vixen_proto_check");
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");

    let proto_path = dir.join("check.proto");

    let mut f = std::fs::File::create(&proto_path).expect("failed to create temp proto file");

    f.write_all(schema.as_bytes())
        .expect("failed to write proto file");

    let output = Command::new("protoc")
        .arg("--descriptor_set_out=/dev/null")
        .arg(&proto_path)
        .arg(format!("--proto_path={}", dir.display()))
        .output()
        .expect("failed to run protoc — is it installed?");

    assert!(
        output.status.success(),
        "protoc validation failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
