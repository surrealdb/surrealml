import ctypes
from pathlib import Path
import platform
from unittest import TestCase, main

class FileInfo(ctypes.Structure):
    _fields_ = [
        ("file_id", ctypes.c_char_p),
        ("name", ctypes.c_char_p),
        ("description", ctypes.c_char_p),
        ("version", ctypes.c_char_p),
        ("error_message", ctypes.c_char_p),  # Optional error message
    ]

class Vecf32Return(ctypes.Structure):
    _fields_ = [
        ("data", ctypes.POINTER(ctypes.c_float)),  # Pointer to f32 array
        ("length", ctypes.c_size_t),              # Length of the array
        ("capacity", ctypes.c_size_t),            # Capacity of the array
        ("is_error", ctypes.c_int),               # Indicates if it's an error
        ("error_message", ctypes.c_char_p),       # Optional error message
    ]


def load_library(lib_name: str) -> ctypes.CDLL:
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


class TestExecution(TestCase):

    def setUp(self) -> None:
        self.lib = load_library("libc_wrapper")

        # Define the Rust function signatures
        self.lib.load_model.argtypes = [ctypes.c_char_p]
        self.lib.load_model.restype = FileInfo

        self.lib.free_file_info.argtypes = [FileInfo]

        self.lib.raw_compute.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_float), ctypes.c_size_t]
        self.lib.raw_compute.restype = Vecf32Return

        self.lib.free_vecf32_return.argtypes = [Vecf32Return]

    def test_raw_compute(self):
        # Load a test model
        current_dir = Path(__file__).parent.joinpath("assets").joinpath("test.surml")
        c_string = str(current_dir).encode('utf-8')
        file_info = self.lib.load_model(c_string)

        if file_info.error_message:
            self.fail(f"Failed to load model: {file_info.error_message.decode('utf-8')}")

        # Prepare input data as a ctypes array
        input_data = (ctypes.c_float * 2)(1.0, 2.0)

        # Call the raw_compute function
        result = self.lib.raw_compute(file_info.file_id, input_data, len(input_data))

        if result.is_error:
            self.fail(f"Error in raw_compute: {result.error_message.decode('utf-8')}")

        # Extract and verify the computation result
        outcome = [result.data[i] for i in range(result.length)]
        print(f"Computation Result: {outcome}")

        # Free allocated memory
        self.lib.free_vecf32_return(result)
        self.lib.free_file_info(file_info)


if __name__ == '__main__':
    main()
