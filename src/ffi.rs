//! 
//! Rust bindings for the C API declared in `<AIR-SDK>/include/FlashRuntimeExtensions.h`.
//! 

#![allow(non_snake_case)]

use super::markers::*;
use super::types::*;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Initialization ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// Defines the signature for native calls that can be invoked via an
/// instance of the AS `ExtensionContext` class.
///
/// @return    The return value corresponds to the return value
///            from the AS `ExtensionContext` class `call()` method. It defaults to
///            [`FREResult::FRE_INVALID_OBJECT`], which is reported as `null` in AS. 
///
pub type FREFunction = unsafe extern "C" fn (
    ctx: FREContext,
    functionData: FREData,
    argc: uint32_t,
    argv: *const FREObject,
) -> FREObject;

/// See [`FREFunction`].
#[repr(C)]
#[derive(Debug)]
pub struct FRENamedFunction {
    pub name: FREStr,
    pub functionData: FREData,
    pub function: FREFunction,
}

///
/// Defines the signature for the initializer that is called each time
/// a new AS `ExtensionContext` object is created.
///
/// @param `extData` The extension client data provided to the [`FREInitializer`] function as `extDataToSet`.
///
/// @param `ctxType` Pointer to the contextType string (UTF8) as provided to the AS `createExtensionContext` call.
///
/// @param `ctx` The [`FREContext`] being initialized.
///
/// @param `numFunctionsToSet` The number of elements in the `functionsToSet` array.
///
/// @param `functionsToSet` A pointer to an array of [`FRENamedFunction`] elements.
///
pub type FREContextInitializer = unsafe extern "C" fn (
    extData: FREData,
    ctxType: FREStr,
    ctx: FREContext,
    numFunctionsToSet: *mut uint32_t,
    functionsToSet: *mut *const FRENamedFunction,
);

///
/// Defines the signature for the finalizer that is called each time
/// an `ExtensionContext` instance is disposed.
///
pub type FREContextFinalizer = Option<unsafe extern "C" fn (
    ctx: FREContext
)>;

///
/// The initialization function provided by each extension must conform
/// to the following signature.
///
/// @param `extDataToSet` Provided for the extension to store per-extension instance data. 
///            For example, if the extension creates
///            globals per-instance, it can store a pointer to them here.
///
/// @param `ctxInitializerToSet` Must be set to a pointer to a function
///            of type [`FREContextInitializer`]. Will be invoked whenever
///            the ActionScript code creates a new context for this extension.
///
/// @param `ctxFinalizerToSet` Must be set to a pointer to a function
///            of type [`FREContextFinalizer`].
///
pub type FREInitializer = unsafe extern "C" fn (
    extDataToSet: *mut FREData,
    ctxInitializerToSet: *mut FREContextInitializer,
    ctxFinalizerToSet: *mut FREContextFinalizer,
);

///
/// Called iff the extension is unloaded from the process. Extensions
/// are not guaranteed to be unloaded; the runtime process may exit without
/// doing so.
///
pub type FREFinalizer = Option<unsafe extern "C" fn (
    extData: FREData,
)>;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Result Codes ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

/// 
/// These values must not be changed.
///
/// <https://help.adobe.com/en_US/air/extensions/WSb464b1207c184b14-2c95362d12937e5c13e-7ff8.html>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FREResult(pub int32_t);
impl FREResult {
    pub const FRE_OK: Self                      = Self(0);
    pub const FRE_NO_SUCH_NAME: Self            = Self(1);
    pub const FRE_INVALID_OBJECT: Self          = Self(2);
    pub const FRE_TYPE_MISMATCH: Self           = Self(3);
    pub const FRE_ACTIONSCRIPT_ERROR: Self      = Self(4);
    pub const FRE_INVALID_ARGUMENT: Self        = Self(5);
    pub const FRE_READ_ONLY: Self               = Self(6);
    pub const FRE_WRONG_THREAD: Self            = Self(7);
    pub const FRE_ILLEGAL_STATE: Self           = Self(8);
    pub const FRE_INSUFFICIENT_MEMORY: Self     = Self(9);
    pub fn is_ok(self) -> bool {self == Self::FRE_OK}
    pub fn is_err(self) -> bool {self != Self::FRE_OK}
}
impl std::fmt::Display for FREResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {self.0.fmt(f)}
}
impl std::fmt::UpperHex for FREResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {self.0.fmt(f)}
}

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Context Data ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

