# Stage 1: Compile the code
# Digest refers to rust:1.93.1-trixie
FROM rust@sha256:80302520b7199f0504975bca59a914015e9fee088f759875dbbc238ca9509ee1 AS builder

ARG SERVICE_NAME

# Set workspace root
WORKDIR /usr/local/mlhub

# Copy everyting so Cargo sees workspace root
COPY . .

# Build using workspace resolution
RUN cargo build --release -p ${SERVICE_NAME} --locked

# Stage 2: Runtime image
FROM debian:trixie-slim

ARG SERVICE_NAME

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl3 && \
    update-ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

COPY --from=builder \
    /usr/local/mlhub/target/release/${SERVICE_NAME} \
    /usr/local/bin/mlhub

ENV RUST_LOG=debug,shared=debug

CMD ["mlhub"]
