use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::ffi::CStr;
use std::fmt::{Debug, Formatter};
use std::ptr;
use libc::c_char;
use semver::{BuildMetadata, Prerelease, Version};

use crate::sys::*;

pub mod context;
pub mod error;
mod alloc;

pub fn version() -> Version {
    let mut major: i32 = 0;
    let mut minor: i32 = 0;
    let mut patch: i32 = 0;
    let mut build: BuildMetadata = BuildMetadata::EMPTY;

    unsafe {
        let mut extra: *const c_char = ptr::null();
        exr_get_library_version(&mut major, &mut minor, &mut patch, &mut extra);
        if !extra.is_null() {
            let str = CStr::from_ptr(extra).to_string_lossy();
            if !str.is_empty() {
                if let Ok(b) = BuildMetadata::new(&str) {
                    build = b;
                }
            }
        }
    }

    Version {
        major: major as u64,
        minor: minor as u64,
        patch: patch as u64,
        pre: Prerelease::EMPTY,
        build
    }
}