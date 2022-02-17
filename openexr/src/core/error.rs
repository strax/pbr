use std::backtrace::Backtrace;
use std::ffi::CStr;
use std::{fmt, io};
use std::fmt::{Debug, Display, Formatter};
use strum::FromRepr;

use crate::sys::*;

use exr_error_code_t::*;

#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, FromRepr)]
pub enum ErrorCode {
    OutOfMemory = EXR_ERR_OUT_OF_MEMORY as i32,
    MissingContextArg = EXR_ERR_MISSING_CONTEXT_ARG as i32,
    InvalidArgument = EXR_ERR_INVALID_ARGUMENT as i32,
    ArgumentOutOfRange = EXR_ERR_ARGUMENT_OUT_OF_RANGE as i32,
    FileAccess = EXR_ERR_FILE_ACCESS as i32,
    BadFileHeader = EXR_ERR_FILE_BAD_HEADER as i32,
    NotOpenRead = EXR_ERR_NOT_OPEN_READ as i32,
    NotOpenWrite = EXR_ERR_NOT_OPEN_WRITE as i32,
    HeaderNotWritten = EXR_ERR_HEADER_NOT_WRITTEN as i32,
    ReadIo = EXR_ERR_READ_IO as i32,
    WriteIo = EXR_ERR_WRITE_IO as i32,
    NameTooLong = EXR_ERR_NAME_TOO_LONG as i32,
    MissingRequiredAttr = EXR_ERR_MISSING_REQ_ATTR as i32,
    InvalidAttr = EXR_ERR_INVALID_ATTR as i32,
    NoAttrByName = EXR_ERR_NO_ATTR_BY_NAME as i32,
    AttrTypeMismatch = EXR_ERR_ATTR_TYPE_MISMATCH as i32,
    AttrSizeMismatch = EXR_ERR_ATTR_SIZE_MISMATCH as i32,
    ScanTileMixedApi = EXR_ERR_SCAN_TILE_MIXEDAPI as i32,
    TileScanMixedApi = EXR_ERR_TILE_SCAN_MIXEDAPI as i32,
    ModifySizeChange = EXR_ERR_MODIFY_SIZE_CHANGE as i32,
    AlreadyWroteAttrs = EXR_ERR_ALREADY_WROTE_ATTRS as i32,
    BadChunkLeader = EXR_ERR_BAD_CHUNK_LEADER as i32,
    CorruptChunk = EXR_ERR_CORRUPT_CHUNK as i32,
    IncorrectPart = EXR_ERR_INCORRECT_PART as i32,
    IncorrectChunk = EXR_ERR_INCORRECT_CHUNK as i32,
    UseScanDeepWrite = EXR_ERR_USE_SCAN_DEEP_WRITE as i32,
    UseTileDeepWrite = EXR_ERR_USE_TILE_DEEP_WRITE as i32,
    UseScanNonDeepWrite = EXR_ERR_USE_SCAN_NONDEEP_WRITE as i32,
    UseTileNonDeepWrite = EXR_ERR_USE_TILE_NONDEEP_WRITE as i32,
    InvalidSampleData = EXR_ERR_INVALID_SAMPLE_DATA as i32,
    FeatureNotImplemented = EXR_ERR_FEATURE_NOT_IMPLEMENTED as i32,
    Unknown = EXR_ERR_UNKNOWN as i32
}

impl ErrorCode {
    pub(crate) fn to_str(&self) -> &'static str {
        unsafe {
            // SAFETY: the default error messages are hardcoded to OpenEXRCore/base.c and are ASCII strings
            std::str::from_utf8_unchecked(
                CStr::from_ptr(exr_get_default_error_message(*self as exr_result_t)).to_bytes()
            )
        }
    }
}

pub struct Error {
    repr: Repr
}

impl Error {
    pub(crate) const fn from_extern(code: exr_result_t) -> Result<()> {
        if code == (EXR_ERR_SUCCESS as i32) {
            return Ok(())
        }
        Err(Error { repr: Repr::ErrorCode(ErrorCode::from_repr(code).unwrap_or(ErrorCode::Unknown)) })
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::ConstMessage(message) => f.debug_struct("Error").field("message", *message).finish(),
            Repr::ErrorCode(code) => f.debug_struct("Error").field("code", &code).finish(),
            Repr::Io(err) => fmt::Debug::fmt(err, f),
            Repr::Other(err) => fmt::Debug::fmt(err, f)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::ConstMessage(message) => f.write_str(*message),
            Repr::ErrorCode(code) => f.write_str(code.to_str()),
            Repr::Other(err) => fmt::Display::fmt(err, f),
            Repr::Io(err) => fmt::Display::fmt(err, f)
        }
    }
}

impl std::error::Error for Error {

}

impl const From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        Error { repr: Repr::ErrorCode(code) }
    }
}

enum Repr {
    ErrorCode(ErrorCode),
    Io(io::Error),
    // Thin pointer to a static string, &'static str would take more space
    ConstMessage(&'static &'static str),
    Other(Box<dyn std::error::Error + 'static>)
}

impl Repr {
    #[inline]
    pub const fn with_error_code(code: ErrorCode) -> Repr {
        Repr::ErrorCode(code)
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;