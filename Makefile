SHELL := /bin/bash
.DEFAULT_GOAL := help

VENUS_DIR := apps/venus
DEV_JWT_SECRET := test-secret-at-least-32-chars-long!

.PHONY: help bootstrap env venus-env ports infra-up infra-down infra-ps stack-up \
	api-dev openapi venus-install venus-sync-types venus-dev venus-build venus-lint \
	venus-tauri-dev fmt fmt-check clippy test doc-check check-backend check-venus \
	check-generated ci

help: ## Show available development commands
	@grep -E '^[a-zA-Z0-9_-]+:.*?## ' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS=":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

env: ## Create root .env from template if missing
	@test -f .env || cp .env.example .env

venus-env: ## Create Venus .env from template if missing
	@test -f $(VENUS_DIR)/.env || cp $(VENUS_DIR)/.env.example $(VENUS_DIR)/.env

bootstrap: env venus-env ## Prepare local environment files
	@echo "Local environment files are ready."

ports: ## Check host port conflicts before starting Docker services
	@bash scripts/check-ports.sh

infra-up: ## Start local infrastructure for app development
	docker compose up -d

infra-down: ## Stop local infrastructure containers
	docker compose down

infra-ps: ## Show local infrastructure container status
	docker compose ps

stack-up: ## Start the full local Docker stack, including the API container
	docker compose up -d

api-dev: ## Run the Axum API locally
	cargo run -p sakaloka-api

openapi: ## Regenerate the OpenAPI spec consumed by Venus
	cargo run -p sakaloka-api --bin openapi > $(VENUS_DIR)/openapi.json

venus-install: ## Install Venus frontend dependencies
	pnpm --dir $(VENUS_DIR) install --frozen-lockfile

venus-sync-types: ## Regenerate Venus OpenAPI-generated types
	pnpm --dir $(VENUS_DIR) run generate:types

venus-dev: ## Run the Venus web app in development mode
	pnpm --dir $(VENUS_DIR) dev

venus-build: ## Build the Venus frontend bundle
	pnpm --dir $(VENUS_DIR) build

venus-lint: ## Lint the Venus frontend
	pnpm --dir $(VENUS_DIR) lint

venus-tauri-dev: ## Run the Venus Tauri desktop app locally
	pnpm --dir $(VENUS_DIR) tauri dev

fmt: ## Format all Rust code
	cargo fmt --all

fmt-check: ## Verify Rust formatting
	cargo fmt --all -- --check

clippy: ## Run Clippy with warnings denied across the workspace
	SAKALOKA_JWT_SECRET=$(DEV_JWT_SECRET) cargo clippy --workspace --all-targets -- -D warnings

test: ## Run the Rust test suite with a local dev JWT secret
	SAKALOKA_JWT_SECRET=$(DEV_JWT_SECRET) cargo test --workspace

doc-check: ## Verify Rust public docs compile with missing_docs denied
	RUSTDOCFLAGS="-D missing_docs" cargo doc --workspace --no-deps

check-backend: fmt-check clippy test doc-check ## Run backend quality gates

check-generated: ## Ensure generated OpenAPI artifacts are committed
	git diff --exit-code -- $(VENUS_DIR)/openapi.json $(VENUS_DIR)/src/types/generated.ts

check-venus: venus-lint venus-build ## Run Venus quality gates

ci: check-backend venus-sync-types check-generated check-venus ## Run the local CI-equivalent workflow
