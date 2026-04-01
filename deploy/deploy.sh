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
    ENV_FILE=".env"
    # On the server, use /srv/apps/sakaloka/<env>/.env if it exists
    if [ -f "/srv/apps/sakaloka/${ENV}/.env" ]; then
      ENV_FILE="/srv/apps/sakaloka/${ENV}/.env"
    fi
    ;;
  *)
    echo "Usage: $0 <local|dev|sit|demo|prod> [up|down|logs|ps]"
    exit 1
    ;;
esac

DOCKER_COMPOSE="docker compose --env-file $ENV_FILE -f docker-compose.${ENV}.yml"

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

DOCKER_COMPOSE="docker compose --env-file $ENV_FILE $COMPOSE"

case "$ACTION" in
  up)
    echo "==> Deploying sakaloka [$ENV]"
    # Ensure external infra containers are on the compose network
    NETWORK="net-${ENV}"
    PG_CONTAINER="infra-${ENV}-postgres"
    if docker ps -a --format '{{.Names}}' | grep -q "^${PG_CONTAINER}$"; then
      echo "==> Ensuring ${PG_CONTAINER} is on ${NETWORK}..."
      docker network connect "$NETWORK" "$PG_CONTAINER" 2>/dev/null || true
    fi
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
