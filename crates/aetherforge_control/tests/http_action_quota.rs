//! Phase 7a — per-session action cap (429 SESSION_ACTION_QUOTA).

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

fn app_quota(limit: u64) -> Router {
    aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: limit,
        ..Default::default()
    })
}

#[tokio::test]
async fn single_actions_hit_quota_then_429() {
    let app = app_quota(2);
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
    let sid = serde_json::from_slice::<Value>(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let one = json!({
        "schema_version": "1.0.0",
        "kind": "a",
        "payload": {}
    })
    .to_string();
    for _ in 0..2 {
        let res = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!("/v1/sessions/{sid}/action"))
                    .header("content-type", "application/json")
                    .body(Body::from(one.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/v1/sessions/{sid}/action"))
                .header("content-type", "application/json")
                .body(Body::from(one))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
    let v: Value = serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap();
    assert_eq!(v["error"]["code"], "SESSION_ACTION_QUOTA");
}

#[tokio::test]
async fn batch_rejected_when_would_exceed_quota_without_partial_apply() {
    let app = app_quota(2);
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

    let body = json!({
        "actions": [
            {"schema_version": "1.0.0", "kind": "x", "payload": {}},
            {"schema_version": "1.0.0", "kind": "y", "payload": {}},
            {"schema_version": "1.0.0", "kind": "z", "payload": {}}
        ]
    })
    .to_string();
    let res = app
        .clone()
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
    assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);

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
    let obs: Value = serde_json::from_slice(&res.into_body().collect().await.unwrap().to_bytes())
        .unwrap();
    assert_eq!(obs["tick"], 0);
}
