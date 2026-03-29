use std::path::PathBuf;
use std::process::Command;

fn scenario_min_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/scenario-min.json")
}

fn tick_lines(stderr: &[u8]) -> Vec<u64> {
    String::from_utf8_lossy(stderr)
        .lines()
        .filter_map(|line| {
            let v: serde_json::Value = serde_json::from_str(line).ok()?;
            if v.get("event")?.as_str()? != "tick" {
                return None;
            }
            v.get("tick")?.as_u64()
        })
        .collect()
}

#[test]
fn emit_tick_json_offline_writes_parseable_stderr_lines() {
    let scenario = scenario_min_path();
    let out = Command::new(env!("CARGO_BIN_EXE_aetherforge_scenario"))
        .args([
            "--offline",
            "--emit-tick-json",
            scenario.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("spawn aetherforge_scenario");
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(tick_lines(&out.stderr), vec![1]);
}

#[test]
fn emit_tick_json_quiet_suppresses_tick_lines() {
    let scenario = scenario_min_path();
    let out = Command::new(env!("CARGO_BIN_EXE_aetherforge_scenario"))
        .args([
            "--offline",
            "--emit-tick-json",
            "--quiet",
            scenario.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("spawn aetherforge_scenario");
    assert!(out.status.success());
    assert!(tick_lines(&out.stderr).is_empty());
}
