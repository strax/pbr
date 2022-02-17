use crate::errors::*;
use crate::context::*;

use libc::{c_int, c_void};

/// Struct describing raw data information about a chunk.
///
/// A chunk is the generic term for a pixel data block in an EXR file,
/// as described in the OpenEXR File Layout documentation. This is
/// common between all different forms of data that can be stored.
#[repr(C)]
pub struct exr_chunk_info_t {
    pub idx: i32,

    /// For tiles, this is the tilex; for scans it is the x.
    pub start_x: i32,
    /// For tiles, this is the tiley; for scans it is the scanline y.
    pub start_y: i32,
    /// For this chunk.
    pub height: i32,
    /// For this chunk.
    pub width: i32,

    /// For tiled files.
    pub level_x: u8,
    /// For tiled files.
    pub level_y: u8,

    pub r#type: u8,
    pub compression: u8,

    pub data_offset: u64,
    pub packed_size: u64,
    pub unpacked_size: u64,

    pub sample_count_data_offset: u64,
    pub sample_count_table_size: u64
}

extern "C" {
    pub fn exr_read_scanline_chunk_info(ctxt: exr_const_context_t, part_index: c_int, y: c_int, cinfo: *mut exr_chunk_info_t) -> exr_result_t;

    pub fn exr_read_tile_chunk_info(
        ctxt: exr_const_context_t,
        part_index: c_int,
        tilex: c_int,
        tiley: c_int,
        levelx: c_int,
        levely: c_int,
        cinfo: *mut exr_chunk_info_t
    ) -> exr_result_t;

    /// Read the packed data block for a chunk.
    ///
    /// This assumes that the buffer pointed to by `packed_data` is
    /// large enough to hold `cinfo->packed_size` bytes.
    pub fn exr_read_chunk(ctxt: exr_const_context_t, part_index: c_int, cinfo: *const exr_chunk_info_t, packed_data: *mut c_void) -> exr_result_t;

    /// Read chunk for deep data.
    ///
    /// This allows one to read the packed data, the sample count data, or both.
    /// [`exr_read_chunk`][exr_read_chunk] also works to read deep data packed data,
    /// but this is a routine to get the sample count table and the packed
    /// data in one go, or if you want to pre-read the sample count data,
    /// you can get just that buffer.
    pub fn exr_read_deep_chunk(
        ctxt: exr_const_context_t,
        part_index: c_int,
        cinfo: *const exr_chunk_info_t,
        packed_data: *mut c_void,
        sample_data: *mut c_void
    ) -> exr_result_t;

    /// Initialize a [`exr_chunk_info_t`][exr_chunk_info_t] structure when encoding scanline
    /// data (similar to read but does not do anything with a chunk
    /// table).
    pub fn exr_write_scanline_chunk_info(ctxt: exr_context_t, part_index: c_int, y: c_int, cinfo: *mut exr_chunk_info_t) -> exr_result_t;

    /// Initialize a [`exr_chunk_info_t`][exr_chunk_info_t] structure when encoding tiled data
    /// (similar to read but does not do anything with a chunk table).
    pub fn exr_write_tile_chunk_info(
        ctxt: exr_context_t,
        part_index: c_int,
        tilex: c_int,
        tiley: c_int,
        levelx: c_int,
        levely: c_int,
        cinfo: *mut exr_chunk_info_t
    ) -> exr_result_t;

    /// `y` must the appropriate starting y for the specified chunk.
    pub fn exr_write_scanline_chunk(
        ctxt: exr_context_t,
        part_index: c_int,
        y: c_int,
        packed_data: *const c_void,
        packed_size: u64
    ) -> exr_result_t;

    /// `y` must the appropriate starting y for the specified chunk.
    pub fn exr_write_deep_scanline_chunk(
        ctxt: exr_context_t,
        part_index: c_int,
        y: c_int,
        packed_data: *const c_void,
        packed_size: u64,
        unpacked_size: u64,
        sample_data: *const c_void,
        sample_data_size: u64
    ) -> exr_result_t;

    pub fn exr_write_tile_chunk(
        ctxt: exr_context_t,
        part_index: c_int,
        tilex: c_int,
        tiley: c_int,
        levelx: c_int,
        levely: c_int,
        packed_data: *const c_void,
        packed_size: u64
    ) -> exr_result_t;

    pub fn exr_write_deep_tile_chunk(
        ctxt: exr_context_t,
        part_index: c_int,
        tilex: c_int,
        tiley: c_int,
        levelx: c_int,
        levely: c_int,
        packed_data: *const c_void,
        packed_size: u64,
        unpacked_size: u64,
        sample_data: *const c_void,
        sample_data_size: u64
    ) -> exr_result_t;
}