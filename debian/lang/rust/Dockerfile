FROM scsys/debian:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder as latest

RUN rustup update && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM latest as nightly

RUN rustup default nightly
