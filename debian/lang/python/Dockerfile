FROM scsys/debian:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y

FROM builder-base as builder

ENV ENV_MODE=dev \
    PYTHONUNBUFFERED=1 \
    POETRY_VIRTUALENVS_CREATE=false

RUN apt-get install -y \
    python3.9 \
    python3.9-dev \
    python3-pip \
    python3-venv

RUN pip3 install pipx

RUN curl -sSL https://install.python-poetry.org | POETRY_HOME=/opt/poetry python3 -
ENV PATH="/opt/poetry/bin:$PATH"

FROM builder as latest

RUN python3 --version && \
    poetry --version && \
    poetry config virtualenvs.create false
