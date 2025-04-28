import ctypes
import platform
from pathlib import Path
import os
from test_utils.return_structs import EmptyReturn


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

    # ctypes.CDLL(str(onnx_path), mode=ctypes.RTLD_GLOBAL)
    onnx_path = current_dir.joinpath("onnxruntime")

    if not lib_path.exists():
        raise FileNotFoundError(f"Shared library not found at: {lib_path}")
    
    loaded_lib = ctypes.CDLL(str(lib_path))
    loaded_lib.link_onnx.argtypes = []
    loaded_lib.link_onnx.restype = EmptyReturn
    c_string = str(onnx_path).encode('utf-8')
    load_info = loaded_lib.link_onnx()
    if load_info.error_message:
        raise OSError(f"Failed to load onnxruntime: {load_info.error_message.decode('utf-8')}")

    return ctypes.CDLL(str(lib_path))
