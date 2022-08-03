#!/usr/cargo/env bash

docker build . --tag jo3mccain/flow:latest
docker run -d -rm --name flow -p 8080:8080 jo3mccain/flow:latest