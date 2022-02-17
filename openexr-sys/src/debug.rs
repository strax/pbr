use libc::c_int;

use crate::errors::*;
use crate::context::*;

extern "C" {
    /// Debug function: print to stdout the parts and attributes of the context `c`.
    pub fn exr_print_context_info(c: exr_const_context_t, verbose: c_int) -> exr_result_t;
}