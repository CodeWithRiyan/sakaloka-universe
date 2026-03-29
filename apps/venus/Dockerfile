# =============================================================
# Sakaloka Venus (POS Web) — Multi-stage Docker Build
#
# Build: docker build --build-arg ENV=dev -t sakaloka-pos:dev .
# Run:   docker run -p 3000:80 sakaloka-pos:dev
# =============================================================

# --- Stage 1: Build ---
FROM node:22-alpine AS build

RUN corepack enable pnpm

WORKDIR /app

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY . .

ARG ENV=dev
RUN cp .env.${ENV} .env 2>/dev/null || true
RUN pnpm run build

# --- Stage 2: Serve with nginx ---
FROM nginx:alpine

# SPA fallback: all routes → index.html
COPY --from=build /app/dist /usr/share/nginx/html

RUN cat > /etc/nginx/conf.d/default.conf <<'EOF'
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml text/javascript image/svg+xml;

    # Cache static assets (hashed filenames from Vite)
    location /assets/ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA fallback — all routes to index.html
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Health check
    location = /healthz {
        access_log off;
        return 200 "ok";
        add_header Content-Type text/plain;
    }
}
EOF

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
