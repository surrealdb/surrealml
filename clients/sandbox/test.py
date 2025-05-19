#!/usr/bin/env python3
import sys
from pathlib import Path

from surrealml import SurMlFile, Engine


def main():
    new_file = SurMlFile.load(path="./linear.surml", engine=Engine.SKLEARN)

    # Make a prediction (both should be the same due to the perfectly correlated example data)
    print(new_file.buffered_compute(value_map={"squarefoot": 5, "num_floors": 6}))
    print(new_file.raw_compute(input_vector=[5, 6]))

if __name__ == "__main__":
    main()
