//! NL-2 stub route (`nl-interpret-stub`) — stable **501** + JSON until a real interpreter ships.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

fn app() -> Router {
    aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
        ..Default::default()
    })
}

#[tokio::test]
async fn interpret_stub_returns_501_with_error_body() {
    let app = app();
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/sessions/any-id/interpret")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_IMPLEMENTED);
    let body: Value = serde_json::from_slice(
        &res.into_body().collect().await.unwrap().to_bytes(),
    )
    .unwrap();
    assert_eq!(
        body["error"]["code"],
        serde_json::json!("NL_INTERPRET_NOT_IMPLEMENTED")
    );
}
