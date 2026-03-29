//! Axum control plane — Phase 2b: observation bytes via `aetherforge_sim::wire` (parity with headless).

use std::collections::HashMap;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
#[cfg(feature = "sse-obs")]
use std::pin::Pin;
#[cfg(feature = "sse-obs")]
use axum::response::sse::{Event, KeepAlive, Sse};
#[cfg(feature = "sse-obs")]
use futures_util::stream::Stream;
use aetherforge_schemas::v1::Action;
use aetherforge_sim::{observation_to_vec, Intent, Simulation, SimulationConfig};
use serde::Deserialize;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::ai_driver_enqueue_intent;
use crate::play_log;

/// Hard cap per Director Phase 4a (abuse resistance for AI agents).
const MAX_ACTIONS_PER_BATCH: usize = 32;

/// Default session lifetime cap (single + batch intents). Override with **`AETHERFORGE_MAX_ACTIONS_PER_SESSION`**.
const DEFAULT_MAX_ACTIONS_PER_SESSION: u64 = 10_000;

struct SessionEntry {
    sim: Simulation,
    actions_applied: u64,
}

#[derive(Clone, Debug)]
pub struct ControlConfig {
    pub max_actions_per_session: u64,
}

impl ControlConfig {
    pub fn from_env() -> Self {
        let n = std::env::var("AETHERFORGE_MAX_ACTIONS_PER_SESSION")
            .ok()
            .and_then(|s| s.parse().ok())
            .filter(|&n| n > 0)
            .unwrap_or(DEFAULT_MAX_ACTIONS_PER_SESSION);
        Self {
            max_actions_per_session: n,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    sessions: Arc<Mutex<HashMap<String, Arc<Mutex<SessionEntry>>>>>,
    max_actions_per_session: u64,
}

#[derive(Debug, Deserialize, Default)]
pub struct CreateSessionBody {
    #[serde(default)]
    pub seed: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct ActionBatchBody {
    pub actions: Vec<Action>,
}

pub fn app_router() -> Router {
    app_router_with_config(ControlConfig::from_env())
}

#[cfg(feature = "farm-stub")]
fn play_log_farm_harvested(obs: &aetherforge_sim::Observation, session_id: &str) {
    let Some(ref f) = obs.farm else {
        return;
    };
    let harvested_total: u64 = f
        .inventory
        .items
        .iter()
        .filter(|(k, _)| k.starts_with("harvested_"))
        .map(|(_, v)| *v as u64)
        .sum();
    play_log::emit(
        "farm_harvested",
        &obs.run_id,
        obs.tick,
        Some(session_id),
        serde_json::json!({
            "plots_remaining": f.plots.len(),
            "harvested_item_total": harvested_total
        }),
    );
}

pub fn app_router_with_config(cfg: ControlConfig) -> Router {
    let state = AppState {
        sessions: Arc::new(Mutex::new(HashMap::new())),
        max_actions_per_session: cfg.max_actions_per_session,
    };
    let app = Router::new()
        .route("/v1/sessions", post(create_session))
        .route("/v1/sessions/:id", delete(delete_session))
        .route("/v1/sessions/:id/action", post(post_action))
        .route("/v1/sessions/:id/actions", post(post_actions_batch))
        .route("/v1/sessions/:id/observation", get(get_observation));
    #[cfg(feature = "sse-obs")]
    let app = app.route(
        "/v1/sessions/:id/observe/stream",
        get(observe_stream),
    );
    app.with_state(state)
}

async fn create_session(
    State(state): State<AppState>,
    Json(body): Json<CreateSessionBody>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, Json<serde_json::Value>)> {
    let id = Uuid::new_v4().to_string();
    let run_id = id.clone();
    let seed = body.seed.unwrap_or_else(rand::random);
    let entry = Arc::new(Mutex::new(SessionEntry {
        sim: Simulation::with_config(SimulationConfig::new(run_id, seed)),
        actions_applied: 0,
    }));
    state.sessions.lock().await.insert(id.clone(), entry);
    play_log::emit(
        "session_created",
        &id,
        0,
        Some(&id),
        serde_json::json!({}),
    );
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "session_id": id,
            "schema_version": "1.0.0",
            "seed": seed
        })),
    ))
}

