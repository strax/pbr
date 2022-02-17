use libc::c_char;

/// Enum for use in a custom allocator in the encode/decode pipelines
/// (that is, so the implementor knows whether to allocate on which
/// device based on the buffer disposition).
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum exr_transcoding_pipeline_buffer_id_t {
    EXR_TRANSCODE_BUFFER_PACKED,
    EXR_TRANSCODE_BUFFER_UNPACKED,
    EXR_TRANSCODE_BUFFER_COMPRESSED,
    EXR_TRANSCODE_BUFFER_SCRATCH1,
    EXR_TRANSCODE_BUFFER_SCRATCH2,
    EXR_TRANSCODE_BUFFER_PACKED_SAMPLES,
    EXR_TRANSCODE_BUFFER_SAMPLES
}

/// Struct for negotiating buffers when decoding/encoding
/// chunks of data.
///
/// This is generic and meant to negotiate exr data bi-directionally,
/// in that the same structure is used for both decoding and encoding
/// chunks for read and write, respectively.
///
/// The first half of the structure will be filled by the library, and
/// the caller is expected to fill the second half appropriately.
///
/// # Usage notes
///
/// The fields `channel_name`, `height`, `x_samples`, `y_samples`, `p_linear`, `bytes_per_elements` and `data_type` must not be modified.
#[repr(C)]
pub struct exr_coding_channel_info_t {
    pub channel_name: *const c_char,
    pub height: i32,
    pub x_samples: i32,
    pub y_samples: i32,
    pub p_linear: u8,
    pub bytes_per_elements: i8,
    pub data_type: u16,

    // Elements below must be edited by the caller to control encoding / decoding.

    /// How many bytes per pixel the input is or output should be
    /// (2 for float16, 4 for float32/uint32). Defaults to same
    /// size as input.
    pub user_bytes_per_element: i16,
    /// Small form of [`exr_pixel_type_t`][1] enum
    /// (`EXR_PIXEL_UINT/HALF/FLOAT`). Defaults to same type as input.
    ///
    /// [1]: crate::attr::exr_pixel_type_t
    pub user_data_type: u16,
    /// Increment to get to next pixel.
    ///
    /// This is in bytes. Must be specified when the decode pointer is
    /// specified (and always for encode).
    ///
    /// This is useful for implementing transcoding generically of
    /// planar or interleaved data. For planar data, where the layout
    /// is RRRRRGGGGGBBBBB, you can pass in 1 * bytes per component.
    pub user_pixel_stride: i32,
    /// When `lines` > 1 for a chunk, this is the increment used to get
    /// from beginning of line to beginning of next line.
    ///
    /// This is in bytes. Must be specified when the decode pointer is
    /// specified (and always for encode).
    pub user_line_stride: i32,
    /// This data member has different requirements reading vs
    /// writing. When reading, if this is left as `NULL`, the channel
    /// will be skipped during read and not filled in.  During a write
    /// operation, this pointer is considered `const` and **not**
    /// modified.
    pub ptr: *mut u8
}