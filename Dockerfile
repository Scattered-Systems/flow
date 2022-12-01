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
RUN cargo build --color always --release --verbose -p flow

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y 

FROM runner-base as runner

ENV CLIENT_ID="" \
    CLIENT_SECRET="" \
    RUST_LOG="info" \
    SERVER_PORT=9000

COPY --from=builder /app/target/release/flow /bin/flow

EXPOSE ${SERVER_PORT}

CMD [ "flow", "system", "on" ]
