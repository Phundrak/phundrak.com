# phundrak.com Backend

The backend for [phundrak.com](https://phundrak.com), built with Rust and the [Poem](https://github.com/poem-web/poem) web framework.

## Features

- **RESTful API** with OpenAPI documentation
- **Type-safe routing** using Poem's declarative API
- **Structured logging** with `tracing`
- **Strict linting** for code quality and safety
- **Comprehensive testing** with integration test support

## Development

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Running the Server

To start the development server:

```bash
cargo run
```

The server will start on the configured port (check your configuration for details).

### Building

For development builds:

```bash
cargo build
```

For optimized production builds:

```bash
cargo build --release
```

The compiled binary will be at `target/release/backend`.

## Testing

Run all tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test <test_name>
```

Run tests with output:

```bash
cargo test -- --nocapture
```

## Code Quality

### Linting

This project uses strict Clippy linting rules:

- `#![deny(clippy::all)]`
- `#![deny(clippy::pedantic)]`
- `#![deny(clippy::nursery)]`

Run Clippy to check for issues:

```bash
cargo clippy --all-targets
```

### Continuous Checking with Bacon

For continuous testing and linting during development, use [bacon](https://dystroy.org/bacon/):

```bash
bacon
```

This will watch your files and automatically run clippy or tests on changes.

## Code Style

### Error Handling

- Use `thiserror` for custom error types
- Always return `Result` types for fallible operations
- Use descriptive error messages

### Logging

- Use `tracing::event!` for logging
- Always set `target: "backend"`
- Use appropriate log levels (trace, debug, info, warn, error)

Example:
```rust
tracing::event!(target: "backend", tracing::Level::INFO, "Server started");
```

### Imports

Organize imports in three groups:
1. Standard library (`std::*`)
2. External crates
3. Local modules

Use explicit paths (e.g., `poem_openapi::ApiResponse` instead of wildcards).

### Testing

- Use `#[cfg(test)]` module blocks
- Leverage Poem's test utilities for endpoint testing
- Use random TCP listeners for integration tests to avoid port conflicts

## Project Structure

```
backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── api/             # API endpoints
│   ├── models/          # Data models
│   ├── services/        # Business logic
│   └── utils/           # Utility functions
├── tests/               # Integration tests
├── Cargo.toml           # Dependencies and metadata
└── README.md            # This file
```

## License

See the root repository for license information.