async fn delete_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    let removed = state.sessions.lock().await.remove(&id).is_some();
    if removed {
        play_log::emit("session_deleted", &id, 0, Some(&id), serde_json::json!({}));
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(err_not_found())
    }
}

async fn post_action(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(action): Json<Action>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let entry_arc = {
        let map = state.sessions.lock().await;
        map.get(&id).ok_or_else(err_not_found)?.clone()
    };
    let mut entry = entry_arc.lock().await;
    if entry.actions_applied >= state.max_actions_per_session {
        return Err(err_session_action_quota(
            state.max_actions_per_session,
            entry.actions_applied,
        ));
    }
    let kind = action.kind.clone();
    ai_driver_enqueue_intent(
        &mut entry.sim,
        Intent {
            kind: kind.clone(),
        },
    );
    entry.sim.step();
    entry.actions_applied += 1;
    let obs = entry.sim.snapshot_observation();
    let tick = obs.tick;
    #[cfg(feature = "farm-stub")]
    if kind == "farm_advance_day" {
        if let Some(ref f) = obs.farm {
            play_log::emit(
                "farm_day_advanced",
                &obs.run_id,
                obs.tick,
                Some(&id),
                serde_json::json!({ "day": f.day }),
            );
        }
    }
    #[cfg(feature = "farm-stub")]
    if kind == "farm_harvest" {
        play_log_farm_harvested(&obs, &id);
    }
    play_log::emit(
        "action_applied",
        &obs.run_id,
        obs.tick,
        Some(&id),
        serde_json::json!({ "kind": kind }),
    );
    play_log::emit(
        "tick_advanced",
        &obs.run_id,
        obs.tick,
        Some(&id),
        serde_json::json!({}),
    );
    Ok(Json(serde_json::json!({
        "ok": true,
        "tick": tick,
        "schema_version": "1.0.0"
    })))
}

async fn post_actions_batch(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<ActionBatchBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if body.actions.is_empty() {
        return Err(err_invalid_batch("actions array must be non-empty"));
    }
    if body.actions.len() > MAX_ACTIONS_PER_BATCH {
        return Err(err_batch_too_large(body.actions.len()));
    }

    let entry_arc = {
        let map = state.sessions.lock().await;
        map.get(&id).ok_or_else(err_not_found)?.clone()
    };
    let mut entry = entry_arc.lock().await;

    let n = body.actions.len() as u64;
    if entry
        .actions_applied
        .checked_add(n)
        .map(|t| t > state.max_actions_per_session)
        .unwrap_or(true)
    {
        return Err(err_session_action_quota(
            state.max_actions_per_session,
            entry.actions_applied,
        ));
    }

    let mut kinds = Vec::with_capacity(body.actions.len());
    for action in &body.actions {
        let kind = action.kind.clone();
        kinds.push(kind.clone());
        ai_driver_enqueue_intent(
            &mut entry.sim,
            Intent {
                kind: kind.clone(),
            },
        );
        entry.sim.step();
        #[cfg(feature = "farm-stub")]
        if kind == "farm_advance_day" {
            let obs = entry.sim.snapshot_observation();
            if let Some(ref f) = obs.farm {
                play_log::emit(
                    "farm_day_advanced",
                    &obs.run_id,
                    obs.tick,
                    Some(&id),
                    serde_json::json!({ "day": f.day }),
                );
            }
        }
        #[cfg(feature = "farm-stub")]
        if kind == "farm_harvest" {
            let obs = entry.sim.snapshot_observation();
            play_log_farm_harvested(&obs, &id);
        }
    }
    entry.actions_applied += n;

    let obs = entry.sim.snapshot_observation();
    let final_tick = obs.tick;
    play_log::emit(
        "batch_actions_applied",
        &obs.run_id,
        final_tick,
        Some(&id),
        serde_json::json!({
            "count": body.actions.len(),
            "kinds": kinds,
            "final_tick": final_tick,
        }),
    );

    Ok(Json(serde_json::json!({
        "ok": true,
        "tick": final_tick,
        "applied": body.actions.len(),
        "schema_version": "1.0.0"
    })))
}

