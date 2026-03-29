//! Binds `127.0.0.1:8787` — Phase 1d control plane (no WebSocket).

use aetherforge_control::{app_router, play_log};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    play_log::try_init_from_env();
    let app = app_router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8787").await?;
    eprintln!("aetherforge_serve listening on http://127.0.0.1:8787");
    axum::serve(listener, app).await?;
    Ok(())
}
