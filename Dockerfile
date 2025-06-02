ARG BASE_IMAGE=ubuntu:24.04

FROM ${BASE_IMAGE}
ARG DEBIAN_FRONTEND=noninteractive
ARG BIN_DIR="$HOME/.local/bin"

RUN apt update && \
    apt install -y --no-install-recommends \
    libdigest-sha-perl \
    ca-certificates \
    curl
RUN apt clean && \
    rm -rf /var/lib/apt/lists/*

RUN curl -sSL https://get.microsandbox.dev | sh

ENV PATH="$BIN_DIR:$PATH"

RUN msb pull microsandbox/python
RUN msb pull microsandbox/node

EXPOSE 5555
CMD ["msb", "server", "start", "--host", "0.0.0.0", "--port", "5555"]
