FROM rust:1.73-buster as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/qr-generator /usr/local/bin/qr-generator
CMD ["qr-generator"]