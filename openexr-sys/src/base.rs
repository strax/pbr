use libc::{c_char, c_int, size_t, c_void};

extern "C" {
    /// Retrieve the current library version. The `p` extra string is for custom installs, and is a
    /// static string, do not free the returned pointer.
    pub fn exr_get_library_version(maj: *mut c_int, min: *mut c_int, patch: *mut c_int, extra: *mut *const c_char);

    /// Limit the size of image allowed to be parsed or created by
    /// the library.
    ///
    /// This is used as a safety check against corrupt files, but can also
    /// serve to avoid potential issues on machines which have very
    /// constrained RAM.
    ///
    /// These values are among the only globals in the core layer of
    /// OpenEXR. The intended use is for applications to define a global
    /// default, which will be combined with the values provided to the
    /// individual context creation routine. The values are used to check
    /// against parsed header values. This adds some level of safety from
    /// memory overruns where a corrupt file given to the system may cause
    /// a large allocation to happen, enabling buffer overruns or other
    /// potential security issue.
    ///
    /// These global values are combined with the values in
    /// [`exr_context_initializer_t`][1] using the following rules:
    /// 1. negative values are ignored.
    /// 2. if either value has a positive (non-zero) value, and the other
    ///    has 0, the positive value is preferred.
    /// 3. If both are positive (non-zero), the minimum value is used.
    /// 4. If both values are 0, this disables the constrained size checks.
    ///
    /// This function does not fail.
    ///
    /// [1]: crate::context::exr_context_initializer_t
    pub fn exr_set_default_maximum_image_size(w: c_int, h: c_int);

    /// Retrieve the global default maximum image size.
    ///
    /// This function does not fail.
    pub fn exr_get_default_maximum_image_size(w: *mut c_int, h: *mut c_int);

    /// Limit the size of an image tile allowed to be parsed or
    /// created by the library.
    ///
    /// Similar to image size, this places constraints on the maximum tile
    /// size as a safety check against bad file data.
    ///
    /// This is used as a safety check against corrupt files, but can also
    /// serve to avoid potential issues on machines which have very
    /// constrained RAM.
    ///
    /// These values are among the only globals in the core layer of
    /// OpenEXR. The intended use is for applications to define a global
    /// default, which will be combined with the values provided to the
    /// individual context creation routine. The values are used to check
    /// against parsed header values. This adds some level of safety from
    /// memory overruns where a corrupt file given to the system may cause
    /// a large allocation to happen, enabling buffer overruns or other
    /// potential security issue.
    ///
    /// These global values are combined with the values in
    /// [`exr_context_initializer_t`][1] using the following rules:
    ///
    /// 1. negative values are ignored.
    /// 2. if either value has a positive (non-zero) value, and the other
    ///    has 0, the positive value is preferred.
    /// 3. If both are positive (non-zero), the minimum value is used.
    /// 4. If both values are 0, this disables the constrained size checks.
    ///
    /// This function does not fail.
    ///
    /// [1]: crate::context::exr_context_initializer_t
    pub fn exr_set_default_maximum_tile_size(w: c_int, h: c_int);

    /// Retrieve the global maximum tile size.
    ///
    /// This function does not fail.
    pub fn exr_get_default_maximum_tile_size(w: *mut c_int, h: *mut c_int);

    /// Assigns a default zip compression level.
    ///
    /// This value may be controlled separately on each part, but this
    /// global control determines the initial value.
    pub fn exr_set_default_zip_compression_level(l: c_int);

    /// Retrieve the global default zip compression value.
    pub fn exr_get_default_zip_compression_level(l: *mut c_int);

    /// Assigns a default DWA compression quality level.
    ///
    /// This value may be controlled separately on each part, but this
    /// global control determines the initial value.
    pub fn exr_set_default_dwa_compression_quality(q: f32);

    /// Retrieve the global default DWA compression quality level.
    pub fn exr_get_default_dwa_compression_quality(q: *mut f32);
}

/// Function pointer used to hold a malloc-like routine.
///
/// Providing these to a context will override what memory is used to
/// allocate the context itself, as well as any allocations which
/// happen during processing of a file or stream. This can be used by
/// systems which provide rich malloc tracking routines to override the
/// internal allocations performed by the library.
///
/// This function is expected to allocate and return a new memory
/// handle, or `NULL` if allocation failed (which the library will then
/// handle and return an out-of-memory error).
///
/// If one is provided, both should be provided.
///
/// See also: [`exr_memory_free_func_t`][exr_memory_free_func_t]
pub type exr_memory_allocation_func_t = Option<unsafe extern "C" fn(bytes: size_t) -> *mut c_void>;

/// Function pointer used to hold a free-like routine.
///
/// Providing these to a context will override what memory is used to
/// allocate the context itself, as well as any allocations which
/// happen during processing of a file or stream. This can be used by
/// systems which provide rich malloc tracking routines to override the
/// internal allocations performed by the library.
///
/// This function is expected to return memory to the system, ala free
/// from the C library.
///
/// If providing one, probably need to provide both routines.
///
/// See also: [`exr_memory_allocation_func_t`][exr_memory_allocation_func_t]
pub type exr_memory_free_func_t = Option<unsafe extern "C" fn(ptr: *mut c_void)>;

extern "C" {
    /// Allow the user to override default allocator used internal
    /// allocations necessary for files, attributes, and other temporary
    /// memory.
    ///
    /// These routines may be overridden when creating a specific context,
    /// however this provides global defaults such that the default can be
    /// applied.
    ///
    /// If either pointer is 0, the appropriate malloc/free routine will be
    /// substituted.
    ///
    /// This function does not fail.
    pub fn exr_set_default_memory_routines(alloc_func: exr_memory_allocation_func_t, free_func: exr_memory_free_func_t);
}