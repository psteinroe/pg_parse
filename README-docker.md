# Docker Build Environment for pg_parse

This project includes Docker configuration to provide a consistent build environment without requiring local installation of Emscripten SDK or other dependencies.

## Prerequisites

- Docker
- Docker Compose
- [just](https://github.com/casey/just) command runner

## Quick Start

### Installing just

```bash
# macOS
brew install just

# Linux
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Or via cargo
cargo install just
```

### Available Commands

List all available commands:

```bash
just
```

### Interactive Development Shell

To enter an interactive shell with all build tools available:

```bash
just shell
```

Once inside the container, you can run standard cargo commands.

### Building

```bash
# Build for native target
just build

# Build for WebAssembly
just build-wasm

# Build all targets
just build-all
```

### Testing

```bash
# Run tests
just test

# Run example
just example

# Run all tests and examples
just test-all
```

### Development Workflow

```bash
# Format code, run clippy, and run tests
just dev

# Run CI checks (format check, clippy, tests)
just ci
```

### Additional Commands

```bash
# Clean build artifacts
just clean

# Deep clean including Docker volumes
just clean-all

# Run custom cargo commands
just cargo check
just cargo tree
just cargo bench

# Update dependencies
just update

# Check for outdated dependencies
just outdated

# Generate documentation
just doc

# Start a persistent dev container
just dev-container

# Show container logs
just logs
```

## Docker Configuration

### Development Dockerfile

The main `Dockerfile` provides a full development environment with:
- Emscripten SDK 3.1.46
- Rust with wasm32-unknown-emscripten target
- Protocol Buffers compiler
- All necessary build tools

### Production Dockerfile

`Dockerfile.slim` provides a multi-stage build for creating minimal production images containing only the built artifacts.

### Docker Compose

The `docker-compose.yml` file configures:
- Volume mounts for the project directory
- Cargo cache persistence across builds
- Separate target directory to avoid conflicts
- Interactive TTY for development

## Volume Mounts

The Docker setup uses several volumes to improve performance:

- **Project directory**: Mounted at `/workspace` for live code editing
- **Cargo registry**: Cached to speed up dependency downloads
- **Cargo git**: Cached for git dependencies
- **Target directory**: Separate volume to avoid conflicts between host and container builds

## Environment Variables

The container sets up:
- `CARGO_HOME` and `RUSTUP_HOME` for Rust
- `EMSDK` and `EM_CONFIG` for Emscripten
- PATH includes both Rust and Emscripten binaries

## Troubleshooting

### Permission Issues

The Dockerfile creates a non-root user (`developer`) to avoid permission issues with mounted volumes. If you encounter permission problems, ensure your local files are accessible.

### Build Cache

To clear the build cache and start fresh:

```bash
docker-compose down -v
docker-compose build --no-cache
```

### Emscripten Version

The Dockerfile uses Emscripten 3.1.46. To use a different version, modify the base image in the Dockerfile:

```dockerfile
FROM emscripten/emsdk:YOUR_VERSION
```

## Benefits

Using Docker for builds provides:

1. **Consistency**: Same build environment across all machines
2. **No Local Setup**: No need to install Emscripten SDK locally
3. **Isolation**: Build dependencies don't affect your system
4. **Reproducibility**: Builds are reproducible across different environments
5. **Easy CI/CD**: Same Docker image can be used in CI pipelines