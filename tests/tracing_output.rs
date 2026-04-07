#![cfg(feature = "tracing")]

use std::path::PathBuf;
use std::process::Command;

#[test]
fn tracing_backend_emits_events() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_manifest = manifest_dir.join("tests/fixtures/tracing_backend/Cargo.toml");
    let target_dir = manifest_dir.join("target/test-fixtures/tracing-backend");

    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--offline")
        .arg("--manifest-path")
        .arg(&fixture_manifest)
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("CARGO_NET_OFFLINE", "true")
        .output()
        .expect("failed to run tracing fixture");

    assert!(
        output.status.success(),
        "fixture failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("INFO"));
    assert!(stdout.contains("DEBUG"));
    assert!(stdout.contains("WARN"));
    assert!(stdout.contains("worker.trace took "));
    assert!(stdout.contains("worker.debug took "));
    assert!(stdout.contains("worker.slow took "));
    assert!(stdout.contains("worker.warn took "));
    assert!(!stdout.contains("worker.suppressed took "));
    assert!(stdout.contains("trace-result=1"));
    assert!(stdout.contains("debug-result=2"));
    assert!(stdout.contains("slow-result=3"));
    assert!(stdout.contains("suppressed-result=4"));
    assert!(stdout.contains("warn-result=5"));
}
