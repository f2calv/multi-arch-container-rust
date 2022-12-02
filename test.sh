#!/bin/sh

TARGETPLATFORM=linux/arm64

if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    rustup target add x86_64-unknown-linux-gnu ; \
    rustup toolchain install x86_64-unknown-linux-gnu ; \
    TARGET=x86_64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    rustup target add aarch64-unknown-linux-gnu ; \
    rustup toolchain install stable-aarch64-unknown-linux-gnu ; \
    TARGET=aarch64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    rustup target add armv7-unknown-linux-gnueabihf ; \
    rustup toolchain install stable-armv7-unknown-linux-gnueabihf ; \
    TARGET=armv7-unknown-linux-gnueabihf ; \
fi \
    && cargo build --release --target=$TARGET