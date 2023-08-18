import torch
import os
import uuid

from surrealml.rust_surrealml import load_cached_raw_model, add_column, add_output, add_normaliser, save_model, add_name, load_model, add_description, add_version
from surrealml.rust_surrealml import raw_compute, buffered_compute

class SurMlFile:

    def __init__(self, model=None, name=None, inputs=None):
        self.model = model
        self.name = name
        self.inputs = inputs
        if self.model is not None:
            self.file_id = self._cache_model()
        else:
            self.file_id = None

    def _cache_model(self):
        cache_folder = '.surmlcache'

        if not os.path.exists(cache_folder):
            os.makedirs(cache_folder)

        unique_id = str(uuid.uuid4())
        file_name = f"{unique_id}.surml"
        file_path = os.path.join(cache_folder, file_name)

        traced_script_module = torch.jit.trace(self.model, self.inputs)
        traced_script_module.save(file_path)
        file_id = load_cached_raw_model(str(file_path))
        os.remove(file_path)
        if self.name is not None:
            add_name(file_id, self.name)
        return file_id

    def add_column(self, name):
        add_column(self.file_id, name)

    def add_output(self, output_name, normaliser_type, one, two):
        add_output(self.file_id, output_name, normaliser_type, one, two)

    def add_description(self, description):
        add_description(self.file_id, description)

    def add_version(self, version):
        add_version(self.file_id, version)

    def add_normaliser(self, column_name, normaliser_type, one, two):
        add_normaliser(self.file_id, column_name, normaliser_type, one, two)

    def save(self, path):
        save_model(path, self.file_id)

    @staticmethod
    def load(path):
        self = SurMlFile()
        self.file_id = load_model(path)
        return self

    def raw_compute(self, input_vector, dims=None):
        return raw_compute(self.file_id, input_vector, dims)

    def buffered_compute(self, value_map):
        return buffered_compute(self.file_id, value_map)
