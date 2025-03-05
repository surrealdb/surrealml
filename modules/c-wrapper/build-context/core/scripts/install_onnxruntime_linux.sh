#!/bin/bash

# Variables
ONNX_VERSION="1.20.0"
ONNX_DOWNLOAD_URL="https://github.com/microsoft/onnxruntime/releases/download/v${ONNX_VERSION}/onnxruntime-linux-x64-${ONNX_VERSION}.tgz"
ONNX_RUNTIME_DIR="/home/maxwellflitton/Documents/github/surreal/surrealml/modules/core/target/debug/build/ort-680c63907dcb00d8/out/onnxruntime"
ONNX_TARGET_DIR="${ONNX_RUNTIME_DIR}/onnxruntime-linux-x64-${ONNX_VERSION}"
LD_LIBRARY_PATH_UPDATE="${ONNX_TARGET_DIR}/lib"

# Step 1: Download and Extract ONNX Runtime
echo "Downloading ONNX Runtime version ${ONNX_VERSION}..."
wget -q --show-progress "${ONNX_DOWNLOAD_URL}" -O "onnxruntime-linux-x64-${ONNX_VERSION}.tgz"

if [ $? -ne 0 ]; then
    echo "Failed to download ONNX Runtime. Exiting."
    exit 1
fi

echo "Extracting ONNX Runtime..."
tar -xvf "onnxruntime-linux-x64-${ONNX_VERSION}.tgz"

if [ ! -d "onnxruntime-linux-x64-${ONNX_VERSION}" ]; then
    echo "Extraction failed. Directory not found. Exiting."
    exit 1
fi

# Step 2: Replace Old ONNX Runtime
echo "Replacing old ONNX Runtime..."
mkdir -p "${ONNX_RUNTIME_DIR}"
mv "onnxruntime-linux-x64-${ONNX_VERSION}" "${ONNX_TARGET_DIR}"

if [ ! -d "${ONNX_TARGET_DIR}" ]; then
    echo "Failed to move ONNX Runtime to target directory. Exiting."
    exit 1
fi

# Step 3: Update LD_LIBRARY_PATH
echo "Updating LD_LIBRARY_PATH..."
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH_UPDATE}:$LD_LIBRARY_PATH"

# Step 4: Verify Library Version
echo "Verifying ONNX Runtime version..."
strings "${LD_LIBRARY_PATH_UPDATE}/libonnxruntime.so" | grep "VERS_${ONNX_VERSION}" > /dev/null

if [ $? -ne 0 ]; then
    echo "ONNX Runtime version ${ONNX_VERSION} not found in library. Exiting."
    exit 1
fi

# Step 5: Install Library Globally (Optional)
echo "Installing ONNX Runtime globally..."
sudo cp "${LD_LIBRARY_PATH_UPDATE}/libonnxruntime.so" /usr/local/lib/
sudo ldconfig

if [ $? -ne 0 ]; then
    echo "Failed to install ONNX Runtime globally. Exiting."
    exit 1
fi

# Step 6: Clean and Rebuild Project
echo "Cleaning and rebuilding project..."
cargo clean
cargo test --features tensorflow-tests

if [ $? -eq 0 ]; then
    echo "ONNX Runtime updated successfully, and tests passed."
else
    echo "ONNX Runtime updated, but tests failed. Check the logs for details."
fi
