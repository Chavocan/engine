//! Optional per-client-IP HTTP rate limiting (token bucket via **`governor`**).
//!
//! Enable crate feature **`rate-limit`** and install the middleware from **`aetherforge_serve`**
//! when **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`** is set. Use **`into_make_service_with_connect_info::<SocketAddr>`**
//! so peer IP is available.

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{ConnectInfo, State};
use axum::http::Request;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use governor::{DefaultKeyedRateLimiter, Quota};

/// Shared keyed limiter: one quota bucket per source IP.
pub type IpRateLimiter = DefaultKeyedRateLimiter<IpAddr>;

/// Build a limiter: **`rps`** sustained requests per second per IP (GCRA burst follows governor defaults).
pub fn keyed_limiter(rps: std::num::NonZeroU32) -> Arc<IpRateLimiter> {
    Arc::new(IpRateLimiter::keyed(Quota::per_second(rps)))
}

/// Convenience: **`rps`** requests per second from a plain u32 (must be > 0).
pub fn keyed_limiter_u32(rps: u32) -> Option<Arc<IpRateLimiter>> {
    std::num::NonZeroU32::new(rps).map(keyed_limiter)
}

pub async fn rate_limit_middleware(
    State(limiter): State<Arc<IpRateLimiter>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    match limiter.check_key(&addr.ip()) {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Err(
            (
                axum::http::StatusCode::TOO_MANY_REQUESTS,
                Json(serde_json::json!({
                    "error": {
                        "code": "HTTP_RATE_LIMIT",
                        "message": "per-IP request rate exceeded; slow down or raise AETHERFORGE_HTTP_RATE_LIMIT_RPS",
                        "request_id": "n/a"
                    },
                    "schema_version": "1.0.0"
                })),
            )
                .into_response(),
        ),
    }
}

/// Parse **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`** (positive integer). **`None`** means disable in-process limit.
pub fn http_rate_limit_rps_from_env() -> Option<std::num::NonZeroU32> {
    let v = std::env::var("AETHERFORGE_HTTP_RATE_LIMIT_RPS").ok()?;
    let n: u32 = v.parse().ok()?;
    std::num::NonZeroU32::new(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quota_nonzero() {
        let lim = keyed_limiter(std::num::NonZeroU32::new(5).unwrap());
        assert!(lim.check_key(&IpAddr::from([127, 0, 0, 1])).is_ok());
    }
}
