//! Low-level bindings to the [mimalloc][1] allocator.
//!
//!
//! [1]: https://microsoft.github.io/mimalloc/index.html

#![allow(bad_style)]
#![feature(extern_types)]
#![no_std]

use libc::{c_void, size_t, c_int, c_char, c_ulonglong, c_long};
use core::mem;

//#region Basic Allocation
extern "C" {
    pub fn mi_calloc(count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_expand(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_free(p: *mut c_void);
    pub fn mi_malloc(size: size_t) -> *mut c_void;
    pub fn mi_mallocn(count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_realloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_reallocf(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_reallocn(p: *mut c_void, count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_realpath(fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;
    pub fn mi_recalloc(p: *mut c_void, count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_strdup(s: *const c_char) -> *mut c_char;
    pub fn mi_strndup(s: *const c_char, n: size_t) -> *mut c_char;
    pub fn mi_zalloc(size: size_t) -> *mut c_void;
}
//#endregion

//#region Extended Functions

/// Maximum size allowed for small allocations in [mi_malloc_small][mi_malloc_small] and [mi_zalloc_small][mi_zalloc_small] (usually `128*sizeof(void*)` (= 1KB on 64-bit systems))
pub const MI_SMALL_SIZE_MAX: usize = 128 * mem::size_of::<*mut c_void>();

pub type mi_deferred_free_fun = Option<unsafe extern "C" fn(force: bool, heartbeat: c_ulonglong, arg: *mut c_void)>;
pub type mi_error_fun = Option<unsafe extern "C" fn(err: c_int, arg: *mut c_void)>;
pub type mi_output_fun = Option<unsafe extern "C" fn(msg: *const c_char, arg: *mut c_void)>;

extern "C" {
    pub fn mi_collect(force: bool);
    pub fn mi_good_size(size: size_t) -> size_t;
    pub fn mi_is_in_heap_region(p: *const c_void) -> bool;
    pub fn mi_is_redirected() -> bool;

    /// Allocate a small object.
    ///
    /// ## Parameters
    ///
    /// <dl>
    ///     <dt>size</dt>
    ///     <dd>The size in bytes, can be at most <code>MI_SMALL_SIZE_MAX</code>.</dd>
    /// </dl>
    ///
    /// ## Returns
    ///
    /// a pointer to newly allocated memory of at least `size` bytes, or `NULL` if out of memory.
    /// This function is meant for use in run-time systems for best performance and does not check if `size` was indeed small – use with care!
    pub fn mi_malloc_small(size: size_t) -> *mut c_void;
    pub fn mi_manage_os_memory(start: *mut c_void, size: size_t, is_committed: bool, is_large: bool, is_zero: bool, numa_node: c_int) -> bool;
    pub fn mi_process_info(
        elapsed_msecs: *mut size_t,
        user_msecs: *mut size_t,
        system_msecs: *mut size_t,
        current_rss: *mut size_t,
        peak_rss: *mut size_t,
        current_commit: *mut size_t,
        peak_commit: *mut size_t,
        page_faults: *mut size_t
    );
    pub fn mi_register_deferred_free(deferred_free: mi_deferred_free_fun, arg: *mut c_void);
    pub fn mi_register_error(errfun: mi_error_fun, arg: *mut c_void);
    pub fn mi_register_output(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_reserve_huge_os_pages_at(pages: size_t, numa_node: c_int, timeout_msecs: size_t) -> c_int;
    pub fn mi_reserve_huge_os_pages_interleave(pages: size_t, numa_node: c_int, timeout_msecs: size_t) -> c_int;
    pub fn mi_reserve_os_memory(size: size_t, commit: bool, allow_large: bool) -> c_int;
    pub fn mi_stats_merge();
    pub fn mi_stats_print(out: *mut c_void);
    pub fn mi_stats_print_out(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_stats_reset();
    pub fn mi_thread_done();
    pub fn mi_thread_init();
    pub fn mi_thread_stats_print_out(out: mi_output_fun, arg: *mut c_void);
    pub fn mi_usable_size(p: *mut c_void) -> size_t;
    pub fn mi_zalloc_small(size: size_t) -> *mut c_void;
}
//#endregion

//#region Aligned Allocation

/// The maximum supported alignment size (currently 1MiB).
pub const MI_ALIGNMENT_MAX: usize = 1024 * 1024;

extern "C" {
    /// Allocate `size` bytes aligned by `alignment`.
    ///
    /// Returns a unique pointer if called with `size` 0.
    ///
    /// ## Parameters
    ///
    /// <dl>
    ///     <dt>size</dt>
    ///     <dd>number of bytes to allocate.</dd>
    ///     <dt>alignment</dt>
    ///     <dd>the minimal alignment of the allocated memory. Must be less than <code>MI_ALIGNMENT_MAX</code>.</dd>
    /// </dl>
    ///
    /// ## Returns
    ///
    /// pointer to the allocated memory or `NULL` if out of memory.
    /// The returned pointer is aligned by alignment, i.e. `(uintptr_t)p % alignment == 0`.
    pub fn mi_malloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_zalloc_aligned(size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_calloc_aligned(count: size_t, size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_realloc_aligned(p: *mut c_void, newsize: size_t, alignment: size_t) -> *mut c_void;

    pub fn mi_malloc_aligned_at(size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_zalloc_aligned_at(size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_calloc_aligned_at(count: size_t, size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_realloc_aligned_at(p: *mut c_void, newsize: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
}
//#endregion

//#region Heap Allocation
extern "C" {
    /// Type of first-class heaps.
    //
    // A heap can only be used for (re)allocation in the thread that created this heap!
    // Any allocated blocks can be freed by any other thread though.
    pub type mi_heap_t;

    pub fn mi_heap_calloc(heap: *mut mi_heap_t, count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_calloc_aligned(heap: *mut mi_heap_t, count: size_t, size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_heap_calloc_aligned_at(heap: *mut mi_heap_t, count: size_t, size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_heap_collect(heap: *mut mi_heap_t, force: bool);
    pub fn mi_heap_delete(heap: *mut mi_heap_t);
    pub fn mi_heap_destroy(heap: *mut mi_heap_t);
    pub fn mi_heap_get_backing() -> *mut mi_heap_t;
    pub fn mi_heap_get_default() -> *mut mi_heap_t;
    pub fn mi_heap_malloc(heap: *mut mi_heap_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_malloc_aligned(heap: *mut mi_heap_t, size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_heap_malloc_aligned_at(heap: *mut mi_heap_t, size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_heap_malloc_small(heap: *mut mi_heap_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_mallocn(heap: *mut mi_heap_t, count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_new() -> *mut mi_heap_t;
    pub fn mi_heap_realloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_heap_realloc_aligned(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_heap_realloc_aligned_at(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_heap_reallocf(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_heap_reallocn(heap: *mut mi_heap_t, p: *mut c_void, count: size_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_realpath(heap: *mut mi_heap_t, fname: *const c_char, resolved_name: *mut c_char) -> *mut c_char;
    pub fn mi_heap_set_default(heap: *mut mi_heap_t) -> *mut mi_heap_t;
    pub fn mi_heap_strdup(heap: *mut mi_heap_t, s: *const c_char) -> *mut c_char;
    pub fn mi_heap_strndup(heap: *mut mi_heap_t, s: *const c_char, n: size_t) -> *mut c_char;
    pub fn mi_heap_zalloc(heap: *mut mi_heap_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_zalloc_aligned(heap: *mut mi_heap_t, size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_heap_zalloc_aligned_at(heap: *mut mi_heap_t, size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
}
//#endregion

//#region Zero initialized re-allocation
extern "C" {
    pub fn mi_heap_recalloc(heap: *mut mi_heap_t, p: *mut c_void, newcount: size_t, size: size_t) -> *mut c_void;
    pub fn mi_heap_recalloc_aligned(heap: *mut mi_heap_t, p: *mut c_void, newcount: size_t, size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_heap_recalloc_aligned_at(heap: *mut mi_heap_t, p: *mut c_void, newcount: size_t, size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_heap_rezalloc(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_heap_rezalloc_aligned(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_heap_rezalloc_aligned_at(heap: *mut mi_heap_t, p: *mut c_void, newsize: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_recalloc_aligned(p: *mut c_void, newcount: size_t, size: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_recalloc_aligned_at(p: *mut c_void, newcount: size_t, size: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
    pub fn mi_rezalloc(p: *mut c_void, newsize: size_t) -> *mut c_void;
    pub fn mi_rezalloc_aligned(p: *mut c_void, newsize: size_t, alignment: size_t) -> *mut c_void;
    pub fn mi_rezalloc_aligned_at(p: *mut c_void, newsize: size_t, alignment: size_t, offset: size_t) -> *mut c_void;
}
//#endregion

//#region Heap Introspection

/// An area of heap space contains blocks of a single size.
///
/// The bytes in freed blocks are `committed - used`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mi_heap_area_t {
    /// size in bytes of one block
    pub block_size: size_t,
    /// start of the area containing heap blocks
    pub blocks: *mut c_void,
    /// current committed bytes of this area
    pub committed: *mut size_t,
    /// bytes reserved for this area
    pub reserved: size_t,
    /// bytes in use by allocated blocks
    pub used: size_t
}

/// Visitor function passed to [`mi_heap_visit_blocks()`][mi_heap_visit_blocks].
///
/// This function is always first called for every `area` with `block` as a `NULL` pointer.
/// If `visit_all_blocks` was true, the function is then called for every allocated block in that area.
///
/// <h2>Returns</h2>
///
/// `true` if ok, `false` to stop visiting (i.e. break).
pub type mi_block_visit_fun = unsafe extern "C" fn(heap: *const mi_heap_t, area: *const mi_heap_area_t, block: *mut c_void, block_size: size_t, arg: *mut c_void);

extern "C" {
    /// Check safely if any pointer is part of the default heap of this thread.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    pub fn mi_check_owned(p: *const c_void) -> bool;

    /// Check safely if any pointer is part of a heap.
    ///
    /// Note: expensive function, linear in the pages in the heap.
    pub fn mi_heap_check_owned(heap: *mut mi_heap_t, p: *const c_void) -> bool;

    /// Does a heap contain a pointer to a previously allocated block?
    pub fn mi_heap_contains_block(heap: *mut mi_heap_t, p: *const c_void) -> bool;

    /// Visit all areas and blocks in a heap.
    ///
    /// ## Parameters
    ///
    /// <dl>
    ///     <dt>heap</dt>
    ///     <dd>The heap to visit.</dd>
    ///     <dt>visit_all_blocks</dt>
    ///     <dd>If <code>true</code> visits all allocated blocks, otherwise <code>visitor</code> is only called for every heap area.</dd>
    ///     <dt>visitor</dt>
    ///     <dd>
    ///         This function is called for every area in the heap (with <code>block</code> as <code>NULL</code>).
    ///         If <code>visit_all_blocks</code> is true, <code>visitor</code> is also called for every allocated block in every area (with <code>block!=NULL</code>).
    ///         return <code>false</code> from this function to stop visiting early.
    ///     </dd>
    /// </dl>
    pub fn mi_heap_visit_blocks(heap: *mut mi_heap_t, visit_all_blocks: bool, visitor: mi_block_visit_fun, arg: *mut c_void);
}
//#endregion

//#region Runtime Options

/// Runtime options.
#[non_exhaustive]
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum mi_option_t {
    /// Print error messages to `stderr`.
    mi_option_show_errors,
    /// Print statistics to `stderr` when the program is done.
    mi_option_show_stats,
    /// Print verbose messages to `stderr`.
    mi_option_verbose,
    /// Eagerly commit segments (4MiB) (enabled by default).
    mi_option_eager_commit,
    /// Eagerly commit large (256MiB) memory regions (enabled by default, except on Windows).
    mi_option_eager_region_commit,
    /// Use large OS pages (2MiB in size) if possible.
    mi_option_large_os_pages,
    /// The number of huge OS pages (1GiB in size) to reserve at the start of the program.
    mi_option_reserve_huge_os_pages,
    /// Reserve huge OS pages at node N.
    mi_option_reserve_huge_os_pages_at,
    /// The number of segments per thread to keep cached.
    mi_option_segment_cache,
    /// Reset page memory after [`mi_option_reset_delay`][mi_option_t::mi_option_reset_delay] milliseconds when it becomes free.
    mi_option_page_reset,
    /// Experimental.
    mi_option_segment_reset,
    /// Delay in milli-seconds before resetting a page (100ms by default).
    mi_option_reset_delay,
    /// Pretend there are at most N NUMA nodes.
    mi_option_use_numa_nodes,
    /// Experimental.
    mi_option_reset_decommits,
    /// Experimental.
    mi_option_eager_commit_delay,
    /// OS tag to assign to mimalloc'd memory.
    mi_option_os_tag
}

extern "C" {
    pub fn mi_option_disable(option: mi_option_t);
    pub fn mi_option_enable(option: mi_option_t);
    pub fn mi_option_get(option: mi_option_t) -> c_long;
    pub fn mi_option_is_enabled(option: mi_option_t) -> bool;
    pub fn mi_option_set(option: mi_option_t, value: c_long);
    pub fn mi_option_set_default(option: mi_option_t, value: c_long);
    pub fn mi_option_set_enabled(option: mi_option_t, enable: bool);
    pub fn mi_option_set_enabled_default(option: mi_option_t, enable: bool);
}
//#endregion