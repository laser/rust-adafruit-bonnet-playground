#!/usr/bin/env bash

set -Eeuo pipefail

scp target/arm-unknown-linux-gnueabihf/release/adafruit-bonnet-playground $1:/home/pi
