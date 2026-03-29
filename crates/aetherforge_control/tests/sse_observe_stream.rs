#![cfg(feature = "sse-obs")]

use std::time::Duration;

use axum::Router;
use bytes::Bytes;
use futures_util::StreamExt;
use serde_json::{json, Value};
use tokio::net::TcpListener;

fn app() -> Router {
    aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
        ..Default::default()
    })
}

#[derive(Default)]
struct SseBuf {
    pending: String,
}

impl SseBuf {
    fn push_chunk(&mut self, chunk: &[u8]) {
        self.pending.push_str(&String::from_utf8_lossy(chunk));
    }

    /// Returns one `data:` payload (trimmed) after a blank line, or `None`.
    fn pop_data_line(&mut self) -> Option<String> {
        let pos = self.pending.find("\n\n")?;
        let frame = self.pending[..pos].to_string();
        self.pending.drain(..pos + 2);
        for line in frame.lines() {
            let t = line.trim_end();
            if let Some(rest) = t.strip_prefix("data:") {
                return Some(rest.trim().to_string());
            }
        }
        None
    }
}

async fn next_json_event<S>(stream: &mut S) -> Value
where
    S: StreamExt<Item = reqwest::Result<Bytes>> + Unpin,
{
    let mut buf = SseBuf::default();
    tokio::time::timeout(Duration::from_secs(3), async {
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.expect("chunk");
            buf.push_chunk(&chunk);
            if let Some(data) = buf.pop_data_line() {
                return serde_json::from_str(&data).expect("observation json");
            }
        }
        panic!("stream ended without event");
    })
    .await
    .expect("timeout waiting for SSE event")
}

#[tokio::test]
async fn observe_stream_emits_when_tick_changes() {
    let app = app();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(40)).await;
    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{base}/v1/sessions"))
        .json(&json!({}))
        .send()
        .await
        .unwrap();
    let sid = res.json::<Value>().await.unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let stream_res = client
        .get(format!("{base}/v1/sessions/{sid}/observe/stream"))
        .header("accept", "text/event-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(stream_res.status(), reqwest::StatusCode::OK);

    let mut byte_stream = stream_res.bytes_stream();
    let v0 = next_json_event(&mut byte_stream).await;
    assert_eq!(v0["tick"], 0);

    let action = json!({
        "schema_version": "1.0.0",
        "kind": "step_once",
        "payload": {}
    });
    let r = client
        .post(format!("{base}/v1/sessions/{sid}/action"))
        .json(&action)
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), reqwest::StatusCode::OK);

    let v1 = next_json_event(&mut byte_stream).await;
    assert_eq!(v1["tick"], 1);
}

#[tokio::test]
async fn second_observe_stream_returns_429_when_cap_is_one() {
    let app = aetherforge_control::app_router_with_config(aetherforge_control::ControlConfig {
        max_actions_per_session: 10_000,
        sse_max_per_session: 1,
        sse_max_global: 8,
    });
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(40)).await;
    let base = format!("http://{}", addr);
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{base}/v1/sessions"))
        .json(&json!({}))
        .send()
        .await
        .unwrap();
    let sid = res.json::<Value>().await.unwrap()["session_id"]
        .as_str()
        .unwrap()
        .to_string();

    let r1 = client
        .get(format!("{base}/v1/sessions/{sid}/observe/stream"))
        .header("accept", "text/event-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(r1.status(), reqwest::StatusCode::OK);

    let r2 = client
        .get(format!("{base}/v1/sessions/{sid}/observe/stream"))
        .header("accept", "text/event-stream")
        .send()
        .await
        .unwrap();
    assert_eq!(r2.status(), reqwest::StatusCode::TOO_MANY_REQUESTS);
    let body = r2.json::<Value>().await.unwrap();
    assert_eq!(body["error"]["code"], "SSE_SESSION_CAP");
}
