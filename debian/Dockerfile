FROM debian as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN apt-get install -y \
        apt-utils \
        build-essential \
        clang \
        curl \
        git \
        libssl-dev \
        llvm \
        pkg-config \
        procps \
        protobuf-compiler \
        unzip \
        wget
