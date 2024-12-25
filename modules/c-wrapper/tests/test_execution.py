import ctypes
import platform
from pathlib import Path
from unittest import TestCase, main
from test_utils.c_lib_loader import load_library
from test_utils.routes import TEST_SURML_PATH


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


class TestExecution(TestCase):

    def setUp(self) -> None:
        self.lib = load_library()

        # Define the Rust function signatures
        self.lib.load_model.argtypes = [ctypes.c_char_p]
        self.lib.load_model.restype = FileInfo

        self.lib.free_file_info.argtypes = [FileInfo]

        self.lib.raw_compute.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_float), ctypes.c_size_t]
        self.lib.raw_compute.restype = Vecf32Return

        self.lib.free_vecf32_return.argtypes = [Vecf32Return]

    def test_raw_compute(self):
        # Load a test model
        c_string = str(TEST_SURML_PATH).encode('utf-8')
        file_info = self.lib.load_model(c_string)

        if file_info.error_message:
            self.fail(f"Failed to load model: {file_info.error_message.decode('utf-8')}")

        # Prepare input data as a ctypes array
        input_data = (ctypes.c_float * 2)(1.0, 4.0)

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
