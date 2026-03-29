//! Phase 6b — HTTP-only policy loop (no direct sim kernel imports in this binary).

use std::process::ExitCode;

use aetherforge_cli::player::{run_player, PlayerConfig, PlayerPolicy};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "aetherforge_player")]
struct Args {
    #[arg(long)]
    base_url: String,
    #[arg(long, default_value_t = 0)]
    seed: u64,
    #[arg(long, value_parser = parse_policy)]
    policy: PlayerPolicyArg,
    #[arg(long, default_value_t = 10)]
    max_steps: u32,
    /// Comma-separated action kinds (allowlist).
    #[arg(long, default_value = "noop")]
    intents: String,
    #[arg(long, default_value_t = false)]
    verbose: bool,
    /// LLM adapter executable (**argv only**, never `sh -c` by default).
    #[arg(long)]
    llm_exec: Option<String>,
    /// Extra arguments for `llm-exec` (repeat flag).
    #[arg(long = "llm-arg", action = clap::ArgAction::Append)]
    llm_arg: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
enum PlayerPolicyArg {
    Random,
    RoundRobin,
}

fn parse_policy(s: &str) -> Result<PlayerPolicyArg, String> {
    match s {
        "random" => Ok(PlayerPolicyArg::Random),
        "round_robin" => Ok(PlayerPolicyArg::RoundRobin),
        _ => Err(format!("policy must be random|round_robin, got {s}")),
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = Args::parse();
    let policy = match args.policy {
        PlayerPolicyArg::Random => PlayerPolicy::Random,
        PlayerPolicyArg::RoundRobin => PlayerPolicy::RoundRobin,
    };
    let intents: Vec<String> = args
        .intents
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let llm_cmd = match args.llm_exec {
        Some(exe) => {
            let mut v = vec![exe];
            v.extend(args.llm_arg);
            Some(v)
        }
        None => {
            if !args.llm_arg.is_empty() {
                eprintln!(
                    "{}",
                    serde_json::json!({
                        "ok": false,
                        "error": "--llm-arg requires --llm-exec"
                    })
                );
                return ExitCode::from(1);
            }
            None
        }
    };
    let cfg = PlayerConfig {
        base_url: args.base_url,
        seed: args.seed,
        policy,
        max_steps: args.max_steps,
        intents,
        verbose: args.verbose,
        llm_cmd,
    };
    match run_player(cfg).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", serde_json::json!({ "ok": false, "error": e.to_string() }));
            ExitCode::from(1)
        }
    }
}
