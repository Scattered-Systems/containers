FROM nixos/nix as builder-base

RUN nix-channel --update && nix-channel --upgrade

FROM builder-base as builder

RUN nix-env -qaP \
    curl \
    git \
    wget
