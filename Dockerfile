FROM ubuntu:16.04

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN apt-get update \
    && apt-get install -y git curl build-essential

RUN git clone --progress --verbose https://github.com/raspberrypi/tools.git --depth=1 /pi-tools

ENV CC=/pi-tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc

ENV PATH=/pi-tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin:$PATH

RUN curl -sSf https://sh.rustup.rs \
    | sh -s -- -y

RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME

RUN rustup target add arm-unknown-linux-gnueabihf

WORKDIR /build
