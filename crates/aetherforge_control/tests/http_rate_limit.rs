#![cfg(feature = "rate-limit")]

//! Per-IP rate limit: burst then **429** `HTTP_RATE_LIMIT`.

use std::net::SocketAddr;

use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::{Request, StatusCode};
use axum::middleware::from_fn_with_state;
use axum::Router;
use http_body_util::BodyExt;
use serde_json::Value;
use tower::ServiceExt;

fn app_with_limit(rps: u32) -> Router {
    let lim = aetherforge_control::rate_limit::keyed_limiter_u32(rps).expect("rps > 0");
    aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
        ..Default::default()
    })
    .layer(from_fn_with_state(
        lim,
        aetherforge_control::rate_limit::rate_limit_middleware,
    ))
}

fn connect_req(
    method: &str,
    path: &str,
    body: Body,
) -> Request<Body> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 42_000));
    Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .extension(ConnectInfo(addr))
        .body(body)
        .unwrap()
}

#[tokio::test]
async fn rate_limit_returns_429_when_exceeded() {
    let app = app_with_limit(1);
    // First request passes
    let r1 = app
        .clone()
        .oneshot(connect_req(
            "POST",
            "/v1/sessions",
            Body::from("{}"),
        ))
        .await
        .unwrap();
    assert_eq!(r1.status(), StatusCode::CREATED);

    // Immediate second from same IP should be limited (1/s quota)
    let r2 = app
        .oneshot(connect_req(
            "POST",
            "/v1/sessions",
            Body::from("{}"),
        ))
        .await
        .unwrap();
    assert_eq!(r2.status(), StatusCode::TOO_MANY_REQUESTS);
    let bytes = r2.into_body().collect().await.unwrap().to_bytes();
    let v: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["error"]["code"], "HTTP_RATE_LIMIT");
}
