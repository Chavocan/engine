//! Concurrent HTTP actions on one session — final tick matches total applied (Phase 1d stress).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use futures_util::future::join_all;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn app() -> Router {
    aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
        ..Default::default()
    })
}

#[tokio::test]
async fn concurrent_actions_on_same_session_final_tick_matches_total() {
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
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let created: Value = serde_json::from_slice(&body).unwrap();
    let sid = created["session_id"].as_str().unwrap().to_string();

    let n: usize = 24;
    let action_body = json!({
        "schema_version": "1.0.0",
        "kind": "till_soil",
        "payload": {}
    })
    .to_string();

    let futures = (0..n).map(|i| {
        let app = app.clone();
        let sid = sid.clone();
        let body = action_body.clone();
        async move {
            let res = app
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(format!("/v1/sessions/{sid}/action"))
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            (i, res.status())
        }
    });

    let outcomes = join_all(futures).await;
    for (i, status) in outcomes {
        assert_eq!(
            status,
            StatusCode::OK,
            "parallel action {i} expected OK, got {status}"
        );
    }

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
    assert_eq!(
        obs["tick"],
        json!(n),
        "observation tick should equal number of serialized actions"
    );
}

#[tokio::test]
async fn concurrent_actions_high_fanout_48_final_tick_matches() {
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
    assert_eq!(res.status(), StatusCode::CREATED);
    let body = res.into_body().collect().await.unwrap().to_bytes();
    let created: Value = serde_json::from_slice(&body).unwrap();
    let sid = created["session_id"].as_str().unwrap().to_string();

    let n: usize = 48;
    let action_body = json!({
        "schema_version": "1.0.0",
        "kind": "till_soil",
        "payload": {}
    })
    .to_string();

    let futures = (0..n).map(|i| {
        let app = app.clone();
        let sid = sid.clone();
        let body = action_body.clone();
        async move {
            let res = app
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(format!("/v1/sessions/{sid}/action"))
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            (i, res.status())
        }
    });

    let outcomes = join_all(futures).await;
    for (i, status) in outcomes {
        assert_eq!(status, StatusCode::OK, "fanout action {i}");
    }

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
    let obs: Value = serde_json::from_slice(
        &res.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap();
    assert_eq!(obs["tick"], json!(n));
}
