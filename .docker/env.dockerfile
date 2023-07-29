FROM rust:latest as base

RUN apt-get update -y && apt-get upgrade -y

FROM base as builder-base

RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash
RUN export NVM_DIR="$HOME/.nvm"

RUN apt-get install nodejs npm

RUN rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly

FROM base as nightly

RUN rustup default nightly
