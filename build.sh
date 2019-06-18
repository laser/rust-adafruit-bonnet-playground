#!/usr/bin/env bash

set -Eeuo pipefail

docker build . -t adafruit-bonnet-playground:latest

mkdir -p /tmp/cargo-registry-cache

docker run -it --rm \
    -w "/build" \
    -v "$(pwd):/build" \
    -v "/tmp/cargo-registry-cache:/usr/local/cargo/registry" \
    adafruit-bonnet-playground:latest \
    cargo build --release --target=arm-unknown-linux-gnueabihf
