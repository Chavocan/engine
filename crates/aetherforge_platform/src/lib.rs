//! Headed window / wgpu loop + optional **`headed-smoke`** / **`windowed`** builds.

pub fn platform_placeholder() -> &'static str {
    "aetherforge_platform: `aetherforge_window` — farm-stub sim, keyboard→Intent (P/D/H/Space), title HUD (see docs/platform-headed-roadmap.md)"
}

/// Create a default wgpu instance (no surface). Used by **`aetherforge_wgpu_smoke`** and CI compile checks.
#[cfg(feature = "headed-smoke")]
pub fn wgpu_headless_smoke() {
    let _instance = wgpu::Instance::default();
}
