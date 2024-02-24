#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

if [ -d run_env ]; then
    rm -rf run_env
fi

python3 -m venv run_env
source run_env/bin/activate

pip install ../../data_access
python src/main.py