FROM scsys/debian:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y software-properties-common

FROM builder-base as latest

RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -

RUN apt-get update -y

RUN apt-get install -y nodejs yarn

# FROM builder-base as stable

# RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -

# RUN apt-get update -y

# RUN apt-get install -y nodejs yarn

# RUN nodejs --version && npm --version && yarn --version