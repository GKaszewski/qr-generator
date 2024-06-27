FROM rust:1.73 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache libgcc libstdc++
COPY --from=builder /usr/src/app/target/release/qr-generator /usr/local/bin/qr-generator
CMD ["qr-generator"]