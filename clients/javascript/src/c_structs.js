const ref = require("ref-napi");
const Struct = require("ref-struct-napi");

// Define basic types
const CharPtr = ref.types.CString;
const Int = ref.types.int;
const SizeT = ref.types.size_t;
const FloatPtr = ref.refType(ref.types.float);
const BytePtr = ref.refType(ref.types.byte);

/**
 * A return type that just returns a string
 */
const StringReturn = Struct({
    string: CharPtr,       // Corresponds to *mut c_char
    is_error: Int,         // Corresponds to c_int
    error_message: CharPtr // Corresponds to *mut c_char
});

/**
 * A return type that just returns nothing
 */
const EmptyReturn = Struct({
    is_error: Int,         // Corresponds to c_int
    error_message: CharPtr // Corresponds to *mut c_char
});

/**
 * A return type when loading the meta of a surml file.
 */
const FileInfo = Struct({
    file_id: CharPtr,       // Corresponds to *mut c_char
    name: CharPtr,          // Corresponds to *mut c_char
    description: CharPtr,   // Corresponds to *mut c_char
    version: CharPtr,       // Corresponds to *mut c_char
    error_message: CharPtr, // Corresponds to *mut c_char
    is_error: Int           // Corresponds to c_int
});

/**
 * A return type when loading the meta of a surml vector.
 */
const Vecf32Return = Struct({
    data: FloatPtr,         // Pointer to f32 array
    length: SizeT,          // Length of the array
    capacity: SizeT,        // Capacity of the array
    is_error: Int,          // Indicates if it's an error
    error_message: CharPtr  // Optional error message
});

/**
 * A return type returning bytes.
 */
const VecU8Return = Struct({
    data: BytePtr,          // Pointer to bytes
    length: SizeT,          // Length of the array
    capacity: SizeT,        // Capacity of the array
    is_error: Int,          // Indicates if it's an error
    error_message: CharPtr
});

// Export the structs
module.exports = {
    StringReturn,
    EmptyReturn,
    FileInfo,
    Vecf32Return,
    VecU8Return
};
