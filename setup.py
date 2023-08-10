#!/usr/bin/env python
import pathlib

from setuptools import setup
from setuptools_rust import Binding, RustExtension


with open("README.md", "r") as fh:
    long_description = fh.read()


with open(str(pathlib.Path(__file__).parent.absolute()) + "/surrealml/VERSION.txt", "r") as fh:
    version = fh.read().split("=")[1].replace("'", "")


setup(
    name="surrealml",
    author="Maxwell Flitton",
    author_email="maxwell@gmail.com",
    description="SurrealMl is a file format for storing machine learning models across python (all versions) and rust.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    version=version,
    rust_extensions=[RustExtension("surrealml.rust_surrealml", binding=Binding.PyO3)],
    packages=[
        "surrealml",
        # "surrealdb.execution_mixins"
    ],
    package_data={
        "surrealml": ["binaries/*"],
    },
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    include_package_data=True
)
