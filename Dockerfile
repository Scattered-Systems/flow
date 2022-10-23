FROM scsys/rust:debian-lts as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose --workspace

FROM debian:buster-slim

ENV MODE="production" \
    SERVER__PORT=9000 \
    RUST_LOG="info"

COPY --from=builder /app/target/release/flow bin/flow

EXPOSE ${SERVER__PORT}/tcp
EXPOSE ${SERVER__PORT}/udp

CMD [ "flow" ]