#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

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
DEST_DIR="tests"


# Copy the library to the tests directory
if [ -f "$SOURCE_DIR/$LIB_NAME" ]; then
  cp "$SOURCE_DIR/$LIB_NAME" "$DEST_DIR/"
  echo "Copied $LIB_NAME to $DEST_DIR"
else
  echo "Library not found: $SOURCE_DIR/$LIB_NAME"
  exit 1
fi