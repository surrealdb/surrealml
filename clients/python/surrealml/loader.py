"""
The loader for the dynamic C lib written in Rust.
"""
import ctypes
import platform
from pathlib import Path

from surrealml.c_structs import EmptyReturn, StringReturn, Vecf32Return, FileInfo, VecU8Return


class Singleton(type):
    """
    Ensures that the loader only loads once throughout the program's lifetime
    """
    _instances = {}

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


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


class LibLoader(metaclass=Singleton):

    def __init__(self, lib_name: str = "libc_wrapper") -> None:
        """
        The constructor for the LibLoader class.

        args:
            lib_name (str): The base name of the library without extension (e.g., "libc_wrapper").
        """
        self.lib = load_library(lib_name=lib_name)
        functions = [
            self.lib.add_name,
            self.lib.add_description,
            self.lib.add_version,
            self.lib.add_column,
            self.lib.add_author,
            self.lib.add_origin,
            self.lib.add_engine,
        ]
        for i in functions:
            i.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
            i.restype = EmptyReturn
        self.lib.load_model.restype = FileInfo
        self.lib.load_model.argtypes = [ctypes.c_char_p]
        self.lib.load_cached_raw_model.restype = StringReturn
        self.lib.load_cached_raw_model.argtypes = [ctypes.c_char_p]
        self.lib.to_bytes.argtypes = [ctypes.c_char_p]
        self.lib.to_bytes.restype = VecU8Return
        self.lib.save_model.restype = EmptyReturn
        self.lib.save_model.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
        self.lib.upload_model.argtypes = [
            ctypes.c_char_p,
            ctypes.c_char_p,
            ctypes.c_size_t,
            ctypes.c_char_p,
            ctypes.c_char_p,
            ctypes.c_char_p,
            ctypes.c_char_p,
        ]
        self.lib.upload_model.restype = EmptyReturn

        # define the compute functions
        self.lib.raw_compute.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_float), ctypes.c_size_t]
        self.lib.raw_compute.restype = Vecf32Return
        self.lib.buffered_compute.argtypes = [
            ctypes.c_char_p,  # file_id_ptr -> *const c_char
            ctypes.POINTER(ctypes.c_float),  # data_ptr -> *const c_float
            ctypes.c_size_t,  # data_length -> usize
            ctypes.POINTER(ctypes.c_char_p),  # strings -> *const *const c_char
            ctypes.c_int  # string_count -> c_int
        ]
        self.lib.buffered_compute.restype = Vecf32Return

        # Define free alloc functions
        self.lib.free_string_return.argtypes = [StringReturn]
        self.lib.free_empty_return.argtypes = [EmptyReturn]
        self.lib.free_vec_u8.argtypes = [VecU8Return]
        self.lib.free_vecf32_return.argtypes = [Vecf32Return]
        self.lib.free_file_info.argtypes = [FileInfo]



