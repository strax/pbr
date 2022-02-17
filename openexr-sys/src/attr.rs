//! Attribute/metadata value types and struct declarations.
//!
//! These are a group of enum values defining valid values for
//! some attributes and then associated structs for other types.
//!
//! Some of these types will be directly representable/storable in
//! the file, some not. There is some overlap here with Imath, and they
//! should be kept in the same order for compatibility. However do note
//! that these are just the raw data, and no useful functions are
//! declared at this layer, that is what Imath is for.

use libc::{c_char, size_t, c_void};

use crate::errors::*;
use crate::context::*;

/// Enum declaring allowed values for `u8` value stored in built-in compression type.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_compression_t {
    EXR_COMPRESSION_NONE,
    EXR_COMPRESSION_RLE,
    EXR_COMPRESSION_ZIPS,
    EXR_COMPRESSION_ZIP,
    EXR_COMPRESSION_PIZ,
    EXR_COMPRESSION_PXR24,
    EXR_COMPRESSION_B44,
    EXR_COMPRESSION_B44A,
    EXR_COMPRESSION_DWAA,
    EXR_COMPRESSION_DWAB,
    /// Invalid value, provided for range checking.
    EXR_COMPRESSION_LAST_TYPE
}

/// Enum declaring allowed values for `u8` value stored in built-in env map type.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_envmap_t {
    EXR_ENVMAP_LATLONG,
    EXR_ENVMAP_CUBE,
    /// Invalid value, provided for range checking.
    EXR_ENVMAP_LAST_TYPE
}

/// Enum declaring allowed values for `u8` value stored in `lineOrder` type.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_lineorder_t {
    EXR_LINEORDER_INCREASING_Y,
    EXR_LINEORDER_DECREASING_Y,
    EXR_LINEORDER_RANDOM_Y,
    /// Invalid value, provided for range checking.
    EXR_LINEORDER_LAST_TYPE
}

/// Enum declaring allowed values for part type.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_storage_t {
    /// Corresponds to type `scanlineimage`.
    EXR_STORAGE_SCANLINE,
    /// Corresponds to type `tiledimage`.
    EXR_STORAGE_TILED,
    /// Corresponds to type `deepscanline`.
    EXR_STORAGE_DEEP_SCANLINE,
    /// Corresponds to type `deeptile`.
    EXR_STORAGE_DEEP_TILED,
    /// Invalid value, provided for range checking.
    EXR_STORAGE_LAST_TYPE
}

/// Values representing what type of tile information is contained.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_tile_level_mode_t {
    /// Single level of image data.
    EXR_TILE_ONE_LEVEL,
    /// Mipmapped image data.
    EXR_TILE_MIPMAP_LEVELS,
    /// Ripmapped image data.
    EXR_TILE_RIPMAP_LEVELS,
    /// Invalid value, provided for range checking.
    EXR_TILE_LAST_TYPE
}

/// Enum representing how to scale positions between levels.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_tile_round_mode_t {
    EXR_TILE_ROUND_DOWN,
    EXR_TILE_ROUND_UP,
    EXR_TILE_ROUND_LAST_TYPE
}

/// Enum capturing the underlying data type on a channel.
#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, Hash)]
pub enum exr_pixel_type_t {
    EXR_PIXEL_UINT,
    EXR_PIXEL_HALF,
    EXR_PIXEL_FLOAT,
    EXR_PIXEL_LAST_TYPE
}

/// Struct to hold color chromaticities to interpret the tristimulus color values in the image data.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_chromaticities_t {
    pub red_x: f32,
    pub red_y: f32,
    pub green_x: f32,
    pub green_y: f32,
    pub blue_x: f32,
    pub blue_y: f32,
    pub white_x: f32,
    pub white_y: f32
}

/// Struct to hold keycode information.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct exr_attr_keycode_t {
    pub film_mfc_code: i32,
    pub film_type: i32,
    pub prefix: i32,
    pub count: i32,
    pub perf_offset: i32,
    pub perfs_per_frame: i32,
    pub perfs_per_count: i32
}

