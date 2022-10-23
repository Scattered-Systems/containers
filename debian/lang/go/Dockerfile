FROM scsys/debian:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ENV PATH="${PATH}:/go/bin"

RUN wget https://go.dev/dl/go1.19.linux-amd64.tar.gz && \
    tar xvf go1.19.linux-amd64.tar.gz

RUN rm go1.19.linux-amd64.tar.gz && \
    export PATH=$PATH:/go/bin
