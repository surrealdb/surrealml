"""
Defines the SurMlFile class which is used to save/load models and perform computations based on those models.
"""
import os
import uuid

import torch
from surrealml.rust_surrealml import load_cached_raw_model, add_column, add_output, add_normaliser, save_model, \
    add_name, load_model, add_description, add_version, to_bytes, add_engine, add_author, add_origin
from surrealml.rust_surrealml import raw_compute, buffered_compute

from surrealml.model_cache import SkLearnModelCache
from surrealml.engine_enum import Engine


class SurMlFile:

    def __init__(self, model=None, name=None, inputs=None, sklearn=False):
        """
        The constructor for the SurMlFile class.

        :param model: the model to be saved.
        :param name: the name of the model.
        :param inputs: the inputs to the model needed to trace the model so the model can be saved.
        :param sklearn: whether the model is an sklearn model or not.
        """
        self.model = model
        self.name = name
        self.inputs = inputs
        self.sklearn = sklearn
        if self.model is not None:
            if sklearn is True:
                self.model = SkLearnModelCache.convert_sklearn_model(model=self.model, inputs=self.inputs)
            self.file_id = self._cache_model()
        else:
            self.file_id = None

    def _cache_model(self):
        """
        Caches a model, so it can be loaded as raw bytes to be fused with the header.

        :return: the file id of the model so it can be retrieved from the cache.
        """
        cache_folder = '.surmlcache'

        if not os.path.exists(cache_folder):
            os.makedirs(cache_folder)

        unique_id = str(uuid.uuid4())
        file_name = f"{unique_id}.surml"
        file_path = os.path.join(cache_folder, file_name)

        if self.sklearn is True:
            traced_script_module = self.model
        else:
            traced_script_module = torch.jit.trace(self.model, self.inputs)
        traced_script_module.save(file_path)
        file_id = load_cached_raw_model(str(file_path))
        os.remove(file_path)
        if self.name is not None:
            add_name(file_id, self.name)
        return file_id

    def add_column(self, name):
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
        add_engine(self.file_id, Engine.PYTORCH.value)
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
        self = SurMlFile()
        self.file_id = load_model(path)
        return self

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
