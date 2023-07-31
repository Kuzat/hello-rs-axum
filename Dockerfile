FROM rust:1.71.0 as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hello-rs-axum /usr/local/bin/hello-rs-axum
CMD ["hello-rs-axum"]