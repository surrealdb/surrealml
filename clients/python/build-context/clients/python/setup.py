import os
import platform
import shutil
import subprocess
from pathlib import Path

from setuptools import setup


def get_c_lib_name() -> str:
    system = platform.system()
    if system == "Linux":
        return "libc_wrapper.so"
    elif system == "Darwin":  # macOS
        return "libc_wrapper.dylib"
    elif system == "Windows":
        return "libc_wrapper.dll"
    raise ValueError(f"Unsupported system: {system}")

# define the paths to the C wrapper and root
DIR_PATH = Path(__file__).parent
ROOT_PATH = DIR_PATH.joinpath("..").joinpath("..")
C_PATH = ROOT_PATH.joinpath("modules").joinpath("c-wrapper")
BINARY_PATH = C_PATH.joinpath("target").joinpath("release").joinpath(get_c_lib_name())
BINARY_DIST = DIR_PATH.joinpath("surrealml").joinpath(get_c_lib_name())

build_flag = False

# build the C lib and copy it over to the python lib
if BINARY_DIST.exists() is False:
    subprocess.Popen("cargo build --release", cwd=str(C_PATH), shell=True).wait()
    shutil.copyfile(BINARY_PATH, BINARY_DIST)
    build_flag = True

setup(
    name="surrealml",
    version="0.1.0",
    description="A machine learning package for interfacing with various frameworks.",
    author="Maxwell Flitton",
    author_email="maxwellflitton@gmail.com",
    url="https://github.com/surrealdb/surrealml",
    license="MIT",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.6",
    install_requires=[
        "numpy==1.26.3",
    ],
    extras_require={
        "sklearn": [
            "skl2onnx==1.16.0",
            "scikit-learn==1.4.0",
        ],
        "torch": [
            "torch==2.1.2",
        ],
        "tensorflow": [
            "tf2onnx==1.16.1",
            "tensorflow==2.16.1",
        ],
    },
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
        "surrealml": ["libc_wrapper.so", "libc_wrapper.dylib", "libc_wrapper.dll"]
    },
    include_package_data=True,
    zip_safe=False,
)

# cleanup after install
if build_flag is True:
    os.remove(BINARY_DIST)
