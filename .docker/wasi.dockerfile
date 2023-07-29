FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

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
RUN cargo xtask build

FROM scratch as runner-base

COPY --from=builder /workspace/flow/pkg /artifacts/pkg

