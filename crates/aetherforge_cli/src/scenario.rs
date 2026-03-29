//! JSON-driven multi-step soak: offline (`Simulation`) or HTTP against `aetherforge_serve`.

use aetherforge_control::ai_driver_enqueue_intent;
use aetherforge_schemas::v1::Action;
use aetherforge_sim::{Intent, MissionOutcome, Simulation, SimulationConfig};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScenarioFile {
    /// Used when running against a live server (`aetherforge_scenario` without `--offline`).
    #[serde(default)]
    pub base_url: Option<String>,
    pub seed: u64,
    pub steps: Vec<Step>,
    /// After all steps, assert `Observation.mission.outcome` (**`won`** / **`lost`**). Requires **`farm-stub`** (or server built with it) when expecting **`won`** from the demo harvest vertical.
    #[serde(default)]
    pub expect_mission_outcome: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "use", rename_all = "lowercase")]
pub enum Step {
    Batch {
        actions: Vec<Action>,
        #[serde(default)]
        expect_tick: Option<u64>,
    },
    Single {
        kind: String,
        #[serde(default = "default_schema_version")]
        schema_version: String,
        #[serde(default)]
        payload: serde_json::Value,
        #[serde(default)]
        expect_tick: Option<u64>,
    },
}

fn default_schema_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug)]
pub struct ScenarioFailure {
    pub step_index: usize,
    pub reason: String,
    pub expected_tick: Option<u64>,
    pub actual_tick: Option<u64>,
}

impl ScenarioFailure {
    pub fn to_json_line(&self) -> String {
        serde_json::json!({
            "ok": false,
            "step_index": self.step_index,
            "reason": self.reason,
            "expected_tick": self.expected_tick,
            "actual_tick": self.actual_tick,
        })
        .to_string()
    }
}

fn check_tick(
    step_index: usize,
    expect: Option<u64>,
    actual: u64,
) -> Result<(), ScenarioFailure> {
    if let Some(want) = expect {
        if want != actual {
            return Err(ScenarioFailure {
                step_index,
                reason: format!("tick mismatch: expected {want}, got {actual}"),
                expected_tick: Some(want),
                actual_tick: Some(actual),
            });
        }
    }
    Ok(())
}

/// Run all scenario steps in-process. `on_tick` is invoked after each `Simulation::step` with the
/// current tick value (offline mode only).
pub fn run_offline_with_ticks(
    file: &ScenarioFile,
    on_tick: &mut impl FnMut(u64),
) -> Result<(), ScenarioFailure> {
    let mut sim = Simulation::with_config(SimulationConfig::new("scenario-offline", file.seed));
    apply_steps(
        &mut sim,
        &file.steps,
        |s| s.snapshot_observation().tick,
        on_tick,
    )?;
    if let Some(ref want) = file.expect_mission_outcome {
        check_expect_mission_outcome(file.steps.len(), want, &sim)?;
    }
    Ok(())
}

pub fn run_offline(file: &ScenarioFile) -> Result<(), ScenarioFailure> {
    run_offline_with_ticks(file, &mut |_| {})
}

fn apply_steps(
    sim: &mut Simulation,
    steps: &[Step],
    current_tick: impl Fn(&Simulation) -> u64,
    on_tick: &mut impl FnMut(u64),
) -> Result<(), ScenarioFailure> {
    for (i, step) in steps.iter().enumerate() {
        match step {
            Step::Batch {
                actions,
                expect_tick,
            } => {
                for a in actions {
                    ai_driver_enqueue_intent(
                        sim,
                        Intent {
                            kind: a.kind.clone(),
                        },
                    );
                    sim.step();
                    on_tick(current_tick(sim));
                }
                check_tick(i, *expect_tick, current_tick(sim))?;
            }
            Step::Single {
                kind,
                schema_version: _,
                payload: _,
                expect_tick,
            } => {
                ai_driver_enqueue_intent(
                    sim,
                    Intent {
                        kind: kind.clone(),
                    },
                );
                sim.step();
                on_tick(current_tick(sim));
                check_tick(i, *expect_tick, current_tick(sim))?;
            }
        }
    }
    Ok(())
}

fn check_expect_mission_outcome(
    step_index: usize,
    want: &str,
    sim: &Simulation,
) -> Result<(), ScenarioFailure> {
    let obs = sim.snapshot_observation();
    let actual = obs.mission.as_ref().map(|m| match m.outcome {
        MissionOutcome::Won => "won",
        MissionOutcome::Lost => "lost",
    });
    check_mission_strings(step_index, want, actual)
}

fn check_mission_strings(
    step_index: usize,
    want: &str,
    actual: Option<&str>,
) -> Result<(), ScenarioFailure> {
    let w = want.trim().to_ascii_lowercase();
    if w == "won" {
        if actual != Some("won") {
            return Err(ScenarioFailure {
                step_index,
                reason: format!("mission outcome: expected won, got {actual:?}"),
                expected_tick: None,
                actual_tick: None,
            });
        }
    } else if w == "lost" {
        if actual != Some("lost") {
            return Err(ScenarioFailure {
                step_index,
                reason: format!("mission outcome: expected lost, got {actual:?}"),
                expected_tick: None,
                actual_tick: None,
            });
        }
    } else {
        return Err(ScenarioFailure {
            step_index,
            reason: format!(
                "expect_mission_outcome: invalid value {want:?} (use \"won\" or \"lost\")"
            ),
            expected_tick: None,
            actual_tick: None,
        });
    }
    Ok(())
}

