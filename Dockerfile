FROM --platform=$BUILDPLATFORM rust AS base
RUN rustup component add clippy
RUN rustup component add rustfmt


FROM base AS dependencies
WORKDIR /app/
#initialize empty application
RUN cargo init
#replace the toml dependency file with our own
COPY ./Cargo.toml ./Cargo.lock /app/
#run a build to download the dependencies
RUN cargo build --release
# ARG TARGETPLATFORM
# RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
#     TARGET=x86_64-unknown-linux-gnu ; \
#     elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
#     TARGET=aarch64-unknown-linux-gnu ; \
#     fi \
#     && cargo build --release --target=$TARGET


FROM dependencies AS source
COPY ./src/ /app/src/


FROM source AS build
RUN cargo build --release
# ARG TARGETPLATFORM
# RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
#     rustup target add x86_64-unknown-linux-gnu ; \
#     rustup toolchain install x86_64-unknown-linux-gnu ; \
#     TARGET=x86_64-unknown-linux-gnu ; \
#     elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
#     rustup target add aarch64-unknown-linux-gnu ; \
#     rustup toolchain install stable-aarch64-unknown-linux-gnu ; \
#     TARGET=aarch64-unknown-linux-gnu ; \
#     elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
#     rustup target add armv7-unknown-linux-gnueabihf ; \
#     rustup toolchain install stable-armv7-unknown-linux-gnueabihf ; \
#     TARGET=armv7-unknown-linux-gnueabihf ; \
#     fi \
#     && cargo build --release --target=$TARGET

FROM gcr.io/distroless/cc AS final
COPY --from=build /app/target/release/multi-arch-container-rust .

ARG GIT_REPO
ENV APP_GIT_REPO=$GIT_REPO
ARG GIT_TAG
ENV APP_GIT_TAG=$GIT_TAG
ARG GIT_BRANCH
ENV APP_GIT_BRANCH=$GIT_BRANCH
ARG GIT_COMMIT
ENV APP_GIT_COMMIT=$GIT_COMMIT

ARG GITHUB_WORKFLOW
ENV APP_GITHUB_WORKFLOW=$GITHUB_WORKFLOW
ARG GITHUB_RUN_ID
ENV APP_GITHUB_RUN_ID=$GITHUB_RUN_ID
ARG GITHUB_RUN_NUMBER
ENV APP_GITHUB_RUN_NUMBER=$GITHUB_RUN_NUMBER

ENTRYPOINT ["./multi-arch-container-rust"]