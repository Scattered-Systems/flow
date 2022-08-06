FROM jo3mccain/rusty as builder

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color always --release --verbose --workspace && \
    cargo test --all-features --color always --release --verbose --workspace


FROM photon as latest

ENV MODE="development" \
    SERVER_PORT=8080 \
    RUST_LOG="info"

COPY --from=builder /workspace/target/release/flow-api /flow-api

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD ["./flow-api"]


FROM photon as cli

ENV MODE="development" \
    RUST_LOG="info"

COPY --from=builder /workspace/target/release/flow-cli /flow-cli

ENTRYPOINT ["./flow-cli"]