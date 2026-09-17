# Copilot Instructions

## Shared Instructions

Shared Copilot instruction files are maintained centrally in the [.github](https://github.com/f2calv/.github) repository under `instructions/`, and are applied to every workspace from the VS Code user profile via `~/.copilot/instructions`. They are deliberately not copied into this repository, so a change there takes effect everywhere without a pull request here.

Everything below is specific to this repository.

## Repository Purpose

This repository is a Rust application that demonstrates how to build multi-architecture container images (amd64, arm64, arm/v7) from a single `Dockerfile` using `docker buildx`. It is a reference implementation, not a production workload.

## Sibling Repositories (alignment is a hard requirement)

Four repositories implement the *same* trivial worker application in four languages:

- [multi-arch-container-dotnet](https://github.com/f2calv/multi-arch-container-dotnet)
- [multi-arch-container-go](https://github.com/f2calv/multi-arch-container-go)
- [multi-arch-container-rust](https://github.com/f2calv/multi-arch-container-rust) (this one)
- [multi-arch-container-python](https://github.com/f2calv/multi-arch-container-python)

Their premise is that a developer fluent in one language can learn another language's containerisation story by diffing two repositories. **Any change made here must be considered for the other two.** Keep the following as close to identical as possible:

- Repository layout and file names.
- `Dockerfile` stage names (`build`, `final`), section comment banners and ordering.
- The `ARG`/`ENV` provenance block and OCI `LABEL` block.
- Environment variable names consumed by the application — both the flat `GIT_*`/`GITHUB_*` provenance variables and the `APP__*` configuration overrides.
- Application file responsibilities: configuration model, logging setup, worker loop, entry-point wiring.
- `.github/workflows/ci.yml` job names and structure.
- `.editorconfig` common section, `.pre-commit-config.yaml`, `.devcontainer/`, `.vscode/extensions.json`.
- `build.sh` / `build.ps1` are byte-identical (all values are derived from git).
- `README.md` section headings.

## No Helm Charts

These repositories are **application code only**. Kubernetes packaging lives in the standalone [f2calv/helm-charts](https://github.com/f2calv/helm-charts) repository, which provides a single multi-purpose chart used by all deployments. Do not reintroduce a `charts/` directory or a `chart` job in `ci.yml`.

## Linting is manual, never an auto-installed git hook

Do **not** wire `pre-commit install` into `.devcontainer/postCreateCommand.sh`, `postStartCommand.sh` or the README. The hook cost is fixed interpreter start-up per hook rather than per file, so a one-file commit pays the same price as a full run — noticeable on slower hardware. Linting is run manually with `pre-commit run --all-files`, and the `lint` job in `ci.yml` is the authoritative gate. A once-per-push hook (`pre-commit install --hook-type pre-push`) is an acceptable opt-in, never a default.

## Project Structure

- `src/main.rs` – entry point; configuration, logging and shutdown wiring only.
- `src/config.rs` – `AppConfig` / `Settings` types and the layered loader.
- `src/telemetry.rs` – `tracing` subscriber installation.
- `src/worker.rs` – the worker loop.
- `appsettings.json` – base configuration.
- `Cargo.toml` / `Cargo.lock` – package manifest and lockfile (both committed).
- `Dockerfile` – two-stage, cross-compiling, multi-architecture build.
- `.github/workflows/ci.yml` – CI/CD using reusable workflows from [f2calv/gha-workflows](https://github.com/f2calv/gha-workflows).
- `.devcontainer/` – VS Code devcontainer (Rust toolchain + Docker-outside-of-Docker). All Rust tooling runs in the container; nothing is installed on the host.
- `build.sh` / `build.ps1` – local build scripts for manual testing.

## Technology Stack

- **Language**: Rust (edition 2021)
- **Async runtime**: Tokio (minimal feature set — `macros`, `rt-multi-thread`, `sync`, `time`)
- **Logging**: `tracing` + `tracing-subscriber`, with a text or JSON layer selected by configuration
- **Configuration**: the `config` crate (appsettings.json → environment variables), deserialised with `serde`
- **Container**: Docker (multi-stage, distroless final image, non-root)
- **CI/CD**: GitHub Actions (reusable workflows from `f2calv/gha-workflows`)
- **Versioning**: GitVersion (MainLine mode)

## Configuration Keys

Configuration keys are **snake_case** in both `appsettings.json` and the environment. This is deliberate: the `config` crate lower-cases environment keys but preserves file keys verbatim, so snake_case is the only casing where both sources resolve to the same key — and it is what the sibling .NET and Go repositories use.

| Key | Environment variable | Default |
| --- | --- | --- |
| `app.greeting` | `APP__GREETING` | `Hello from a multi-architecture container` |
| `app.interval_seconds` | `APP__INTERVAL_SECONDS` | `3` |
| `app.log_format` | `APP__LOG_FORMAT` | `text` |

The flat provenance variables (`GIT_REPOSITORY`, `GIT_BRANCH`, `GIT_COMMIT`, `GIT_TAG`, `GITHUB_WORKFLOW`, `GITHUB_RUN_ID`, `GITHUB_RUN_NUMBER`) are baked into the image by the `ARG`/`ENV` block of the `Dockerfile` and deserialised onto `Settings`.

Log verbosity is controlled separately by `RUST_LOG` (`EnvFilter` syntax), defaulting to `info`.

## Target Platforms

The Dockerfile maps `TARGETARCH`+`TARGETVARIANT` onto a Rust target triple plus a GNU cross toolchain:

- `linux/amd64` → `x86_64-unknown-linux-gnu`
- `linux/arm64` → `aarch64-unknown-linux-gnu`
- `linux/arm/v7` → `armv7-unknown-linux-gnueabihf`

The mapping is resolved **once** in the toolchain layer and written to `/etc/rust-target.env`, which later layers source. Do not duplicate the `case` statement.

## Container Conventions

- Keep the `Dockerfile` single-file with multi-stage builds; never add per-architecture Dockerfiles.
- The final image is `gcr.io/distroless/cc-debian12:nonroot`. `cc` (not `static`) is required because the `*-unknown-linux-gnu` targets link dynamically against glibc.
- The Dockerfile builds with `--locked`, so `Cargo.lock` must be regenerated (`cargo fetch`) whenever `Cargo.toml` changes.
- Heredoc `RUN <<EOF` blocks require **LF line endings**. `.gitattributes` enforces this; a CRLF `Dockerfile` fails at build time with `/bin/sh: set: Illegal option -`.
