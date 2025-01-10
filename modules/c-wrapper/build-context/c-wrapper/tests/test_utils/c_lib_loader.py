import ctypes
import platform
from pathlib import Path


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

    if system_name == "Windows":
        lib_path = current_dir.joinpath(f"{lib_name}.dll")
    elif system_name == "Darwin":  # macOS
        lib_path = current_dir.joinpath(f"{lib_name}.dylib")
    elif system_name == "Linux":
        lib_path = current_dir.joinpath(f"{lib_name}.so")
    else:
        raise OSError(f"Unsupported operating system: {system_name}")

    if not lib_path.exists():
        raise FileNotFoundError(f"Shared library not found at: {lib_path}")

    return ctypes.CDLL(str(lib_path))
