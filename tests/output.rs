use std::path::PathBuf;
use std::process::Command;

#[test]
fn impl_method_prints_readable_timing_output() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_manifest = manifest_dir.join("tests/fixtures/impl_stdout/Cargo.toml");
    let target_dir = manifest_dir.join("target/test-fixtures/impl-stdout");

    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--offline")
        .arg("--manifest-path")
        .arg(&fixture_manifest)
        .env("CARGO_TARGET_DIR", &target_dir)
        .env("CARGO_NET_OFFLINE", "true")
        .output()
        .expect("failed to run impl stdout fixture");

    assert!(
        output.status.success(),
        "fixture failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("[exec_time] run took "));
    assert!(stdout.contains(" ms"));
    assert!(!stdout.contains("mills"));
    assert!(stdout.contains("result=42"));
    assert!(stdout.contains("[exec_time] worker.job took "));
    assert!(stdout.contains(" us"));
    assert!(stdout.contains("named-result=7"));
}
