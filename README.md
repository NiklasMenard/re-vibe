# Re-Vibe: Clothing Demo Website

## Project Overview

Welcome to Re-Vibe! This project is designed to showcase a demo website for selling used clothing and potentially other items in the future. The aim is to create a platform where users can list, browse, and purchase second-hand clothes. As we develop the project, we plan to incorporate additional features and enhancements to improve the user experience and expand functionality. This project utilizes Rust for the backend, along with the Diesel ORM and PostgreSQL for robust data management.

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

4. Create a `.env` file in the project root directory and configure the database connection settings. You can use the following template:

   ```env
   DATABASE_URL=postgres://username:password@localhost/re-vibe
   ```

   Replace `username` and `password` with your PostgreSQL credentials.

5. Set up the database schema and run initial migrations using Diesel:

   ```bash
   diesel setup
   diesel migration run
   ```

## Usage

To launch the Re-Vibe server, execute the following command in the project directory:

```bash
cargo run
```

The server will start and listen on the specified port. You can interact with the website and its API using your web browser or an API client.

## Testing

For testing, you can use the provided `API_TEST.http` file to send test requests. You may use the VSCode Rest Client plugin for this purpose.

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