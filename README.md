# Rust Live Coding Session

Welcome to the Rust Live Coding Session repository! This project was created as part of an internal "TED Talk" to
introduce our team to the Rust programming language through a live coding experience.
The Video of the Coding Session is here: https://youtu.be/PDF20YdlHTw

## Overview

During the session, we explored Rust's features by building a simple application. This repository contains:

- The source code developed during the live session
- Examples demonstrating key Rust concepts
- Additional resources for further learning

## Technologies Used

The application leverages several Rust libraries and frameworks to provide a robust, asynchronous, and efficient system:

- **Tokio**: [Tokio](https://crates.io/crates/tokio) is the main component of the application, offering an event-driven,
  non-blocking I/O platform for writing asynchronous applications.

- **Axum**: The primary web framework used is [Axum](https://crates.io/crates/axum), which provides a modular and
  ergonomic way to build web services on top of Tokio.

- **Serde**: For serialization and deserialization of JSON, we utilize [Serde](https://crates.io/crates/serde), a
  powerful framework for handling Rust data structures efficiently.

- **Utoipa**: To generate API documentation (OpenAPI), we integrated [Utoipa](https://crates.io/crates/utoipa) with
  Axum, enabling automatic and up-to-date API docs.

- **Reqwest**: For making API calls to other systems, we use [Reqwest](https://crates.io/crates/reqwest), a convenient
  and high-level HTTP client.

- **Diesel**: As our ORM, [Diesel](https://diesel.rs) provides safe and efficient interactions with the database,
  leveraging Rust's type system to prevent runtime errors.

By combining these technologies, the application achieves high performance, safety, and scalability, embodying Rust's
strengths in systems programming and modern web development.

## Getting Started

To run the code from this repository, you'll need to have Rust installed on your machine.

## Prerequisites

- Rust: Install Rust by following the instructions on the official website: https://www.rust-lang.org/tools/install

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/tspycher/rust_livecoding.git
   ```

2. Navigate to the project directory:

   ```
   cd rust_livecoding
   cp .env_example .env
   ```

## Running the Application

To build and run the application, execute:

```
cargo run
```

## Running Tests

If the project includes tests, you can run them with:

```
cargo test
```

## Project Structure

- `src/` - Main source code directory
    - `main.rs` - The main entry point of the application
- `Cargo.toml` - Project configuration file

## Learning Resources

- The Rust Programming Language Book: https://doc.rust-lang.org/book/
- Rust By Example: https://doc.rust-lang.org/rust-by-example/
- Official Rust Documentation: https://doc.rust-lang.org/

## Contributing

This project is intended for educational purposes within our team. If you have suggestions or improvements, feel free to
create a branch or discuss them during our next meeting.

## License

This project is licensed under the MIT License (see the LICENSE file for details).

## Contact

For any questions or discussions, please reach out to Your Name at your.email@example.com.

---

Happy coding! 🚀