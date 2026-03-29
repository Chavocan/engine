# Fast local smoke: headless + offline farm scenario. Run from repo root:
#   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/dev_smoke.ps1

$ErrorActionPreference = 'Stop'
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host '== aetherforge_headless (one observation line) =='
cargo run -q -p aetherforge_cli --bin aetherforge_headless
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host ''
Write-Host '== aetherforge_scenario farm_demo_loop (offline, farm-stub) =='
cargo run -q -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host ''
Write-Host 'OK: dev_smoke passed (kernel + offline scenario). For HTTP + SDK see docs/getting-started.md'
