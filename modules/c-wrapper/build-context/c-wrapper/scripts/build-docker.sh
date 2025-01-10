#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

BUILD_DIR="build-context"

if [ -d "$BUILD_DIR" ]; then
    echo "Cleaning up existing build directory..."
    rm -rf "$BUILD_DIR"
fi

mkdir "$BUILD_DIR"
mkdir "$BUILD_DIR"/c-wrapper
cp -r src "$BUILD_DIR"/c-wrapper/src
cp -r tests "$BUILD_DIR"/c-wrapper/tests
cp -r scripts "$BUILD_DIR"/c-wrapper/scripts
cp Cargo.toml "$BUILD_DIR"/c-wrapper/Cargo.toml

cp -r ../core "$BUILD_DIR"/core

rm -rf "$BUILD_DIR"/core/.git
rm -rf "$BUILD_DIR"/core/target/

cp Dockerfile "$BUILD_DIR"/Dockerfile
cd "$BUILD_DIR"
docker build --no-cache -t c-wrapper-tests .

docker run c-wrapper-tests

# docker run -it c-wrapper-tests /bin/bash