# Use the official Emscripten image as base
FROM emscripten/emsdk:3.1.46

# Install additional dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add wasm32-unknown-emscripten target
RUN rustup target add wasm32-unknown-emscripten

# Install additional cargo tools
RUN cargo install wasm-bindgen-cli

# Set working directory
WORKDIR /workspace

# Set environment variables for Emscripten
ENV EMSDK=/emsdk
ENV EM_CONFIG=/emsdk/.emscripten
ENV EMSDK_NODE=/emsdk/node/16.20.0_64bit/bin/node
ENV PATH="${EMSDK}:${EMSDK}/upstream/emscripten:${PATH}"

# Create a non-root user for development (optional but recommended)
RUN useradd -m -s /bin/bash developer && \
    chown -R developer:developer /workspace

# Copy the project files
COPY --chown=developer:developer . /workspace

# Switch to non-root user
USER developer

# Set Rust environment for the non-root user
ENV PATH="/home/developer/.cargo/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    /home/developer/.cargo/bin/rustup target add wasm32-unknown-emscripten

CMD ["/bin/bash"]