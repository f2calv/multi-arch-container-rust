FROM --platform=$BUILDPLATFORM rust AS base
WORKDIR /app
RUN apt-get update && apt-get upgrade -y
RUN rustup component add clippy
RUN rustup component add rustfmt
#todo: create fmt/clippy/test stages
ARG TARGETPLATFORM
RUN \
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    echo 'need to test below libs and also test building x86_64 FROM arm??... ' ; \
    apt-get install -y g++-x86-64-linux-gnu libc6-dev-amd64-cross ; \
    rustup target add x86_64-unknown-linux-gnu ; \
    rustup toolchain install stable-x86_64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    apt-get install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross ; \
    rustup target add aarch64-unknown-linux-gnu ; \
    rustup toolchain install stable-aarch64-unknown-linux-gnu ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    apt-get install -y g++-arm-linux-gnueabihf libc6-dev-armhf-cross ; \
    rustup target add armv7-unknown-linux-gnueabihf ; \
    rustup toolchain install stable-armv7-unknown-linux-gnueabihf ; \
fi



FROM base AS dependencies
WORKDIR /app
#initialize an empty application & replace the dependencies file with our own
RUN cargo init
COPY Cargo.toml Cargo.lock /app
ARG TARGETPLATFORM
RUN \
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    TARGET=x86_64-unknown-linux-gnu ; \
    echo 'need to add correct env vars here and also test building from arm??... ' ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    TARGET=aarch64-unknown-linux-gnu ; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc ; \
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc ; \
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    TARGET=armv7-unknown-linux-gnueabihf ; \
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc ; \
    export CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc ; \
    export CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ ; \
fi \
&& cargo fetch --target $TARGET
#Note: for some reason we can't download and 'pre-build' the dependencies here... but it worked prior version of dockerfile
#Note: if we stick with fetch we can remove all the exports from this section
#TODO: move exports into external script due to repetition?
#&& cargo build --release --target $TARGET



FROM dependencies AS source
COPY src src



FROM source AS build
ARG TARGETPLATFORM
RUN \
if [ "$TARGETPLATFORM" = "linux/amd64" ]; then \
    TARGET=x86_64-unknown-linux-gnu ; \
    echo 'need to add correct env vars here and also test building from arm??... ' ; \
elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    TARGET=aarch64-unknown-linux-gnu ; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc ; \
    export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc ; \
    export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ ; \
elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then \
    TARGET=armv7-unknown-linux-gnueabihf ; \
    export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc ; \
    export CC_armv7_unknown_Linux_gnueabihf=arm-linux-gnueabihf-gcc ; \
    export CXX_armv7_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++ ; \
fi \
&& cargo build --release --target $TARGET && mv /app/target/$TARGET /app/target/final
#Note: we rename the target path to 'final' so we don't need the TARGET variable in the final stage


FROM gcr.io/distroless/cc AS final
COPY --from=build /app/target/final/release/multi-arch-container-rust .

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