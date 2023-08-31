"""
This file contains the Engine enum, which is used to specify the engine to use for a given model.
"""
from enum import Enum


class Engine(Enum):
    """
    The Engine enum is used to specify the engine to use for a given model.

    Attributes:
        PYTORCH: The PyTorch engine which will be PyTorch and tch-rs.
        NATIVE: The native engine which will be native rust and linfa.
        UNDEFINED: The undefined engine which will be used when the engine is not defined.
    """
    PYTORCH = "pytorch"
    NATIVE = "native"
    UNDEFINED = ""
