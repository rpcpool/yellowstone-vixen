use std::{io::Write, process::Command};

/// Validate that a protobuf schema string is syntactically correct by running `protoc`.
///
/// Skipped when `protoc` is not found on `$PATH` or when the `SKIP_PROTOC` env var is set.
///
/// ```sh
/// SKIP_PROTOC=1 cargo test          # skip protoc validation
/// ```
pub fn check_protobuf_format(schema: &str) {
    if std::env::var("SKIP_PROTOC").is_ok() {
        return;
    }

    let Ok(output) = Command::new("protoc").arg("--version").output() else {
        eprintln!("protoc not found — skipping proto schema validation");
        return;
    };

    if !output.status.success() {
        eprintln!("protoc not working — skipping proto schema validation");
        return;
    }

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
        .expect("protoc invocation failed");

    assert!(
        output.status.success(),
        "protoc validation failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
