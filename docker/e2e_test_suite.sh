#!/usr/bin/env bash
set -e

CLIENT_URL="http://ferrox-client-node:8080"
VPS_URL="http://ferrox-vps-gateway:9090"

echo "========================================================"
echo "🛡️ FERROX LOCAL DOCKER E2E RED-TEAM & ONBOARDING SUITE 🛡️"
echo "========================================================"
echo "Target Client Node: $CLIENT_URL"
echo "Founder VPS Gateway: $VPS_URL"
echo "--------------------------------------------------------"

# 1. Wait for services to be healthy
echo "[STEP 1/5] Verifying service connectivity..."
until curl -s "$CLIENT_URL/health" > /dev/null; do
  echo "Waiting for Client Node ($CLIENT_URL)..."
  sleep 2
done
until curl -s "$VPS_URL/api/v1/founder/overview" > /dev/null; do
  echo "Waiting for VPS Gateway ($VPS_URL)..."
  sleep 2
done
echo "✅ Both Client Node and VPS Gateway are live & reachable!"
echo ""

# 2. Test Public Ingress Header Hygiene & Tech Leak Audit (Kali nmap & curl)
echo "[STEP 2/5] Running Kali Public Ingress Header Hygiene & Tech Leak Audit..."
HEADERS=$(curl -sI "$CLIENT_URL/")
echo "Inspecting Headers from $CLIENT_URL:"
echo "$HEADERS"

if echo "$HEADERS" | grep -q -i "x-frame-options: DENY" && echo "$HEADERS" | grep -q -i "x-content-type-options: nosniff"; then
  echo "✅ OWASP Mandatory Headers verified! (X-Frame-Options: DENY, X-Content-Type-Options: nosniff present)"
else
  echo "❌ FAILED: Missing mandatory OWASP security headers on client ingress"
  exit 1
fi

if echo "$HEADERS" | grep -q -i "x-powered-by"; then
  echo "❌ FAILED: Leaked technology header X-Powered-By"
  exit 1
else
  echo "✅ Technology Leak Prevention Verified: X-Powered-By header stripped cleanly!"
fi
echo ""

# 3. Test Honeypot Decoy Trap Reaction
echo "[STEP 3/5] Testing Honeypot Decoy Trap (/admin/config.json)..."
HONEYPOT_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$CLIENT_URL/admin/config.json")
echo "Honeypot Response Code: $HONEYPOT_STATUS"
if [ "$HONEYPOT_STATUS" -eq 403 ]; then
  echo "✅ Honeypot Decoy Trap Active! HTTP 403 Forbidden returned as expected."
else
  echo "❌ FAILED: Honeypot decoy returned code $HONEYPOT_STATUS instead of 403 Forbidden"
  exit 1
fi
echo ""

# 4. Test Autonomous Client Self-Onboarding API to Founder VPS
echo "[STEP 4/5] Executing Autonomous Client Self-Onboarding Request to VPS Gateway..."
ONBOARD_PAYLOAD='{
  "client_name": "Docker E2E Test Enterprise",
  "domain": "e2e-test-node.internal",
  "ip_address": "172.28.0.10",
  "ciso_email": "ciso@e2e-test-node.internal",
  "technical_contact": "devops@e2e-test-node.internal",
  "product_type": "EnterpriseBoilerplate",
  "region": "EuCentral",
  "guard_manifest": {
    "sentinel_version": "v0.1.2",
    "squeezer_hash": "hash_sq_e2e_01",
    "merkle_logger_hash": "hash_log_e2e_01",
    "active_rules_mask": 15,
    "self_test_passed": true
  },
  "zk_attestation_proof": "a8f3b20c99d10e"
}'

RESPONSE=$(curl -s -X POST -H "Content-Type: application/json" -d "$ONBOARD_PAYLOAD" "$VPS_URL/api/v1/founder/client/onboard")
echo "VPS Gateway Onboarding Response:"
echo "$RESPONSE"

if echo "$RESPONSE" | grep -q '"approved":true'; then
  echo "✅ Autonomous Client Self-Onboarding APPROVED by VPS Gateway!"
else
  echo "❌ FAILED: Client self-onboarding was rejected by VPS Gateway"
  exit 1
fi
echo ""

# 5. Verify Telemetry Sync & Fleet Registry Overview
echo "[STEP 5/5] Auditing Founder Fleet Registry Overview..."
OVERVIEW=$(curl -s "$VPS_URL/api/v1/founder/overview")
echo "Fleet Overview:"
echo "$OVERVIEW"

if echo "$OVERVIEW" | grep -q '"total_nodes"'; then
  echo "✅ Fleet Registry Overview verified successfully!"
else
  echo "❌ FAILED: Fleet Overview invalid"
  exit 1
fi

echo ""
echo "========================================================"
echo "🎉 ALL 5 E2E DOCKER RED-TEAM & ONBOARDING STAGES PASSED! 🎉"
echo "========================================================"
exit 0
