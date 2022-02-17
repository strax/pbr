//! Low-level bindings to the [OpenEXRCore][1] library.
//!
//! [1]: https://openexr.readthedocs.io/en/latest/OpenEXRCoreAPI.html

#![allow(bad_style)]
#![feature(extern_types)]
#![no_std]

pub mod base;
pub mod errors;
pub mod context;
pub mod attr;
pub mod debug;
pub mod encode;
pub mod coding;
pub mod part;
pub mod chunkio;
pub mod decode;

pub use base::*;
pub use errors::*;
pub use context::*;
pub use attr::*;
pub use part::*;
pub use chunkio::*;
pub use encode::*;
pub use decode::*;
pub use debug::*;