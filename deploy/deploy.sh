#!/usr/bin/env bash
# =============================================================
# Sakaloka-Universe — Deploy helper
#
# Usage:
#   ./deploy.sh <env>          # Pull and deploy (dev|sit|demo|prod)
#   ./deploy.sh <env> down     # Stop environment
#   ./deploy.sh <env> logs     # Tail logs
#   ./deploy.sh <env> ps       # Show running containers
#   ./deploy.sh <env> pull     # Pull latest images
# =============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

ENV="${1:-dev}"
ACTION="${2:-up}"

case "$ENV" in
  dev|sit|demo|prod) ;;
  *)
    echo "Usage: $0 <dev|sit|demo|prod> [up|down|logs|ps|pull]"
    exit 1
    ;;
esac

ENV_FILE=".env.${ENV}"

if [ ! -f "$ENV_FILE" ]; then
  echo "Error: $ENV_FILE not found. Copy .env.template to $ENV_FILE and fill in values."
  exit 1
fi

DOCKER_COMPOSE="docker compose --env-file $ENV_FILE -p sakaloka-${ENV}"

case "$ACTION" in
  up)
    echo "==> Deploying sakaloka [$ENV]"
    $DOCKER_COMPOSE pull
    $DOCKER_COMPOSE up -d --wait --wait-timeout 120
    echo "==> Deployment complete!"
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
    echo "Usage: $0 <dev|sit|demo|prod> [up|down|logs|ps|pull]"
    exit 1
    ;;
esac
