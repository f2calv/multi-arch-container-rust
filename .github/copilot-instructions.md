# Copilot Instructions

## Repository Overview

This repository is a Rust application that demonstrates how to build multi-architecture container images (amd64, arm64, arm/v7) from a single `Dockerfile` using `docker buildx`. It serves as a reference implementation for cross-compiling Rust to multiple target platforms.

## Project Structure

- `src/main.rs` – Entry point for the Rust application. A simple worker loop that logs system/environment info.
- `Cargo.toml` / `Cargo.lock` – Rust package manifest and lockfile.
- `Dockerfile` – Multi-stage Dockerfile for cross-compiling and packaging the app for multiple architectures.
- `.github/workflows/ci.yml` – CI/CD pipeline using reusable workflows from [f2calv/gha-workflows](https://github.com/f2calv/gha-workflows).
- `charts/` – Helm chart for Kubernetes deployment.
- `.devcontainer/` – VS Code devcontainer configuration (Rust toolchain + Docker-outside-of-Docker).
- `build.sh` / `build.ps1` – Local build scripts for manual testing.

## Technology Stack

- **Language**: Rust (edition 2021)
- **Async runtime**: Tokio
- **Configuration**: `config` crate (reads environment variables with `APP_` prefix)
- **Logging**: `env_logger` / `log`
- **Container**: Docker (multi-stage, distroless final image)
- **CI/CD**: GitHub Actions
- **Helm**: Chart packaging and publishing to GHCR

## Build & Test Commands

```bash
# Build the Rust application
cargo build

# Build in release mode
cargo build --release

# Run tests
cargo test

# Lint with Clippy
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check
```

## Local Container Build

Use the provided helper scripts from the repository root:

```bash
# Shell
. build.sh

# PowerShell
./build.ps1
```

Or manually using `docker buildx`:

```bash
docker buildx create --name multiarchcontainerrust --use
docker buildx build \
    -t multi-arch-container-rust:dev \
    --platform linux/amd64 \
    --pull \
    -o type=docker \
    .
```

## Configuration

The application reads configuration from environment variables prefixed with `APP_`:

| Environment Variable    | Description                  |
|-------------------------|------------------------------|
| `APP_GIT_REPOSITORY`    | Git repository name          |
| `APP_GIT_BRANCH`        | Git branch name              |
| `APP_GIT_COMMIT`        | Git commit SHA               |
| `APP_GIT_TAG`           | Git tag                      |
| `APP_GITHUB_WORKFLOW`   | GitHub Actions workflow name |
| `APP_GITHUB_RUN_ID`     | GitHub Actions run ID        |
| `APP_GITHUB_RUN_NUMBER` | GitHub Actions run number    |

## Target Platforms

The Dockerfile supports cross-compilation to the following platforms:

- `linux/amd64` → `x86_64-unknown-linux-gnu`
- `linux/arm64` → `aarch64-unknown-linux-gnu`
- `linux/arm/v7` → `armv7-unknown-linux-gnueabihf`

## Coding Conventions

- Follow standard Rust idioms and the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).
- Use `cargo fmt` for formatting and `cargo clippy` for linting before submitting changes.
- Keep the `Dockerfile` single-file with multi-stage builds; avoid separate per-architecture Dockerfiles.
- Configuration is injected via environment variables (no config files bundled in the image).
- The final container image is based on `gcr.io/distroless/cc` to minimize attack surface.
