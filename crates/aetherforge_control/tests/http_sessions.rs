//! Integration tests — session create → action → observation + wire parity (Phase 2b).

use aetherforge_control::ai_driver_enqueue_intent;
use aetherforge_sim::{
    observation_to_vec, Intent, Simulation, SimulationConfig,
};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn app() -> Router {
    aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
        ..Default::default()
    })
}

fn required_observation_keys(v: &Value) -> bool {
    v.get("schema_version").and_then(|x| x.as_str()).is_some()
        && v.get("tick").and_then(|x| x.as_u64()).is_some()
        && v.get("run_id").and_then(|x| x.as_str()).is_some()
        && v.get("message").and_then(|x| x.as_str()).is_some()
        && v.get("rng_draw").and_then(|x| x.as_u64()).is_some()
        && v
            .get("world")
            .and_then(|w| w.get("world_version"))
            .and_then(|x| x.as_str())
            .is_some()
        && v.get("world").and_then(|w| w.get("entities")).and_then(|e| e.as_array()).is_some()
}

#[tokio::test]
async fn create_session_returns_ids() {
    let app = app();
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&body).unwrap();
    assert!(v["session_id"].as_str().unwrap().len() >= 8);
    assert_eq!(v["schema_version"], "1.0.0");
    assert!(v["seed"].as_u64().is_some());
}

