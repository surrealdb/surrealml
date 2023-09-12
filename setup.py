#!/usr/bin/env python
import os
import sys


from setuptools import setup
from setuptools_rust import Binding, RustExtension
import torch

torch_path = str(os.path.dirname(torch.__file__))


with open("README.md", "r") as fh:
    long_description = fh.read()

# setting the environment variable will get the Rust build to use the python torch installation to prevent clashes
os.environ["LIBTORCH_USE_PYTORCH"] = "1"
os.environ["LIBTORCH_CXX11_ABI"] = "0"
site_packages_lib_path = os.path.join(torch_path, "lib")

os.environ["LIBTORCH"] = torch_path
os.environ["DYLD_LIBRARY_PATH"] = str(site_packages_lib_path)

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
        "torch==2.0.0",
        "hummingbird-ml==0.4.9"
    ]
)
