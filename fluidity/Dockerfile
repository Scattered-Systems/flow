FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN rustup update 

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose --workspace

FROM debian:buster-slim

RUN apt-get update -y && apt-get upgrade -y 

ENV MODE="production" \
    SERVER__PORT=9000 \
    RUST_LOG="info"

COPY --from=builder /app/target/release/flow /bin/flow

EXPOSE 9000:${SERVER__PORT}/tcp
EXPOSE 9000:${SERVER__PORT}/udp

CMD [ "flow" ]