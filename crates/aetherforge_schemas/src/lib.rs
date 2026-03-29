//! JSON Schema ↔ Rust types; **`schema-export`** feature generates JSON Schema for drift checks in CI.

pub mod v1 {
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "schema-export")]
    use schemars::{schema::Schema, JsonSchema};

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
    #[serde(deny_unknown_fields)]
    #[cfg_attr(feature = "schema-export", derive(JsonSchema))]
    pub struct Action {
        pub schema_version: String,
        pub kind: String,
        #[serde(default)]
        #[cfg_attr(
            feature = "schema-export",
            schemars(schema_with = "payload_object_schema")
        )]
        pub payload: serde_json::Value,
    }

    #[cfg(feature = "schema-export")]
    fn payload_object_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        serde_json::from_value(serde_json::json!({
            "type": "object",
            "additionalProperties": true
        }))
        .expect("inline schema")
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
                schema_version: "1.2.0".to_string(),
                tick: 0,
                run_id: String::new(),
                message: String::new(),
                rng_draw: 0,
                world: WorldSnapshot::default(),
            }
        }
    }
}
