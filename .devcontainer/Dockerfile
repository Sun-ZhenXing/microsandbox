ARG VERSION=1.0.25
ARG VARIANT=bookworm

FROM mcr.microsoft.com/devcontainers/rust:${VERSION}-${VARIANT}
ARG DEBIAN_FRONTEND=noninteractive
WORKDIR /workspaces

# Install additional dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    patchelf bc libelf-dev flex bison python3-pip
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*

USER vscode
RUN pip3 install pyelftools --break-system-packages
