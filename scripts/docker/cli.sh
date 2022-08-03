#!/usr/cargo/env bash

docker build . --tag jo3mccain/flow:cli --target cli
docker run -d -rm --name flow-cli jo3mccain/flow:cli