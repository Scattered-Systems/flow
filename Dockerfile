FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    apt-utils \
    protobuf-compiler

RUN rustup update 

FROM builder-base as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose --workspace

FROM scratch as cache 

COPY --from=builder /app/target/release/flow /space/app/flow

VOLUME /space

FROM debian:buster-slim

RUN apt-get update -y && apt-get upgrade -y 

ENV MODE="production" \
    SERVER__PORT=9000 \
    RUST_LOG="info"

COPY Flow.toml ./
COPY --from=cache /space/app/flow /bin/flow

EXPOSE ${SERVER__PORT}

CMD [ "flow" ]