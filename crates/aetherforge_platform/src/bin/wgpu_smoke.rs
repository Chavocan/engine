//! Headless wgpu instance smoke — proves the GPU stack links; no window.

fn main() {
    aetherforge_platform::wgpu_headless_smoke();
    println!("aetherforge_wgpu_smoke: ok");
}
