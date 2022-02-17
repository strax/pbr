//! Context related definitions.
//!
//! A context is a single instance of an OpenEXR file or stream. Beyond
//! a particular file or stream handle, it also has separate controls
//! for error handling and memory allocation. This is done to enable
//! encoding or decoding on mixed hardware.

use core::ptr;
use core::mem;

use crate::errors::*;
use crate::base::*;

use libc::{c_char, c_int, c_void, size_t};

extern "C" {
    #[doc(hidden)]
    pub type _priv_exr_context_t;
}

/// Opaque context handle
///
/// The implementation of this is partly opaque to provide better
/// version portability, and all accesses to relevant data should
/// happen using provided functions. This handle serves as a container
/// and identifier for all the metadata and parts associated with a
/// file and/or stream.
pub type exr_context_t = *mut _priv_exr_context_t;
pub type exr_const_context_t = *const _priv_exr_context_t;

/// Stream error notifier
///
/// This function pointer is provided to the stream functions by the
/// library such that they can provide a nice error message to the
/// user during stream operations.
pub type exr_stream_error_func_ptr_t = Option<unsafe extern "C" fn(ctxt: exr_const_context_t, code: exr_result_t, fmt: *const c_char, ...)>;

/// Error callback function
///
/// Because a file can be read from using many threads at once, it is
/// difficult to store an error message for later retrieval. As such,
/// when a file is constructed, a callback function can be provided
/// which delivers an error message for the calling application to
/// handle. This will then be delivered on the same thread causing the
/// error.
pub type exr_error_handler_cb_t = Option<unsafe extern "C" fn(ctxt: exr_const_context_t, code: exr_result_t, msg: *const c_char)>;

/// Destroy custom stream function pointer
///
/// Generic callback to clean up user data for custom streams.
/// This is called when the file is closed and expected not to
/// error.
///
/// # Parameters
///
/// * failed – Indicates the write operation failed, the implementor may wish to cleanup temporary files
pub type exr_destroy_stream_func_ptr_t = Option<unsafe extern "C" fn(ctxt: exr_const_context_t, userdata: *mut c_void, failed: c_int)>;

/// Query stream size function pointer
///
/// Used to query the size of the file, or amount of data representing
/// the openexr file in the data stream.
///
/// This is used to validate requests against the file. If the size is
/// unavailable, return -1, which will disable these validation steps
/// for this file, although appropriate memory safeguards must be in
/// place in the calling application.
pub type exr_query_size_func_ptr_t = Option<unsafe extern "C" fn(ctxt: exr_const_context_t, userdata: *mut c_void) -> i64>;

/// Read custom function pointer
///
/// Used to read data from a custom output. Expects similar semantics to
/// `pread` or `ReadFile` with overlapped data under Win32.
///
/// It is required that this provides thread-safe concurrent access to
/// the same file. If the stream/input layer you are providing does
/// not have this guarantee, your are responsible for providing
/// appropriate serialization of requests.
///
/// A file should be expected to be accessed in the following pattern:
/// - upon open, the header and part information attributes will be read
/// - upon the first image read request, the offset tables will be read
///   multiple threads accessing this concurrently may actually read
///   these values at the same time
/// - chunks can then be read in any order as preferred by the
///   application
///
/// While this should mean that the header will be read in 'stream'
/// order (no seeks required), no guarantee is made beyond that to
/// retrieve image/deep data in order. So if the backing file is
/// truly a stream, it is up to the provider to implement appropriate
/// caching of data to give the appearance of being able to seek/read
/// atomically.
pub type exr_read_func_ptr_t = Option<unsafe extern "C" fn(ctxt: exr_const_context_t, userdata: *mut c_void, buffer: *mut c_void, sz: u64, offset: u64, error_cb: exr_stream_error_func_ptr_t) -> i64>;

/// Write custom function pointer
///
/// Used to write data to a custom output. Expects similar semantics to
/// `pwrite` or `WriteFile` with overlapped data under Win32.
///
/// It is required that this provides thread-safe concurrent access to
/// the same file. While it is unlikely that multiple threads will
/// be used to write data for compressed forms, it is possible.
///
/// A file should be expected to be accessed in the following pattern:
/// - upon open, the header and part information attributes is constructed.
/// - when the `write_header` routine is called, the header becomes immutable
///   and is written to the file. This computes the space to store the chunk
///   offsets, but does not yet write the values.
/// - image chunks are written to the file, and appear in the order
///   they are written, not in the ordering that is required by the
///   chunk offset table (unless written in that order). This may vary
///   slightly if the size of the chunks is not directly known and
///   tight packing of data is necessary.
/// - at file close, the chunk offset tables are written to the file.
pub type exr_write_func_ptr_t = Option<unsafe extern "C" fn (ctxt: exr_const_context_t, userdata: *mut c_void, buffer: *const c_void, sz: u64, offset: u64, error_cb: exr_stream_error_func_ptr_t) -> i64>;

