import shutil
import subprocess
from pathlib import Path
import sys
import os
import urllib.request
import tarfile
import zipfile
import platform

from setuptools import setup

# ===================================== Define the paths for the install =================================================

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
# path of the root of this github repo
ROOT_PATH = DIR_PATH.joinpath("..").joinpath("..")
# path to where the c-wrapper code for the surrealML core is
C_PATH = ROOT_PATH.joinpath("modules").joinpath("c-wrapper")
# path to where the binary is after building the c-wrapper
BINARY_PATH = C_PATH.joinpath("target").joinpath("release").joinpath(get_c_lib_name())
# path to where the c-wrapper is stored if held in the surrealML python package
BINARY_DIST = DIR_PATH.joinpath("surrealml").joinpath(get_c_lib_name())
# path to where the onnxruntime is stored if held in the surrealML python package
ONNXRUNTIME_DIR = DIR_PATH.joinpath("surrealml").joinpath("onnxruntime")

# get information about the system
ONNX_VERSION = "1.20.0"
PYTHON_PACKAGE_VERSION = "1.20.0"
DYNAMIC_LIB_VERSION = "1.7"
OS_NAME = sys.platform
ARCH = platform.machine().lower()
SYSTEM = platform.system().lower()  # 'linux', 'darwin' (macOS), 'windows'

if OS_NAME.startswith("linux"):
    ONNX_LIB_NAME = "libonnxruntime.so"
elif OS_NAME == "darwin":
    ONNX_LIB_NAME = "libonnxruntime.dylib"
elif OS_NAME == "win32":
    ONNX_LIB_NAME = "libonnxruntime.dll"

ROOT_DEP_DIR = os.path.expanduser("~/surrealml_deps")

# Ensure the base directory exists
os.makedirs(ROOT_DEP_DIR, exist_ok=True)

# Paths for versioned libraries
ONNX_LIB_DIR = os.path.join(ROOT_DEP_DIR, "onnxruntime", ONNX_VERSION)
DYNAMIC_LIB_DIR = os.path.join(ROOT_DEP_DIR, "core_ml_lib", DYNAMIC_LIB_VERSION)
ONNX_DOWNLOAD_CACHE = os.path.join(ONNX_LIB_DIR, "download_cache.tgz")
DYNAMIC_LIB_DIST = DIR_PATH.joinpath(DYNAMIC_LIB_DIR).joinpath(get_c_lib_name())
DYNAMIC_LIB_DOWNLOAD_CACHE = os.path.join(DYNAMIC_LIB_DIR, "download_cache.tgz")

# create the directories if they don't exist
os.makedirs(ONNX_LIB_DIR, exist_ok=True)
os.makedirs(DYNAMIC_LIB_DIR, exist_ok=True)

# ===================================== download the onnxruntime into the dep directory =========================================================

def get_onnxruntime_url():
    base_url = f"https://github.com/microsoft/onnxruntime/releases/download/v{ONNX_VERSION}/"

    if OS_NAME.startswith("linux"):
        if ARCH == "x86_64":
            return f"{base_url}onnxruntime-linux-x64-{ONNX_VERSION}.tgz", f"onnxruntime-linux-x64-{ONNX_VERSION}"
        elif ARCH in ("arm64", "aarch64"):
            return f"{base_url}onnxruntime-linux-aarch64-{ONNX_VERSION}.tgz", f"onnxruntime-linux-aarch64-{ONNX_VERSION}"
    
    elif OS_NAME == "darwin":
        if ARCH == "x86_64":
            return f"{base_url}onnxruntime-osx-x86_64-{ONNX_VERSION}.tgz", f"onnxruntime-osx-x86_64-{ONNX_VERSION}"
        elif ARCH == "arm64":
            return f"{base_url}onnxruntime-osx-arm64-{ONNX_VERSION}.tgz", f"onnxruntime-osx-arm64-{ONNX_VERSION}"

    elif OS_NAME == "win32":
        if ARCH == "x86_64":
            return f"{base_url}onnxruntime-win-x64-{ONNX_VERSION}.zip", f"onnxruntime-win-x64-{ONNX_VERSION}"
        elif ARCH == "arm64":
            return f"{base_url}onnxruntime-win-arm64-{ONNX_VERSION}.zip", f"onnxruntime-win-arm64-{ONNX_VERSION}"
    
    raise Exception(f"Unsupported platform or architecture: {OS_NAME}")


