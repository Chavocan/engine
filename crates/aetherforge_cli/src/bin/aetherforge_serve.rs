//! Control plane HTTP server (Phase 1d; no WebSocket).
//!
//! **Bind:** **`AETHERFORGE_HTTP_ADDR`** (default **`127.0.0.1:8787`**). Use **`127.0.0.1`** for local-only; put TLS in front for production (see **`docs/adr/0003-deployment-tls-and-auth.md`**).
//!
//! Optional per-IP rate limit: build with **`--features rate-limit`** and set **`AETHERFORGE_HTTP_RATE_LIMIT_RPS`**
//! (see **`docs/deployment-rate-limiting.md`** in the repo root).

use std::net::SocketAddr;

use aetherforge_control::{app_router, play_log};
#[cfg(feature = "rate-limit")]
use axum::middleware::from_fn_with_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    play_log::try_init_from_env();
    #[cfg(not(feature = "rate-limit"))]
    let app = app_router();
    #[cfg(feature = "rate-limit")]
    let app = match aetherforge_control::rate_limit::http_rate_limit_rps_from_env() {
        Some(rps) => {
            let lim = aetherforge_control::rate_limit::keyed_limiter(rps);
            eprintln!(
                "aetherforge_serve: per-IP HTTP rate limit enabled ({rps} req/s); see AETHERFORGE_HTTP_RATE_LIMIT_RPS"
            );
            app_router().layer(from_fn_with_state(
                lim,
                aetherforge_control::rate_limit::rate_limit_middleware,
            ))
        }
        None => app_router(),
    };

    let addr: SocketAddr = std::env::var("AETHERFORGE_HTTP_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8787".into())
        .parse()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("AETHERFORGE_HTTP_ADDR: {e}"),
            )
        })?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    eprintln!("aetherforge_serve listening on http://{addr}");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}
