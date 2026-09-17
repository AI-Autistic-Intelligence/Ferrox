#!/usr/bin/env bash
set -e

echo "========================================================"
echo "🚀 FERROX REPRODUCIBLE DOCKER E2E TEST RUNNER (LOCAL) 🚀"
echo "========================================================"

if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not present in PATH."
    exit 1
fi

echo "📦 Building and orchestrating Ferrox E2E Docker Mesh..."
docker compose -f docker-compose.e2e.yml build

echo "⚡ Running E2E Red-Team Audit & Autonomous Onboarding Test Suite..."
set +e
docker compose -f docker-compose.e2e.yml up --exit-code-from e2e-runner --remove-orphans
EXIT_CODE=$?
set -e

echo "🧹 Cleaning up isolated Docker mesh containers..."
docker compose -f docker-compose.e2e.yml down --volumes --remove-orphans

if [ $EXIT_CODE -eq 0 ]; then
    echo "========================================================"
    echo "🎉 ALL DOCKER E2E TEST STAGES PASSED CLEANLY! (EXIT 0) 🎉"
    echo "========================================================"
else
    echo "========================================================"
    echo "❌ E2E DOCKER TEST SUITE FAILED (EXIT $EXIT_CODE)"
    echo "========================================================"
    exit $EXIT_CODE
fi
