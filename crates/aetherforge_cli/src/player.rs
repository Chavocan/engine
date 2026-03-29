//! HTTP-only autonomous player (Phase 6b). **Do not import the sim kernel crate here** — remote-AI parity.

use std::process::{Command, Stdio};

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy)]
pub enum PlayerPolicy {
    Random,
    RoundRobin,
}

#[derive(Debug)]
pub struct PlayerConfig {
    pub base_url: String,
    pub seed: u64,
    pub policy: PlayerPolicy,
    pub max_steps: u32,
    /// Non-empty allowlist of action `kind` strings for random / round-robin.
    pub intents: Vec<String>,
    pub verbose: bool,
    /// `argv` only (no `sh -c`) — first element is the executable.
    pub llm_cmd: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct PlayerError {
    pub message: String,
}

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PlayerError {}

#[derive(Deserialize)]
struct LlmIntentLine {
    kind: String,
}

pub async fn run_player(cfg: PlayerConfig) -> Result<(), PlayerError> {
    if cfg.intents.is_empty() {
        return Err(PlayerError {
            message: "intents allowlist must be non-empty".into(),
        });
    }

    let base = cfg.base_url.trim_end_matches('/');
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| PlayerError {
            message: format!("http client: {e}"),
        })?;

    let create_url = format!("{base}/v1/sessions");
    let res = client
        .post(&create_url)
        .json(&json!({ "seed": cfg.seed }))
        .send()
        .await
        .map_err(|e| PlayerError {
            message: format!("POST sessions: {e}"),
        })?;
    if !res.status().is_success() {
        return Err(PlayerError {
            message: format!("POST sessions: HTTP {}", res.status()),
        });
    }
    let body: Value = res.json().await.map_err(|e| PlayerError {
        message: format!("sessions json: {e}"),
    })?;
    let sid = body["session_id"]
        .as_str()
        .ok_or_else(|| PlayerError {
            message: "missing session_id".into(),
        })?;

    let mut rng = StdRng::seed_from_u64(cfg.seed);

    for step in 0..cfg.max_steps {
        let obs_url = format!("{base}/v1/sessions/{sid}/observation");
        let res = client.get(&obs_url).send().await.map_err(|e| PlayerError {
            message: format!("GET observation: {e}"),
        })?;
        if !res.status().is_success() {
            return Err(PlayerError {
                message: format!("GET observation: HTTP {}", res.status()),
            });
        }
        let obs: Value = res.json().await.map_err(|e| PlayerError {
            message: format!("observation json: {e}"),
        })?;

        if cfg.verbose {
            eprintln!(
                "{}",
                json!({
                    "player_verbose": true,
                    "step": step,
                    "tick": obs["tick"],
                })
            );
        }

        let kind = if let Some(argv) = &cfg.llm_cmd {
            if argv.is_empty() {
                return Err(PlayerError {
                    message: "llm-cmd must have at least argv[0] executable".into(),
                });
            }
            let obs_line = serde_json::to_string(&obs).map_err(|e| PlayerError {
                message: format!("serialize obs for llm: {e}"),
            })?;
            let mut child = Command::new(&argv[0])
                .args(&argv[1..])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| PlayerError {
                    message: format!("spawn llm-cmd: {e}"),
                })?;
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin
                    .write_all(obs_line.as_bytes())
                    .map_err(|e| PlayerError {
                        message: format!("llm stdin: {e}"),
                    })?;
                stdin.write_all(b"\n").map_err(|e| PlayerError {
                    message: format!("llm stdin newline: {e}"),
                })?;
            }
            let out = child.wait_with_output().map_err(|e| PlayerError {
                message: format!("llm wait: {e}"),
            })?;
            if !out.status.success() {
                return Err(PlayerError {
                    message: format!(
                        "llm-cmd exit {:?}: stderr={}",
                        out.status.code(),
                        String::from_utf8_lossy(&out.stderr)
                    ),
                });
            }
            let line = out
                .stdout
                .split(|&b| b == b'\n')
                .next()
                .and_then(|s| std::str::from_utf8(s).ok())
                .ok_or_else(|| PlayerError {
                    message: "llm-cmd produced no stdout line".into(),
                })?;
            let parsed: LlmIntentLine = serde_json::from_str(line).map_err(|e| PlayerError {
                message: format!("llm stdout json: {e}"),
            })?;
            parsed.kind
        } else {
            match cfg.policy {
                PlayerPolicy::Random => cfg
                    .intents
                    .choose(&mut rng)
                    .expect("intents non-empty")
                    .clone(),
                PlayerPolicy::RoundRobin => {
                    cfg.intents[step as usize % cfg.intents.len()].clone()
                }
            }
        };

        let action_url = format!("{base}/v1/sessions/{sid}/action");
        let res = client
            .post(&action_url)
            .json(&json!({
                "schema_version": "1.0.0",
                "kind": kind,
                "payload": {},
            }))
            .send()
            .await
            .map_err(|e| PlayerError {
                message: format!("POST action: {e}"),
            })?;
        if !res.status().is_success() {
            let t = res.text().await.unwrap_or_default();
            return Err(PlayerError {
                message: format!("POST action: HTTP body={t}"),
            });
        }
    }

    Ok(())
}
