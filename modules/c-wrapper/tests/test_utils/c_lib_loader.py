import ctypes
import platform
from pathlib import Path
import os


def load_library(lib_name: str = "libc_wrapper") -> ctypes.CDLL:
    """
    Load the correct shared library based on the operating system.

    Args:
        lib_name (str): The base name of the library without extension (e.g., "libc_wrapper").

    Returns:
        ctypes.CDLL: The loaded shared library.
    """
    current_dir = Path(__file__).parent
    system_name = platform.system()

    # os.environ["ORT_LIB_LOCATION"] = str(current_dir.joinpath("onnxruntime.dll"))

    if system_name == "Windows":
        lib_path = current_dir.joinpath(f"{lib_name}.dll")
        onnx_path = current_dir.joinpath("onnxruntime").joinpath("lib").joinpath("onnxruntime.dll")
    elif system_name == "Darwin":  # macOS
        lib_path = current_dir.joinpath(f"{lib_name}.dylib")
        onnx_path = current_dir.joinpath("onnxruntime").joinpath("lib").joinpath("onnxruntime.dylib")
    elif system_name == "Linux":
        lib_path = current_dir.joinpath(f"{lib_name}.so")
        onnx_path = current_dir.joinpath("onnxruntime").joinpath("lib").joinpath("onnxruntime.so.1")
    else:
        raise OSError(f"Unsupported operating system: {system_name}")
    

    # onnx_lib_path = current_dir.joinpath("onnxruntime").joinpath("lib")
    # current_ld_library_path = os.environ.get("LD_LIBRARY_PATH", "")
    # # Update LD_LIBRARY_PATH
    # os.environ["LD_LIBRARY_PATH"] = f"{onnx_lib_path}:{current_ld_library_path}"
    # os.environ["ORT_LIB_LOCATION"] = str(onnx_lib_path)

    ctypes.CDLL(str(onnx_path), mode=ctypes.RTLD_GLOBAL)

    if not lib_path.exists():
        raise FileNotFoundError(f"Shared library not found at: {lib_path}")

    return ctypes.CDLL(str(lib_path))
