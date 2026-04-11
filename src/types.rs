//! 
//! C-compatible type aliases.
//! 

#![allow(non_camel_case_types)]

use std::ffi::{c_void};

pub type void = c_void;
pub type uint8_t  = u8;
pub type uint32_t = u32;
pub type int32_t  = i32;
pub type double = f64;