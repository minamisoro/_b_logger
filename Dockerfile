# Multi-stage Dockerfile for _B_logger

# ============================================================================
# Stage 1: Build Rust Backend
# ============================================================================
FROM rust:1.83-slim AS backend-builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend

# Build the backend in release mode
RUN cargo build --release --workspace

# ============================================================================
# Stage 2: Build Frontend (Web + Admin)
# ============================================================================
FROM node:22-slim AS frontend-builder

WORKDIR /app

# Copy package files
COPY package*.json ./
COPY frontend/web/package*.json ./frontend/web/
COPY frontend/admin/package*.json ./frontend/admin/
COPY frontend/lib ./frontend/lib

# Install dependencies
RUN npm ci --workspace=blogger-web --workspace=blogger-admin

# Copy frontend source
COPY frontend ./frontend

# Build both frontend projects
RUN npm run build --workspace=blogger-web
RUN npm run build --workspace=blogger-admin

# ============================================================================
# Stage 3: Backend Runtime
# ============================================================================
FROM debian:bookworm-slim AS backend-runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy backend binary and config
COPY --from=backend-builder /app/target/release/blogger-api /app/blogger-api
COPY config.toml /app/config.toml
COPY backend/api/migrations /app/migrations

# Create a non-root user
RUN useradd -m -u 1000 blogger && chown -R blogger:blogger /app
USER blogger

EXPOSE 8080

CMD ["/app/blogger-api"]

# ============================================================================
# Stage 4: Web Frontend Runtime (Nginx)
# ============================================================================
FROM nginx:alpine AS web-runtime

# Copy built web assets
COPY --from=frontend-builder /app/frontend/web/dist /usr/share/nginx/html

# Copy nginx config
COPY <<EOF /etc/nginx/conf.d/default.conf
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Enable gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    location / {
        try_files \$uri \$uri/ /index.html;
    }

    # API proxy
    location /api/ {
        proxy_pass http://api:8080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

EXPOSE 80

# ============================================================================
# Stage 5: Admin Frontend Runtime (Nginx)
# ============================================================================
FROM nginx:alpine AS admin-runtime

# Copy built admin assets
COPY --from=frontend-builder /app/frontend/admin/dist /usr/share/nginx/html

# Copy nginx config
COPY <<EOF /etc/nginx/conf.d/default.conf
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Enable gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    location / {
        try_files \$uri \$uri/ /index.html;
    }

    # API proxy
    location /api/ {
        proxy_pass http://api:8080/;
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;
        proxy_set_header X-Real-IP \$remote_addr;
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto \$scheme;
    }
}
EOF

EXPOSE 80