unsafe extern "C" {
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] If `nativeData` is null.
///
pub fn FREGetContextNativeData(ctx: FREContext, nativeData: *mut FREData) -> FREResult;

///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRESetContextNativeData(ctx: FREContext, nativeData: FREData) -> FREResult;

///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] If `actionScriptData` is null.
///
pub fn FREGetContextActionScriptData(ctx: FREContext, actionScriptData: *mut FREObject) -> FREResult;

///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_WRONG_THREAD`]
/// 
pub fn FRESetContextActionScriptData(ctx: FREContext, actionScriptData: FREObject) -> FREResult;
}

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Primitive Types ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

/// 
/// These values must not be changed.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FREObjectType(pub int32_t);
impl FREObjectType {
    pub const FRE_TYPE_OBJECT: Self             = Self(0);
    pub const FRE_TYPE_NUMBER: Self             = Self(1);
    pub const FRE_TYPE_STRING: Self             = Self(2);
    pub const FRE_TYPE_BYTEARRAY: Self          = Self(3);
    pub const FRE_TYPE_ARRAY: Self              = Self(4);
    pub const FRE_TYPE_VECTOR: Self             = Self(5);
    pub const FRE_TYPE_BITMAPDATA: Self         = Self(6);
    pub const FRE_TYPE_BOOLEAN: Self            = Self(7);
    pub const FRE_TYPE_NULL: Self               = Self(8);
    pub fn is_null(self) -> bool {self == Self::FRE_TYPE_NULL}
}
impl std::fmt::Display for FREObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {self.0.fmt(f)}
}
impl std::fmt::UpperHex for FREObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {self.0.fmt(f)}
}

unsafe extern "C" {
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] If `objectType` is null.
///
pub fn FREGetObjectType(object: FREObject, objectType: *mut FREObjectType) -> FREResult;

///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREGetObjectAsInt32(object: FREObject, value: *mut int32_t) -> FREResult;

/// 
/// Ses [`FREGetObjectAsInt32`].
/// 
pub fn FREGetObjectAsUint32(object: FREObject, value: *mut uint32_t) -> FREResult;

/// 
/// Ses [`FREGetObjectAsInt32`].
/// 
pub fn FREGetObjectAsDouble(object: FREObject, value: *mut double) -> FREResult;

/// 
/// Ses [`FREGetObjectAsInt32`].
/// 
pub fn FREGetObjectAsBool(object: FREObject, value: *mut FREBoolean) -> FREResult;

///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
/// 
pub fn FRENewObjectFromInt32(value: int32_t, object: *mut FREObject) -> FREResult;

/// 
/// Ses [`FRENewObjectFromInt32`].
/// 
pub fn FRENewObjectFromUint32(value: uint32_t, object: *mut FREObject) -> FREResult;

/// 
/// Ses [`FRENewObjectFromInt32`].
/// 
pub fn FRENewObjectFromDouble(value: double, object: *mut FREObject) -> FREResult;
/// 
/// Ses [`FRENewObjectFromInt32`].
/// 
pub fn FRENewObjectFromBool(value: FREBoolean, object: *mut FREObject) -> FREResult;

///
/// Retrieves a string representation of the object referred to by
/// the given object. The referenced string is immutable and valid 
/// only for duration of the call to a registered function. If the 
/// caller wishes to keep the the string, they must keep a copy of it.
///
/// @param `object` The string to be retrieved.
///
/// @param `length` The size, in bytes, of the string. Includes the
///               null terminator. **(AIR-SDK-51.1.3.1) DOES NOT INCLUDE A NUL-TERMINATOR IN PRACTICE.**
///
/// @param `value`  A pointer to a possibly temporary copy of the string.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREGetObjectAsUTF8(object: FREObject, length: *mut uint32_t, value: *mut FREStr) -> FREResult;