/// struct to hold a 32-bit floating-point 3x3 matrix.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_m33f_t {
    pub m: [f32; 9]
}

/// struct to hold a 64-bit floating-point 3x3 matrix.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_m33d_t {
    pub m: [f64; 9]
}

/// Struct to hold a 32-bit floating-point 4x4 matrix.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_m44f_t {
    pub m: [f32; 16]
}

/// Struct to hold a 64-bit floating-point 4x4 matrix.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_m44d_t {
    pub m: [f64; 16]
}

/// Struct to hold an integer ratio value.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct exr_attr_rational_t {
    pub num: i32,
    pub denom: u32
}

/// Struct to hold timecode information.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct exr_attr_timecode_t {
    pub time_and_flags: u32,
    pub user_data: u32
}

/// Struct to hold a 2-element integer vector.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct exr_attr_v2i_t {
    pub x: i32,
    pub y: i32
}

/// Struct to hold a 2-element 32-bit float vector.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_v2f_t {
    pub x: f32,
    pub y: f32
}

/// Struct to hold a 2-element 64-bit float vector.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_v2d_t {
    pub x: f64,
    pub y: f64
}

/// Struct to hold a 3-element integer vector.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct exr_attr_v3i_t {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

/// Struct to hold a 3-element 32-bit float vector.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_v3f_t {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

/// Struct to hold a 3-element 64-bit float vector.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_v3d_t {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// Struct to hold an integer box/region definition.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct exr_attr_box2i_t {
    pub min: exr_attr_v2i_t,
    pub max: exr_attr_v2i_t
}

/// Struct to hold a floating-point box/region definition.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct exr_attr_box2f_t {
    pub min: exr_attr_v2f_t,
    pub max: exr_attr_v2f_t
}

/// Struct holding base tiledesc attribute type defined in spec
///
///  NB: This is in a tightly packed area so it can be read directly, be
///  careful it doesn't become padded to the next `u32` boundary.
#[repr(C, packed(1))]
#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub struct exr_attr_tiledesc_t {
    pub x_size: u32,
    pub y_size: u32,
    pub level_and_round: u8
}

/// Storage for a string.
#[repr(C)]
pub struct exr_attr_string_t {
    pub length: i32,
    /// If this is non-zero, the string owns the data, if 0, is a const ref to a static string.
    pub alloc_size: i32,
    pub str: *const c_char
}

/// Storage for a string vector.
#[repr(C)]
pub struct exr_attr_string_vector_t {
    pub n_strings: i32,
    /// If this is non-zero, the string owns the data, if 0, is a const ref to a static string.
    pub alloc_size: i32,
    pub strings: *const exr_attr_string_t
}

/// Float vector storage struct.
#[repr(C)]
pub struct exr_attr_float_vector_t {
    pub length: i32,
    /// If this is non-zero, the float vector owns the data, if 0, is a const ref.
    pub alloc_size: i32,
    pub arr: *const f32
}

/// Hint for lossy compression methods about how to treat values
/// (logarithmic or linear), meaning a human sees values like R, G, B,
/// luminance difference between 0.1 and 0.2 as about the same as 1.0
/// to 2.0 (logarithmic), where chroma coordinates are closer to linear
/// (0.1 and 0.2 is about the same difference as 1.0 and 1.1).
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum exr_perceptual_treatment_t {
    EXR_PERCEPTUALLY_LOGARITHMIC,
    EXR_PERCEPTUALLY_LINEAR
}

/// Individual channel information.
#[repr(C)]
pub struct exr_attr_chlist_entry_t {
    pub name: exr_attr_string_t,
    /// Data representation for these pixels: uint, half, float.
    pub pixel_type: exr_pixel_type_t,
    /// Possible values are 0 and 1 per docs exr_perceptual_treatment_t.
    pub p_linear: u8,
    pub reserved: [u8; 3],
    pub x_sampling: i32,
    pub y_sampling: i32
}

