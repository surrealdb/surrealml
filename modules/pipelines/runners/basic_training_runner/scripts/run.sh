#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

cargo build --release

if [ -d run_env ]; then
    rm -rf run_env
fi

mkdir run_env

cp ../../target/release/basic_training_runner ./run_env/basic_training_runner
cp assets/listener.py ./run_env/listener.py
cp assets/test.jpg ./run_env/test.jpg

cd run_env
./basic_training_runner | python listener.py
