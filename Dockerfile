FROM jo3mccain/rusty as builder


ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build --color always --release --workspace && \
    cargo test --all-features --color always --release --workspace


FROM photon as latest

ENV MODE="production" \
    SERVER_PORT=8080 \
    RUST_LOG="info"

COPY --from=builder /workspace/target/release/flow /flow

EXPOSE ${SERVER_PORT}/tcp
EXPOSE ${SERVER_PORT}/udp

CMD ["./flow"]