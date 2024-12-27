from typing import Optional
import ctypes
import warnings
import platform
from pathlib import Path
from surrealml.c_structs import EmptyReturn, StringReturn, Vecf32Return, FileInfo
from surrealml.loader import LibLoader
from typing import List, Tuple

from surrealml.engine import Engine


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


class RustAdapter:

    def __init__(self, file_id: str, engine: Engine) -> None:
        self.file_id: str = file_id
        self.engine: Engine = engine
        self.loader = LibLoader()

    @staticmethod
    def pass_raw_model_into_rust(file_path: str) -> str:
        """
        Points to a raw ONNX file and passes it into the rust library so it can be loaded
        and tagged with a unique id so the Rust library can reference this model again
        from within the rust library.

        :param file_path: the path to the raw ONNX file.

        :return: the unique id of the model.
        """
        c_path = file_path.encode("utf-8")
        loader = LibLoader()
        outcome: StringReturn = loader.lib.load_cached_raw_model(c_path)
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        file_path = outcome.string.decode("utf-8")
        loader.lib.free_string_return(outcome)
        return file_path

    def add_column(self, name: str) -> None:
        """
        Adds a column to the model to the metadata (this needs to be called in order of the columns).

        :param name: the name of the column.
        :return: None
        """
        outcome: EmptyReturn = self.loader.lib.add_column(
            self.file_id.encode("utf-8"),
            name.encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        self.loader.lib.free_empty_return(outcome)

    def add_output(self, output_name: str, normaliser_type: str, one: float, two: float) -> None:
        """
        Adds an output to the model to the metadata.
        :param output_name: the name of the output.
        :param normaliser_type: the type of normaliser to use.
        :param one: the first parameter of the normaliser.
        :param two: the second parameter of the normaliser.
        :return: None
        """
        outcome: EmptyReturn = self.loader.lib.add_output(
            self.file_id.encode("utf-8"),
            output_name.encode("utf-8"),
            normaliser_type.encode("utf-8"),
            str(one).encode("utf-8"),
            str(two).encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        self.loader.lib.free_empty_return(outcome)

    def add_description(self, description: str) -> None:
        """
        Adds a description to the model to the metadata.

        :param description: the description of the model.
        :return: None
        """
        outcome: EmptyReturn = self.loader.lib.add_description(
            self.file_id.encode("utf-8"),
            description.encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        self.loader.lib.free_empty_return(outcome)

    def add_version(self, version: str) -> None:
        """
        Adds a version to the model to the metadata.

        :param version: the version of the model.
        :return: None
        """
        outcome: EmptyReturn = self.loader.lib.add_version(
            self.file_id.encode("utf-8"),
            version.encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        self.loader.lib.free_empty_return(outcome)

    def add_name(self, name: str) -> None:
        """
        Adds a name to the model to the metadata.

        :param name: the version of the model.
        :return: None
        """
        outcome: EmptyReturn = self.loader.lib.add_name(
            self.file_id.encode("utf-8"),
            name.encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        self.loader.lib.free_empty_return(outcome)

    def add_normaliser(self, column_name, normaliser_type, one, two) -> None:
        """
        Adds a normaliser to the model to the metadata for a column.

        :param column_name: the name of the column (column already needs to be in the metadata to create mapping)
        :param normaliser_type: the type of normaliser to use.
        :param one: the first parameter of the normaliser.
        :param two: the second parameter of the normaliser.
        :return: None
        """
        outcome: EmptyReturn = self.loader.lib.add_normaliser(
            self.file_id.encode("utf-8"),
            column_name.encode("utf-8"),
            normaliser_type.encode("utf-8"),
            str(one).encode("utf-8"),
            str(two).encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        self.loader.lib.free_empty_return(outcome)

    def add_author(self, author: str) -> None:
        """
        Adds an author to the model to the metadata.

        :param author: the author of the model.
        :return: None
        """
        # add_author(self.file_id, author)
        pass

    def save(self, path: str, name: Optional[str]) -> None:
        """
        Saves the model to a file.

        :param path: the path to save the model to.
        :param name: the name of the model.

        :return: None
        """
        pass
        # add_engine(self.file_id, self.engine.value)
        # add_origin(self.file_id, "local")
        # if name is not None:
        #     add_name(self.file_id, name)
        # else:
        #     warnings.warn(
        #         "You are saving a model without a name, you will not be able to upload this model to the database"
        #     )
        # save_model(path, self.file_id)

    def to_bytes(self):
        """
        Converts the model to bytes.

        :return: the model as bytes.
        """
        pass
        # return to_bytes(self.file_id)

    @staticmethod
    def load(path) -> Tuple[str, str, str, str]:
        """
        Loads a model from a file.

        :param path: the path to load the model from.
        :return: the id of the model being loaded.
        """
        loader = LibLoader()
        outcome: FileInfo = loader.lib.load_model(
            path.encode("utf-8"),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        package = (
            outcome.file_id.decode("utf-8"),
            outcome.name.decode("utf-8"),
            outcome.description.decode("utf-8"),
            outcome.version.decode("utf-8"),
        )
        loader.lib.free_file_info(outcome)
        return package

    @staticmethod
    def upload(
            path: str,
            url: str,
            chunk_size: int,
            namespace: str,
            database: str,
            username: Optional[str] = None,
            password: Optional[str] = None
    ) -> None:
        """
        Uploads a model to a remote server.

        :param path: the path to load the model from.
        :param url: the url of the remote server.
        :param chunk_size: the size of each chunk to upload.
        :param namespace: the namespace of the remote server.
        :param database: the database of the remote server.
        :param username: the username of the remote server.
        :param password: the password of the remote server.

        :return: None
        """
        pass
        # upload_model(
        #     path,
        #     url,
        #     chunk_size,
        #     namespace,
        #     database,
        #     username,
        #     password
        # )

    def raw_compute(self, input_vector, dims=None) -> List[float]:
        """
        Calculates an output from the model given an input vector.

        :param input_vector: a 1D vector of inputs to the model.
        :param dims: the dimensions of the input vector to be sliced into
        :return: the output of the model.
        """
        array_type = ctypes.c_float * len(input_vector)
        input_data = array_type(*input_vector)
        outcome: Vecf32Return = self.loader.lib.raw_compute(
            self.file_id.encode("utf-8"),
            input_data,
            len(input_data),
        )
        if outcome.is_error == 1:
            raise RuntimeError(outcome.error_message.decode("utf-8"))
        package = [outcome.data[i] for i in range(outcome.length)]
        self.loader.lib.free_vecf32_return(outcome)
        return package

        # return raw_compute(self.file_id, input_vector, dims)

    def buffered_compute(self, value_map):
        """
        Calculates an output from the model given a value map.

        :param value_map: a dictionary of inputs to the model with the column names as keys and floats as values.
        :return: the output of the model.
        """
        pass
        # return buffered_compute(self.file_id, value_map)
