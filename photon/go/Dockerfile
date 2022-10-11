FROM scsys/photon:latest as latest

ENV PATH="${PATH}:/go/bin"

RUN wget https://go.dev/dl/go1.19.linux-amd64.tar.gz && \
    tar xvf go1.19.linux-amd64.tar.gz

RUN rm go1.19.linux-amd64.tar.gz && \
    export PATH=$PATH:/go/bin


FROM scsys/photon:loaded as loaded

ENV PATH="${PATH}:/go/bin"

RUN wget https://go.dev/dl/go1.19.linux-amd64.tar.gz && \
    tar xvf go1.19.linux-amd64.tar.gz

RUN rm go1.19.linux-amd64.tar.gz && \
    export PATH=$PATH:/go/bin