/// Struct used to pass function pointers into the context
///  initialization routines.
///
/// This partly exists to avoid the chicken and egg issue around
/// creating the storage needed for the context on systems which want
/// to override the malloc/free routines.
///
/// However, it also serves to make a tidier/simpler set of functions
/// to create and start processing exr files.
///
/// The size member is required for version portability.
///
/// It can be initialized using `Default::default()`.
///
/// # Examples (C)
///
/// ```c
/// exr_context_initializer_t myctxtinit = DEFAULT_CONTEXT_INITIALIZER;
/// myctxtinit.error_cb = &my_super_cool_error_callback_function;
/// ...
/// ```
#[repr(C)]
pub struct exr_context_initializer_t {
    /// Size member to tag initializer for version stability.
    ///
    /// This should be initialized to the size of the current
    /// structure. This allows EXR to add functions or other
    /// initializers in the future, and retain version compatibility
    pub size: size_t,
    /// Error callback function pointer
    ///
    /// The error callback is allowed to be `NULL`, and will use a
    ///  default print which outputs to `stderr`.
    pub error_handler_fn: exr_error_handler_cb_t,
    /// Custom allocator, if `NULL`, will use malloc.
    pub alloc_fn: exr_memory_allocation_func_t,
    /// Custom deallocator, if `NULL`, will use free.
    pub free_fn: exr_memory_free_func_t,
    /// Blind data passed to custom read, size, write, destroy
    /// functions below. Up to user to manage this pointer.
    pub user_data: *mut c_void,
    /// Custom read routine.
    ///
    /// This is only used during read or update contexts. If this is
    /// provided, it is expected that the caller has previously made
    /// the stream available, and placed whatever stream/file data
    /// into `user_data` above.
    ///
    /// If this is `NULL`, and the context requested is for reading an
    /// exr file, an internal implementation is provided for reading
    /// from normal filesystem files, and the filename provided is
    /// attempted to be opened as such.
    ///
    /// Expected to be `NULL` for a write-only operation, but is ignored
    /// if it is provided.
    ///
    /// For update contexts, both read and write functions must be
    /// provided if either is.
    pub read_fn: exr_read_func_ptr_t,
    /// Custom size query routine.
    ///
    /// Used to provide validation when reading header values. If this
    /// is not provided, but a custom read routine is provided, this
    /// will disable some of the validation checks when parsing the
    /// image header.
    ///
    /// Expected to be `NULL` for a write-only operation, but is ignored
    /// if it is provided.
    pub size_fn: exr_query_size_func_ptr_t,
    /// Custom write routine.
    ///
    /// This is only used during write or update contexts. If this is
    /// provided, it is expected that the caller has previously made
    /// the stream available, and placed whatever stream/file data
    /// into `user_data` above.
    ///
    /// If this is `NULL`, and the context requested is for writing an
    /// exr file, an internal implementation is provided for reading
    /// from normal filesystem files, and the filename provided is
    /// attempted to be opened as such.
    ///
    /// For update contexts, both read and write functions must be
    /// provided if either is.
    pub write_fn: exr_write_func_ptr_t,
    /// Optional function to destroy the user data block of a custom stream.
    ///
    /// Allows one to free any user allocated data, and close any handles.
    pub destroy_fn: exr_destroy_stream_func_ptr_t,
    /// Initialize a field specifying what the maximum image width
    /// allowed by the context is. See [`exr_set_default_maximum_image_size()`][1] to
    /// understand how this interacts with global defaults.
    ///
    /// [1]: crate::base::exr_set_default_maximum_image_size
    pub max_image_width: c_int,
    /// Initialize a field specifying what the maximum image height
    /// allowed by the context is. See [`exr_set_default_maximum_image_size()`][1] to
    /// understand how this interacts with global defaults.
    ///
    /// [1]: crate::base::exr_set_default_maximum_image_size
    pub max_image_height: c_int,
    /// Initialize a field specifying what the maximum tile width
    /// allowed by the context is. See [`exr_set_default_maximum_tile_size()`][1] to
    /// understand how this interacts with global defaults.
    ///
    /// [1]: crate::base::exr_set_default_maximum_tile_size
    pub max_tile_width: c_int,
    /// Initialize a field specifying what the maximum tile height
    /// allowed by the context is. See [`exr_set_default_maximum_tile_size()`][1] to
    /// understand how this interacts with global defaults.
    ///
    /// [1]: crate::base::exr_set_default_maximum_tile_size
    pub max_tile_height: c_int,
    /// Initialize a field specifying what the default zip compression level should be
    /// for this context. See [`exr_set_default_zip_compression_level()`][1] to
    /// set it for all contexts.
    ///
    /// [1]: crate::base::exr_set_default_zip_compression_level
    pub zip_level: c_int,
    /// Initialize the default dwa compression quality. See
    /// [`exr_set_default_dwa_compression_quality()`][1] to set the default
    /// for all contexts.
    ///
    /// [1]: crate::base::exr_set_default_dwa_compression_quality
    pub dwa_quality: f32
}

