# Default recipe to display help
default:
    @just --list

# Build the Docker image
docker-build:
    docker-compose build

# Enter an interactive shell in the Docker container
shell:
    docker-compose run --rm pg-parse-dev /bin/bash

# Build for native target
build:
    docker-compose run --rm pg-parse-dev cargo build --release

# WASM example (located in wasm_example/)
wasm-test:
    @echo "Running WASM example tests..."
    cd wasm_example && just test

# Run tests
test:
    docker-compose run --rm pg-parse-dev cargo test

# Clean build artifacts
clean:
    docker-compose run --rm pg-parse-dev cargo clean

# Deep clean including Docker volumes
clean-all: clean
    docker-compose down -v

# Format code
fmt:
    docker-compose run --rm pg-parse-dev cargo fmt

# Run clippy lints
clippy:
    docker-compose run --rm pg-parse-dev cargo clippy -- -D warnings

# Check code without building
check:
    docker-compose run --rm pg-parse-dev cargo check

# Run a custom cargo command
cargo *ARGS:
    docker-compose run --rm pg-parse-dev cargo {{ARGS}}

# Development workflow: format, clippy, test
fix: fmt clippy test

# Show WASM build size (from wasm_example)
wasm-size:
    @echo "WASM module sizes:"
    cd wasm_example && just build-complete
    @echo "Built WASM files:"
    @ls -lh wasm_example/target/pg_parse_wasm.wasm wasm_example/target/pg_parse_wasm.js 2>/dev/null || echo "Files not found. Run 'just wasm-test' first."