///
/// Creates a new `String` object that contains a copy of the specified
/// string.
///
/// @param `length` The length, in bytes, of the original string. Must include
///               the null terminator.
///
/// @param `value`  A pointer to the original string.
///
/// @param `object` Receives a reference to the new string object.
/// 
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRENewObjectFromUTF8(length: uint32_t, value: FREStr, object: *mut FREObject) -> FREResult;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Utility methods ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// Outputs a trace to the ActionScript debugger
///
/// @param `strTrace` A null-terminated string to output to the debugger.
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] if `strTrace` is null
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRETrace(ctx: FREContext, strTrace: FREStr) -> FREResult;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FRERenderMode(pub int32_t);
impl FRERenderMode {
    pub const FRE_RENDERMODE_UNKNOWN: Self              = Self(0);
    pub const FRE_RENDERMODE_NONE: Self                 = Self(1);
    pub const FRE_RENDERMODE_CPU: Self                  = Self(2);
    pub const FRE_RENDERMODE_DIRECT_OGLES: Self         = Self(3);
    pub const FRE_RENDERMODE_DIRECT_OGL: Self           = Self(4);
    pub const FRE_RENDERMODE_DIRECT_D3D9: Self          = Self(5);
    pub const FRE_RENDERMODE_DIRECT_D3D10: Self         = Self(6);
    pub const FRE_RENDERMODE_DIRECT_D3D11: Self         = Self(7);
    pub const FRE_RENDERMODE_SOFTWARE_GDI: Self         = Self(8);
    pub const FRE_RENDERMODE_GPU_OGLES: Self            = Self(9);
}
impl std::fmt::Display for FRERenderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {self.0.fmt(f)}
}
impl std::fmt::UpperHex for FRERenderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {self.0.fmt(f)}
}

unsafe extern "C" {
///
/// Returns the current render mode and mechanism for AIR
///
/// @param `stage` The AS3 `Stage` object for which to return the render mode. If this parameter
///              is passed as NULL, the function will use the main/initial stage.
///
/// @param `pRenderMode` A pointer to a byte that will be populated with one
///                    of the render modes from the [`FRERenderMode`] enumeration.
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] if `pRenderMode` is null
///          [`FREResult::FRE_INVALID_OBJECT`] if `stage` is non-null but not a `Stage` object
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREGetRenderMode(ctx: FREContext, stage: FREObject, pRenderMode: *mut uint8_t) -> FREResult;

///
/// Sets a `MediaBuffer` object as the rendering source for a `DisplayObject`.
///
/// @param `source` The AS3 `MediaBuffer` object that will be used to render into the display object.
///
/// @param `target` The AS3 `DisplayObject` object that will render the contents from the `MediaBuffer`.
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] if either element is null
///          [`FREResult::FRE_INVALID_OBJECT`] if the objects are incorrect types
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRESetRenderSource(ctx: FREContext, source: FREObject, target: FREObject) -> FREResult;

///
/// Locks a `MediaBuffer` bitmap and returns information about the data storage.
///
/// Note that for every call to [`FREMediaBufferLock`], a corresponding call to [`FREMediaBufferUnlock`] must be made.
/// Ideally this would be as swift as possible, as locking a media buffer would impact rendering of any bitmap
/// it contains. The unlock must be called prior to returning from the ANE function call though.
///
/// @param `ctx` The [`FREContext`] for the native extension instance.
/// @param `mediaBuffer` The AS3 `MediaBuffer` object for which the image data is requested.
/// @param `pData` Pointer to a byte pointer that will be set to the internal image data.
/// @param `pWidth` Pointer to a value that will be set to the width of the image data.
/// @param `pHeight` Pointer to a value that will be set to the height of the image data.
/// @param `pStride` Pointer to a value that will be set to the stride i.e. number of bytes between the start of each row on the image.
/// @param `pFormat` Pointer to a value that will be set to a format value (for future usage: currently images are ARGB format).
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] if any parameter is null
///          [`FREResult::FRE_INVALID_OBJECT`] if the `MediaBuffer` objects has an incorrect type
///          [`FREResult::FRE_WRONG_THREAD`] if called from the wrong thread; at some point this function will be updated to allow multi-thread calling
///
pub fn FREMediaBufferLock(
    ctx: FREContext,
    mediaBuffer: FREObject,
    pData: *mut FREBytes,
    pWidth: *mut uint32_t,
    pHeight: *mut uint32_t,
    pStride: *mut uint32_t,
    pFormat: *mut uint32_t,
) -> FREResult;

