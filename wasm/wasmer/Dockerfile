FROM scsys/rust:debian-nightly as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y libncurses5

FROM builder-base as builder

RUN curl https://get.wasmer.io -sSfL | bash

FROM builder

VOLUME [ "/usr/src" ]
VOLUME [ "/config" ]
VOLUME [ "/data" ]
