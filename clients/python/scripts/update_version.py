#!/usr/bin/env python3
import json
import sys
import argparse
from pathlib import Path
import re


SEMVER_RE = re.compile(r'^\d+\.\d+\.\d+$')
config_json_path = Path(__file__).parent.parent.joinpath("config.json")

def semver_type(value: str) -> str:
    if not SEMVER_RE.match(value):
        raise argparse.ArgumentTypeError(
            f"Invalid version '{value}': must be in format X.Y.Z"
        )
    return value

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("new_version",  type=semver_type)
    args = parser.parse_args()
    return args.new_version

def load_config():
    try: 
        with open(config_json_path, "r") as json_data:
            config = json.load(json_data)
            return config
    except Exception as e:
        print(f"Error loading file from '{config_json_path}': {e}", file=sys.stderr)
        sys.exit(1)

def write_config(new_config: dict):
    try:
        with open(config_json_path, "w") as json_data:
            json.dump(new_config, json_data, indent=4)
            json_data.write("\n")
    except Exception as e:
        print(f"Error writing file to '{config_json_path}': {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    new_version = parse_args()
    config = load_config()
    config["version"] = new_version
    write_config(config)
