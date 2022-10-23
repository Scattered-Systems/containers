FROM photon:latest as builder-base

RUN yum update -y && yum upgrade -y

FROM builder-base as base

RUN yum install -y \
    build-essential \
    bzip2-devel \
    clang \
    git \
    libffi-devel \
    libxml2-devel \
    llvm \
    make \
    nodejs \
    openssl-devel \
    pkg-config \
    protobuf-c \
    sqlite3 \
    unzip \
    wget