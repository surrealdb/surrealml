# Use an official Rust image
FROM rust:1.88-slim

# Install necessary tools
RUN apt-get update && apt-get install -y \
    wget \
    build-essential \
    libssl-dev \
    pkg-config \
    ca-certificates \
    curl \
    gnupg \
    lsb-release \
    vim \
    && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y python3 python3-pip

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# # Set the ONNX Runtime library path
# ENV ORT_LIB_LOCATION=/onnxruntime/lib
# ENV LD_LIBRARY_PATH=$ORT_LIB_LOCATION:$LD_LIBRARY_PATH

# Clean and build the Rust project
RUN cargo clean
RUN cargo build

# Run the tests
CMD ["cargo", "test"]
