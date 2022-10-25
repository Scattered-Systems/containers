FROM scsys/rust:debian-nightly as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

FROM builder

ENTRYPOINT [ "wasm-pack" ]