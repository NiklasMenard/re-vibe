# Re-Vibe

A demo marketplace for selling used clothing, built with Rust (Rocket, Diesel) and PostgreSQL.

## Quick Start

```bash
make install    # One-time setup
make dev        # Start app (http://localhost:8000)
make test       # Run tests
```

## Prerequisites

- [Rust](https://rustup.rs) 1.88.0 (auto-installed via `rust-toolchain.toml`)
- [Docker](https://docker.com) & Docker Compose
- Make (pre-installed on macOS/Linux)

## Development

### Common Commands

```bash
make help         # Show all commands
make dev          # Start database + run app
make test         # Run all tests
make test-watch   # Auto-rerun tests on changes
make db-reset     # Fresh database
make lint         # Run clippy
make fmt          # Format code
```

### VS Code Setup

**Enable clippy warnings + auto-format on save:**

1. Install `rust-analyzer` extension
2. Add to settings (`Cmd+Shift+P` → "Open User Settings (JSON)"):

```json
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.extraArgs": ["--all-targets", "--all-features", "--", "-D", "warnings"],
  "rust-analyzer.checkOnSave": true,
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}
```

3. Restart rust-analyzer: `Cmd+Shift+P` → "rust-analyzer: Restart server"

**Benefits:** Red squiggles for warnings, auto-format on save, quick fixes

### Environment Variables

Create `.env` in project root:

```env
DATABASE_URL=postgres://postgres:yourpassword@localhost:5433/postgres
POSTGRES_PASSWORD=yourpassword
JWT_SECRET=your-secret-key
BUCKET_ACCESS_KEY=your-access-key
BUCKET_SECRET_ACCESS_KEY=your-secret-key
BUCKET_ENDPOINT_URL=https://your-endpoint-url
```

## Testing

### Run Tests

```bash
make test              # All tests
make test-watch        # Watch mode
./run_tests.sh         # Using script
```

### Test CI Locally

```bash
brew install act       # Install act (macOS)
make ci-local          # Test GitHub Actions locally
```

## Database

```bash
make db-up             # Start PostgreSQL
make db-down           # Stop database
make db-reset          # Drop, recreate, migrate
make migrate           # Run migrations
make seed              # Add sample data
```

**Manual:**
```bash
diesel migration generate migration_name  # Create migration
diesel migration run                      # Apply migrations
diesel migration revert                   # Rollback
```

## Project Structure

```
re-vibe/
├── src/
│   ├── api/            # Rocket endpoints
│   ├── application/    # Business logic
│   ├── domain/         # Models & schema
│   ├── infrastructure/ # Database, S3
│   ├── shared/         # Utilities
│   └── UI/             # Frontend (Vite)
├── migrations/         # Database migrations
├── Makefile            # Development commands
└── hooks/              # Git hooks (pre-commit)
```

## CI/CD

- **CI Tests** (`.github/workflows/ci.yml`): Runs on every push/PR
  - Clippy linting
  - Code formatting check
  - All tests

- **Production** (`.github/workflows/prod.yml`): Runs on push/PR to `main`
  - Build Docker image
  - Deploy to server

**Pre-commit Hook:** Automatically formats code and runs clippy before every commit.

## Troubleshooting

**Diesel CLI fails to install:**
```bash
# macOS
brew install postgresql

# Ubuntu/Debian
sudo apt-get install libpq-dev
```

**Database connection fails:**
- Check Docker is running: `docker ps`
- Verify port (5433 for dev, 5434 for test)
- Check `.env` file

**Rust version mismatch:**
```bash
rustup update
rustc --version  # Should show 1.88.0
```

## Performance Tips

- First build is slow (~2-3 min) - subsequent builds are fast (5-10s)
- Keep `target/` directory (don't delete - it's your build cache)
- Use `make test-watch` for TDD workflow
- Incremental compilation is enabled by default

## Docker (Alternative)

```bash
make docker-build     # Build image
make docker-up        # Start containers
make docker-down      # Stop containers
make docker-clean     # Remove all Docker resources
```

## Contributing

1. Create feature branch
2. Make changes (pre-commit hook runs clippy + format)
3. Push and create PR
4. CI tests must pass before merging

## License

MIT
