# Use an official Rust image
FROM rust:1.83-slim

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

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Download ONNX Runtime 1.20.0
# RUN wget https://github.com/microsoft/onnxruntime/releases/download/v1.20.0/onnxruntime-linux-x64-1.20.0.tgz \
#     && tar -xvf onnxruntime-linux-x64-1.20.0.tgz \
#     && mv onnxruntime-linux-x64-1.20.0 /onnxruntime

# Set the ONNX Runtime library path
# ENV ORT_LIB_LOCATION=/onnxruntime/lib
# ENV LD_LIBRARY_PATH=$ORT_LIB_LOCATION:$LD_LIBRARY_PATH

# Set the ONNX Runtime library path
# ENV ORT_LIB_LOCATION=$(pwd)/c-wrapper/tests/test_utils/onnxruntime/lib
# ENV LD_LIBRARY_PATH=$ORT_LIB_LOCATION:$LD_LIBRARY_PATH

# install python for the tests
RUN apt-get update && apt-get install -y python3 python3-pip

# Clean and build the Rust project
# RUN cd c-wrapper/scripts && bash prep_tests.sh
# RUN cd c-wrapper && cargo build --verbose > build_log.txt 2>&1
RUN cd c-wrapper && cargo build && bash scripts/copy_over_lib.sh

# RUN rm /onnxruntime

# Run the tests
CMD ["bash", "c-wrapper/scripts/run_tests.sh"]