def get_lib_url():
    if OS_NAME.startswith("linux"):
        if ARCH == "x86_64":
            return f"https://github.com/maxwellflitton/test-deploy/releases/download/v{DYNAMIC_LIB_VERSION}/surrealml-v{DYNAMIC_LIB_VERSION}-x86_64-unknown-linux-gnu.tar.gz", f"surrealml-v{DYNAMIC_LIB_VERSION}-x86_64-unknown-linux-gnu.tar.gz"
        elif ARCH in ("arm64", "aarch64"):
            return f"https://github.com/maxwellflitton/test-deploy/releases/download/v{DYNAMIC_LIB_VERSION}/surrealml-v{DYNAMIC_LIB_VERSION}-arm64-unknown-linux-gnu.tar.gz", f"surrealml-v{DYNAMIC_LIB_VERSION}-arm64-unknown-linux-gnu.tar.gz"

    elif OS_NAME == "darwin":
        if ARCH == "x86_64":
            return f"https://github.com/maxwellflitton/test-deploy/releases/download/v{DYNAMIC_LIB_VERSION}/surrealml-v{DYNAMIC_LIB_VERSION}-x86_64-apple-darwin.tar.gz", f"surrealml-v{DYNAMIC_LIB_VERSION}-x86_64-apple-darwin.tar.gz"
        elif ARCH == "arm64":
            return f"https://github.com/maxwellflitton/test-deploy/releases/download/v{DYNAMIC_LIB_VERSION}/surrealml-v{DYNAMIC_LIB_VERSION}-arm64-apple-darwin.tar.gz", f"surrealml-v{DYNAMIC_LIB_VERSION}-arm64-apple-darwin.tar.gz"

    elif OS_NAME == "win32":
        if ARCH == "x86_64":
            return f"https://github.com/maxwellflitton/test-deploy/releases/download/v{DYNAMIC_LIB_VERSION}/surrealml-v{DYNAMIC_LIB_VERSION}-x86_64-pc-windows-msvc.tar.gz", f"surrealml-v{DYNAMIC_LIB_VERSION}-x86_64-pc-windows-msvc.tar.gz"
        elif ARCH == "arm64":
            pass

    raise Exception(f"Unsupported platform or architecture: {OS_NAME}")


def download_and_extract_onnxruntime():
    url, extracted_dir = get_onnxruntime_url()

    if not os.path.exists(ONNX_DOWNLOAD_CACHE):
        print(f"Downloading ONNX Runtime from {url}")
        urllib.request.urlretrieve(url, ONNX_DOWNLOAD_CACHE)
    else:
        print(f"the {ONNX_DOWNLOAD_CACHE} alread exists so not downloading onnxruntime")

    # Extract based on file type
    if ONNX_DOWNLOAD_CACHE.endswith(".tgz"):
        with tarfile.open(ONNX_DOWNLOAD_CACHE, "r:gz") as tar:
            tar.extractall(path=ONNX_LIB_DIR)
    elif ONNX_DOWNLOAD_CACHE.endswith(".zip"):
        with zipfile.ZipFile(ONNX_DOWNLOAD_CACHE, "r") as zip_ref:
            zip_ref.extractall(ONNX_LIB_DIR)

    return extracted_dir

def download_and_extract_c_lib():
    url, extracted_dir = get_lib_url()
    if not os.path.exists(DYNAMIC_LIB_DOWNLOAD_CACHE):
        print(f"Downloading surrealML lib from {url}")
        urllib.request.urlretrieve(url, DYNAMIC_LIB_DOWNLOAD_CACHE)
    else:
        print(f"the {DYNAMIC_LIB_DOWNLOAD_CACHE} already exists so not downloading surrealML lib")

    if DYNAMIC_LIB_DOWNLOAD_CACHE.endswith(".tgz"):
        with tarfile.open(DYNAMIC_LIB_DOWNLOAD_CACHE, "r:gz") as tar:
            tar.extractall(path=DYNAMIC_LIB_DIR)
    elif DYNAMIC_LIB_DOWNLOAD_CACHE.endswith(".zip"):
        with zipfile.ZipFile(DYNAMIC_LIB_DOWNLOAD_CACHE, "r") as zip_file:
            zip_file.extractall(path=DYNAMIC_LIB_DIR)
    return extracted_dir



onnx_lib_dist = Path(ONNX_LIB_DIR).joinpath(ONNX_LIB_NAME)

# downloads and unpacks the onnx lib if the onnx lib is not present
if os.path.exists(onnx_lib_dist) is False:
    print("downloading the path")
    onnxruntime_path = download_and_extract_onnxruntime()
    onnx_lib_path = Path(ONNX_LIB_DIR).joinpath(onnxruntime_path).joinpath("lib").joinpath(ONNX_LIB_NAME)
    shutil.copyfile(onnx_lib_path, onnx_lib_dist)
    shutil.rmtree(Path(ONNX_LIB_DIR).joinpath(onnxruntime_path))
    os.remove(ONNX_DOWNLOAD_CACHE)

# ===================================== Build the rust binary into a dynamic C lib =======================================

BUILD_FLAG = False

# build the C lib and copy it over to the python lib
if DYNAMIC_LIB_DIST.exists() is False and os.environ.get("LOCAL_BUILD") == "TRUE":
    print("building core ML lib locally")
    subprocess.Popen("cargo build --release", cwd=str(C_PATH), shell=True).wait()
    shutil.copyfile(BINARY_PATH, DYNAMIC_LIB_DIST)
    BUILD_FLAG = True

else:
    if os.path.exists(DYNAMIC_LIB_DIST) is False:
        print("downloading the core ML lib")
        lib_path = download_and_extract_c_lib()
        os.remove(DYNAMIC_LIB_DOWNLOAD_CACHE)
        # lib_path = Path(DYNAMIC_LIB_DIST).joinpath(lib_path).joinpath(get_c_lib_name())
        # shutil.copyfile(lib_path, DYNAMIC_LIB_DIST)
        # DYNAMIC_LIB_DIST
        # DYNAMIC_LIB_DOWNLOAD_CACHE

# ===================================== run the setup for the python package =============================================

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
)
