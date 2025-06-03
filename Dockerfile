FROM debian:bookworm-slim

# Basic tooling
RUN apt-get update && apt-get install -y \
    curl build-essential pkg-config libssl-dev \
    python3 python3-pip python3-venv \
    git ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install just (latest release binary)
# ENV JUST_VERSION=1.25.2
# RUN curl -L -o /usr/local/bin/just https://github.com/casey/just/releases/download/${JUST_VERSION}/just-x86_64-unknown-linux-musl && \
#     chmod +x /usr/local/bin/just

RUN cargo install just

# Create a workspace
WORKDIR /workspace

# Entrypoint for interactive use
CMD ["/bin/bash"]
