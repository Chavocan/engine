//! JSON Schema ↔ Rust types (stubs — expand with `schemars` in a later slice).

pub mod v1 {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct WorldSnapshot {
        pub world_version: String,
        #[serde(default)]
        pub entities: Vec<serde_json::Value>,
    }

    impl Default for WorldSnapshot {
        fn default() -> Self {
            Self {
                world_version: "1.0.0".to_string(),
                entities: Vec::new(),
            }
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Action {
        pub schema_version: String,
        pub kind: String,
        #[serde(default)]
        pub payload: serde_json::Value,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Observation {
        pub schema_version: String,
        pub tick: u64,
        pub run_id: String,
        pub message: String,
        pub rng_draw: u32,
        #[serde(default)]
        pub world: WorldSnapshot,
    }

    impl Default for Observation {
        fn default() -> Self {
            Self {
                schema_version: "1.1.0".to_string(),
                tick: 0,
                run_id: String::new(),
                message: String::new(),
                rng_draw: 0,
                world: WorldSnapshot::default(),
            }
        }
    }
}
