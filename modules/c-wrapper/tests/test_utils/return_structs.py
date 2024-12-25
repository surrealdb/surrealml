"""
Defines all the C structs that are used in the tests.
"""
from ctypes import Structure, c_char_p, c_int


class StringReturn(Structure):
    """
    A return type that just returns a string

    Fields:
        string: the string that is being returned (only present if successful)
        is_error: 1 if error, 0 if not
        error_message: the error message (only present if error)
    """
    _fields_ = [
        ("string", c_char_p),         # Corresponds to *mut c_char
        ("is_error", c_int),          # Corresponds to c_int
        ("error_message", c_char_p)   # Corresponds to *mut c_char
    ]

class EmptyReturn(Structure):
    """
    A return type that just returns nothing

    Fields:
        is_error: 1 if error, 0 if not
        error_message: the error message (only present if error)
    """
    _fields_ = [
        ("is_error", c_int),          # Corresponds to c_int
        ("error_message", c_char_p)   # Corresponds to *mut c_char
    ]


class FileInfo(Structure):
    """
    A return type when loading the meta of a surml file.

    Fields:
        file_id: a unique identifier for the file in the state of the C lib
        name: a name of the model
        description: a description of the model
        error_message: the error message (only present if error)
        is_error: 1 if error, 0 if not
    """
    _fields_ = [
        ("file_id", c_char_p),        # Corresponds to *mut c_char
        ("name", c_char_p),           # Corresponds to *mut c_char
        ("description", c_char_p),    # Corresponds to *mut c_char
        ("version", c_char_p),        # Corresponds to *mut c_char
        ("error_message", c_char_p),  # Corresponds to *mut c_char
        ("is_error", c_int)           # Corresponds to c_int
    ]
