#!/usr/bin/env bash
set -euo pipefail

# =============================================================
# Sakaloka-Universe — Deploy helper
#
# Usage:
#   ./deploy.sh <env>          # Pull and deploy (dev|sit|demo|prod)
#   ./deploy.sh local          # Build from source and deploy dev
#   ./deploy.sh <env> down     # Stop environment
#   ./deploy.sh <env> logs     # Tail logs
#   ./deploy.sh <env> ps       # Show running containers
# =============================================================

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

ENV="${1:-dev}"
ACTION="${2:-up}"

BASE="-f docker-compose.base.yml"

case "$ENV" in
  local)
    COMPOSE="$BASE -f docker-compose.dev.yml -f docker-compose.local.yml"
    ENV_FILE=".env"
    ;;
  dev|sit|demo|prod)
    COMPOSE="$BASE -f docker-compose.${ENV}.yml"
    ENV_FILE=".env"
    # On the server, use /opt/sakaloka/<env>/.env if it exists
    if [ -f "/opt/sakaloka/${ENV}/.env" ]; then
      ENV_FILE="/opt/sakaloka/${ENV}/.env"
    fi
    ;;
  *)
    echo "Usage: $0 <local|dev|sit|demo|prod> [up|down|logs|ps]"
    exit 1
    ;;
esac

DOCKER_COMPOSE="docker compose --env-file $ENV_FILE $COMPOSE"

case "$ACTION" in
  up)
    echo "==> Deploying sakaloka [$ENV]"
    $DOCKER_COMPOSE up -d
    echo "==> Waiting for health check..."
    sleep 5
    $DOCKER_COMPOSE ps
    ;;
  down)
    echo "==> Stopping sakaloka [$ENV]"
    $DOCKER_COMPOSE down
    ;;
  logs)
    $DOCKER_COMPOSE logs -f --tail=100
    ;;
  ps)
    $DOCKER_COMPOSE ps
    ;;
  pull)
    echo "==> Pulling latest images [$ENV]"
    $DOCKER_COMPOSE pull
    ;;
  *)
    echo "Usage: $0 <local|dev|sit|demo|prod> [up|down|logs|ps|pull]"
    exit 1
    ;;
esac
