# Builder
FROM rust:1.82.0 AS builder

COPY . /app

WORKDIR /app
RUN cargo build --release
RUN strip target/release/rust-http-server

# Release
# This image is required because the binary links libc
FROM gcr.io/distroless/cc-debian12  AS release

COPY --from=builder /app/target/release/rust-http-server .

CMD ["./rust-http-server"]