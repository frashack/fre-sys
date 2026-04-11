# fre-sys

Rust bindings for the AIR SDK C API (`FlashRuntimeExtensions.h`), used to build Flash Runtime Extensions (FRE).

This crate requires the AIR SDK.

The AIR SDK is currently maintained by Harman (Samsung), originally developed by Adobe.

You must obtain the AIR SDK from the official distributor (currently Harman) and set it up manually.

This project does not distribute any proprietary SDK files.

Most users should use `fre-rs`, which provides a safer and more ergonomic abstraction over this crate.

## References

- https://airsdk.harman.com/
- https://airsdk.dev/docs
- https://airsdk.dev/reference/actionscript/3.0/
- https://help.adobe.com/en_US/air/extensions/index.html
- https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/index.html

## Safety

This crate provides low-level bindings over the AIR runtime:

- Underlying objects may be modified externally by the runtime.
- No guarantee of exclusive access.
- Correct usage of the AIR C API is required.

Users must ensure correct lifetimes and ownership assumptions.

## License

MIT OR Apache-2.0

## Example

```rust
use fre_sys::prelude::*;
#[allow(unsafe_op_in_unsafe_fn, non_snake_case, unused_variables)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Initializer (
    ext_data_to_set: *mut FREData,
    ctx_initializer_to_set: *mut FREContextInitializer,
    ctx_finalizer_to_set: *mut FREContextFinalizer,
) {
    *ctx_initializer_to_set = ctx_initializer;
}
#[allow(unsafe_op_in_unsafe_fn, unused_variables)]
unsafe extern "C" fn ctx_initializer (
    ext_data: FREData,
    ctx_type: FREStr,
    ctx: FREContext,
    num_funcs_to_set: *mut u32,
    funcs_to_set: *mut *const FRENamedFunction,
) {
    *num_funcs_to_set = 1;
    *funcs_to_set = std::mem::transmute(&FUNC);
}
#[repr(transparent)]
struct Function (FRENamedFunction);
unsafe impl Sync for Function {}
static FUNC: Function = Function(FRENamedFunction {
    name: c"hello_extension".as_ptr() as FREStr,
    functionData: std::ptr::null_mut(),
    function: hello,
});
unsafe extern "C" fn hello(_: FREContext, _: FREData, _: u32, _: *const FREObject) -> FREObject {
    let bytes = "Hello! Flash Runtime".as_bytes();
    let mut str_obj = std::ptr::null_mut();
    _ = unsafe {FRENewObjectFromUTF8(bytes.len() as u32, bytes.as_ptr(), &mut str_obj)};
    str_obj
}
```
