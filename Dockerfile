FROM rust AS base
RUN apt-get update && apt-get install -yq lld
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


FROM dependencies AS source
COPY ./src/ /app/src/


FROM source AS build
RUN cargo build --release


FROM gcr.io/distroless/cc AS final
#RUN apt-get update && apt-get install -yq lld
COPY --from=build /app/target/release/app app
ENTRYPOINT ["./app"]