FROM scsys/debian:latest as builder-base

FROM builder-base as environment-base

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

FROM environment-base as environment

RUN rustup install nightly && \
    rustup target add wasm32-unknown-unknown

FROM environment as builder

RUN curl -fsSL https://sh.iroh.computer/setup.sh | sh -s -- quickstart