///
/// Unlocks a `MediaBuffer` bitmap.
///
/// Currently the `bUpdate` parameter is ignored: changes to the data provided by the [`FREMediaBufferLock`] function will
/// always have an impact on the rendered bitmap. This parameter is provided to allow for future flexibility.
///
/// @param `ctx` The [`FREContext`] for the native extension instance.
/// @param `mediaBuffer` The AS3 `MediaBuffer` object for which the image data is released.
/// @param `bUpdate` Whether to use the updated data for rendering. Note that this parameter may not have any effect.
///
/// @returns [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] if the `ctx` or `mediaBuffer` parameter is null
///          [`FREResult::FRE_INVALID_OBJECT`] if the `MediaBuffer` objects has an incorrect type
///          [`FREResult::FRE_WRONG_THREAD`] if called from the wrong thread; at some point this function will be updated to allow multi-thread calling
///
pub fn FREMediaBufferUnlock(ctx: FREContext, mediaBuffer: FREObject, bUpdate: uint32_t) -> FREResult;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Object Access ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// @param `className` UTF8-encoded name of the class being constructed.
///
/// @param `thrownException` A pointer to a handle that can receive the handle of any ActionScript 
///            `Error` thrown during execution. May be null if the caller does not
///            want to receive this handle. If not null and no error occurs, is set an
///            invalid handle value.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_ACTIONSCRIPT_ERROR`] If an ActionScript exception results from calling this method.
///              In this case, `thrownException` will be set to the handle of the thrown value. 
///          [`FREResult::FRE_ILLEGAL_STATE`] If a `ByteArray` or `BitmapData` has been acquired and not yet released.
///          [`FREResult::FRE_NO_SUCH_NAME`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRENewObject(
    className: FREStr,
    argc: uint32_t,
    argv: *mut FREObject,
    object: *mut FREObject,
    thrownException: *mut FREObject,
) -> FREResult;

///
/// @param `propertyName` UTF8-encoded name of the property being fetched.
///
/// @param `thrownException` A pointer to a handle that can receive the handle of any ActionScript 
///            `Error` thrown during getting the property. May be null if the caller does not
///            want to receive this handle. If not null and no error occurs, is set an
///            invalid handle value.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_ACTIONSCRIPT_ERROR`] If an ActionScript exception results from getting this property.
///              In this case, `thrownException` will be set to the handle of the thrown value. 
///          [`FREResult::FRE_NO_SUCH_NAME`] If the named property doesn't exist, or if the reference is ambiguous
///              because the property exists in more than one namespace.
///          [`FREResult::FRE_ILLEGAL_STATE`] If a `ByteArray` or `BitmapData` has been acquired and not yet released.
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREGetObjectProperty(
    object: FREObject,
    propertyName: FREStr,
    propertyValue: *mut FREObject,
    thrownException: *mut FREObject,
) -> FREResult;

///
/// @param `propertyName` UTF8-encoded name of the property being set.
///
/// @param `thrownException` A pointer to a handle that can receive the handle of any ActionScript 
///            `Error` thrown during method execution. May be null if the caller does not
///            want to receive this handle. If not null and no error occurs, is set an
///            invalid handle value.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_ACTIONSCRIPT_ERROR`] If an ActionScript exception results from getting this property.
///              In this case, `thrownException` will be set to the handle of the thrown value. 
///          [`FREResult::FRE_NO_SUCH_NAME`] If the named property doesn't exist, or if the reference is ambiguous
///              because the property exists in more than one namespace.
///          [`FREResult::FRE_ILLEGAL_STATE`] If a `ByteArray` or `BitmapData` has been acquired and not yet released.
///          [`FREResult::FRE_READ_ONLY`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRESetObjectProperty(
    object: FREObject,
    propertyName: FREStr,
    propertyValue: FREObject,
    thrownException: *mut FREObject,
) -> FREResult;