/// After each HTTP step, `on_tick` receives the session tick from the observation endpoint.
pub async fn run_http<F>(
    file: &ScenarioFile,
    base_url: &str,
    on_tick: &mut F,
) -> Result<(), ScenarioFailure>
where
    F: FnMut(u64) + Send,
{
    let base = base_url.trim_end_matches('/');
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| ScenarioFailure {
            step_index: 0,
            reason: format!("http client: {e}"),
            expected_tick: None,
            actual_tick: None,
        })?;

    let create_url = format!("{base}/v1/sessions");
    let res = client
        .post(&create_url)
        .json(&serde_json::json!({ "seed": file.seed }))
        .send()
        .await
        .map_err(|e| ScenarioFailure {
            step_index: 0,
            reason: format!("POST sessions: {e}"),
            expected_tick: None,
            actual_tick: None,
        })?;

    if !res.status().is_success() {
        return Err(ScenarioFailure {
            step_index: 0,
            reason: format!("POST sessions: HTTP {}", res.status()),
            expected_tick: None,
            actual_tick: None,
        });
    }

    let body: serde_json::Value = res.json().await.map_err(|e| ScenarioFailure {
        step_index: 0,
        reason: format!("sessions response json: {e}"),
        expected_tick: None,
        actual_tick: None,
    })?;

    let sid = body["session_id"]
        .as_str()
        .ok_or_else(|| ScenarioFailure {
            step_index: 0,
            reason: "missing session_id in create response".to_string(),
            expected_tick: None,
            actual_tick: None,
        })?;

    for (i, step) in file.steps.iter().enumerate() {
        match step {
            Step::Batch {
                actions,
                expect_tick,
            } => {
                let url = format!("{base}/v1/sessions/{sid}/actions");
                let res = client
                    .post(&url)
                    .json(&serde_json::json!({ "actions": actions }))
                    .send()
                    .await
                    .map_err(|e| ScenarioFailure {
                        step_index: i,
                        reason: format!("POST actions: {e}"),
                        expected_tick: *expect_tick,
                        actual_tick: None,
                    })?;
                if !res.status().is_success() {
                    let text = res.text().await.unwrap_or_default();
                    return Err(ScenarioFailure {
                        step_index: i,
                        reason: format!("POST actions: HTTP body={text}"),
                        expected_tick: *expect_tick,
                        actual_tick: None,
                    });
                }
            }
            Step::Single {
                kind,
                schema_version,
                payload,
                expect_tick,
            } => {
                let action = Action {
                    schema_version: schema_version.clone(),
                    kind: kind.clone(),
                    payload: payload.clone(),
                };
                let url = format!("{base}/v1/sessions/{sid}/action");
                let res = client
                    .post(&url)
                    .json(&action)
                    .send()
                    .await
                    .map_err(|e| ScenarioFailure {
                        step_index: i,
                        reason: format!("POST action: {e}"),
                        expected_tick: *expect_tick,
                        actual_tick: None,
                    })?;
                if !res.status().is_success() {
                    let text = res.text().await.unwrap_or_default();
                    return Err(ScenarioFailure {
                        step_index: i,
                        reason: format!("POST action: HTTP body={text}"),
                        expected_tick: *expect_tick,
                        actual_tick: None,
                    });
                }
            }
        }

        let tick = fetch_tick(&client, base, sid, i).await?;
        check_tick(i, step.expect_tick(), tick)?;
        on_tick(tick);
    }

    if let Some(ref want) = file.expect_mission_outcome {
        let url = format!("{base}/v1/sessions/{sid}/observation");
        let res = client.get(&url).send().await.map_err(|e| ScenarioFailure {
            step_index: file.steps.len(),
            reason: format!("GET observation (mission check): {e}"),
            expected_tick: None,
            actual_tick: None,
        })?;
        if !res.status().is_success() {
            return Err(ScenarioFailure {
                step_index: file.steps.len(),
                reason: format!(
                    "GET observation (mission check): HTTP {}",
                    res.status()
                ),
                expected_tick: None,
                actual_tick: None,
            });
        }
        let body: serde_json::Value = res.json().await.map_err(|e| ScenarioFailure {
            step_index: file.steps.len(),
            reason: format!("observation json (mission check): {e}"),
            expected_tick: None,
            actual_tick: None,
        })?;
        let actual = body
            .get("mission")
            .and_then(|m| m.get("outcome"))
            .and_then(|x| x.as_str());
        check_mission_strings(file.steps.len(), want, actual)?;
    }

    Ok(())
}

impl Step {
    fn expect_tick(&self) -> Option<u64> {
        match self {
            Step::Batch { expect_tick, .. } => *expect_tick,
            Step::Single { expect_tick, .. } => *expect_tick,
        }
    }
}

async fn fetch_tick(
    client: &reqwest::Client,
    base: &str,
    sid: &str,
    step_index: usize,
) -> Result<u64, ScenarioFailure> {
    let url = format!("{base}/v1/sessions/{sid}/observation");
    let res = client.get(&url).send().await.map_err(|e| ScenarioFailure {
        step_index,
        reason: format!("GET observation: {e}"),
        expected_tick: None,
        actual_tick: None,
    })?;
    if !res.status().is_success() {
        return Err(ScenarioFailure {
            step_index,
            reason: format!("GET observation: HTTP {}", res.status()),
            expected_tick: None,
            actual_tick: None,
        });
    }
    let body: serde_json::Value = res.json().await.map_err(|e| ScenarioFailure {
        step_index,
        reason: format!("observation json: {e}"),
        expected_tick: None,
        actual_tick: None,
    })?;
    let tick = body["tick"].as_u64().ok_or_else(|| ScenarioFailure {
        step_index,
        reason: "observation missing tick".to_string(),
        expected_tick: None,
        actual_tick: None,
    })?;
    Ok(tick)
}
