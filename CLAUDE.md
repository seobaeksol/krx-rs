# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Build and Compilation
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Check if code compiles without building
cargo check
```

### Testing
```bash
# Run all tests
cargo test

# Run tests in release mode
cargo test --release

# Run a specific test
cargo test test_name

# Run tests with output displayed
cargo test -- --nocapture

# Run integration tests only
cargo test --test '*'

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --verbose --all-features --workspace --out Xml
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check

# Run clippy linter
cargo clippy --all-targets --all-features -- -D warnings

# Check examples compile
cargo check --example simple_fetch
cargo check --example with_logging
```

### Documentation
```bash
# Generate and open documentation
cargo doc --open

# Generate docs with dependencies
cargo doc --no-deps
```

## Architecture Overview

This is a Rust client library for the Korea Exchange (KRX) Open API, designed with a modular, type-safe architecture.

### Core Components

1. **Client Module** (`src/client.rs`)
   - Central entry point for all API interactions
   - Manages HTTP client, authentication, and request handling
   - Provides builder pattern for configuration
   - Handles logging, timeouts, and error responses

2. **API Modules** (`src/api/`)
   - Each market type has its own module: `stock`, `index`, `bond`, `etp`, `derivative`, `general`, `esg`
   - Each module provides builder patterns for constructing API requests
   - All builders require either `.date()` or `.latest()` to specify the reference date

3. **Data Models** (`src/data/`)
   - Mirrors the API module structure with corresponding data types
   - All responses deserialize into these strongly-typed structures
   - Integrates with Polars DataFrame for data analysis

4. **Error Handling** (`src/error.rs`)
   - Comprehensive error types covering network, parsing, API, and validation errors
   - All functions return `Result<T, Error>` for consistent error handling

5. **Logging** (`src/logging.rs`)
   - Structured logging using the `tracing` crate
   - Configurable log levels, formats, and outputs

### API Design Pattern

All API endpoints follow a consistent builder pattern:
```rust
client.{market}()      // e.g., stock(), bond(), index()
    .{endpoint}()      // e.g., stock_daily(), etf_daily()
    .date("YYYYMMDD")  // or .latest() for most recent data
    .fetch()           // async execution
    .await?
```

### Key Implementation Notes

- **Base URL**: `http://data-dbg.krx.co.kr/svc/apis` (defined in `client.rs`)
- **Authentication**: Uses `AUTH_KEY` header for all requests
- **Date Requirement**: KRX API requires explicit date parameters (T-1 data only)
- **Response Format**: All data returned as Polars DataFrames for easy analysis
- **Async Runtime**: Built on Tokio for high-performance async operations

### Testing Strategy

- Unit tests are colocated with source files
- Integration tests in `tests/` directory use mock servers (wiremock)
- Examples in `examples/` demonstrate real usage patterns
- Environment variable `KRX_AUTH_KEY` used for live API testing (when available)

## Important Constraints

1. **Data Availability**: KRX provides data from 2010 onwards, up to T-1 (previous business day only)
2. **No Real-time Data**: Current day's data is not available through this API
3. **Rate Limiting**: API implements rate limiting (429 responses handled gracefully)
4. **Auth Key Required**: Valid KRX Open API authentication key needed for all requests