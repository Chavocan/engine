//! Headed window / wgpu loop (Phase 1c stub) + optional **`headed-smoke`** compile check.

pub fn platform_placeholder() -> &'static str {
    "aetherforge_platform: headed loop not wired in 1c"
}

/// Create a default wgpu instance (no surface). Used by **`aetherforge_wgpu_smoke`** and CI compile checks.
#[cfg(feature = "headed-smoke")]
pub fn wgpu_headless_smoke() {
    let _instance = wgpu::Instance::default();
}
