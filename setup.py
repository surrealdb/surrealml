#!/usr/bin/env python
import os

from setuptools import setup
from setuptools_rust import Binding, RustExtension


with open("README.md", "r") as fh:
    long_description = fh.read()

# setting the environment variable will get the Rust build to use the python torch installation to prevent clashes
os.environ["LIBTORCH_USE_PYTORCH"] = "1"

setup(
    name="surrealml",
    author="Maxwell Flitton",
    author_email="maxwell@gmail.com",
    description="SurrealMl is a file format for storing machine learning models across python (all versions) and rust.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    version="0.0.1",
    rust_extensions=[RustExtension("surrealml.rust_surrealml", binding=Binding.PyO3, features=["python"])],
    packages=[
        "surrealml",
        # "surrealdb.execution_mixins"
    ],
    package_data={
        "surrealml": ["binaries/*"],
    },
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    include_package_data=True,
    requirements=[
        "pyyaml>=3.13",
        "numpy",
        "torch>=2.0.0"
    ]
)
