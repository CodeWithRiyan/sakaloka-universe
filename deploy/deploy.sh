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

case "$ENV" in
  local)
    ENV_FILE=".env"
    ;;
  dev|sit|demo|prod)
    # Use .env in the current directory (where deploy.sh runs)
    ENV_FILE=".env"
    ;;
  *)
    echo "Usage: $0 <local|dev|sit|demo|prod> [up|down|logs|ps]"
    exit 1
    ;;
esac

COMPOSE_FILE="-f docker-compose.${ENV}.yml"
DOCKER_COMPOSE="docker compose --env-file $ENV_FILE $COMPOSE_FILE"

case "$ACTION" in
  up)
    echo "==> Deploying sakaloka [$ENV]"
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
    echo "Usage: $0 <local|dev|sit|demo|prod> [up|down|logs|ps|pull]"
    exit 1
    ;;
esac