#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..

# if venv does not exist, create it
if [ ! -d "venv" ]; then
  python3 -m venv venv
  source venv/bin/activate
  pip install -r requirements.txt
  deactivate
fi

# remove assets directory if it exists
if [ -d "assets" ]; then
  rm -rf assets
fi

# create assets directory
mkdir assets

# activate venv
source venv/bin/activate

python scripts/build_dummy_model.py
