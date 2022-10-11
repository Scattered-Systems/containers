FROM photon:latest as builder-base

RUN yum update -y && yum upgrade -y

FROM builder-base as base

RUN yum install -y \
    build-essential \
    pkg-config \
    protobuf-c \
    wget

FROM base as loaded


RUN yum install -y \
    bzip2-devel \
    clang \
    git \
    libffi-devel \
    llvm \
    nodejs \
    openssl-devel \
    python3 \
    unzip