//! JSON play-log lines (`target: "aetherforge.play"`) — enable with `AETHERFORGE_PLAY_LOG=1`.

use serde_json::Value;

fn payload_string(payload: &Value) -> String {
    let s = payload.to_string();
    if s.len() <= 2048 {
        s
    } else {
        format!("{}…(truncated)", &s[..2020])
    }
}

/// Emit one play-log event (consumed by the JSON `tracing` layer when installed).
pub fn emit(
    event: &'static str,
    run_id: &str,
    tick: u64,
    session_id: Option<&str>,
    payload: Value,
) {
    let ps = payload_string(&payload);
    let sid = session_id.unwrap_or("");
    tracing::info!(
        target: "aetherforge.play",
        ts = %chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        run_id = run_id,
        tick = tick,
        event = event,
        session_id = sid,
        payload = %ps,
    );
}

/// Install tracing: human lines by default; when `AETHERFORGE_PLAY_LOG=1`, add JSON lines for `aetherforge.play`.
pub fn try_init_from_env() {
    use tracing_subscriber::filter::FilterFn;
    use tracing_subscriber::fmt::format::Format;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::EnvFilter;

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let play_on = std::env::var("AETHERFORGE_PLAY_LOG").ok().as_deref() == Some("1");

    if play_on {
        let play_fmt = Format::default().json().flatten_event(true);
        let _ = tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_filter(FilterFn::new(|m| m.target() != "aetherforge.play")),
            )
            .with(
                fmt::layer()
                    .event_format(play_fmt)
                    .with_filter(FilterFn::new(|m| m.target() == "aetherforge.play")),
            )
            .try_init();
    } else {
        let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
    }
    // Subscriber may already be set (e.g. tests) — ignore `Err`.
}
