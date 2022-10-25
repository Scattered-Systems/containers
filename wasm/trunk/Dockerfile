FROM scsys/rust:debian-nightly as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN cargo install --locked trunk && \
    cargo install wasm-bindgen-cli

FROM builder

ENTRYPOINT [ "/root/.cargo/bin/trunk" ]
