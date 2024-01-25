#!/usr/bin/env python
from setuptools import setup
from setuptools_rust import Binding, RustExtension


with open("README.md", "r") as fh:
    long_description = fh.read()


setup(
    name="surrealml",
    author="Maxwell Flitton",
    author_email="maxwell@gmail.com",
    description="SurrealMl is a file format for storing machine learning models across python (all versions) and rust.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    version="0.0.1",
    rust_extensions=[RustExtension("surrealml.rust_surrealml", binding=Binding.PyO3)],
    packages=[
        "surrealml",
        "surrealml.engine",
        "surrealml.model_templates"
    ],
    package_data={
        "surrealml": ["binaries/*"],
    },
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
    include_package_data=True,
    requirements=[
        "numpy==1.26.3",
    ],
    extras_require={
        "sklearn": [
            "skl2onnx==1.16.0",
            "scikit-learn==1.4.0"
        ],
        "torch": [
            "torch==2.1.2"
        ]
    }
)
