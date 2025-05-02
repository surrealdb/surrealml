# Use an official Rust image
FROM rust:1.83-slim

# Install necessary tools
RUN apt-get update && apt-get install -y \
    wget \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Download ONNX Runtime 1.20.0
RUN wget https://github.com/microsoft/onnxruntime/releases/download/v1.20.0/onnxruntime-linux-x64-1.20.0.tgz \
    && tar -xvf onnxruntime-linux-x64-1.20.0.tgz \
    && mv onnxruntime-linux-x64-1.20.0 /onnxruntime

# # Download ONNX Runtime 1.16.0
# RUN wget https://github.com/microsoft/onnxruntime/releases/download/v1.16.0/onnxruntime-linux-x64-1.16.0.tgz \
#     && tar -xvf onnxruntime-linux-x64-1.16.0.tgz \
#     && mv onnxruntime-linux-x64-1.16.0 /onnxruntime

# Set the ONNX Runtime library path
ENV ORT_LIB_LOCATION=/onnxruntime/lib
ENV LD_LIBRARY_PATH=$ORT_LIB_LOCATION:$LD_LIBRARY_PATH

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Clean and build the Rust project
RUN cargo clean
RUN cargo build -p surrealml-core --features tensorflow-tests

# Run the tests
CMD ["cargo", "test", "--features", "tensorflow-tests"]
