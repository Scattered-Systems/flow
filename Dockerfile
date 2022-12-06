FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN apt-get install -y \
    protobuf-compiler

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release -v --workspace

FROM debian:buster-slim as runner-base

ENV RUST_LOG="info" \
    SERVER_PORT=9090

RUN apt-get update -y && apt-get upgrade -y 

COPY Flow.toml /config/Flow.toml
VOLUME ["/config"]

COPY --from=builder /app/target/release/flow /bin/flow

FROM runner

EXPOSE ${SERVER_PORT}

ENTRYPOINT [ "flow" ]
CMD [ "system", "on" ]
