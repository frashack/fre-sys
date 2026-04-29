//! 
//! Type aliases for C API–compatible pointer types and semantic markers used by the extension API.
//! 


use crate::types::*;


/// Opaque pointer to an unspecified data type.
/// 
pub type FREHandle = *mut void;

/// Handle to user-defined data owned and managed by extensions.
/// 
pub type FREData = FREHandle;

/// Handle to an extension context managed by the runtime.
/// 
/// This is **NOT** the AS3 `ExtensionContext` object itself,
/// but a low-level handle associated with it.
/// 
pub type FREContext = FREHandle;

/// Handle to an AS3 object.
/// 
pub type FREObject = FREHandle;

/// Handle to a native window; the underlying type is platform-specific.
/// 
pub type FRENativeWindow = FREHandle;

/// Pointer to a byte buffer.
/// 
pub type FREBytes = *mut uint8_t;

/// Borrowed pointer to UTF-8 string.
/// 
/// This is a pointer to the beginning of the string buffer only.
/// Length and termination are determined by the API contract:
/// - If a length is provided alongside this pointer, the string is
/// interpreted as a byte slice of that length (not NUL-terminated).
/// - If no length is provided, the string is interpreted as NUL-terminated.
/// 
pub type FREStr = *const uint8_t;

/// 32-bit integer representation of an ActionScript `Boolean`.
/// 
/// A non-zero value represents `true`, and 0 represents `false`.
/// 
pub type FREBoolean = uint32_t;

