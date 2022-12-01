FROM nixos/nix as builder-base

RUN nix-channel --update

RUN nix-env -qaP \
    curl \
    git \
    wget

FROM builder-base as lang-base

RUN curl https://sh.rustup.rs -Sf | sh -y -- nix-shell

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup update && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown wasm32-wasi --toolchain nightly
