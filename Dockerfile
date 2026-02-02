FROM rust:latest

# Install additional tools
RUN rustup component add rustfmt clippy rust-analyzer

# Install common development tools
RUN apt-get update && apt-get install -y \
    git \
    vim \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace

# Keep container running
CMD ["tail", "-f", "/dev/null"]
