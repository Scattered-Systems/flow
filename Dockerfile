FROM jo3mccain/rusty as builder

ADD . /app
WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:buster-slim as application

ENV MODE="development" \
    PORT=8888 \
    RUST_LOG="info"

COPY --from=builder /app/target/release/flow /flow

EXPOSE ${PORT}/tcp
ENTRYPOINT ["./flow"]