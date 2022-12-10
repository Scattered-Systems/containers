FROM ./Dockerfile as nightly

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

FROM nightly as trunkrs

RUN cargo install trunk wasm-bindgen-cli

FROM nightly as wasm-pack

RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -

RUN apt-get update -y

RUN apt-get install -y nodejs yarn

RUN npm install -g wasm-pack

