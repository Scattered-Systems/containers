FROM jo3mccain/container as builder-base

ENV PATH="$PATH:/usr/local/go/bin"

RUN wget https://go.dev/dl/go1.19.linux-amd64.tar.gz

RUN tar xvf go1.19.linux-amd64.tar.gz
RUN echo "export PATH=$PATH:/usr/local/go/bin"
