FROM debian as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

RUN apte-get install -y apt-utils build-essential curl git libssl-dev pkg-config procps protobuf-compiler unzip wget

FROM builder as extras

RUN apt-get install -y \
    clang \
    libffi-dev \
    liblzma-dev \
    libsqlite3-dev \
    libxml2-dev \
    libxmlsec1-dev \ 
    llvm \
    make \
    postgresql \
    postgresql-contrib \
    software-properties-common \
    zlib1g-dev
