FROM nixos/nix as builder-base

RUN nix-channel --update

FROM builder-base as builder

RUN nix-env -qaP \
    curl \
    git \
    openssl \
    wget
