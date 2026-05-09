# Build stage
FROM rust:1.94.1-slim-trixie as builder
RUN apt-get update && apt-get install -y musl-tools protobuf-compiler && \
  rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl

# Run stage
FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/users .
EXPOSE 3000
CMD ["./users"]
