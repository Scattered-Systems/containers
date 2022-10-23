FROM scsys/rust:debian-lts as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    procps

FROM builder-base as builder

RUN git clone https://github.com/n0-computer/iroh && \
    cd iroh && cargo build --release --workspace

# RUN curl -fsSL https://sh.iroh.computer/setup.sh | sh -s -- quickstart

FROM builder

# ENV PATH="/iroh/target/release/iroh"

EXPOSE 9050/tcp
EXPOSE 9050/udp