impl Default for exr_context_initializer_t {
    fn default() -> Self {
        // See EXR_DEFAULT_CONTEXT_INITIALIZER
        exr_context_initializer_t {
            size: mem::size_of::<exr_context_initializer_t>(),
            error_handler_fn: None,
            alloc_fn: None,
            free_fn: None,
            user_data: ptr::null_mut(),
            read_fn: None,
            size_fn: None,
            write_fn: None,
            destroy_fn: None,
            max_image_width: 0,
            max_image_height: 0,
            max_tile_width: 0,
            max_tile_height: 0,
            zip_level: -2,
            dwa_quality: -1.0
        }
    }
}

extern "C" {
    /// Check the magic number of the file and report
    /// [`EXR_ERR_SUCCESS`][1] if the file appears to be a valid file (or at least
    /// has the correct magic number and can be read).
    ///
    /// [1]: crate::errors::exr_error_code_t
    pub fn exr_test_file_header(filename: *const c_char, ctxtdata: *const exr_context_initializer_t) -> exr_result_t;

    /// Close and free any internally allocated memory,
    /// calling any provided destroy function for custom streams.
    ///
    /// If the file was opened for write, first save the chunk offsets
    /// or any other unwritten data.
    pub fn exr_finish(ctxt: *mut exr_context_t) -> exr_result_t;

    /// Create and initialize a read-only exr read context.
    ///
    /// If a custom read function is provided, the filename is for
    /// informational purposes only, the system assumes the user has
    /// previously opened a stream, file, or whatever and placed relevant
    /// data in userdata to access that.
    ///
    /// One notable attribute of the context is that once it has been
    /// created and returned a successful code, it has parsed all the
    /// header data. This is done as one step such that it is easier to
    /// provide a safe context for multiple threads to request data from
    /// the same context concurrently.
    ///
    /// Once finished reading data, use [`exr_finish()`][exr_finish] to clean up
    /// the context.
    ///
    /// If you have custom I/O requirements, see the [initializer context documentation][`exr_context_initializer_t`].
    /// The `ctxtdata` parameter is optional, if `NULL`, default values will be used.
    pub fn exr_start_read(ctxt: *mut exr_context_t, filename: *const c_char, ctxtdata: *const exr_context_initializer_t) -> exr_result_t;
}

/// Enum describing how default files are handled during write.
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum exr_default_write_mode_t {
    /// Overwrite filename provided directly, deleted upon error.
    EXR_WRITE_FILE_DIRECTLY = 0,
    /// Create a temporary file, renaming it upon successful write, leaving original upon error
    EXR_INTERMEDIATE_TEMP_FILE = 1
}