/// List of channel information (sorted alphabetically).
#[repr(C)]
pub struct exr_attr_chlist_t {
    pub num_channels: i32,
    pub num_alloced: i32,
    pub entries: *const exr_attr_chlist_entry_t
}

/// Struct to define attributes of an embedded preview image.
#[repr(C)]
pub struct exr_attr_preview_t {
    pub width: u32,
    pub height: u32,
    /// If this is non-zero, the preview owns the data, if 0, is a const ref.
    pub alloc_size: size_t,
    pub rgba: *const u8
}

/// Custom storage structure for opaque data.
/// Handlers for opaque types can be registered, then when a
/// non-builtin type is encountered with a registered handler, the
/// function pointers to unpack/pack it will be set up.
///
/// See also: [`exr_register_attr_type_handler`][exr_register_attr_type_handler]
#[repr(C)]
pub struct exr_attr_opaquedata_t {
    pub size: i32,
    pub unpacked_size: i32,
    /// If this is non-zero, the struct owns the data, if 0, is a const ref.
    pub packed_alloc_size: i32,
    pub pad: [u8; 4],
    pub packed_data: *mut c_void,
    /// When an application wants to have custom data, they can store
    /// an unpacked form here which will be requested to be destroyed
    /// upon destruction of the attribute.
    pub unpacked_data: *mut c_void,
    pub unpack_func_ptr: Option<unsafe extern "C" fn(exr_context_t, *const c_void, i32, *mut i32, *mut *mut c_void) -> exr_result_t>,
    pub pack_func_ptr: Option<unsafe extern "C" fn (exr_context_t, *const c_void, i32, *mut i32, *mut c_void) -> exr_result_t>,
    pub destroy_unpacked_func_ptr: Option<unsafe extern "C" fn(exr_context_t, *mut c_void, i32)>,
}

#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum exr_attribute_type_t {
    /// Type indicating an error or uninitialized attribute.
    EXR_ATTR_UNKNOWN = 0,
    /// Integer region definition. See [`exr_attr_box2i_t`][exr_attr_box2i_t].
    EXR_ATTR_BOX2I,
    /// Float region definition. See [`exr_attr_box2f_t`][exr_attr_box2f_t].
    EXR_ATTR_BOX2F,
    /// Definition of channels in file. See [`exr_attr_chlist_entry_t`][exr_attr_chlist_entry_t].
    EXR_ATTR_CHLIST,
    /// Values to specify color space of colors in file. See [`exr_attr_chromaticities_t`][exr_attr_chromaticities_t].
    EXR_ATTR_CHROMATICITIES,
    /// `u8` declaring compression present.
    EXR_ATTR_COMPRESSION,
    /// Double precision floating point number.
    EXR_ATTR_DOUBLE,
    /// `u8` declaring environment map type.
    EXR_ATTR_ENVMAP,
    /// Normal (4 byte) precision floating point number.
    EXR_ATTR_FLOAT,
    /// List of normal (4 byte) precision floating point numbers.
    EXR_ATTR_FLOAT_VECTOR,
    /// 32-bit signed integer value.
    EXR_ATTR_INT,
    /// Struct recording keycode. See [`exr_attr_keycode_t`][exr_attr_keycode_t].
    EXR_ATTR_KEYCODE,
    /// `u8` declaring scanline ordering.
    EXR_ATTR_LINEORDER,
    /// 9 32-bit floats representing a 3x3 matrix.
    EXR_ATTR_M33F,
    /// 9 64-bit floats representing a 3x3 matrix.
    EXR_ATTR_M33D,
    /// 16 32-bit floats representing a 4x4 matrix.
    EXR_ATTR_M44F,
    /// 16 64-bit floats representing a 4x4 matrix.
    EXR_ATTR_M44D,
    /// 2 `u32` values followed by 4 x w x h `u8` image.
    EXR_ATTR_PREVIEW,
    /// `i32` followed by `u32`.
    EXR_ATTR_RATIONAL,
    /// `i32` (length) followed by char string data.
    EXR_ATTR_STRING,
    /// 0 or more text strings (int + string). number is based on attribute size.
    EXR_ATTR_STRING_VECTOR,
    /// 2 `u32` values `xSize`, `ySize` followed by mode.
    EXR_ATTR_TILEDESC,
    /// 2 `u32` values time and flags, user data.
    EXR_ATTR_TIMECODE,
    /// Pair of 32-bit integers.
    EXR_ATTR_V2I,
    /// Pair of 32-bit floats.
    EXR_ATTR_V2F,
    /// Pair of 64-bit floats.
    EXR_ATTR_V2D,
    /// Set of 3 32-bit integers.
    EXR_ATTR_V3I,
    /// Set of 3 32-bit floats.
    EXR_ATTR_V3F,
    /// Set of 3 64-bit floats.
    EXR_ATTR_V3D,
    /// User/unknown provided type.
    EXR_ATTR_OPAQUE,
    EXR_ATTR_LAST_KNOWN_TYPE
}

