# Rust Blog API Project README

Welcome to the Rust Blog API project! This project is designed to help you learn the Rust programming language by building a blog API using a modular and organized project structure. The project uses Rust, the Diesel ORM, and PostgreSQL to provide a solid foundation for further development.

## Table of Contents

- [Project Overview](#project-overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [Future Improvements](#future-improvements)
- [License](#license)

## Project Overview

The Rust Blog API project offers an interactive learning experience for mastering Rust concepts while constructing a versatile blog API. The project utilizes Diesel ORM for efficient database interactions, PostgreSQL as the database system, and follows a modular structure to ensure maintainability and extensibility.

## Getting Started

### Prerequisites

Before you begin, ensure that the following prerequisites are installed on your system:

- [Rust](https://www.rust-lang.org/tools/install) (including Cargo)
- [Diesel CLI](http://diesel.rs/guides/getting-started/) for managing database schema and migrations
- [PostgreSQL](https://www.postgresql.org/download/) database server

### Installation

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your-username/rust-blog-api.git
   ```

2. Navigate to the project directory:

   ```bash
   cd rust-blog-api
   ```

3. Install project dependencies using Cargo:

   ```bash
   cargo build
   ```

4. Create a `.env` file in the project root directory and configure the database connection settings. You can use the following template:

   ```env
   DATABASE_URL=postgres://username:password@localhost/blog_db
   ```

   Replace `username` and `password` with your PostgreSQL credentials.

5. Set up the database schema and run initial migrations using Diesel:

   ```bash
   diesel setup
   diesel migration run
   ```

## Usage

To launch the Rust Blog API server, execute the following command in the `api` directory:

```bash
cargo run
```

The server will start and listen on the specified port. You can interact with the API using your preferred API client.

## Testing

Testing for now you can use the API_TEST.http file to send test requests with vscode Rest client plugin.

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

In the future I will consider enhancing the project with features such as:

- Advanced query options for blog posts
- User authentication and authorization
- Support for media uploads
- Integration with external APIs for content sharing
- Robust error handling and validation
- Integration of logging and monitoring tools

## License

This project is licensed under the [MIT License](LICENSE).

---

Feel free to explore the structured project components and their interactions to deepen your understanding of Rust and API development. Enjoy your coding journey!
