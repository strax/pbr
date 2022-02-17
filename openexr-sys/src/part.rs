//! Part related definitions.
//!
//! A part is a separate entity in the OpenEXR file. This was
//! formalized in the OpenEXR 2.0 timeframe to allow there to be a
//! clear set of eyes for stereo, or just a simple list of AOVs within
//! a single OpenEXR file. Prior, it was managed by name convention,
//! but with a multi-part file, they are clearly separate types, and
//! can have separate behavior.
//!
//! This is a set of functions to query, or set up when writing, that
//! set of parts within a file. This remains backward compatible to
//! OpenEXR files from before this change, in that a file with a single
//! part is a subset of a multi-part file. As a special case, creating
//! a file with a single part will write out as if it is a file which
//! is not multi-part aware, so as to be compatible with those old
//! libraries.

use crate::errors::*;
use crate::context::*;
use crate::attr::*;

use libc::{c_char, c_int, c_void};

//#region PartInfo
extern "C" {
    /// Query how many parts are in the file.
    pub fn exr_get_count(ctxt: exr_const_context_t, count: *mut c_int) -> exr_result_t;

    /// Query the part name for the specified part.
    ///
    /// NB: If this file is a single part file and name has not been set, this
    /// will return `NULL`.
    pub fn exr_get_name(ctxt: exr_const_context_t, part_index: c_int, out: *mut *const c_char) -> exr_result_t;

    /// Query the storage type for the specified part.
    pub fn exr_get_storage(ctxt: exr_const_context_t, part_index: c_int, out: *mut exr_storage_t) -> exr_result_t;

    /// Define a new part in the file.
    pub fn exr_add_part(ctxt: exr_context_t, partname: *const c_char, r#type: exr_storage_t, new_index: *mut c_int) -> exr_result_t;

    /// Query how many levels are in the specified part.
    ///
    /// If the part is a tiled part, fill in how many tile levels are present.
    ///
    /// Return `ERR_SUCCESS` on success, an error otherwise (i.e. if the part
    /// is not tiled).
    ///
    /// It is valid to pass `NULL` to either of the `levelsx` or `levelsy`
    /// arguments, which enables testing if this part is a tiled part, or
    /// if you don't need both (i.e. in the case of a mip-level tiled
    /// image)
    pub fn exr_get_tile_levels(ctxt: exr_const_context_t, part_index: c_int, levelsx: *mut i32, levelsy: *mut i32) -> exr_result_t;

    /// Query the tile size for a particular level in the specified part.
    ///
    /// If the part is a tiled part, fill in the tile size for the
    /// specified part/level.
    ///
    /// Return `ERR_SUCCESS` on success, an error otherwise (i.e. if the
    /// part is not tiled).
    ///
    /// It is valid to pass `NULL` to either of the `tilew` or `tileh`
    /// arguments, which enables testing if this part is a tiled part, or
    /// if you don't need both (i.e. in the case of a mip-level tiled
    /// image)
    pub fn exr_get_tile_sizes(ctxt: exr_const_context_t, part_index: c_int, levelx: c_int, levely: c_int, tilew: *mut i32, tileh: *mut i32) -> exr_result_t;

    /// Query the data sizes for a particular level in the specified part.
    ///
    /// If the part is a tiled part, fill in the width/height for the
    /// specified levels.
    ///
    /// Return `ERR_SUCCESS` on success, an error otherwise (i.e. if the part
    /// is not tiled).
    ///
    /// It is valid to pass `NULL` to either of the `levw` or `levh`
    /// arguments, which enables testing if this part is a tiled part, or
    /// if you don't need both for some reason.
    pub fn exr_get_level_sizes(ctxt: exr_const_context_t, part_index: c_int, levelx: c_int, levely: c_int, levw: *mut i32, levh: *mut i32) -> exr_result_t;

    /// Return the number of chunks contained in this part of the file.
    ///
    /// As in the technical documentation for OpenEXR, the chunk is the
    /// generic term for a pixel data block. This is the atomic unit that
    /// this library uses to negotiate data to and from a context.
    ///
    /// This should be used as a basis for splitting up how a file is
    /// processed. Depending on the compression, a different number of
    /// scanlines are encoded in each chunk, and since those need to be
    /// encoded/decoded as a block, the chunk should be the basis for I/O
    /// as well.
    pub fn exr_get_chunk_count(ctxt: exr_const_context_t, part_index: c_int, out: *mut i32) -> exr_result_t;

    /// Return the number of scanlines chunks for this file part.
    ///
    /// When iterating over a scanline file, this may be an easier metric
    /// for multi-threading or other access than only negotiating chunk
    /// counts, and so is provided as a utility.
    pub fn exr_get_scanlines_per_chunk(ctxt: exr_const_context_t, part_index: c_int, out: *mut i32) -> exr_result_t;

    /// Return the maximum unpacked size of a chunk for the file part.
    //
    // This may be used ahead of any actual reading of data, so can be
    // used to pre-allocate buffers for multiple threads in one block or
    // whatever your application may require.
    pub fn exr_get_chunk_unpacked_size(ctxt: exr_const_context_t, part_index: c_int, out: *mut u64) -> exr_result_t;

    /// Retrieve the zip compression level used for the specified part.
    ///
    /// This only applies when the compression method involves using zip
    /// compression (zip, zips, some modes of DWAA/DWAB).
    ///
    /// This value is NOT persisted in the file, and only exists for the
    /// lifetime of the context, so will be at the default value when just
    /// reading a file.
    pub fn exr_get_zip_compression_level(ctxt: exr_const_context_t, part_index: c_int, out: *mut c_int) -> exr_result_t;

    /// Set the zip compression method used for the specified part.
    ///
    /// This only applies when the compression method involves using zip
    /// compression (zip, zips, some modes of DWAA/DWAB).
    ///
    /// This value is NOT persisted in the file, and only exists for the
    /// lifetime of the context, so this value will be ignored when
    /// reading a file.
    pub fn exr_set_zip_compression_level(ctxt: exr_context_t, part_index: c_int, level: c_int) -> exr_result_t;

    /// Retrieve the dwa compression level used for the specified part.
    ///
    /// This only applies when the compression method is DWAA/DWAB.
    ///
    /// This value is NOT persisted in the file, and only exists for the
    /// lifetime of the context, so will be at the default value when just
    /// reading a file.
    pub fn exr_get_dwa_compression_level(ctxt: exr_const_context_t, part_index: c_int, out: *mut f32) -> exr_result_t;

    /// Set the dwa compression method used for the specified part.
    ///
    /// This only applies when the compression method is DWAA/DWAB.
    ///
    /// This value is NOT persisted in the file, and only exists for the
    /// lifetime of the context, so this value will be ignored when
    /// reading a file.
    pub fn exr_set_dwa_compression_level(ctxt: exr_context_t, part_index: c_int, level: f32) -> exr_result_t;
}
//#endregion

//#region PartMetadata
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum exr_attr_list_access_mode_t {
    /// Order they appear in the file
    EXR_ATTR_LIST_FILE_ORDER,
    /// Alphabetically sorted
    EXR_ATTR_LIST_SORTED_ORDER
}

extern "C" {
    /// Query the count of attributes in a part.
    pub fn exr_get_attribute_count(ctxt: exr_const_context_t, part_index: c_int, count: *mut i32) -> exr_result_t;

    /// Query a particular attribute by index.
    pub fn exr_get_attribute_by_index(ctxt: exr_const_context_t, part_index: c_int, mode: exr_attr_list_access_mode_t, idx: i32, outattr: *mut *const exr_attribute_t) -> exr_result_t;

    /// Query a particular attribute by name.
    pub fn exr_get_attribute_by_name(ctxt: exr_const_context_t, part_index: c_int, name: *const c_char, outattr: *mut *const exr_attribute_t) -> exr_result_t;

    /// Query the list of attributes in a part.
    ///
    /// This retrieves a list of attributes currently defined in a part.
    ///
    /// If outlist is `NULL`, this function still succeeds, filling only the
    /// count. In this manner, the user can allocate memory for the list of
    /// attributes, then re-call this function to get the full list.
    pub fn exr_get_attribute_list(ctxt: exr_const_context_t, part_index: c_int, mode: exr_attr_list_access_mode_t, count: *mut i32, outlist: *mut *const exr_attribute_t) -> exr_result_t;

    /// Declare an attribute within the specified part.
    ///
    /// Only valid when a file is opened for write.
    pub fn exr_attr_declare_by_type(ctxt: exr_const_context_t, part_index: c_int, name: *const c_char, r#type: *const c_char, newattr: *mut *mut exr_attribute_t) -> exr_result_t;

    /// Declare an attribute within the specified part.
    ///
    /// Only valid when a file is opened for write.
    pub fn exr_attr_declare(ctxt: exr_const_context_t, part_index: c_int, name: *const c_char, r#type: exr_attribute_type_t, newattr: *mut *mut exr_attribute_t) -> exr_result_t;
}
//#endregion

//#region RequiredAttributeHelpers
extern "C" {
    /// Initialize all required attributes for all files.
    ///
    /// NB: other file types do require other attributes, such as the tile
    /// description for a tiled file.
    pub fn exr_initialize_required_attr(
        ctxt: exr_context_t,
        part_index: c_int,
        displayWindow: *const exr_attr_box2i_t,
        dataWindow: *const exr_attr_box2i_t,
        pixelaspectratio: f32,
        screenWindowCenter: *const exr_attr_v2f_t,
        screenWindowWidth: f32,
        lineorder: exr_lineorder_t,
        ctype: exr_compression_t
    ) -> exr_result_t;

    /// Initialize all required attributes to default values:
    ///
    /// - `displayWindow` is set to (0, 0 -> `width` - 1, `height` - 1)
    /// - `dataWindow` is set to (0, 0 -> `width` - 1, `height` - 1)
    /// - `pixelAspectRatio` is set to 1.0
    /// - `screenWindowCenter` is set to 0.f, 0.f
    /// - `screenWindowWidth` is set to 1.f
    /// - `lineorder` is set to `INCREASING_Y`
    /// - `compression` is set to `ctype`
    pub fn exr_initialize_required_attr_simple(
        ctxt: exr_context_t,
        part_index: c_int,
        with: i32,
        height: i32,
        ctype: exr_compression_t
    ) -> exr_result_t;

    /// Copy the attributes from one part to another.
    ///
    /// This allows one to quickly unassigned attributes from one source to another.
    ///
    /// If an attribute in the source part has not been yet set in the
    /// destination part, the item will be copied over.
    ///
    /// For example, when you add a part, the storage type and name
    /// attributes are required arguments to the definition of a new part,
    /// but channels has not yet been assigned. So by calling this with an
    /// input file as the source, you can copy the channel definitions (and
    /// any other unassigned attributes from the source).
    pub fn exr_copy_unset_attributes(
        ctxt: exr_context_t,
        part_index: c_int,
        source: exr_const_context_t,
        src_part_index: c_int
    ) -> exr_result_t;

    /// Retrieve the list of channels.
    pub fn exr_get_channels(ctxt: exr_const_context_t, part_index: c_int, chlist: *mut *const exr_attr_chlist_t);

    /// Define a new channel to the output file part.
    ///
    /// The `percept` parameter is used for lossy compression techniques
    /// to indicate that the value represented is closer to linear (1) or
    /// closer to logarithmic (0). For r, g, b, luminance, this is normally
    /// 0.
    pub fn exr_add_channel(
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        ptype: exr_pixel_type_t,
        percept: exr_perceptual_treatment_t,
        xsamp: i32,
        ysamp: i32
    ) -> c_int;

    /// Copy the channels from another source.
    ///
    /// Useful if you are manually constructing the list or simply copying
    /// from an input file.
    pub fn exr_set_channels(ctxt: exr_context_t, part_index: c_int, channels: *const exr_attr_chlist_t) -> exr_result_t;

    /// Retrieve the compression method used for the specified part.
    pub fn exr_get_compression(ctxt: exr_const_context_t, part_index: c_int, compression: *mut exr_compression_t) -> exr_result_t;
    /// Set the compression method used for the specified part.
    pub fn exr_set_compression(ctxt: exr_context_t, part_index: c_int, ctype: exr_compression_t) -> exr_result_t;

    /// Retrieve the data window for the specified part.
    pub fn exr_get_data_window(ctxt: exr_const_context_t, part_index: c_int, out: *mut exr_attr_box2i_t) -> exr_result_t;
    /// Set the data window for the specified part.
    pub fn exr_set_data_window(ctxt: exr_context_t, part_index: c_int, dw: *const exr_attr_box2i_t) -> c_int;

    /// Retrieve the display window for the specified part.
    pub fn exr_get_display_window(ctxt: exr_const_context_t, part_index: c_int, out: *mut exr_attr_box2i_t) -> exr_result_t;
    /// Set the display window for the specified part.
    pub fn exr_set_display_window(ctxt: exr_context_t, part_index: c_int, dw: *const exr_attr_box2i_t) -> c_int;

    /// Retrieve the line order for storing data in the specified part (use 0 for single part images).
    pub fn exr_get_lineorder(ctxt: exr_const_context_t, part_index: c_int, out: *mut exr_lineorder_t) -> exr_result_t;
    /// Set the line order for storing data in the specified part (use 0 for single part images).
    pub fn exr_set_lineorder(ctxt: exr_context_t, part_index: c_int, lo: exr_lineorder_t) -> exr_result_t;

    /// Retrieve the pixel aspect ratio for the specified part (use 0 for single part images).
    pub fn exr_get_pixel_aspect_ratio(ctxt: exr_const_context_t, part_index: c_int, par: *mut f32) -> exr_result_t;
    /// Set the pixel aspect ratio for the specified part (use 0 for single part images).
    pub fn exr_set_pixel_aspect_ratio(ctxt: exr_context_t, part_index: c_int, par: f32) -> exr_result_t;

    /// Retrieve the screen oriented window center for the specified part (use 0 for single part images).
    pub fn exr_get_screen_window_center(ctxt: exr_const_context_t, part_index: c_int, wc: *mut exr_attr_v2f_t) -> exr_result_t;
    /// Set the screen oriented window center for the specified part (use 0 for single part images).
    pub fn exr_set_screen_window_center(ctxt: exr_context_t, part_index: c_int, wc: *const exr_attr_v2f_t) -> c_int;

    /// Retrieve the screen oriented window width for the specified part (use 0 for single part images).
    pub fn exr_get_screen_window_width(ctxt: exr_const_context_t, part_index: c_int, out: *mut f32) -> exr_result_t;
    /// Set the screen oriented window width for the specified part (use 0 for single part images).
    pub fn exr_set_screen_window_width(ctxt: exr_context_t, part_index: c_int, ssw: f32) -> exr_result_t;

    /// Retrieve the tiling info for a tiled part (use 0 for single part images).
    pub fn exr_get_tile_descriptor(
        ctxt: exr_const_context_t,
        part_index: c_int,
        xsize: *mut u32,
        ysize: *mut u32,
        level: *mut exr_tile_level_mode_t,
        round: *mut exr_tile_round_mode_t,
    ) -> exr_result_t;
    /// Set the tiling info for a tiled part (use 0 for single part images).
    pub fn exr_set_tile_descriptor(
        ctxt: exr_context_t,
        part_index: c_int,
        x_size: u32,
        y_size: u32,
        level_mode: exr_tile_level_mode_t,
        round_mode: exr_tile_round_mode_t,
    ) -> exr_result_t;

    pub fn exr_set_name(ctxt: exr_context_t, part_index: c_int, val: *const c_char) -> exr_result_t;

    pub fn exr_get_version(ctxt: exr_const_context_t, part_index: c_int, out: *mut i32) -> exr_result_t;
    pub fn exr_set_version(ctxt: exr_context_t, part_index: c_int, val: i32) -> exr_result_t;

    pub fn exr_set_chunk_count(ctxt: exr_context_t, part_index: c_int, val: i32) -> exr_result_t;
}
//#endregion

extern "C" {
    pub fn exr_attr_get_box2i (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        outval: *mut exr_attr_box2i_t
    ) -> exr_result_t;

    pub fn exr_attr_set_box2i (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        val: *const exr_attr_box2i_t
    ) -> exr_result_t;

    pub fn exr_attr_get_box2f (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        outval: *mut exr_attr_box2f_t
    ) -> exr_result_t;

    pub fn exr_attr_set_box2f (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        val: *const exr_attr_box2f_t
    ) -> exr_result_t;

    /// Zero-copy query of channel data.
    ///
    /// Do not free or manipulate the `chlist` data, or use
    /// after the lifetime of the context.
    pub fn exr_attr_get_channels (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        chlist: *mut *const exr_attr_chlist_t
    ) -> exr_result_t;

    /// This allows one to quickly copy the channels from one file
    /// to another.
    pub fn exr_attr_set_channels (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        channels: *const exr_attr_chlist_t
    ) -> exr_result_t;

    pub fn exr_attr_get_chromaticities (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        chroma: *mut exr_attr_chromaticities_t
    ) -> exr_result_t;

    pub fn exr_attr_set_chromaticities (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        chroma: *const exr_attr_chromaticities_t
    ) -> exr_result_t;

    pub fn exr_attr_get_compression (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_compression_t
    ) -> exr_result_t;

    pub fn exr_attr_set_compression (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        comp: exr_compression_t
    ) -> exr_result_t;

    pub fn exr_attr_get_double (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut f64
    ) -> exr_result_t;

    pub fn exr_attr_set_double (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        val: f64
    ) -> exr_result_t;

    pub fn exr_attr_get_envmap (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_envmap_t
    ) -> exr_result_t;

    pub fn exr_attr_set_envmap (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        emap: exr_envmap_t
    ) -> exr_result_t;

    pub fn exr_attr_get_float (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut f32
    ) -> exr_result_t;

    pub fn exr_attr_set_float (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        val: f32
    ) -> exr_result_t;

    /// Zero-copy query of float data.
    ///
    /// Do not free or manipulate the `out` data, or use after the
    /// lifetime of the context.
    pub fn exr_attr_get_float_vector (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        sz: *mut i32,
        out: *mut *const f32
    ) -> exr_result_t;


    pub fn exr_attr_set_float_vector (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        sz: i32,
        vals: *const f32
    ) -> exr_result_t;

    pub fn exr_attr_get_int (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut i32
    ) -> exr_result_t;

    pub fn exr_attr_set_int (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        val: i32
    ) -> exr_result_t;

    pub fn exr_attr_get_keycode (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_keycode_t
    ) -> exr_result_t;

    pub fn exr_attr_set_keycode (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        kc: *const exr_attr_keycode_t
    ) -> exr_result_t;

    pub fn exr_attr_get_lineorder (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_lineorder_t
    ) -> exr_result_t;

    pub fn exr_attr_set_lineorder (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        lo: exr_lineorder_t
    ) -> exr_result_t;

    pub fn exr_attr_get_m33f (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_m33f_t
    ) -> exr_result_t;

    pub fn exr_attr_set_m33f (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        m: *const exr_attr_m33f_t
    ) -> exr_result_t;

    pub fn exr_attr_get_m33d (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_m33d_t
    ) -> exr_result_t;

    pub fn exr_attr_set_m33d (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        m: *const exr_attr_m33d_t
    ) -> exr_result_t;

    pub fn exr_attr_get_m44f (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_m44f_t
    ) -> exr_result_t;

    pub fn exr_attr_set_m44f (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        m: *const exr_attr_m44f_t
    ) -> exr_result_t;

    pub fn exr_attr_get_m44d (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_m44d_t
    ) -> exr_result_t;

    pub fn exr_attr_set_m44d (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        m: *const exr_attr_m44d_t
    ) -> exr_result_t;

    pub fn exr_attr_get_preview (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_preview_t
    ) -> exr_result_t;

    pub fn exr_attr_set_preview (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        p: *const exr_attr_preview_t
    ) -> exr_result_t;

    pub fn exr_attr_get_rational (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_rational_t
    ) -> exr_result_t;

    pub fn exr_attr_set_rational (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        r: *const exr_attr_rational_t
    ) -> exr_result_t;

    /// Zero-copy query of string value.
    ///
    /// Do not modify the string pointed to by `out`, and do not use
    /// after the lifetime of the context.
    pub fn exr_attr_get_string (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        length: *mut i32,
        out: *mut *const c_char
    ) -> exr_result_t;

    pub fn exr_attr_set_string (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        s: *const c_char
    ) -> exr_result_t;

    /// Zero-copy query of string data.
    ///
    /// Do not free the strings pointed to by the array.
    ///
    /// Must provide `size`.
    ///
    /// `out` must be a `const char**` array large enough to hold
    /// the string pointers for the string vector when provided.
    pub fn exr_attr_get_string_vector (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        size: *mut i32,
        out: *mut *const c_char
    ) -> exr_result_t;

    pub fn exr_attr_set_string_vector (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        size: i32,
        sv: *mut *const c_char
    ) -> exr_result_t;

    pub fn exr_attr_get_tiledesc (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_tiledesc_t
    ) -> exr_result_t;

    pub fn exr_attr_set_tiledesc (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        td: *const exr_attr_tiledesc_t
    ) -> exr_result_t;

    pub fn exr_attr_get_timecode (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_timecode_t
    ) -> exr_result_t;

    pub fn exr_attr_set_timecode (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        tc: *const exr_attr_timecode_t
    ) -> exr_result_t;

    pub fn exr_attr_get_v2i (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_v2i_t
    ) -> exr_result_t;

    pub fn exr_attr_set_v2i (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        v: *const exr_attr_v2i_t
    ) -> exr_result_t;

    pub fn exr_attr_get_v2f (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_v2f_t
    ) -> exr_result_t;

    pub fn exr_attr_set_v2f (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        v: *const exr_attr_v2f_t
    ) -> exr_result_t;

    pub fn exr_attr_get_v2d (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_v2d_t
    ) -> exr_result_t;

    pub fn exr_attr_set_v2d (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        v: *const exr_attr_v2d_t
    ) -> exr_result_t;

    pub fn exr_attr_get_v3i (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_v3i_t
    ) -> exr_result_t;

    pub fn exr_attr_set_v3i (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        v: *const exr_attr_v3i_t
    ) -> exr_result_t;

    pub fn exr_attr_get_v3f (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_v3f_t
    ) -> exr_result_t;

    pub fn exr_attr_set_v3f (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        v: *const exr_attr_v3f_t
    ) -> exr_result_t;

    pub fn exr_attr_get_v3d (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        out: *mut exr_attr_v3d_t
    ) -> exr_result_t;

    pub fn exr_attr_set_v3d (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        v: *const exr_attr_v3d_t
    ) -> exr_result_t;

    pub fn exr_attr_get_user (
        ctxt: exr_const_context_t,
        part_index: c_int,
        name: *const c_char,
        r#type: *mut *const c_char,
        size: *mut i32,
        out: *mut *const c_void
    ) -> exr_result_t;

    pub fn exr_attr_set_user (
        ctxt: exr_context_t,
        part_index: c_int,
        name: *const c_char,
        r#type: *const c_char,
        size: i32,
        out: *const c_void
    ) -> exr_result_t;
}