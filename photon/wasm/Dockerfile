FROM scsys/photon:loaded

RUN git clone https://github.com/wasmerio/wasmer.git && \
    cd wasmer && \
    make build-wasmer

RUN cp wasmer/target/release/wasmer bin/wasmer && \
    rm wasmer -f -r
