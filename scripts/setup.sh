#!/usr/bin/env bash
rustup install nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
sudo apt -y update && sudo apt -y upgrade && sudo apt -y autoremove && sudo apt install -y apt-utils protobuf-compiler