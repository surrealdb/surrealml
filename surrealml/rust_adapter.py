try:
    from surrealml.rust_surrealml import load_cached_raw_model, add_column, add_output, add_normaliser, save_model, \
        add_name, load_model, add_description, add_version, to_bytes, add_engine, add_author, add_origin
    from surrealml.rust_surrealml import raw_compute, buffered_compute, upload_model
except ImportError:
    load_cached_raw_model = None
    add_column = None
    add_output = None
    add_normaliser = None
    save_model = None
    add_name = None
    load_model = None
    add_description = None
    add_version = None
    to_bytes = None
    add_engine = None
    add_author = None
    add_origin = None
    raw_compute = None
    buffered_compute = None
    upload_model = None

from typing import Optional

from surrealml.engine import Engine


class RustAdapter:

    def __init__(self, file_id: str, engine: Engine) -> None:
        self.file_id: str = file_id
        self.engine: Engine = engine

    @staticmethod
    def pass_raw_model_into_rust(file_path: str) -> str:
        """
        Points to a raw ONNX file and passes it into the rust library so it can be loaded
        and tagged with a unique id so the Rust library can reference this model again
        from within the rust library.

        :param file_path: the path to the raw ONNX file.

        :return: the unique id of the model.
        """
        return load_cached_raw_model(file_path)

    def add_column(self, name: str) -> None:
        """
        Adds a column to the model to the metadata (this needs to be called in order of the columns).

        :param name: the name of the column.
        :return: None
        """
        add_column(self.file_id, name)

    def add_output(self, output_name, normaliser_type, one, two):
        """
        Adds an output to the model to the metadata.
        :param output_name: the name of the output.
        :param normaliser_type: the type of normaliser to use.
        :param one: the first parameter of the normaliser.
        :param two: the second parameter of the normaliser.
        :return: None
        """
        add_output(self.file_id, output_name, normaliser_type, one, two)

    def add_description(self, description):
        """
        Adds a description to the model to the metadata.

        :param description: the description of the model.
        :return: None
        """
        add_description(self.file_id, description)

    def add_version(self, version):
        """
        Adds a version to the model to the metadata.

        :param version: the version of the model.
        :return: None
        """
        add_version(self.file_id, version)

    def add_normaliser(self, column_name, normaliser_type, one, two):
        """
        Adds a normaliser to the model to the metadata for a column.

        :param column_name: the name of the column (column already needs to be in the metadata to create mapping)
        :param normaliser_type: the type of normaliser to use.
        :param one: the first parameter of the normaliser.
        :param two: the second parameter of the normaliser.
        :return: None
        """
        add_normaliser(self.file_id, column_name, normaliser_type, one, two)

    def add_author(self, author):
        """
        Adds an author to the model to the metadata.

        :param author: the author of the model.
        :return: None
        """
        add_author(self.file_id, author)

    def save(self, path):
        """
        Saves the model to a file.

        :param path: the path to save the model to.
        :return: None
        """
        # right now the only engine is pytorch so we can hardcode it but when we add more engines we will need to
        # add a parameter to the save function to specify the engine
        add_engine(self.file_id, self.engine.value)
        add_origin(self.file_id, "local")
        save_model(path, self.file_id)

    def to_bytes(self):
        """
        Converts the model to bytes.

        :return: the model as bytes.
        """
        return to_bytes(self.file_id)

    @staticmethod
    def load(path):
        """
        Loads a model from a file.

        :param path: the path to load the model from.
        :return:
        """
        return load_model(path)

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
        upload_model(
            path,
            url,
            chunk_size,
            namespace,
            database,
            username,
            password
        )

    def raw_compute(self, input_vector, dims=None):
        """
        Calculates an output from the model given an input vector.

        :param input_vector: a 1D vector of inputs to the model.
        :param dims: the dimensions of the input vector to be sliced into
        :return: the output of the model.
        """
        return raw_compute(self.file_id, input_vector, dims)

    def buffered_compute(self, value_map):
        """
        Calculates an output from the model given a value map.

        :param value_map: a dictionary of inputs to the model with the column names as keys and floats as values.
        :return: the output of the model.
        """
        return buffered_compute(self.file_id, value_map)
