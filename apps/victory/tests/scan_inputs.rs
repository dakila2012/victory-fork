use std::path::Path;
use std::process::Command;

#[test]
fn scans_report_a_missing_input_even_after_a_valid_file() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let missing = manifest.join("missing-input");
    for command in ["census", "shader-info", "translate"] {
        let output = Command::new(env!("CARGO_BIN_EXE_victory"))
            .arg(command)
            .arg(&manifest)
            .arg(&missing)
            .output()
            .expect("launch victory");
        assert!(
            !output.status.success(),
            "{command} accepted a missing path"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(missing.to_string_lossy().as_ref()),
            "{command}: {stderr}"
        );
    }
}
