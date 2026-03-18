use std::{io::Write, process::Command};

use yellowstone_vixen_core::Pubkey;

/// Parse a base58 public key string.
pub fn p(s: &str) -> Pubkey { s.parse().unwrap() }

/// Validate that a protobuf schema string is syntactically correct by running `protoc`.
///
/// Skipped when `protoc` is not found on `$PATH` or when the `SKIP_PROTOC` env var is set.
///
/// Each invocation uses a unique temp directory to avoid races when tests run
/// in parallel.
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

    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let proto_path = dir.path().join("check.proto");

    let mut f = std::fs::File::create(&proto_path).expect("failed to create temp proto file");

    f.write_all(schema.as_bytes())
        .expect("failed to write proto file");

    let output = Command::new("protoc")
        .arg("--descriptor_set_out=/dev/null")
        .arg(&proto_path)
        .arg(format!("--proto_path={}", dir.path().display()))
        .output()
        .expect("protoc invocation failed");

    assert!(
        output.status.success(),
        "protoc validation failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
