"""
Defines the functionality of caching and processing a sklearn model.
"""
import os
import shutil
import uuid
import zipfile

import torch
from hummingbird.ml import convert


class SkLearnModelCache:
    """
    This class is responsible for caching and converting an sklearn model to a torchscript model.
    """

    @staticmethod
    def create_file_cache():
        """
        Creates a file cache for the model.

        :return: the path to the cache created with a unique id to prevent collisions.
        """
        cache_folder = '.surmlcache'

        if not os.path.exists(cache_folder):
            os.makedirs(cache_folder)
        unique_id = str(uuid.uuid4())
        file_name = f"{unique_id}.surml"
        return os.path.join(cache_folder, file_name)

    # @staticmethod
    # def cache_model(model, inputs, name=None):
    #     """
    #     Caches a model and returns the file id.
    #
    #     :param model:
    #     :param inputs:
    #     :param name:
    #     :return:
    #     """
    #     file_path = SkLearnModelCache.create_file_cache()
    #
    #     traced_script_module = torch.jit.trace(model, inputs)
    #     traced_script_module.save(file_path)
    #     file_id = load_cached_raw_model(str(file_path))
    #     os.remove(file_path)
    #     if name is not None:
    #         add_name(file_id, name)
    #     return file_id
    
    @staticmethod
    def convert_sklearn_model(model, inputs):
        """
        Converts the sklearn model to a torchscript model.

        :param model: the sklearn model to convert.
        :param inputs: the inputs to the model needed to trace the model
        :return: the converted model.
        """
        file_path = SkLearnModelCache.create_file_cache()
        model = convert(model, 'torch.jit', inputs)
        file_path = str(file_path).replace(".surml", "")
        model.save(file_path)
        zip_path = str(file_path) + ".zip"

        # Open the zip archive
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            # Extract all the contents to the specified directory
            zip_ref.extractall(file_path)

        model = torch.jit.load(os.path.join(file_path, "deploy_model.zip"))
        shutil.rmtree(file_path)
        os.remove(zip_path)
        return model
