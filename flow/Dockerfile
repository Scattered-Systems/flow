FROM rust as builder

RUN update-ca-certificates

ADD . /application
WORKDIR /application

COPY . .
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /application/target/release/flow /flow

EXPOSE 8080:8080
CMD ["./flow"]