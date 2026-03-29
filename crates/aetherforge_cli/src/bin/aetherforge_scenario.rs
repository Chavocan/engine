//! Phase 4b — run a JSON scenario offline (`Simulation`) or against a live control plane (HTTP).

use std::path::PathBuf;
use std::process::ExitCode;

use aetherforge_cli::scenario::{run_http, run_offline, ScenarioFile};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aetherforge_scenario")]
struct Args {
    /// Drive `Simulation` in-process (no network; CI-friendly).
    #[arg(long)]
    offline: bool,
    /// Scenario JSON path.
    path: PathBuf,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let raw = match std::fs::read_to_string(&args.path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "ok": false,
                    "step_index": 0,
                    "reason": format!("read scenario file: {e}"),
                    "expected_tick": serde_json::Value::Null,
                    "actual_tick": serde_json::Value::Null,
                })
            );
            return ExitCode::from(1);
        }
    };

    let scenario: ScenarioFile = match serde_json::from_str(&raw) {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "{}",
                serde_json::json!({
                    "ok": false,
                    "step_index": 0,
                    "reason": format!("parse scenario json: {e}"),
                    "expected_tick": serde_json::Value::Null,
                    "actual_tick": serde_json::Value::Null,
                })
            );
            return ExitCode::from(1);
        }
    };

    let result = if args.offline {
        run_offline(&scenario)
    } else {
        let base = match scenario.base_url.as_deref() {
            Some(b) if !b.is_empty() => b,
            _ => {
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "ok": false,
                        "step_index": 0,
                        "reason": "HTTP mode requires non-empty \"base_url\" in scenario JSON (or use --offline)",
                        "expected_tick": serde_json::Value::Null,
                        "actual_tick": serde_json::Value::Null,
                    })
                );
                return ExitCode::from(1);
            }
        };
        run_http(&scenario, base).await
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(fail) => {
            eprintln!("{}", fail.to_json_line());
            ExitCode::from(1)
        }
    }
}
