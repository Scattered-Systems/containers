FROM debian as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN apt-get install -y \
    build-essential \
    clang \
    curl \
    git \
    libffi-dev \
    liblzma-dev \
    libsqlite3-dev \
    libssl-dev \
    libxml2-dev \
    libxmlsec1-dev \ 
    llvm \
    make \
    pkg-config \
    protobuf-compiler \
    unzip \
    wget \
    zlib1g-dev

