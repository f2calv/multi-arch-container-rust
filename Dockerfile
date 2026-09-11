# syntax=docker/dockerfile:1
#
# Multi-architecture container image built from a SINGLE Dockerfile.
#
# This file deliberately mirrors its sibling repositories stage-for-stage and
# comment-for-comment, so that a developer fluent in one language can learn the
# containerisation story of another by diffing the two files:
#
#   https://github.com/f2calv/multi-arch-container-dotnet
#   https://github.com/f2calv/multi-arch-container-go
#   https://github.com/f2calv/multi-arch-container-rust      <- you are here
#   https://github.com/f2calv/multi-arch-container-python
#
# ------------------------------------------------------------------------------
# Stage 1 of 2: build
#
# Pinned to $BUILDPLATFORM (the native architecture of the machine running the
# build) and CROSS-COMPILES to $TARGETPLATFORM. The alternative - emulating the
# target architecture under QEMU - is typically 10-50x slower.
#
# Unlike .NET and Go, Rust produces a natively-linked binary, so cross-compiling
# needs three things: the rustup std library for the target triple, a GNU cross
# linker/compiler, and cargo told which of each to use.
# ------------------------------------------------------------------------------
FROM --platform=$BUILDPLATFORM rust:1-bookworm AS build
WORKDIR /src

ARG APP_NAME=multi-arch-container-rust
ARG PROFILE=release

# buildx injects TARGETARCH/TARGETVARIANT automatically:
#   linux/amd64  -> TARGETARCH=amd64  TARGETVARIANT=
#   linux/arm64  -> TARGETARCH=arm64  TARGETVARIANT=
#   linux/arm/v7 -> TARGETARCH=arm    TARGETVARIANT=v7
# Concatenating the two gives a single flat token to switch on: amd64|arm64|armv7.
ARG TARGETARCH
ARG TARGETVARIANT

# -- Toolchain layer -----------------------------------------------------------
# Resolve the platform ONCE and persist the result to /etc/rust-target.env, which
# every later stage sources. This keeps the platform mapping in a single place
# rather than repeating the same case statement in each stage.
# https://doc.rust-lang.org/nightly/rustc/platform-support.html
RUN <<EOF
set -eux
case "${TARGETARCH}${TARGETVARIANT}" in
    amd64) TARGET=x86_64-unknown-linux-gnu      ; GNU=x86-64-linux-gnu    ; CC_PREFIX=x86_64-linux-gnu    ; LIBC=amd64 ;;
    arm64) TARGET=aarch64-unknown-linux-gnu     ; GNU=aarch64-linux-gnu   ; CC_PREFIX=aarch64-linux-gnu   ; LIBC=arm64 ;;
    armv7) TARGET=armv7-unknown-linux-gnueabihf ; GNU=arm-linux-gnueabihf ; CC_PREFIX=arm-linux-gnueabihf ; LIBC=armhf ;;
    *) echo "unsupported platform: linux/${TARGETARCH}/${TARGETVARIANT}" >&2; exit 1 ;;
esac
apt-get update
apt-get install -y --no-install-recommends "g++-${GNU}" "libc6-dev-${LIBC}-cross"
rm -rf /var/lib/apt/lists/*
rustup target add "$TARGET"
# cargo derives the linker/compiler override variable names from the target
# triple: dashes become underscores, and CARGO_TARGET_* is upper-cased.
UPPER=$(echo "$TARGET" | tr 'a-z-' 'A-Z_')
LOWER=$(echo "$TARGET" | tr '-' '_')
{
    echo "export CARGO_BUILD_TARGET=$TARGET"
    echo "export CARGO_TARGET_${UPPER}_LINKER=${CC_PREFIX}-gcc"
    echo "export CC_${LOWER}=${CC_PREFIX}-gcc"
    echo "export CXX_${LOWER}=${CC_PREFIX}-g++"
} > /etc/rust-target.env
EOF

# -- Dependency layer ----------------------------------------------------------
# Copy ONLY the files that influence dependency resolution so that editing a .rs
# file reuses the cached download. `cargo fetch` populates $CARGO_HOME, which is
# a BuildKit cache mount and therefore survives across builds on the same host.
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked <<EOF
set -eux
. /etc/rust-target.env
# cargo refuses to parse a manifest that declares no target, so stub out
# src/main.rs - the real sources overwrite it in the compile layer below.
mkdir -p src
echo 'fn main() {}' > src/main.rs
cargo fetch --locked --target "$CARGO_BUILD_TARGET"
EOF

# -- Compile layer -------------------------------------------------------------
COPY src src

# The target directory is a cache mount (huge, and worthless in the final image),
# so the finished binary must be copied OUT of it inside the same RUN instruction.
# The cache id is per-architecture to stop the three platform legs thrashing it.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    --mount=type=cache,target=/src/target,id=cargo-target-${TARGETARCH}${TARGETVARIANT},sharing=locked <<EOF
set -eux
. /etc/rust-target.env
cargo build --locked --profile "$PROFILE" --target "$CARGO_BUILD_TARGET"
install -D "target/${CARGO_BUILD_TARGET}/${PROFILE}/${APP_NAME}" "/out/${APP_NAME}"
EOF

# ------------------------------------------------------------------------------
# Stage 2 of 2: final
#
# No --platform override here, so buildx resolves the base image for
# $TARGETPLATFORM and the resulting image is genuinely native to the target.
#
# `cc` is the distroless variant that ships glibc + libgcc, which is what the
# *-unknown-linux-gnu targets link against. Alternatives:
#   gcr.io/distroless/cc-debian12:nonroot  glibc, ~25MB, non-root  (used here)
#   gcr.io/distroless/static-debian12      only for fully-static *-musl targets
#   scratch                                only for fully-static *-musl targets
# ------------------------------------------------------------------------------
FROM gcr.io/distroless/cc-debian12:nonroot AS final
WORKDIR /app
COPY --link --from=build /out/multi-arch-container-rust .
# Base configuration; every value can be overridden by an environment variable at runtime.
COPY appsettings.json .

# -- Provenance ----------------------------------------------------------------
# Supplied by the CI workflow (.github/workflows/ci.yml) or by build.sh/build.ps1.
ARG GIT_REPOSITORY=n/a
ENV GIT_REPOSITORY=$GIT_REPOSITORY
ARG GIT_BRANCH=n/a
ENV GIT_BRANCH=$GIT_BRANCH
ARG GIT_COMMIT=n/a
ENV GIT_COMMIT=$GIT_COMMIT
ARG GIT_TAG=n/a
ENV GIT_TAG=$GIT_TAG

ARG GITHUB_WORKFLOW=n/a
ENV GITHUB_WORKFLOW=$GITHUB_WORKFLOW
ARG GITHUB_RUN_ID=0
ENV GITHUB_RUN_ID=$GITHUB_RUN_ID
ARG GITHUB_RUN_NUMBER=0
ENV GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER

# https://github.com/opencontainers/image-spec/blob/main/annotations.md
LABEL org.opencontainers.image.title="multi-arch-container-rust" \
    org.opencontainers.image.description="Multi-architecture container build (amd64/arm64/armv7) w/Rust" \
    org.opencontainers.image.source="https://github.com/f2calv/multi-arch-container-rust" \
    org.opencontainers.image.licenses="MIT" \
    org.opencontainers.image.version="$GIT_TAG" \
    org.opencontainers.image.revision="$GIT_COMMIT"

# The :nonroot distroless tag already runs as uid/gid 65532 - setting it
# explicitly documents the intent and keeps the four sibling repos consistent.
USER nonroot:nonroot

ENTRYPOINT ["/app/multi-arch-container-rust"]
