//! Error handling.
//!
//! These are a group of definitions related to error handling.
//!
//! All functions in the C layer will return a result, which will
//! correspond to one of these codes. To ensure binary stability, the
//! return type is separate from the error code, and is a fixed size.

use libc::c_char;

/// Error codes that may be returned by various functions.
#[repr(C)]
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum exr_error_code_t {
    EXR_ERR_SUCCESS = 0,
    EXR_ERR_OUT_OF_MEMORY,
    EXR_ERR_MISSING_CONTEXT_ARG,
    EXR_ERR_INVALID_ARGUMENT,
    EXR_ERR_ARGUMENT_OUT_OF_RANGE,
    EXR_ERR_FILE_ACCESS,
    EXR_ERR_FILE_BAD_HEADER,
    EXR_ERR_NOT_OPEN_READ,
    EXR_ERR_NOT_OPEN_WRITE,
    EXR_ERR_HEADER_NOT_WRITTEN,
    EXR_ERR_READ_IO,
    EXR_ERR_WRITE_IO,
    EXR_ERR_NAME_TOO_LONG,
    EXR_ERR_MISSING_REQ_ATTR,
    EXR_ERR_INVALID_ATTR,
    EXR_ERR_NO_ATTR_BY_NAME,
    EXR_ERR_ATTR_TYPE_MISMATCH,
    EXR_ERR_ATTR_SIZE_MISMATCH,
    EXR_ERR_SCAN_TILE_MIXEDAPI,
    EXR_ERR_TILE_SCAN_MIXEDAPI,
    EXR_ERR_MODIFY_SIZE_CHANGE,
    EXR_ERR_ALREADY_WROTE_ATTRS,
    EXR_ERR_BAD_CHUNK_LEADER,
    EXR_ERR_CORRUPT_CHUNK,
    EXR_ERR_INCORRECT_PART,
    EXR_ERR_INCORRECT_CHUNK,
    EXR_ERR_USE_SCAN_DEEP_WRITE,
    EXR_ERR_USE_TILE_DEEP_WRITE,
    EXR_ERR_USE_SCAN_NONDEEP_WRITE,
    EXR_ERR_USE_TILE_NONDEEP_WRITE,
    EXR_ERR_INVALID_SAMPLE_DATA,
    EXR_ERR_FEATURE_NOT_IMPLEMENTED,
    EXR_ERR_UNKNOWN
}

#[must_use = "the result must be checked for success"]
pub type exr_result_t = i32;

extern "C" {
    /// Return a static string corresponding to the specified error code.
    ///
    /// The string should not be freed (it is compiled into the binary).
    pub fn exr_get_default_error_message(code: exr_result_t) -> *const c_char;

    /// Return a static string corresponding to the specified error code.
    ///
    /// The string should not be freed (it is compiled into the binary).
    pub fn exr_get_error_code_as_string(code: exr_result_t) -> *const c_char;
}