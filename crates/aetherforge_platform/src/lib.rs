//! Headed window / wgpu loop + optional **`headed-smoke`** / **`windowed`** builds.

pub fn platform_placeholder() -> &'static str {
    "aetherforge_platform: for a headed viewport run `cargo run -p aetherforge_platform --features windowed --bin aetherforge_window` (see docs/platform-headed-roadmap.md)"
}

/// Create a default wgpu instance (no surface). Used by **`aetherforge_wgpu_smoke`** and CI compile checks.
#[cfg(feature = "headed-smoke")]
pub fn wgpu_headless_smoke() {
    let _instance = wgpu::Instance::default();
}