/// An union of possible values associated with an attribute.
///
/// See [`exr_attribute_t`][exr_attribute_t] for details.
#[repr(C)]
pub union exr_attribute_data_t {
    pub uc: u8,
    pub d: f64,
    pub f: f32,
    pub i: i32,
    pub box2i: *mut exr_attr_box2i_t,
    pub box2f: *mut exr_attr_box2f_t,
    pub chlist: *mut exr_attr_chlist_t,
    pub chromaticies: *mut exr_attr_chromaticities_t,
    pub keycode: *mut exr_attr_keycode_t,
    pub floatvector: *mut exr_attr_float_vector_t,
    pub m33f: *mut exr_attr_m33f_t,
    pub m33d: *mut exr_attr_m33d_t,
    pub m44f: *mut exr_attr_m44f_t,
    pub m44d: *mut exr_attr_m44d_t,
    pub preview: *mut exr_attr_preview_t,
    pub rational: *mut exr_attr_rational_t,
    pub string: *mut exr_attr_string_t,
    pub stringvector: *mut exr_attr_string_vector_t,
    pub tiledesc: *mut exr_attr_tiledesc_t,
    pub timecode: *mut exr_attr_timecode_t,
    pub v2i: *mut exr_attr_v2i_t,
    pub v2f: *mut exr_attr_v2f_t,
    pub v2d: *mut exr_attr_v2d_t,
    pub v3i: *mut exr_attr_v3i_t,
    pub v3f: *mut exr_attr_v3f_t,
    pub v3d: *mut exr_attr_v3d_t,
    pub opaque: *mut exr_attr_opaquedata_t,
    pub rawptr: *mut u8
}

#[repr(C)]
/// Storage, name and type information for an attribute.
///
/// Attributes (metadata) for the file cause a surprising amount of
/// overhead. It is not uncommon for a production-grade EXR to have
/// many attributes. As such, the attribute struct is designed in a
/// slightly more complicated manner. It is optimized to have the
/// storage for that attribute: the struct itself, the name, the type,
/// and the data all allocated as one block. Further, the type and
/// standard names may use a static string to avoid allocating space
/// for those as necessary with the pointers pointing to static strings
/// (not to be freed). Finally, small values are optimized for.
pub struct exr_attribute_t {
    /// Name of the attribute.
    pub name: *const c_char,
    /// String type name of the attribute.
    pub type_name: *const c_char,
    /// Length of name string (short flag is 31 max, long allows 255).
    pub name_length: u8,
    /// Length of type string (short flag is 31 max, long allows 255).
    pub type_name_length: u8,
    pub pad: [u8; 2],
    /// Enum of the attribute type.
    pub r#type: exr_attribute_type_t,
    /// The associated value or a pointer to the value of the attribute.
    pub data: exr_attribute_data_t
}