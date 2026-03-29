//! Phase 2c — play-log JSON line shape (tracing buffer).

use std::io::Write;
use std::sync::{Arc, Mutex};

use tracing_subscriber::filter::FilterFn;
use tracing_subscriber::fmt::format::Format;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

#[derive(Clone)]
struct Capture(Arc<Mutex<Vec<u8>>>);

impl Write for Capture {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn play_emit_produces_parseable_json_with_contract_keys() {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let cap = Capture(buf.clone());
    let event_fmt = Format::default().json().flatten_event(true);
    let layer = fmt::layer()
        .event_format(event_fmt)
        .with_writer(move || cap.clone())
        .with_filter(FilterFn::new(|m| m.target() == "aetherforge.play"));

    let _g = tracing_subscriber::registry().with(layer).set_default();
    tracing::callsite::rebuild_interest_cache();

    aetherforge_control::play_log::emit(
        "test_contract",
        "run-z",
        7,
        Some("sess-z"),
        serde_json::json!({ "x": 1 }),
    );

    let bytes = buf.lock().unwrap().clone();
    let line = std::str::from_utf8(&bytes).unwrap().trim();
    let v: serde_json::Value = serde_json::from_str(line).expect("valid json line");
    for k in ["ts", "run_id", "tick", "event", "session_id", "payload"] {
        assert!(v.get(k).is_some(), "missing {k}");
    }
    assert_eq!(v["event"], "test_contract");
    assert_eq!(v["run_id"], "run-z");
    assert_eq!(v["tick"], 7);
}