extern "C" {
    /// Create and initialize a write-only context.
    ///
    /// If a custom write function is provided, the filename is for
    /// informational purposes only, and the `default_mode` parameter will be
    /// ignored. As such, the system assumes the user has previously opened
    /// a stream, file, or whatever and placed relevant data in userdata to
    /// access that.
    ///
    /// Multi-Threading: To avoid issues with creating multi-part EXR
    /// files, the library approaches writing as a multi-step process, so
    /// the same concurrent guarantees can not be made for writing a
    /// file. The steps are:
    ///
    /// 1. Context creation (this function)
    /// 2. Part definition (required attributes and additional metadata)
    /// 3. Transition to writing data (this "commits" the part definitions,
    ///    any changes requested after will result in an error)
    /// 4. Write part data in sequential order of parts (part<sub>0</sub> -> part<sub>N-1</sub>).
    /// 5. Within each part, multiple threads can be encoding and writing
    ///    data concurrently. For some EXR part definitions, this may be able
    ///    to write data concurrently when it can predict the chunk sizes, or
    ///    data is allowed to be padded. For others, it may need to
    ///    temporarily cache chunks until the data is received to flush in
    ///    order. The concurrency around this is handled by the library
    /// 6. Once finished writing data, use [`exr_finish()`][exr_finish] to clean
    ///    up the context, which will flush any unwritten data such as the
    ///    final chunk offset tables, and handle the temporary file flags.
    ///
    /// If you have custom I/O requirements, see the [initializer context documentation][exr_context_initializer_t].
    /// The `ctxtdata` parameter is optional, if `NULL`, default values will be used.
    pub fn exr_start_write(ctxt: *mut exr_context_t, filename: *const c_char, default_mode: exr_default_write_mode_t, ctxtdata: *const exr_context_initializer_t) -> exr_result_t;

    /// Create a new context for updating an exr file in place.
    ///
    /// This is a custom mode that allows one to modify the value of a
    /// metadata entry, although not to change the size of the header, or
    /// any of the image data.
    ///
    /// If you have custom I/O requirements, see the [initializer context documentation][exr_context_initializer_t].
    /// The `ctxtdata` parameter is optional, if `NULL`, default values will be used.
    pub fn exr_start_inplace_header_update(ctxt: *mut exr_context_t, filename: *const c_char, ctxtdata: *const exr_context_initializer_t) -> exr_result_t;

    /// Retrieve the file name the context is for as provided
    /// during the start routine.
    ///
    /// Do not free the resulting string.
    pub fn exr_get_file_name(ctxt: exr_const_context_t, name: *mut *const c_char) -> exr_result_t;

    /// Query the user data the context was constructed with. This
    /// is perhaps useful in the error handler callback to jump back into
    /// an object the user controls.
    pub fn exr_get_user_data(ctxt: exr_const_context_t, userdata: *mut *mut c_void) -> exr_result_t;

    /// Any opaque attribute data entry of the specified type is tagged
    /// with these functions enabling downstream users to unpack (or pack)
    /// the data.
    ///
    /// The library handles the memory packed data internally, but the
    /// handler is expected to allocate and manage memory for the
    /// *unpacked* buffer (the library will call the destroy function).
    ///
    /// NB: the pack function will be called twice (unless there is a
    /// memory failure), the first with a `NULL` buffer, requesting the
    /// maximum size (or exact size if known) for the packed buffer, then
    /// the second to fill the output packed buffer, at which point the
    /// size can be re-updated to have the final, precise size to put into
    /// the file.
    pub fn exr_register_attr_type_handler(
        ctxt: exr_context_t,
        r#type: *const c_char,
        unpack_func_ptr: Option<unsafe extern "C" fn(ctxt: exr_context_t, data: *const c_void, attrsize: i32, outsize: *mut i32, outbuffer: *mut *mut c_void) -> exr_result_t>,
        pack_func_ptr: Option<unsafe extern "C" fn(ctxt: exr_context_t, data: *const c_void, datasize: i32, outsize: *mut i32, outbuffer: *mut c_void) -> exr_result_t>,
        destroy_unpacked_func_ptr: Option<unsafe extern "C" fn(ctxt: exr_context_t, data: *mut c_void, datasize: i32)>
    ) -> exr_result_t;

    /// Enable long name support in the output context.
    pub fn exr_set_longname_support(ctxt: exr_context_t, onoff: c_int) -> exr_result_t;

    /// Write the header data.
    ///
    /// Opening a new output file has a small initialization state problem
    /// compared to opening for read/update: we need to enable the user
    /// to specify an arbitrary set of metadata across an arbitrary number
    /// of parts. To avoid having to create the list of parts and entire
    /// metadata up front, prior to calling the above [`exr_start_write()`][exr_start_write],
    /// allow the data to be set, then once this is called, it switches
    /// into a mode where the library assumes the data is now valid.
    ///
    /// It will recompute the number of chunks that will be written, and
    /// reset the chunk offsets. If you modify file attributes or part
    /// information after a call to this, it will error.
    pub fn exr_write_header(ctxt: exr_context_t) -> exr_result_t;
}