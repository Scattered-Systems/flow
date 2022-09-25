FROM rust:latest as builder


ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose --workspace


FROM photon as latest

ENV MODE="production" \
    SERVER_PORT=8080 \
    RUST_LOG="info"

COPY --from=builder /app/target/release/flow bin/flow

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

