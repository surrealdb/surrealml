#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

# download onnxruntime
# Detect operating system
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

# Detect architecture
ARCH=$(uname -m)

# Download the correct onnxruntime
if [ "$ARCH" == "x86_64" ] && [ "$OS" == "linux" ]; then
  wget https://github.com/microsoft/onnxruntime/releases/download/v1.20.0/onnxruntime-linux-x64-1.20.0.tgz
  tar -xvf onnxruntime-linux-x64-1.20.0.tgz
  mv onnxruntime-linux-x64-1.20.0 tests/test_utils/onnxruntime
else
  echo "Unsupported operating system and arch: $OS $ARCH"
  exit 1
fi

export ORT_LIB_LOCATION=$(pwd)/tests/test_utils/onnxruntime/lib
export LD_LIBRARY_PATH=$ORT_LIB_LOCATION:$LD_LIBRARY_PATH

cargo build

# Get the operating system
OS=$(uname)

# Set the library name and extension based on the OS
case "$OS" in
  "Linux")
    LIB_NAME="libc_wrapper.so"
    ;;
  "Darwin")
    LIB_NAME="libc_wrapper.dylib"
    ;;
  "CYGWIN"*|"MINGW"*)
    LIB_NAME="libc_wrapper.dll"
    ;;
  *)
    echo "Unsupported operating system: $OS"
    exit 1
    ;;
esac

# Source directory (where Cargo outputs the compiled library)
SOURCE_DIR="target/debug"

# Destination directory (tests directory)
DEST_DIR="tests/test_utils"


# Copy the library to the tests directory
if [ -f "$SOURCE_DIR/$LIB_NAME" ]; then
  cp "$SOURCE_DIR/$LIB_NAME" "$DEST_DIR/"
  echo "Copied $LIB_NAME to $DEST_DIR"
else
  echo "Library not found: $SOURCE_DIR/$LIB_NAME"
  exit 1
fi