///
/// @param `methodName` UTF8-encoded null-terminated name of the method being invoked.
///
/// @param `thrownException` A pointer to a handle that can receive the handle of any ActionScript 
///            `Error` thrown during method execution. May be null if the caller does not
///            want to receive this handle. If not null and no error occurs, is set an
///            invalid handle value.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_ACTIONSCRIPT_ERROR`] If an ActionScript exception results from calling this method.
///              In this case, `thrownException` will be set to the handle of the thrown value. 
///          [`FREResult::FRE_NO_SUCH_NAME`] If the named method doesn't exist, or if the reference is ambiguous
///              because the method exists in more than one namespace.
///          [`FREResult::FRE_ILLEGAL_STATE`] If a `ByteArray` or `BitmapData` has been acquired and not yet released.
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRECallObjectMethod(
    object: FREObject,
    methodName: FREStr,
    argc: uint32_t,
    argv: *mut FREObject,
    result: *mut FREObject,
    thrownException: *mut FREObject,
) -> FREResult;
}

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BitmapData Access ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

#[derive(Debug, Default)]
#[repr(C)]
pub struct FREBitmapData {
    /// width of the `BitmapData` bitmap
    pub width: uint32_t,
    /// height of the `BitmapData` bitmap
    pub height: uint32_t,
    /// if non-zero, pixel format is ARGB32, otherwise pixel format is _RGB32, host endianness
    pub hasAlpha: FREBoolean,
    /// pixel color values are premultiplied with alpha if non-zero, un-multiplied if zero
    pub isPremultiplied: FREBoolean,
    /// line stride in number of 32 bit values, typically the same as `width`
    pub lineStride32: uint32_t,
    /// pointer to the first 32-bit pixel of the bitmap data
    pub bits32: *mut uint32_t,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct FREBitmapData2 {
    /// width of the `BitmapData` bitmap
    pub width: uint32_t,
    /// height of the `BitmapData` bitmap
    pub height: uint32_t,
    /// if non-zero, pixel format is ARGB32, otherwise pixel format is _RGB32, host endianness
    pub hasAlpha: FREBoolean,
    /// pixel color values are premultiplied with alpha if non-zero, un-multiplied if zero
    pub isPremultiplied: FREBoolean,
    /// line stride in number of 32 bit values, typically the same as `width`
    pub lineStride32: uint32_t,
    /// if non-zero, last row of pixels starts at bits32, otherwise, first row of pixels starts at bits32.
    pub isInvertedY: FREBoolean,
    /// pointer to the first 32-bit pixel of the bitmap data
    pub bits32: *mut uint32_t,
}

unsafe extern "C" {
///
/// Referenced data is valid only for duration of the call
/// to a registered function.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FREAcquireBitmapData(
    object: FREObject,
    descriptorToSet: *mut FREBitmapData,
) -> FREResult;

///
/// Referenced data is valid only for duration of the call
/// to a registered function.
///
/// Use of this API requires that the extension and application must be packaged for 
/// the 3.1 namespace or later.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FREAcquireBitmapData2(
    object: FREObject,
    descriptorToSet: *mut FREBitmapData2
) -> FREResult;

///
/// `BitmapData` must be acquired to call this. Clients must invalidate any region
/// they modify in order to notify AIR of the changes. Only invalidated regions
/// are redrawn.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///
pub fn FREInvalidateBitmapDataRect(
    object: FREObject,
    x: uint32_t,
    y: uint32_t,
    width: uint32_t,
    height: uint32_t,
) -> FREResult;

///
/// Release data that has been acquired with an earlier call to [`FREAcquireBitmapData`] or [`FREAcquireBitmapData2`]
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///
pub fn FREReleaseBitmapData(object: FREObject) -> FREResult;
}

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ ByteArray Access ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

#[derive(Debug)]
#[repr(C)]
pub struct FREByteArray {
    pub length: uint32_t,
    pub bytes: FREBytes,
}

unsafe extern "C" {
///
/// Creates a new byte array using optional input information (`length` and optional `byte`)
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FRENewByteArray(
    byteArrayData: *mut FREByteArray,
    handle: *mut FREObject,
) -> FREResult;

///
/// Referenced data is valid only for duration of the call
/// to a registered function.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FREAcquireByteArray(
    object: FREObject,
    byteArrayToSet: *mut FREByteArray,
) -> FREResult;

