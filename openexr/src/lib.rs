//! High-level bindings for OpenEXR.
//!
//! OpenEXR provides the specification and reference implementation of the EXR file format, the professional-grade image storage format of the motion picture industry.
//!
//! The purpose of EXR format is to accurately and efficiently represent high-dynamic-range scene-linear image data and associated metadata, with strong support for multi-part, multi-channel use cases.
//!
//! OpenEXR is widely used in host application software where accuracy is critical, such as photorealistic rendering, texture access, image compositing, deep compositing, and DI.
//!
//! # About OpenEXR
//!
//! OpenEXR is a project of the Academy Software Foundation. The format and library were originally developed by Industrial Light & Magic and first released in 2003. Weta Digital, Walt Disney Animation Studios, Sony Pictures Imageworks, Pixar Animation Studios, DreamWorks, and other studios, companies, and individuals have made contributions to the code base.
//!
//! OpenEXR is included in the VFX Reference Platform.

#![feature(backtrace)]
#![feature(const_trait_impl)]
#![feature(const_mut_refs)]
#![feature(const_option_ext)]
#![feature(decl_macro)]
#![feature(allocator_api)]
#![feature(ptr_to_from_bits)]
#![feature(int_roundings)]
#![feature(min_specialization)]

pub use openexr_sys as sys;

pub mod core;

pub use crate::core::error::{Error};
