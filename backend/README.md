# phundrak.com Backend

The backend for [phundrak.com](https://phundrak.com), built with Rust and the [Poem](https://github.com/poem-web/poem) web framework.

## Features

- **RESTful API** with automatic OpenAPI/Swagger documentation
- **Rate limiting** with configurable per-second limits using the
  Generic Cell Rate Algorithm (thanks to
  [`governor`](https://github.com/boinkor-net/governor))
- **Contact form** with SMTP email relay (supports TLS, STARTTLS, and
  unencrypted)
- **Type-safe routing** using Poem's declarative API
- **Hierarchical configuration** with YAML files and environment
  variable overrides
- **Structured logging** with `tracing` and `tracing-subscriber`
- **Strict linting** for code quality and safety
- **Comprehensive testing** with integration test support

## API Endpoints

The application provides the following endpoints:

- **Swagger UI**: `/` - Interactive API documentation
- **OpenAPI Spec**: `/specs` - OpenAPI specification in YAML format
- **Health Check**: `GET /api/health` - Returns server health status
- **Application Metadata**: `GET /api/meta` - Returns version and build info
- **Contact Form**: `POST /api/contact` - Submit contact form (relays to SMTP)

## Configuration

Configuration is loaded from multiple sources in order of precedence:

1. `settings/base.yaml` - Base configuration
2. `settings/{environment}.yaml` - Environment-specific (development/production)
3. Environment variables prefixed with `APP__` (e.g., `APP__APPLICATION__PORT=8080`)

The environment is determined by the `APP_ENVIRONMENT` variable (defaults to "development").

### Configuration Example

```yaml
application:
  port: 3100
  version: "0.1.0"

email:
  host: smtp.example.com
  port: 587
  user: user@example.com
  from: Contact Form <noreply@example.com>
  password: your_password
  recipient: Admin <admin@example.com>
  starttls: true  # Use STARTTLS (typically port 587)
  tls: false      # Use implicit TLS (typically port 465)

rate_limit:
  enabled: true        # Enable/disable rate limiting
  burst_size: 10       # Maximum requests allowed in time window
  per_seconds: 60      # Time window in seconds (100 req/60s = ~1.67 req/s)
```

You can also use a `.env` file for local development settings.

### Rate Limiting

The application includes built-in rate limiting to protect against abuse:

- Uses the **Generic Cell Rate Algorithm (GCRA)** via the `governor` crate
- **In-memory rate limiting** - no external dependencies like Redis required
- **Configurable limits** via YAML configuration or environment variables
- **Per-second rate limiting** with burst support
- Returns `429 Too Many Requests` when limits are exceeded

Default configuration: 100 requests per 60 seconds (approximately 1.67 requests per second with burst capacity).

To disable rate limiting, set `rate_limit.enabled: false` in your configuration.

## Development

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Running the Server

To start the development server:

```bash
cargo run
```

The server will start on the configured port (default: 3100).

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
# or
just test
```

Run a specific test:

```bash
cargo test <test_name>
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run tests with coverage:

```bash
cargo tarpaulin --config .tarpaulin.local.toml
# or
just coverage
```

### Testing Notes

- Integration tests use random TCP ports to avoid conflicts
- Tests use `get_test_app()` helper for consistent test setup
- Telemetry is automatically disabled during tests
- Tests are organized in `#[cfg(test)]` modules within each file

## Code Quality

### Linting

This project uses extremely strict Clippy linting rules:

- `#![deny(clippy::all)]`
- `#![deny(clippy::pedantic)]`
- `#![deny(clippy::nursery)]`
- `#![warn(missing_docs)]`

Run Clippy to check for issues:

```bash
cargo clippy --all-targets
# or
just lint
```

All code must pass these checks before committing.

### Continuous Checking with Bacon

For continuous testing and linting during development, use [bacon](https://dystroy.org/bacon/):

```bash
bacon           # Runs clippy-all by default
bacon test      # Runs tests continuously
bacon clippy    # Runs clippy on default target only
```

Press 'c' in bacon to run clippy-all.

## Code Style

### Error Handling

- Use `thiserror` for custom error types
- Always return `Result` types for fallible operations
- Use descriptive error messages

### Logging

Always use `tracing::event!` with proper target and level:

```rust
tracing::event!(
    target: "backend",  // or "backend::module_name"
    tracing::Level::INFO,
    "Message here"
);
```

### Imports

Organize imports in three groups:
1. Standard library (`std::*`)
2. External crates (poem, serde, etc.)
3. Local modules (`crate::*`)

### Testing Conventions

- Use `#[tokio::test]` for async tests
- Use descriptive test names that explain what is being tested
- Test both success and error cases
- For endpoint tests, verify both status codes and response bodies

## Project Structure

```
backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── lib.rs           # Library root with run() and prepare()
│   ├── startup.rs       # Application builder, server setup
│   ├── settings.rs      # Configuration management
│   ├── telemetry.rs     # Logging and tracing setup
│   ├── middleware/      # Custom middleware
│   │   ├── mod.rs       # Middleware module
│   │   └── rate_limit.rs # Rate limiting middleware
│   └── route/           # API route handlers
│       ├── mod.rs       # Route organization
│       ├── contact.rs   # Contact form endpoint
│       ├── health.rs    # Health check endpoint
│       └── meta.rs      # Metadata endpoint
├── settings/            # Configuration files
│   ├── base.yaml        # Base configuration
│   ├── development.yaml # Development overrides
│   └── production.yaml  # Production overrides
├── Cargo.toml           # Dependencies and metadata
└── README.md            # This file
```

## Architecture

### Application Initialization Flow

1. `main.rs` calls `run()` from `lib.rs`
2. `run()` calls `prepare()` which:
   - Loads environment variables from `.env` file
   - Initializes `Settings` from YAML files and environment variables
   - Sets up telemetry/logging (unless in test mode)
   - Builds the `Application` with optional TCP listener
3. `Application::build()`:
   - Sets up OpenAPI service with all API endpoints
   - Configures Swagger UI at the root path (`/`)
   - Configures API routes under `/api` prefix
   - Creates server with TCP listener
4. Application runs with CORS middleware and settings injected as data

### Email Handling

The contact form supports multiple SMTP configurations:
- **Implicit TLS (SMTPS)** - typically port 465
- **STARTTLS (Always/Opportunistic)** - typically port 587
- **Unencrypted** (for local dev) - with or without authentication

The `SmtpTransport` is built dynamically from `EmailSettings` based on
TLS/STARTTLS configuration.

## License

AGPL-3.0-only - See the root repository for full license information.
