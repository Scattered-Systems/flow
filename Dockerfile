FROM scsys/rust:photon-loaded-stable as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always -p flow --release -v

FROM scsys/photon

ENV MODE="production" \
    SERVER_PORT=9000 \
    RUST_LOG="info"

COPY --from=builder /app/target/release/flow bin/flow

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD [ "flow" ]