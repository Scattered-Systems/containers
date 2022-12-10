FROM debian:latest as base

RUN apt-get update -y && apt-get upgrade -y && apt-get autoremove -y

FROM base as builder-base

RUN apt-get install -y \
        build-essential \
        clang \
        curl \
        git \
        libssl-dev \
        llvm \
        node \
        pkg-config \
        procps \
        protobuf-compiler \
        unzip \
        wget

FROM builder-base as environment

RUN curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash
RUN export NVM_DIR="$HOME/.nvm"

FROM builder-base as builder

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
VOLUME [ "/root/.cargo/bin" ]

FROM builder as volumes

RUN mkdir data
VOLUME [ "/data" ]

FROM volumes as stable

RUN rustup default stable

FROM stable as nightly

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly
