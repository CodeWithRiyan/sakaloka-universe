#!/usr/bin/env bash
# =============================================================
# 🌌 Sakaloka-Universe — Port Conflict Pre-Check Script
# US-05: Ensures no FOREIGN service conflicts with Sakaloka ports
# Compatible with bash 3.2+ (macOS default)
# =============================================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo ""
echo "🌌 Sakaloka-Universe — Port Conflict Check"
echo "============================================"

CONFLICTS=0
SAKALOKA_RUNNING=0

check_port() {
    local port="$1"
    local service="$2"
    local container_name="$3"

    if lsof -iTCP:"$port" -sTCP:LISTEN -n -P 2>/dev/null | grep -q LISTEN; then
        # Check if the port is held by our own Sakaloka Docker container
        if docker ps --filter "name=$container_name" --format "{{.Names}}" 2>/dev/null | grep -q "$container_name"; then
            echo -e "${CYAN}🐳 SAKALOKA${NC} — Port $port in use by $service (our container ✅)"
            SAKALOKA_RUNNING=$((SAKALOKA_RUNNING + 1))
        else
            echo -e "${RED}❌ CONFLICT${NC} — Port $port is used by a FOREIGN service ($service)"
            CONFLICTS=$((CONFLICTS + 1))
        fi
    else
        echo -e "${GREEN}✅ FREE${NC}    — Port $port ($service)"
    fi
}

check_port "53000" "🌍 Earth (API)"          "sakaloka-universe-sakaloka-api"
check_port "54000" "🔴 Mars (Ockam)"         "sakaloka-universe-ockam"
check_port "57447" "🪐 Saturn (Zenoh)"       "sakaloka-universe-zenoh"
check_port "56333" "🔵 Uranus (Qdrant)"      "sakaloka-universe-qdrant"
check_port "58000" "🪐 Jupiter (SurrealDB)"  "sakaloka-universe-surrealdb"

echo ""
echo -e "  Planets running: ${CYAN}$SAKALOKA_RUNNING${NC} | Foreign conflicts: ${RED}$CONFLICTS${NC}"
echo ""

if [ "$CONFLICTS" -gt 0 ]; then
    echo -e "${RED}🚫 ABORT: $CONFLICTS foreign port conflict(s) detected.${NC}"
    echo -e "${YELLOW}   Stop the conflicting services before running 'docker compose up'.${NC}"
    echo ""
    exit 1
else
    echo -e "${GREEN}✅ No foreign conflicts. Safe to run 'docker compose up'.${NC}"
    echo ""
    exit 0
fi
