# Re-Vibe Makefile
# Simplifies common development tasks

.PHONY: help install dev test test-watch clean db-up db-down db-reset migrate seed ci-local docker-build docker-up docker-down docker-clean

# Default target - show help
help:
	@echo "Re-Vibe Development Commands"
	@echo "============================"
	@echo ""
	@echo "Setup & Installation:"
	@echo "  make install       - Install all dependencies (Rust, diesel CLI, Git hooks, etc.)"
	@echo "  make install-hooks - Install Git hooks (pre-commit clippy check)"
	@echo ""
	@echo "Development:"
	@echo "  make dev           - Start database and run the application locally"
	@echo "  make test          - Run all tests with database setup"
	@echo "  make test-watch    - Run tests in watch mode (auto-rerun on changes)"
	@echo ""
	@echo "Database:"
	@echo "  make db-up         - Start development database"
	@echo "  make db-down       - Stop development database"
	@echo "  make db-reset      - Reset database (drop, recreate, migrate)"
	@echo "  make migrate       - Run database migrations"
	@echo "  make seed          - Seed database with sample data"
	@echo ""
	@echo "Testing & CI:"
	@echo "  make ci-local      - Test CI workflows locally with act"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build  - Build production Docker image"
	@echo "  make docker-up     - Start production containers"
	@echo "  make docker-down   - Stop production containers"
	@echo "  make docker-clean  - Clean up all Docker resources"
	@echo ""
	@echo "Utilities:"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make fmt           - Format code"
	@echo "  make lint          - Run clippy linter"
	@echo "  make check         - Check code without building"

# Install all dependencies
install:
	@echo "📦 Installing dependencies..."
	@command -v rustc >/dev/null 2>&1 || (echo "❌ Rust not found. Install from https://rustup.rs/" && exit 1)
	@echo "✓ Rust is installed"
	@command -v diesel >/dev/null 2>&1 || (echo "Installing diesel CLI..." && cargo install diesel_cli --no-default-features --features postgres)
	@echo "✓ Diesel CLI is installed"
	@command -v docker >/dev/null 2>&1 || (echo "⚠️  Docker not found. Install from https://docker.com/" && exit 1)
	@echo "✓ Docker is installed"
	@test -f .env || (echo "⚠️  .env file not found. Please create one." && exit 1)
	@echo "✓ .env file exists"
	@$(MAKE) install-hooks
	@echo "✅ All dependencies installed!"

# Install Git hooks
install-hooks:
	@echo "🔗 Installing Git hooks..."
	@mkdir -p .git/hooks
	@cp hooks/pre-commit .git/hooks/pre-commit
	@chmod +x .git/hooks/pre-commit
	@echo "✓ Pre-commit hook installed (runs clippy before commits)"

# Start database and run application locally
dev:
	@./run_local.sh

# Run all tests
test:
	@./run_tests.sh

# Run tests in watch mode
test-watch:
	@echo "🔄 Running tests in watch mode..."
	@command -v cargo-watch >/dev/null 2>&1 || (echo "Installing cargo-watch..." && cargo install cargo-watch)
	@cargo watch -x "test --workspace -- --test-threads=1"

# Start development database
db-up:
	@echo "🐘 Starting development database..."
	@docker compose up db -d
	@echo "⏳ Waiting for database to be ready..."
	@sleep 3
	@docker exec re-vibe-db-1 pg_isready -U postgres || (echo "❌ Database not ready" && exit 1)
	@echo "✅ Database is ready"

# Stop development database
db-down:
	@echo "🛑 Stopping development database..."
	@docker compose down

# Reset database (drop, recreate, migrate)
db-reset:
	@echo "🔄 Resetting database..."
	@docker compose down -v
	@$(MAKE) db-up
	@$(MAKE) migrate
	@echo "✅ Database reset complete"

# Run database migrations
migrate:
	@echo "🔄 Running migrations..."
	@diesel migration run
	@echo "✅ Migrations complete"

# Seed database with sample data
seed:
	@echo "🌱 Seeding database..."
	@cargo run --bin seeder
	@echo "✅ Database seeded"

# Test CI workflows locally with act
ci-local:
	@./test_ci_local.sh

# Build production Docker image
docker-build:
	@echo "🐳 Building Docker image..."
	@docker compose build

# Start production containers
docker-up:
	@echo "🚀 Starting production containers..."
	@docker compose up -d
	@echo "✅ Containers started"

# Stop production containers
docker-down:
	@echo "🛑 Stopping production containers..."
	@docker compose down

# Clean up all Docker resources
docker-clean:
	@echo "🧹 Cleaning Docker resources..."
	@docker compose down -v --remove-orphans
	@docker network prune -f
	@echo "✅ Docker cleanup complete"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@echo "✅ Clean complete"

# Format code
fmt:
	@echo "✨ Formatting code..."
	@cargo fmt --all
	@echo "✅ Code formatted"

# Run clippy linter
lint:
	@echo "🔍 Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Check code without building
check:
	@echo "🔍 Checking code..."
	@cargo check --workspace
