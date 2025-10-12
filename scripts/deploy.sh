#!/bin/bash

# GitHub Webhook Deployment Script
# This script is triggered by the webhook endpoint when a new release is published
# It pulls the latest changes and rebuilds the Docker containers

set -e  # Exit on error

TAG="${1:-latest}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
LOGFILE="${PROJECT_ROOT}/deploy.log"

# Logging function
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOGFILE"
}

# Error handler
error_exit() {
    log "ERROR: $1"
    exit 1
}

log "=========================================="
log "Starting deployment for tag: $TAG"
log "=========================================="

# Change to project directory
cd "$PROJECT_ROOT" || error_exit "Failed to change to project directory"

# Verify we're in a git repository
if [ ! -d .git ]; then
    error_exit "Not a git repository"
fi

log "Pulling latest changes from git..."
git fetch --all --tags || error_exit "Failed to fetch from git"

# Checkout the specific tag
log "Checking out tag: $TAG"
git checkout "tags/$TAG" || error_exit "Failed to checkout tag $TAG"

# Pull latest changes (if on a branch)
git pull origin $(git rev-parse --abbrev-ref HEAD) 2>/dev/null || true

# Show current commit
CURRENT_COMMIT=$(git rev-parse HEAD)
log "Current commit: $CURRENT_COMMIT"

# Stop running containers
log "Stopping running containers..."
docker compose down || log "Warning: Failed to stop containers (they may not be running)"

# Remove old images to force rebuild
log "Removing old images..."
docker compose rm -f || true
docker rmi blogger-api blogger-web blogger-admin 2>/dev/null || log "Note: Some images were not found"

# Build new images
log "Building Docker images..."
docker compose build --no-cache || error_exit "Failed to build Docker images"

# Start containers
log "Starting containers..."
docker compose up -d || error_exit "Failed to start containers"

# Wait for services to be healthy
log "Waiting for services to be ready..."
sleep 10

# Check container status
log "Container status:"
docker compose ps | tee -a "$LOGFILE"

# Health check
log "Performing health check..."
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -sf http://localhost:8080/health > /dev/null 2>&1; then
        log "API health check passed"
        break
    fi
    RETRY_COUNT=$((RETRY_COUNT + 1))
    log "Waiting for API to be ready... ($RETRY_COUNT/$MAX_RETRIES)"
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    error_exit "API health check failed after $MAX_RETRIES attempts"
fi

# Verify web frontend
if curl -sf http://localhost:8081 > /dev/null 2>&1; then
    log "Web frontend is responding"
else
    log "Warning: Web frontend health check failed"
fi

# Verify admin frontend
if curl -sf http://localhost:8082 > /dev/null 2>&1; then
    log "Admin frontend is responding"
else
    log "Warning: Admin frontend health check failed"
fi

# Clean up old Docker resources
log "Cleaning up unused Docker resources..."
docker system prune -f || log "Warning: Failed to prune Docker resources"

log "=========================================="
log "Deployment completed successfully!"
log "Tag: $TAG"
log "Commit: $CURRENT_COMMIT"
log "=========================================="

exit 0
