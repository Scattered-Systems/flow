FROM jo3mccain/rusty as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --color always --release --verbose --workspace

FROM photon as application

ENV MODE="development" \
    PORT=8080 \
    RUST_LOG="info"

COPY --from=builder /app/target/release/flow-api /flow-api

EXPOSE ${PORT}/tcp
EXPOSE ${PORT}/udp

CMD ["./flow-api"]