///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREReleaseByteArray(object: FREObject) -> FREResult;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Array and Vector Access ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREGetArrayLength(
    arrayOrVector: FREObject,
    length: *mut uint32_t,
) -> FREResult;

///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] If `length` is greater than 2^32.
///          [`FREResult::FRE_READ_ONLY`]   If the handle refers to a `Vector`
///              of fixed size.
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_INSUFFICIENT_MEMORY`]
///
pub fn FRESetArrayLength(
    arrayOrVector: FREObject,
    length: uint32_t,
) -> FREResult;

///
/// If an `Array` is sparse and an element that isn't defined is requested, the
/// return value will be FRE_OK but the handle value will be invalid.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_INVALID_ARGUMENT`] If the handle refers to a vector and the index is
///              greater than the size of the array.
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREGetArrayElementAt(
    arrayOrVector: FREObject,
    index: uint32_t,
    value: *mut FREObject,
) -> FREResult;

///
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_TYPE_MISMATCH`] If an attempt to made to set a value in a `Vector`
///              when the type of the value doesn't match the `Vector`'s item type.
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FRESetArrayElementAt(
    arrayOrVector: FREObject,
    index: uint32_t,
    value: FREObject,
) -> FREResult;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ NativeWindow Access ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// Referenced handle is valid only for duration of the call
/// to a registered function.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FREAcquireNativeWindowHandle(
    nativeWindow: FREObject,
    handle: *mut FRENativeWindow,
) -> FREResult;

///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///          [`FREResult::FRE_WRONG_THREAD`]
///
pub fn FREReleaseNativeWindowHandle(nativeWindow: FREObject) -> FREResult;

///
/// Access the native context type for a `Context3D` object. The type depends on the
/// render mode: currently this is only valid for OpenGL ES rendering and will return
/// an `EGLContext` object. This object should be valid until the `Context3D` object is
/// disposed, or the device/application loses the graphics context.
///
/// **Example usage in C :**
/// ```c
/// EGLContext airContext = EGL_NO_CONTEXT;
/// if ( (FRE_OK == FREGetNativeContext3DHandle(args[0], (void**)&airContext)) && (airContext != EGL_NO_CONTEXT) )
/// // ... do something with the handle ...
/// ```
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FREGetNativeContext3DHandle(
    context3D: FREObject,
    handle: *mut FREHandle,
) -> FREResult;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Extension Context Access ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// Get the [`FREContext`] object associated with any `ExtensionContext` object. Note that
/// the [`FREContext`] object may become invalid based on what happens with the other `ExtensionContext`,
/// so this value should not be cached between function calls.
///
/// @param `objExtensionContext` The `ExtensionContext` (ActionScript object) for which the [`FREContext`] handle is required.
///
/// @param `pContext` The [`FREContext`] (C handle) associated with the given `ExtensionContext`.
///
/// @return  [`FREResult::FRE_OK`]
///          [`FREResult::FRE_TYPE_MISMATCH`]
///          [`FREResult::FRE_INVALID_OBJECT`]
///          [`FREResult::FRE_INVALID_ARGUMENT`]
///          [`FREResult::FRE_WRONG_THREAD`]
///          [`FREResult::FRE_ILLEGAL_STATE`]
///
pub fn FREGetFREContextFromExtensionContext(
    objExtensionContext: FREObject,
    pContext: *mut FREContext,
) -> FREResult;

// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Callbacks ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫

///
/// Causes a `StatusEvent` to be dispatched from the associated
/// `ExtensionContext` object.
///
/// Dispatch happens asynchronously, even if this is called during
/// a call to a registered function.
///
/// The ActionScript portion of this extension can listen for that event
/// and, upon receipt, query the native portion for details of the event
/// that occurred.
///
/// This call is thread-safe and may be invoked from any thread. The string
/// values are copied before the call returns.
///
/// @return  [`FREResult::FRE_OK`] In all circumstances, as the referenced object cannot
///              necessarily be checked for validity on the invoking thread.
///              However, no event will be dispatched if the object is
///              invalid or not an `EventDispatcher`.
///          [`FREResult::FRE_INVALID_ARGUMENT`] If code or level is NULL
///
pub fn FREDispatchStatusEventAsync(
    ctx: FREContext,
    code: FREStr,
    level: FREStr,
) -> FREResult;
}