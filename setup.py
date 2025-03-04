#!/usr/bin/env python
from setuptools import setup
from setuptools_rust import Binding, RustExtension
import sys
import os
import urllib.request
import tarfile
import zipfile
import platform


with open("README.md", "r") as fh:
    long_description = fh.read()


def get_onnxruntime_url():
    onnxruntime_version = "1.20.0"
    base_url = f"https://github.com/microsoft/onnxruntime/releases/download/v{onnxruntime_version}/"
    
    os_name = sys.platform
    arch = platform.machine().lower()

    if os_name.startswith("linux"):
        if arch == "x86_64":
            return f"{base_url}onnxruntime-linux-x64-{onnxruntime_version}.tgz", f"onnxruntime-linux-x64-{onnxruntime_version}"
        elif arch in ("arm64", "aarch64"):
            return f"{base_url}onnxruntime-linux-aarch64-{onnxruntime_version}.tgz", f"onnxruntime-linux-aarch64-{onnxruntime_version}"
    
    elif os_name == "darwin":
        if arch == "x86_64":
            return f"{base_url}onnxruntime-osx-x86_64-{onnxruntime_version}.tgz", f"onnxruntime-osx-x86_64-{onnxruntime_version}"
        elif arch == "arm64":
            return f"{base_url}onnxruntime-osx-arm64-{onnxruntime_version}.tgz", f"onnxruntime-osx-arm64-{onnxruntime_version}"

    elif os_name == "win32":
        if arch == "x86_64":
            return f"{base_url}onnxruntime-win-x64-{onnxruntime_version}.zip", f"onnxruntime-win-x64-{onnxruntime_version}"
        elif arch == "arm64":
            return f"{base_url}onnxruntime-win-arm64-{onnxruntime_version}.zip", f"onnxruntime-win-arm64-{onnxruntime_version}"
    
    raise Exception(f"Unsupported platform or architecture: {os_name}")

def download_and_extract_onnxruntime():
    url, extracted_dir = get_onnxruntime_url()

    os.makedirs("onnxruntime", exist_ok=True)
    file_path = os.path.join("onnxruntime", os.path.basename(url))

    if not os.path.exists(file_path):
        print(f"Downloading ONNX Runtime from {url}")
        urllib.request.urlretrieve(url, file_path)

    # Extract based on file type
    if file_path.endswith(".tgz"):
        with tarfile.open(file_path, "r:gz") as tar:
            tar.extractall(path="onnxruntime")
    elif file_path.endswith(".zip"):
        with zipfile.ZipFile(file_path, "r") as zip_ref:
            zip_ref.extractall("onnxruntime")

    return extracted_dir

onnxruntime_path = download_and_extract_onnxruntime()


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
        "surrealml.model_templates",
        "surrealml.model_templates.datasets",
        "surrealml.model_templates.sklearn",
        "surrealml.model_templates.torch",
        "surrealml.model_templates.tensorflow",
    ],
    package_data={
        "surrealml": [
            "binaries/*",
            f"../c_libs/*",
            f"../onnxruntime/{onnxruntime_path}/*",
        ],
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
        ],
        "tensorflow": [
            "tf2onnx==1.16.1",
            "tensorflow==2.16.1"
        ]
    }
)
