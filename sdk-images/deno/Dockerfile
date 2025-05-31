# First stage: Build the portal binary
FROM rust:slim AS builder

# Install dependencies and build the portal binary
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/* \
    && mkdir -p /usr/src/microsandbox

WORKDIR /usr/src/microsandbox
COPY . .
# Build with deno feature enabled
RUN cargo build --release --bin portal --features deno

# Second stage: Create the Deno image
FROM denoland/deno:latest

# Set environment variables
ENV DENO_DIR=/home/deno/.cache/deno \
    DENO_INSTALL_ROOT=/home/deno/.deno

ARG USER_NAME="deno"
ARG USER_UID="1000"
ARG USER_GID="100"

# Install system dependencies, create directories and set permissions in one layer
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    curl \
    ca-certificates \
    bash \
    && apt-get clean && rm -rf /var/lib/apt/lists/* \
    # Create necessary directories and set proper ownership
    && mkdir -p /home/$USER_NAME/work \
    && mkdir -p /etc/microsandbox/portal \
    && mkdir -p $DENO_DIR \
    && mkdir -p $DENO_INSTALL_ROOT \
    && chown -R $USER_UID:$USER_GID /home/$USER_NAME \
    && chown -R $USER_UID:$USER_GID /etc/microsandbox \
    && chown -R $USER_UID:$USER_GID $DENO_DIR \
    && chown -R $USER_UID:$USER_GID $DENO_INSTALL_ROOT

# Copy the portal binary from the builder stage and set permissions
COPY --from=builder /usr/src/microsandbox/target/release/portal /usr/local/bin/
RUN chmod +x /usr/local/bin/portal

# Switch to the deno user
USER $USER_NAME
WORKDIR /home/$USER_NAME/work

# Pre-cache commonly used Deno modules for better performance
RUN deno cache --reload \
    https://deno.land/std@0.224.0/assert/mod.ts \
    https://deno.land/std@0.224.0/http/mod.ts \
    https://deno.land/std@0.224.0/fmt/colors.ts \
    https://deno.land/std@0.224.0/testing/asserts.ts

# Set a command that starts portal and keeps the container running
CMD ["bash", "-c", "echo 'Deno environment with microsandbox-portal ready' && RUST_LOG=debug portal & tail -f /dev/null"]
