//! Integration: `run_player` against in-process Axum control plane.

use aetherforge_cli::player::{run_player, PlayerConfig, PlayerPolicy};
use axum::Router;
use tokio::net::TcpListener;

#[tokio::test]
async fn player_round_robin_stops_after_max_steps() {
    let app: Router = aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
    });
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    run_player(PlayerConfig {
        base_url: format!("http://{}", addr),
        seed: 99,
        policy: PlayerPolicy::RoundRobin,
        max_steps: 3,
        intents: vec!["a".into(), "b".into()],
        verbose: false,
        llm_cmd: None,
    })
    .await
    .expect("player completes");
}
