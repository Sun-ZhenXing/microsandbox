ARG BASE_IMAGE=ubuntu:24.04

FROM ${BASE_IMAGE}
ARG DEBIAN_FRONTEND=noninteractive
ARG BIN_DIR="$HOME/.local/bin"
ARG PULL_PYTHON=0
ARG PULL_NODE=0

RUN apt update && \
    apt install -y --no-install-recommends \
    ca-certificates \
    curl
RUN apt clean && \
    rm -rf /var/lib/apt/lists/*

COPY ms* ${BIN_DIR}/
COPY *.so.* ${BIN_DIR}/

RUN cd ${BIN_DIR} && \
    ln -sf libkrun.so.* libkrun.so && \
    ln -sf libkrunfw.so.* libkrunfw.so

ENV PATH="$BIN_DIR:$PATH"

RUN if [ "$PULL_PYTHON" -eq 1 ]; then \
    msb pull microsandbox/python; \
    fi
RUN if [ "$PULL_NODE" -eq 1 ]; then \
    msb pull microsandbox/node; \
    fi

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 \
    CMD curl -f "http://localhost:5555/api/v1/health" || exit 1

EXPOSE 5555
CMD ["msb", "server", "start", "--host", "0.0.0.0", "--port", "5555"]
