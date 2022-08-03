#!/usr/cargo/env bash

docker build . --tag jo3mccain/flow:api --target api
docker run -d -rm --name flow-api -p 8080:8080 jo3mccain/flow:api