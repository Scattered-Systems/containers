FROM scsys/rust:debian-lts as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN curl -fsSL https://sh.iroh.computer/setup.sh | sh -s -- quickstart

