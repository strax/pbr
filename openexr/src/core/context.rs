use std::alloc::{Allocator, Global, Layout};
use std::ffi::CString;
use std::{cmp, io, slice};
use std::io::{Seek, SeekFrom, Write};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::path::Path;
use libc::{c_char, c_void, ENOTSUP, size_t};
use parking_lot::Mutex;
use errno::{Errno, set_errno};

use os_str_bytes::OsStrBytes;
use log::trace;

use crate::sys::*;
use super::error::{Error, Result};

use exr_default_write_mode_t::*;
use openexr_sys::exr_error_code_t::EXR_ERR_WRITE_IO;
use crate::core::alloc::{exr_alloc, exr_free};

#[repr(transparent)]
struct RawContext(exr_context_t);

impl RawContext {
    pub fn start_read(filename: &Path, init: &ContextInitializer) -> Result<RawContext> {
        let filename = CString::new(filename.as_os_str().to_raw_bytes()).unwrap();
        let mut ctxt: MaybeUninit<exr_context_t> = MaybeUninit::uninit();
        unsafe {
            trace!("exr_start_read");
            Error::from_extern(exr_start_read(ctxt.as_mut_ptr(), filename.as_ptr(), &init.0))?;
            Ok(RawContext(ctxt.assume_init()))
        }
    }

    pub fn start_write(filename: &Path, init: &ContextInitializer) -> Result<RawContext> {
        let filename = CString::new(filename.as_os_str().to_raw_bytes()).unwrap();
        let mut ctxt: MaybeUninit<exr_context_t> = MaybeUninit::uninit();
        let default_write_mode: DefaultWriteMode = DefaultWriteMode::Direct;
        unsafe {
            trace!("exr_start_write");
            Error::from_extern(exr_start_write(ctxt.as_mut_ptr(), filename.as_ptr(), default_write_mode.into(), &init.0))?;
            Ok(RawContext(ctxt.assume_init()))
        }
    }
}

impl Drop for RawContext {
    fn drop(&mut self) {
        unsafe {
            trace!("exr_finish");
            exr_finish(&mut self.0);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum DefaultWriteMode {
    Direct,
    IntermediateTempFile
}

impl Into<exr_default_write_mode_t> for DefaultWriteMode {
    fn into(self) -> exr_default_write_mode_t {
        match self {
            DefaultWriteMode::Direct => EXR_WRITE_FILE_DIRECTLY,
            DefaultWriteMode::IntermediateTempFile => EXR_INTERMEDIATE_TEMP_FILE
        }
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ContextInitializer(exr_context_initializer_t);

pub struct WriteContext<W: Write + Seek, A: Allocator = Global> {
    ctxt: RawContext,
    writer: W,
    _phantom: PhantomData<A>
}

impl<W: Write + Seek> WriteContext<W, Global> {


    pub fn new(writer: W) -> Result<Self> {
        let writer = Box::into_raw(Box::new(Mutex::new(writer)));
        let mut initializer = exr_context_initializer_t::default();
        initializer.alloc_fn = Some(exr_alloc);
        initializer.free_fn = Some(exr_free);
        initializer.user_data = writer.cast();
        initializer.write_fn = Some(<W as WriteContextOps>::write);
        todo!()
    }
}

trait WriteContextOps {
    unsafe extern "C" fn write(
        ctxt: exr_const_context_t,
        userdata: *mut c_void,
        buffer: *const c_void,
        sz: u64,
        offset: u64,
        error_cb: exr_stream_error_func_ptr_t
    ) -> i64;
}

impl<W: Write + Seek> WriteContextOps for W {
    default unsafe extern "C" fn write(
        ctxt: exr_const_context_t,
        userdata: *mut c_void,
        buffer: *const c_void,
        sz: u64,
        offset: u64,
        error_cb: exr_stream_error_func_ptr_t
    ) -> i64 {
        let mutex: &Mutex<W> = &*userdata.cast();
        let mut writer = mutex.lock();
        let src = slice::from_raw_parts(buffer.cast::<u8>(), cmp::min(sz as usize, i64::MAX as usize));
        if let Err(err) = writer.seek(SeekFrom::Start(offset)) {
            // Release the mutex in case error_cb unwinds
            drop(writer);
            let message = CString::new(err.to_string()).unwrap();
            (error_cb.unwrap_unchecked())(ctxt, EXR_ERR_WRITE_IO as exr_result_t, message.as_ptr());
            return -1;
        }
        match writer.write(src) {
            Ok(written) => written as i64,
            Err(err) => {
                // Release the mutex in case error_cb unwinds
                drop(writer);
                let message = CString::new(err.to_string()).unwrap();
                (error_cb.unwrap_unchecked())(ctxt, EXR_ERR_WRITE_IO as exr_result_t, message.as_ptr());
                return -1;
            }
        }
    }
}