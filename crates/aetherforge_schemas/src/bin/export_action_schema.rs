//! Emit JSON Schema for [`aetherforge_schemas::v1::Action`] (stdout).

use schemars::schema_for;

fn main() {
    let root = schema_for!(aetherforge_schemas::v1::Action);
    println!(
        "{}",
        serde_json::to_string_pretty(&root).expect("serialize schema")
    );
}
