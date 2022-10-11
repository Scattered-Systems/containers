FROM scsys/photon:latest as builder-base

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder-base as stable

RUN rustup component add rustfmt clippy

FROM stable as nightly

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM scsys/photon:loaded as loaded-base

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

FROM builder-base as lstable

RUN rustup component add rustfmt clippy

FROM lstable as lnightly

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly
