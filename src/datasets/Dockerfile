FROM rust:1.82.0 as builder
RUN mkdir /usr/local/mlhub
WORKDIR /usr/local/mlhub
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/mlhub/target/release/datasets /usr/local/bin/mlhub
CMD ["mlhub"]