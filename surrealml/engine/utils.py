import os
import uuid


def create_file_cache_path():
    """
    Creates a file cache path for the model (creating the file cache if not there).

    :return: the path to the cache created with a unique id to prevent collisions.
    """
    cache_folder = '.surmlcache'

    if not os.path.exists(cache_folder):
        os.makedirs(cache_folder)
    unique_id = str(uuid.uuid4())
    file_name = f"{unique_id}.surml"
    return os.path.join(cache_folder, file_name)
