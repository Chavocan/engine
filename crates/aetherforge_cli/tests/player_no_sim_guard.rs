//! Phase 7b — player modules must not contain direct `use aetherforge_sim` lines.

use std::fs;

#[test]
fn player_sources_have_no_direct_sim_import_lines() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for rel in ["src/player.rs", "src/bin/aetherforge_player.rs"] {
        let p = root.join(rel);
        let s = fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {p:?}: {e}"));
        for (i, line) in s.lines().enumerate() {
            let t = line.trim_start();
            if t.starts_with("//") || t.starts_with("//!") {
                continue;
            }
            let bad = t.starts_with("use aetherforge_sim")
                || t.starts_with("use ::aetherforge_sim")
                || t.starts_with("extern crate aetherforge_sim");
            assert!(
                !bad,
                "{}:{}: player path must not import sim kernel directly: {}",
                p.display(),
                i + 1,
                line
            );
        }
    }
}
