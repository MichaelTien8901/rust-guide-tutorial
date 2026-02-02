---
layout: default
title: Docker Setup
parent: Part 1 - Getting Started
nav_order: 5
---

# Docker Development Environment

Don't want to install Rust locally? Use Docker for a containerized development environment.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed
- [Docker Compose](https://docs.docker.com/compose/install/) (included with Docker Desktop)

## Quick Start

```bash
# Clone the examples repository
git clone https://github.com/MichaelTien8901/rust-guide-tutorial.git
cd rust-guide-tutorial

# Start the Rust container
docker-compose up -d

# Run a Rust command
docker-compose exec rust cargo --version
```

## Dockerfile

Create a `Dockerfile` in your project root:

```dockerfile
FROM rust:latest

# Install additional tools
RUN rustup component add rustfmt clippy rust-analyzer

# Install common development tools
RUN apt-get update && apt-get install -y \
    git \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace

# Keep container running
CMD ["tail", "-f", "/dev/null"]
```

## Docker Compose

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  rust:
    build: .
    volumes:
      - .:/workspace
      - cargo-cache:/usr/local/cargo/registry
    working_dir: /workspace
    stdin_open: true
    tty: true

volumes:
  cargo-cache:
```

## Usage

### Start the Environment

```bash
docker-compose up -d
```

### Run Rust Commands

```bash
# Create a new project
docker-compose exec rust cargo new my-project

# Build
docker-compose exec rust cargo build

# Run
docker-compose exec rust cargo run

# Test
docker-compose exec rust cargo test

# Format code
docker-compose exec rust cargo fmt

# Run clippy
docker-compose exec rust cargo clippy
```

### Interactive Shell

```bash
docker-compose exec rust bash
```

### Stop the Environment

```bash
docker-compose down
```

## VS Code Dev Containers

For the best Docker + VS Code experience, use Dev Containers.

### Setup

1. Install the "Dev Containers" extension in VS Code
2. Create `.devcontainer/devcontainer.json`:

```json
{
    "name": "Rust Development",
    "dockerFile": "../Dockerfile",
    "customizations": {
        "vscode": {
            "extensions": [
                "rust-lang.rust-analyzer",
                "vadimcn.vscode-lldb",
                "tamasfe.even-better-toml",
                "usernamehw.errorlens",
                "serayuzgur.crates"
            ],
            "settings": {
                "rust-analyzer.checkOnSave.command": "clippy",
                "[rust]": {
                    "editor.formatOnSave": true
                }
            }
        }
    },
    "mounts": [
        "source=rust-cargo-cache,target=/usr/local/cargo/registry,type=volume"
    ],
    "postCreateCommand": "rustup component add rustfmt clippy"
}
```

### Usage

1. Open the project folder in VS Code
2. Click "Reopen in Container" when prompted
3. VS Code will build and connect to the container

## Architecture

```mermaid
graph TD
    subgraph "Host Machine"
        A[Your Code]
        B[Docker Daemon]
    end

    subgraph "Docker Container"
        C[Rust Toolchain]
        D[Cargo Cache]
        E[/workspace]
    end

    A -->|volume mount| E
    B --> C
    D -->|cached dependencies| C
```

## Tips

### Caching Dependencies

The `cargo-cache` volume persists downloaded crates between container restarts:

```yaml
volumes:
  cargo-cache:
```

This dramatically speeds up builds after the first time.

### Using Specific Rust Versions

```dockerfile
# Use a specific version
FROM rust:1.75

# Or nightly
FROM rustlang/rust:nightly
```

### Slim Images

For smaller images:

```dockerfile
FROM rust:slim

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
```

### Multi-Stage Builds

For production deployments:

```dockerfile
# Build stage
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/myapp /usr/local/bin/
CMD ["myapp"]
```

## Troubleshooting

### Permission Denied

On Linux, you may need to run Docker without sudo:

```bash
sudo usermod -aG docker $USER
# Log out and back in
```

### Slow First Build

The first build downloads dependencies. Subsequent builds use the cached volume.

### File Changes Not Detected

Ensure your code is properly mounted:

```yaml
volumes:
  - .:/workspace
```

### Out of Disk Space

Clean up Docker:

```bash
docker system prune -a
```

## Advantages of Docker Development

| Benefit | Description |
|---------|-------------|
| Consistent environment | Same setup across all machines |
| Easy cleanup | Remove container, no traces left |
| Version isolation | Different projects can use different Rust versions |
| CI/CD ready | Same container for local and CI |
| Quick onboarding | New team members get started instantly |

## Next Steps

With your environment ready, let's write your [first Rust program]({% link part1/06-hello-world.md %}).
