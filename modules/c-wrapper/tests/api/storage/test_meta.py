"""
Tests all the meta data functions
"""
import ctypes
from unittest import TestCase, main

from test_utils.c_lib_loader import load_library
from test_utils.return_structs import EmptyReturn, FileInfo
from test_utils.routes import TEST_SURML_PATH


class TestMeta(TestCase):

    def setUp(self) -> None:
        self.lib = load_library()
        self.lib.add_name.restype = EmptyReturn

        # Define the signatues of the basic meta functions
        self.functions = [
            self.lib.add_name,
            self.lib.add_description,
            self.lib.add_version,
            self.lib.add_column,
            self.lib.add_author,
            self.lib.add_origin,
            self.lib.add_engine,
        ]
        for i in self.functions:
            i.argtypes = [ctypes.c_char_p, ctypes.c_char_p]
            i.restype = EmptyReturn

        # Define the load model signature
        self.lib.load_model.restype = FileInfo
        self.lib.load_model.argtypes = [ctypes.c_char_p]
        self.lib.free_file_info.argtypes = [FileInfo]
        self.model: FileInfo = self.lib.load_model(str(TEST_SURML_PATH).encode('utf-8'))
        self.file_id = self.model.file_id.decode('utf-8')

    def tearDown(self) -> None:
        self.lib.free_file_info(self.model)

    def test_null_protection(self):
        placeholder = "placeholder".encode('utf-8')
        file_id = self.file_id.encode('utf-8')

        # check that they all protect against file ID null pointers
        for i in self.functions:
            outcome: EmptyReturn = i(None, placeholder)
            self.assertEqual(1, outcome.is_error)
            self.assertEqual(
                "Received a null pointer for file id",
                outcome.error_message.decode('utf-8')
            )

        # check that they all protect against null pointers for the field type
        outcomes = [
            "model name",
            "description",
            "version",
            "column name",
            "author",
            "origin",
            "engine",
        ]
        counter = 0
        for i in self.functions:
            outcome: EmptyReturn = i(file_id, None)
            self.assertEqual(1, outcome.is_error)
            self.assertEqual(
                f"Received a null pointer for {outcomes[counter]}",
                outcome.error_message.decode('utf-8')
            )
            counter += 1

    def test_model_not_found(self):
        placeholder = "placeholder".encode('utf-8')

        # check they all return errors if not found
        for i in self.functions:
            outcome: EmptyReturn = i(placeholder, placeholder)
            self.assertEqual(1, outcome.is_error)
            self.assertEqual("Model not found", outcome.error_message.decode('utf-8'))


if __name__ == '__main__':
    main()
