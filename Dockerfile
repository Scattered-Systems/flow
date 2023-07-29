FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash

RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash
RUN export NVM_DIR="$HOME/.nvm"

RUN apt-get install nodejs npm

RUN rustup default nightly \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM builder-base as builder

ENV CARGO_TERM_COLOR=always

ADD . /workspace
WORKDIR /workspace

COPY . .
RUN cargo build -r -v --workspace

FROM debian:buster-slim as runner-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y ca-certificates libssl-dev

FROM runner-base as runner

COPY --from=builder /workspace/target/release/flow /usr/local/bin/flow

ENTRYPOINT [ "flow" ]
CMD [ "--help" ]
