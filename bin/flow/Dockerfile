FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    apt-utils \
    protobuf-compiler

RUN rustup update 

FROM builder-base as builder

ADD . /app
RUN chmod -R 777 /app

WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose -p flow

FROM scratch as cache 

COPY --from=builder /app/target/release/flow /space/bin/flow
VOLUME /space

FROM debian:buster-slim

RUN apt-get update -y && apt-get upgrade -y 

ENV LOG_LEVEL="info" \
    SERVER_PORT=9000

COPY --from=builder /app/Flow.toml /flow/Flow.toml
COPY --from=cache /space/bin/flow /bin/flow
RUN chmod -R 777 /flow

EXPOSE ${SERVER_PORT}

CMD [ "flow" ]