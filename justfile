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

# Build for WebAssembly target
build-wasm:
    docker-compose run --rm pg-parse-dev cargo build --target wasm32-unknown-emscripten --release

# Build for all targets
build-all: build build-wasm

# Run tests
test:
    docker-compose run --rm pg-parse-dev cargo test

# Run the example
example:
    docker-compose run --rm pg-parse-dev cargo run --example test_parse

# Run all tests and examples
test-all: test example

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

# Build without Docker (requires local Rust and Emscripten)
local-build:
    cargo build --release

# Build WASM without Docker (requires local Rust and Emscripten)
local-build-wasm:
    cargo build --target wasm32-unknown-emscripten --release

# Run tests without Docker
local-test:
    cargo test

# Rebuild from scratch
rebuild: clean-all docker-build build-all

# Development workflow: format, clippy, test
dev: fmt clippy test

# CI workflow: check format, run clippy, run all tests
ci:
    docker-compose run --rm pg-parse-dev bash -c "cargo fmt -- --check && cargo clippy -- -D warnings && cargo test"

# Update dependencies
update:
    docker-compose run --rm pg-parse-dev cargo update

# Show outdated dependencies
outdated:
    docker-compose run --rm pg-parse-dev cargo outdated

# Generate and view documentation
doc:
    docker-compose run --rm pg-parse-dev cargo doc --open

# Publish to crates.io (dry run)
publish-dry:
    docker-compose run --rm pg-parse-dev cargo publish --dry-run

# Start a development container that stays running
dev-container:
    docker-compose up -d pg-parse-dev
    @echo "Container started. Use 'docker-compose exec pg-parse-dev bash' to enter."
    @echo "Stop with 'docker-compose down'"

# Show container logs
logs:
    docker-compose logs -f

# Check if Docker is running
check-docker:
    @docker version > /dev/null 2>&1 || (echo "Docker is not running. Please start Docker first." && exit 1)