async fn get_observation(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let entry_arc = {
        let map = state.sessions.lock().await;
        map.get(&id).ok_or_else(err_not_found)?.clone()
    };
    let entry = entry_arc.lock().await;
    let obs = entry.sim.snapshot_observation();
    play_log::emit(
        "observation_served",
        &obs.run_id,
        obs.tick,
        Some(&id),
        serde_json::json!({ "schema_version": obs.schema_version }),
    );
    let bytes = observation_to_vec(&obs).map_err(|_| internal_error())?;
    match Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(bytes))
    {
        Ok(r) => Ok(r),
        Err(_) => Err(internal_error()),
    }
}

#[cfg(feature = "sse-obs")]
async fn observe_stream(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<
    Sse<Pin<Box<dyn Stream<Item = std::io::Result<Event>> + Send>>>,
    StatusCode,
> {
    let entry_arc = {
        let map = state.sessions.lock().await;
        map.get(&id).cloned().ok_or(StatusCode::NOT_FOUND)?
    };

    const POLL_MS: u64 = 25;
    let stream = async_stream::stream! {
        let mut last_sent_tick: Option<u64> = None;
        loop {
            let (tick, payload) = {
                let g = entry_arc.lock().await;
                let obs = g.sim.snapshot_observation();
                let t = obs.tick;
                let bytes = match observation_to_vec(&obs) {
                    Ok(b) => b,
                    Err(e) => {
                        yield Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            e.to_string(),
                        ));
                        break;
                    }
                };
                (t, String::from_utf8_lossy(&bytes).to_string())
            };
            if last_sent_tick != Some(tick) {
                last_sent_tick = Some(tick);
                yield Ok(Event::default().data(payload));
            }
            tokio::time::sleep(std::time::Duration::from_millis(POLL_MS)).await;
        }
    };

    let boxed: Pin<Box<dyn Stream<Item = std::io::Result<Event>> + Send>> = Box::pin(stream);
    Ok(
        Sse::new(boxed).keep_alive(
            KeepAlive::new()
                .interval(std::time::Duration::from_secs(15))
                .text("keepalive"),
        ),
    )
}

fn err_not_found() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "error": {
                "code": "SESSION_NOT_FOUND",
                "message": "session not found",
                "request_id": "n/a"
            },
            "schema_version": "1.0.0"
        })),
    )
}

fn internal_error() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "error": {
                "code": "INTERNAL",
                "message": "internal error",
                "request_id": "n/a"
            },
            "schema_version": "1.0.0"
        })),
    )
}

fn err_batch_too_large(got: usize) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::PAYLOAD_TOO_LARGE,
        Json(serde_json::json!({
            "error": {
                "code": "BATCH_TOO_LARGE",
                "message": format!(
                    "at most {} actions per request; got {}",
                    MAX_ACTIONS_PER_BATCH, got
                ),
                "request_id": "n/a"
            },
            "schema_version": "1.0.0"
        })),
    )
}

fn err_invalid_batch(msg: impl Into<String>) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "error": {
                "code": "INVALID_BATCH",
                "message": msg.into(),
                "request_id": "n/a"
            },
            "schema_version": "1.0.0"
        })),
    )
}

fn err_session_action_quota(limit: u64, applied: u64) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::TOO_MANY_REQUESTS,
        Json(serde_json::json!({
            "error": {
                "code": "SESSION_ACTION_QUOTA",
                "message": format!(
                    "session action cap {limit} reached (currently {applied} applied); create a new session or raise AETHERFORGE_MAX_ACTIONS_PER_SESSION"
                ),
                "request_id": "n/a"
            },
            "schema_version": "1.0.0"
        })),
    )
}
