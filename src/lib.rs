//!
//! # fre-sys
//! 
//! Rust bindings for the AIR Native Extension (ANE) C API (`FlashRuntimeExtensions.h`).
//! 
//! ## Example
//!
//! ```rust
//! use fre_sys::prelude::*;
//! #[allow(unsafe_op_in_unsafe_fn, non_snake_case, unused_variables)]
//! #[unsafe(no_mangle)]
//! pub unsafe extern "C" fn Initializer (
//!     ext_data_to_set: *mut FREData,
//!     ctx_initializer_to_set: *mut FREContextInitializer,
//!     ctx_finalizer_to_set: *mut FREContextFinalizer,
//! ) {
//!     *ctx_initializer_to_set = ctx_initializer;
//! }
//! #[allow(unsafe_op_in_unsafe_fn, unused_variables)]
//! unsafe extern "C" fn ctx_initializer (
//!     ext_data: FREData,
//!     ctx_type: FREStr,
//!     ctx: FREContext,
//!     num_funcs_to_set: *mut u32,
//!     funcs_to_set: *mut *const FRENamedFunction,
//! ) {
//!     *num_funcs_to_set = 1;
//!     *funcs_to_set = std::mem::transmute(&FUNC);
//! }
//! #[repr(transparent)]
//! struct Function (FRENamedFunction);
//! unsafe impl Sync for Function {}
//! static FUNC: Function = Function(FRENamedFunction {
//!     name: c"hello_extension".as_ptr() as FREStr,
//!     functionData: std::ptr::null_mut(),
//!     function: hello,
//! });
//! unsafe extern "C" fn hello(_: FREContext, _: FREData, _: u32, _: *const FREObject) -> FREObject {
//!     let bytes = "Hello! Flash Runtime".as_bytes();
//!     let mut str_obj = std::ptr::null_mut();
//!     _ = unsafe {FRENewObjectFromUTF8(bytes.len() as u32, bytes.as_ptr(), &mut str_obj)};
//!     str_obj
//! }
//! ```
//! 

pub mod ffi;
pub mod markers;
pub mod types;
pub mod prelude {
    pub use crate::{
        ffi::*,
        markers::*,
    };
}