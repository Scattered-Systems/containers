FROM nixos/nix as builder-base

RUN nix-env -qaP \
    curl \
    git \
    openssl \
    wget

FROM builder-base as builder

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup update && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM builder as nightly

RUN rustup component add clippy rustfmt --toolchain nightly && \
    rustup default nightly 

FROM builder as latest

RUN rustup component add clippy rustfmt --toolchain stable && \
    rustup default stable
