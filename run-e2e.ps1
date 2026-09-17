# Ferrox 1-Click Reproducible Docker E2E Test Suite Execution Script (PowerShell)
$ErrorActionPreference = "Stop"

Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "🚀 FERROX REPRODUCIBLE DOCKER E2E TEST RUNNER (LOCAL) 🚀" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan

# Check Docker is available
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Error "❌ Docker is not installed or not present in PATH."
    exit 1
}

Write-Host "📦 Building and orchestrating Ferrox E2E Docker Mesh..." -ForegroundColor Yellow
docker compose -f docker-compose.e2e.yml build

Write-Host "⚡ Running E2E Red-Team Audit & Autonomous Onboarding Test Suite..." -ForegroundColor Yellow
docker compose -f docker-compose.e2e.yml up --exit-code-from e2e-runner --remove-orphans

$exitCode = $LASTEXITCODE

Write-Host "🧹 Cleaning up isolated Docker mesh containers..." -ForegroundColor Gray
docker compose -f docker-compose.e2e.yml down --volumes --remove-orphans

if ($exitCode -eq 0) {
    Write-Host "========================================================" -ForegroundColor Green
    Write-Host "🎉 ALL DOCKER E2E TEST STAGES PASSED CLEANLY! (EXIT 0) 🎉" -ForegroundColor Green
    Write-Host "========================================================" -ForegroundColor Green
} else {
    Write-Host "========================================================" -ForegroundColor Red
    Write-Host "❌ E2E DOCKER TEST SUITE FAILED (EXIT $exitCode)" -ForegroundColor Red
    Write-Host "========================================================" -ForegroundColor Red
    exit $exitCode
}
