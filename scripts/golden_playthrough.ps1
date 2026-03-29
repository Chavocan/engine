# Golden bundle: offline farm demo + in-process HTTP session tests + HTTP player loop.
# Fails on any error. Run from repo root: pwsh -File scripts/golden_playthrough.ps1
# Parity with scripts/golden_playthrough.sh (CI uses the bash script on Ubuntu).

$ErrorActionPreference = 'Stop'
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

cargo run -p aetherforge_cli --features farm-stub --bin aetherforge_scenario -- --offline examples/farm_demo_loop.json
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

cargo test -p aetherforge_control --test http_sessions -q
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

cargo test -p aetherforge_player --test player_http_loop -q
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