#[tokio::test]
async fn round_trip_action_observation() {
    let app = app();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let created: Value = serde_json::from_slice(&body).unwrap();
    let sid = created["session_id"].as_str().unwrap().to_string();

    let action_body = json!({
        "schema_version": "1.0.0",
        "kind": "till_soil",
        "payload": {}
    });
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sessions/{sid}/action"))
                .header("content-type", "application/json")
                .body(Body::from(action_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let act: Value = serde_json::from_slice(
        &res.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap();
    assert_eq!(act["tick"], 1);

    let res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/sessions/{sid}/observation"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let obs: Value = serde_json::from_slice(
        &res.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap();
    assert!(required_observation_keys(&obs));
    assert_eq!(obs["tick"], 1);
    assert!(obs["message"].as_str().unwrap().contains("till_soil"));
}

#[tokio::test]
async fn unknown_session_404_shape() {
    let app = app();
    let res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/v1/sessions/does-not-exist/observation")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let v: Value = serde_json::from_slice(
        &res.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap();
    assert_eq!(v["error"]["code"], "SESSION_NOT_FOUND");
    assert_eq!(v["schema_version"], "1.0.0");
}

#[tokio::test]
async fn delete_session_lifecycle() {
    let app = app();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let created: Value = serde_json::from_slice(&body).unwrap();
    let sid = created["session_id"].as_str().unwrap().to_string();

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("/v1/sessions/{sid}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/sessions/{sid}/observation"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn http_same_seed_parallel_observation_probes_match() {
    let app = app();
    let seed = 12_345_u64;
    let body = json!({ "seed": seed }).to_string();

    let mk_session = || async {
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/sessions")
                    .header("content-type", "application/json")
                    .body(Body::from(body.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::CREATED);
        let v: Value =
            serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes()).unwrap();
        v["session_id"].as_str().unwrap().to_string()
    };

    let sid_a = mk_session().await;
    let sid_b = mk_session().await;

    for kind in ["x", "y", "z"] {
        let action_body = json!({
            "schema_version": "1.0.0",
            "kind": kind,
            "payload": {}
        })
        .to_string();
        for sid in [&sid_a, &sid_b] {
            let res = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(format!("/v1/sessions/{sid}/action"))
                        .header("content-type", "application/json")
                        .body(Body::from(action_body.clone()))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(res.status(), StatusCode::OK);
        }
        let oa = get_obs_json(&app, sid_a.as_str()).await;
        let ob = get_obs_json(&app, sid_b.as_str()).await;
        assert_eq!(oa["tick"], ob["tick"]);
        assert_eq!(oa["rng_draw"], ob["rng_draw"]);
        assert_eq!(oa["message"], ob["message"]);
    }
}

async fn get_obs_json(app: &Router, sid: &str) -> Value {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/sessions/{sid}/observation"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes()).unwrap()
}

#[tokio::test]
async fn two_sequential_action_clients_no_deadlock() {
    let app = app();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    let sid = serde_json::from_slice::<Value>(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    for kind in ["first", "second"] {
        let action_body = json!({
            "schema_version": "1.0.0",
            "kind": kind,
            "payload": {}
        })
        .to_string();
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{sid}/action"))
                    .header("content-type", "application/json")
                    .body(Body::from(action_body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    let obs = get_obs_json(&app, sid.as_str()).await;
    assert_eq!(obs["tick"], 2);
}

#[tokio::test]
async fn http_get_observation_bytes_match_wire_replay() {
    let app = app();
    let seed = 777_u64;
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "seed": seed }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let sid = serde_json::from_slice::<Value>(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let action_body = json!({
        "schema_version": "1.0.0",
        "kind": "parity_probe",
        "payload": {}
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sessions/{sid}/action"))
                .header("content-type", "application/json")
                .body(Body::from(action_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri(format!("/v1/sessions/{sid}/observation"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let got = res.into_body().collect().await.unwrap().to_bytes().to_vec();

    let mut sim = Simulation::with_config(SimulationConfig::new(sid.clone(), seed));
    ai_driver_enqueue_intent(
        &mut sim,
        Intent {
            kind: "parity_probe".to_string(),
        },
    );
    sim.step();
    let want = observation_to_vec(&sim.snapshot_observation()).unwrap();
    assert_eq!(got, want);
}

#[tokio::test]
async fn batch_three_noops_matches_three_single_posts_tick() {
    let app = app();
    let seed = 555_u64;

    let sid_batch = session_with_seed(&app, seed).await;
    let batch_body = json!({
        "actions": [
            {"schema_version": "1.0.0", "kind": "n1", "payload": {}},
            {"schema_version": "1.0.0", "kind": "n2", "payload": {}},
            {"schema_version": "1.0.0", "kind": "n3", "payload": {}},
        ]
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sessions/{sid_batch}/actions"))
                .header("content-type", "application/json")
                .body(Body::from(batch_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let ack: Value = serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap();
    assert_eq!(ack["applied"], 3);
    assert_eq!(ack["tick"], 3);

    let sid_seq = session_with_seed(&app, seed).await;
    for k in ["n1", "n2", "n3"] {
        let one = json!({
            "schema_version": "1.0.0",
            "kind": k,
            "payload": {}
        })
        .to_string();
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{sid_seq}/action"))
                    .header("content-type", "application/json")
                    .body(Body::from(one))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    let ob = get_obs_json(&app, sid_batch.as_str()).await;
    let os = get_obs_json(&app, sid_seq.as_str()).await;
    assert_eq!(ob["tick"], os["tick"]);
    assert_eq!(ob["tick"], 3);
    assert_eq!(ob["message"], os["message"]);
    assert_eq!(ob["rng_draw"], os["rng_draw"]);
}

async fn session_with_seed(app: &Router, seed: u64) -> String {
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions")
                .header("content-type", "application/json")
                .body(Body::from(json!({ "seed": seed }).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);
    serde_json::from_slice::<Value>(&res.into_body().collect().await.unwrap().to_bytes()).unwrap()
        ["session_id"]
        .as_str()
        .unwrap()
        .to_string()
}

#[tokio::test]
async fn batch_too_many_actions_returns_413() {
    let app = app();
    let sid = session_with_seed(&app, 1).await;
    let mut actions = Vec::new();
    for i in 0..33 {
        actions.push(json!({
            "schema_version": "1.0.0",
            "kind": format!("k{i}"),
            "payload": {}
        }));
    }
    let body = json!({ "actions": actions }).to_string();
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sessions/{sid}/actions"))
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let v: Value = serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap();
    assert_eq!(v["error"]["code"], "BATCH_TOO_LARGE");
}

#[tokio::test]
async fn batch_empty_actions_returns_400() {
    let app = app();
    let sid = session_with_seed(&app, 2).await;
    let body = json!({ "actions": [] }).to_string();
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sessions/{sid}/actions"))
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let v: Value = serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap();
    assert_eq!(v["error"]["code"], "INVALID_BATCH");
}
