# Local Development Guide

This guide explains how to run and test the Re-Vibe application locally, outside of Docker containers.

## Prerequisites

- Rust 1.88.0 (automatically installed via `rust-toolchain.toml`)
- Docker and Docker Compose (for database)
- Make (usually pre-installed on macOS/Linux)

## Editor Setup (VS Code)

### Enable Clippy Warnings in Editor

To see clippy warnings directly in VS Code as you code:

1. **Make sure rust-analyzer is installed** (not the old "Rust" extension)
2. **Add to your VS Code settings** (`Cmd+Shift+P` → "Preferences: Open User Settings (JSON)"):

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.extraArgs": [
    "--all-targets",
    "--all-features",
    "--",
    "-D",
    "warnings"
  ],
  "rust-analyzer.checkOnSave": true,
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

3. **Restart rust-analyzer**: `Cmd+Shift+P` → "rust-analyzer: Restart server"

**Benefits:**
- Red squiggly lines under code with clippy warnings
- Hover tooltips showing warning messages
- Quick fixes available via lightbulb icon
- Auto-format on save with rustfmt
- Same strict rules as CI (`-D warnings`)

**Troubleshooting:**
- If warnings don't appear, check Output panel: `View` → `Output` → "Rust Analyzer Language Server"
- Restart rust-analyzer if needed

## Quick Start

### 1. Install Dependencies

**Using Makefile (recommended):**
```bash
make install
```

This automatically installs:
- Diesel CLI (for database migrations)
- Verifies Rust and Docker installations
- Checks for required configuration files

**Manual installation:**
```bash
# Install diesel CLI (one-time setup)
cargo install diesel_cli --no-default-features --features postgres

# Install act for local CI testing (optional)
brew install act  # macOS
# or
# curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash  # Linux
```

### 2. Start the Database

**Using Makefile:**
```bash
make db-up
```

**Manual:**
```bash
# Start only the PostgreSQL database
docker compose up db -d

# Wait for it to be ready
sleep 5

# Verify it's running
docker ps
```

### 3. Run Migrations

**Using Makefile:**
```bash
make migrate
```

**Manual:**
```bash
# Set the database URL (or use .env file)
export DATABASE_URL="postgres://postgres:${POSTGRES_PASSWORD}@localhost:5433/postgres"

# Run migrations
diesel migration run
```

### 4. Run the Application

**Using Makefile (recommended):**
```bash
make dev
```

This handles everything: starts database, runs migrations, and starts the app.

**Manual:**
```bash
# Build and run in development mode
cargo run --bin main

# Or build and run in release mode (faster)
cargo build --release
./target/release/main
```

The application will be available at `http://localhost:8000`.

## Running Tests

### Option 1: Using Makefile (Recommended)

```bash
make test              # Run all tests
make test-watch        # Run tests in watch mode (auto-rerun on changes)
```

### Option 2: Using the Test Script

```bash
./run_tests.sh
```

### Option 3: Manual Testing

```bash
# Start test database
docker-compose -f docker-compose.test.yml up -d

# Wait for database
sleep 5

# Run migrations
DATABASE_URL=postgres://postgres:testpassword@localhost:5434/re_vibe_test diesel migration run

# Run tests
cargo test --workspace -- --test-threads=1

# Cleanup
docker-compose -f docker-compose.test.yml down -v
```

## Testing CI Changes Locally

Use [act](https://github.com/nektos/act) to run GitHub Actions workflows locally:

```bash
# Run all workflows triggered by push
act push

# Run specific job
act -j test

# Run with secrets (create .secrets file first)
act --secret-file .secrets

# List available workflows
act -l

# Dry run to see what would happen
act -n
```

### Creating a `.secrets` file for act

```bash
# .secrets
DOCKER_USERNAME=your_username
DOCKER_PASSWORD=your_password
POSTGRES_PASSWORD=your_password
JWT_SECRET=your_secret
# ... other secrets
```

## Environment Variables

The application requires these environment variables:

```bash
# Database
DATABASE_URL=postgres://postgres:password@localhost:5433/postgres

# Authentication
JWT_SECRET=your-secret-key

# AWS S3 Bucket
BUCKET_ACCESS_KEY=your-access-key
BUCKET_SECRET_ACCESS_KEY=your-secret-key
BUCKET_ENDPOINT_URL=your-endpoint-url
```

You can set these in:
- `.env` file (for local development)
- `.env.test` file (for testing)
- Environment variables directly

## Common Issues

### Diesel CLI Installation Fails

If `cargo install diesel_cli` fails, ensure you have PostgreSQL development libraries:

```bash
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt-get install libpq-dev

# Fedora/RHEL
sudo dnf install postgresql-devel
```

### Database Connection Fails

Check that:
1. Docker container is running: `docker ps`
2. Port is correct (5433 for dev, 5434 for test)
3. Password matches your `.env` file
4. Database URL is correct

### Rust Version Mismatch

The project uses `rust-toolchain.toml` to pin Rust to version 1.88.0. If you have issues:

```bash
# Update rustup
rustup update

# Verify version
rustc --version  # Should show 1.88.0
```

## Project Structure

```
re-vibe/
├── src/
│   ├── api/           # API endpoints (Rocket)
│   ├── application/   # Business logic
│   ├── domain/        # Domain models
│   ├── infrastructure/# Database, S3, etc.
│   ├── shared/        # Shared utilities
│   └── UI/            # Frontend (Vite)
├── migrations/        # Database migrations
├── Dockerfile         # Production container
├── docker-compose.yml # Production setup
├── docker-compose.test.yml # Test database
└── run_tests.sh       # Test runner script
```

## Performance & Caching

### Compilation Caching

Rust uses incremental compilation to cache builds. After the first compilation:
- Subsequent builds are much faster (only changed files recompile)
- Test builds reuse compilation artifacts
- The `target/` directory stores cached builds

**Tips for faster builds:**
```bash
# Keep target/ directory (don't delete it)
# Cargo automatically manages the cache

# Use cargo-watch for automatic recompilation on file changes
cargo install cargo-watch
cargo watch -x test

# Use sccache for distributed compilation caching (optional)
cargo install sccache
export RUSTC_WRAPPER=sccache
```

**Why first run is slow:**
- First compilation builds all dependencies from scratch
- Subsequent runs only rebuild changed code (much faster)
- The `./run_tests.sh` script now optimizes for incremental builds

### Build Artifacts Location

```
target/
├── debug/          # Development builds (faster compilation)
├── release/        # Optimized builds (slower compilation, faster runtime)
└── .rustc_info.json  # Compiler cache metadata
```

**Don't commit `target/` to git** - it's already in `.gitignore`

## Useful Commands

```bash
# Check code formatting
cargo fmt --all -- --check

# Run linter
cargo clippy --all-targets --all-features

# Build for release
cargo build --release

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Create new migration
diesel migration generate migration_name

# Revert last migration
diesel migration revert
```

## Docker Development

If you prefer to develop inside Docker:

```bash
# Build and run everything
docker-compose up --build

# View logs
docker-compose logs -f api

# Stop everything
docker-compose down

# Remove volumes (clean slate)
docker-compose down -v
```
