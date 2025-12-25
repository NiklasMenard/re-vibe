# Re-Vibe: Clothing Demo Website

## Project Overview

Welcome to Re-Vibe! This project is designed to showcase a demo website for selling used clothing and potentially other items in the future. The aim is to create a platform where users can list, browse, and purchase second-hand clothes. As I develop the project, I plan to incorporate additional features and enhancements to improve the user experience and expand functionality. This project utilizes Rust for the backend, along with the Rocket, Diesel ORM and PostgreSQL for robust data management.

## Table of Contents

- [Project Overview](#project-overview)
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

## Getting Started

### Prerequisites

Before you begin, ensure that the following prerequisites are installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (including Cargo)
- [Diesel CLI](http://diesel.rs/guides/getting-started/) for managing database schema and migrations
- [PostgreSQL](https://www.postgresql.org/download/) database server

### Installation

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your-username/re-vibe.git
   ```

2. Navigate to the project directory:

   ```bash
   cd re-vibe
   ```

3. Install project dependencies using Cargo:

   ```bash
   cargo build
   ```

4. Create a `.env` file in the project root directory and configure the database connection settings, keys, and secrets. You can use the following template:

   ```env
   # Database connection URL. Replace 'username' and 'password' with your PostgreSQL credentials.
   DATABASE_URL=postgres://username:password@localhost/re-vibe

   # PostgreSQL password for the user specified in DATABASE_URL.
   POSTGRES_PASSWORD=postgrespw

   # Secret key used to sign and verify JSON Web Tokens (JWT). Should be a strong, secret string.
   JWT_SECRET=jwtsecret

   # Storage Bucket Access Key for authenticating API requests to Storage Bucket services.
   BUCKET_SECRET_ACCESS_KEY=key

   # Storage Bucket Secret Access Key for authenticating API requests to Storage Bucket services.
   BUCKET_SECRET_ACCESS_KEY=key
   ```

   Replace `username` and `password` with your PostgreSQL credentials.

5. Create a `.env.development` file in the project `src/UI` directory to create a local Vite API URL reference for your server API. You can use the following template:

   ```env
   # Base URL for API requests. Replace with your server's URL and port.
   VITE_API_BASE_URL=http://localhost:8000
   ```

6. Set up the database schema and run initial migrations using Diesel:

   ```bash
   diesel setup
   diesel migration run
   ```

### Database Management

If you need to reset the database (drop all tables and re-run migrations), follow these steps:

1. Drop the existing database:

   ```bash
   diesel database reset
   ```

   This will drop and recreate the database.

2. Re-run migrations:

   ```bash
   diesel migration run
   ```

3. (Optional) Seed the database with product data and bucket keys that reference images in the Digital Ocean bucket:

   ```bash
   cargo run --bin seeder
   ```

## Usage

To launch the Re-Vibe server, execute the following command in the project directory:

```bash
cargo run --bin main
```

The server will start and listen on the specified port. You can interact with the website and its API using your web browser or an API client.

## Testing

The project includes comprehensive unit and integration tests for all crates. Tests use a separate PostgreSQL test database managed via Docker Compose.

### Quick Start - Automated Test Runner

The easiest way to run all tests is using the provided test runner script:

```bash
./run_tests.sh
```

This script automatically:
1. Creates `.env.test` if it doesn't exist
2. Starts the test database with Docker Compose
3. Waits for the database to be ready
4. Runs database migrations
5. Executes all tests
6. Cleans up (stops database and removes volumes)

### Manual Test Setup

If you prefer to run tests manually:

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
