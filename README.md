# Re-Vibe: Clothing Demo Website

## Project Overview

Welcome to Re-Vibe! This project is designed to showcase a demo website for selling used clothing and potentially other items in the future. The aim is to create a platform where users can list, browse, and purchase second-hand clothes. As I develop the project, I plan to incorporate additional features and enhancements to improve the user experience and expand functionality. This project utilizes Rust for the backend, along with the Rocket, Diesel ORM and PostgreSQL for robust data management.

## Table of Contents

- [Project Overview](#project-overview)
- [Quick Reference](#quick-reference)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Database Management](#database-management)
- [Usage](#usage)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [Future Improvements](#future-improvements)
- [License](#license)

## Quick Reference

**Common Commands (using Makefile):**
```bash
make help              # Show all available commands
make install           # Install all dependencies
make dev               # Start database and run app locally
make test              # Run all tests
make db-reset          # Reset database
make docker-up         # Start production containers
```

**Alternative (using scripts directly):**
```bash
./run_local.sh         # Start database and run app locally
./run_tests.sh         # Run all tests with Docker
./test_ci_local.sh     # Test CI workflows locally with act
```

**See Also:**
- [`Makefile`](Makefile) - All available development commands
- [`LOCAL_DEVELOPMENT.md`](LOCAL_DEVELOPMENT.md) - Comprehensive development guide
- [`.github/workflows/ci.yml`](.github/workflows/ci.yml) - CI pipeline that runs on every push
- [`.github/workflows/prod.yml`](.github/workflows/prod.yml) - Production deployment workflow


## Getting Started

### Prerequisites

Before you begin, ensure that the following prerequisites are installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (version 1.88.0, automatically managed via `rust-toolchain.toml`)
- [Docker](https://www.docker.com/get-started) and Docker Compose
- [Make](https://www.gnu.org/software/make/) (usually pre-installed on macOS/Linux)

**Optional:**
- [act](https://github.com/nektos/act) for testing CI workflows locally

### Installation

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your-username/re-vibe.git
   ```

2. Navigate to the project directory:

   ```bash
   cd re-vibe
   ```

3. Install all dependencies (Rust toolchain, diesel CLI, etc.):

   ```bash
   make install
   ```

   This will:
   - Verify Rust installation
   - Install diesel CLI (if not already installed)
   - Verify Docker installation
   - Check for `.env` file

4. Create a `.env` file in the project root directory and configure the database connection settings, keys, and secrets. You can use the following template:

   ```env
   # Database connection URL (for local development with Docker)
   DATABASE_URL=postgres://postgres:yourpassword@localhost:5433/postgres

   # PostgreSQL password
   POSTGRES_PASSWORD=yourpassword

   # Secret key used to sign and verify JSON Web Tokens (JWT)
   JWT_SECRET=your-secret-key-here

   # AWS S3 / DigitalOcean Spaces credentials
   BUCKET_ACCESS_KEY=your-access-key
   BUCKET_SECRET_ACCESS_KEY=your-secret-key
   BUCKET_ENDPOINT_URL=https://your-endpoint-url
   ```

   Replace the placeholder values with your actual credentials.

5. Create a `.env.development` file in the project `src/UI` directory for Vite frontend configuration:

   ```env
   # Base URL for API requests
   VITE_API_BASE_URL=http://localhost:8000
   ```

6. Start the database and run migrations:

   ```bash
   make db-up      # Start PostgreSQL in Docker
   make migrate    # Run database migrations
   ```

   Or use the all-in-one development command:

   ```bash
   make dev        # Starts database, runs migrations, and starts the app
   ```

### Database Management

**Reset the database (drop all tables and re-run migrations):**

```bash
make db-reset
```

This will:
1. Stop and remove the database container
2. Start a fresh database container
3. Run all migrations

**Other database commands:**

```bash
make db-up       # Start database
make db-down     # Stop database
make migrate     # Run migrations
make seed        # Seed database with sample data
```

**Manual database management (if needed):**

1. Drop the existing database:

   ```bash
   diesel database reset
   ```

2. Re-run migrations:

   ```bash
   make migrate
   ```

3. (Optional) Seed the database with sample data:

   ```bash
   make seed
   ```

## Usage

To launch the Re-Vibe server:

**Using Makefile (recommended):**
```bash
make dev
```

This will:
- Start the PostgreSQL database in Docker
- Run database migrations
- Start the application on port 8000

**Using cargo directly:**
```bash
cargo run --bin main
```

The server will start and listen on port 8000. You can access the website at `http://localhost:8000`.

## Testing

The project includes comprehensive unit and integration tests for all crates. Tests use a separate PostgreSQL test database managed via Docker Compose.

### Quick Start

**Run all tests (recommended):**
```bash
make test
```

**Run tests in watch mode (auto-rerun on file changes):**
```bash
make test-watch
```

**Using the script directly:**
```bash
./run_tests.sh
```

The test runner automatically:
1. Creates `.env.test` if it doesn't exist
2. Starts the test database with Docker Compose
3. Waits for the database to be ready
4. Runs database migrations
5. Builds and executes all tests (with caching for faster subsequent runs)
6. Cleans up (stops database and removes volumes)

### Manual Test Setup

If you prefer to run tests manually or need more control:

1. **Create test environment file:**

   Copy the example environment configuration:
   ```bash
   cp env.test.example .env.test
   ```

   The `.env.test` file should contain:
   ```env
   DATABASE_URL=postgres://postgres:testpassword@localhost:5434/re_vibe_test
   POSTGRES_PASSWORD=testpassword
   JWT_SECRET=test_jwt_secret_for_testing_only
   BUCKET_ACCESS_KEY=test_access_key
   BUCKET_SECRET_ACCESS_KEY=test_secret_key
   BUCKET_ENDPOINT_URL=https://mock-s3-endpoint.com
   ```

2. **Start the test database:**

   ```bash
   docker-compose -f docker-compose.test.yml up -d
   ```

   This starts a PostgreSQL test database on port 5434 (separate from the development database on port 5433).

3. **Run database migrations on test database:**

   ```bash
   DATABASE_URL=postgres://postgres:testpassword@localhost:5434/re_vibe_test diesel migration run
   ```

### Running Tests

**Run all tests across the entire workspace:**

```bash
cargo test --workspace
```

**Run tests for a specific crate:**

```bash
# Domain tests (model logic, no database required)
cargo test -p domain

# Infrastructure tests (validation, auth)
cargo test -p infrastructure

# Application tests (business logic with database)
cargo test -p application

# API tests (HTTP handlers with Rocket)
cargo test -p api
```

**Run tests with output:**

```bash
cargo test --workspace -- --nocapture
```

**Run tests sequentially (recommended for database tests):**

```bash
cargo test --workspace -- --test-threads=1
```

### Test Coverage

- **Domain (`src/domain/tests/`)**: Model validation, password hashing
- **Infrastructure (`src/infrastructure/tests/`)**: Validation logic, JWT authentication
- **Application (`src/application/tests/`)**: User registration/login, product operations
- **API (`src/api/tests/`)**: HTTP endpoint testing with Rocket

### Stopping Test Database

When finished testing manually:

```bash
docker-compose -f docker-compose.test.yml down
```

To remove test data volumes:

```bash
docker-compose -f docker-compose.test.yml down -v
```

### Continuous Integration

For CI/CD pipelines, use the test runner script:

```bash
./run_tests.sh
```

Or manually:

```bash
# Start test database
docker-compose -f docker-compose.test.yml up -d

# Wait for database to be ready
sleep 5

# Run migrations
DATABASE_URL=postgres://postgres:testpassword@localhost:5434/re_vibe_test diesel migration run

# Run tests
cargo test --workspace -- --test-threads=1

# Cleanup
docker-compose -f docker-compose.test.yml down -v
```

## Project Structure

The project employs a modular structure for better organization:

- `api/`: Contains the API entry point.
- `application/`: Handles application-level logic.
- `domain/`: Houses the core domain logic.
- `infrastructure/`: Manages database interactions and external services.
- `shared/`: Provides shared utilities and common functionalities.

Each component contains its own `Cargo.toml` file, allowing for independent development and testing.

## Contributing

Contributions to this project are encouraged! If you have ideas, bug reports, or enhancements, please open an issue or submit a pull request.

## Future Improvements

In the future, we plan to enhance the project with features such as:

- User authentication and account management
- Advanced search and filtering options for items
- Integration with payment gateways
- Support for media uploads (e.g., item images)
- Enhanced user interface and experience
- Analytics and reporting tools

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to explore the structured project components and their interactions to deepen your understanding of Rust and web development. Enjoy building and improving Re-Vibe!
