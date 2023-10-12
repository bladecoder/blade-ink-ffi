//! C API for bladeink.

use std::{ffi::CString, os::raw::c_char};

pub mod cchoices;
pub mod cstory;
pub mod ctags;

const BINK_OK: u32 = 0;
const BINK_FAIL: u32 = 1;
const BINK_FAIL_NULL_POINTER: u32 = 2;

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_cstring_free(cstring: *mut c_char) {
    unsafe {
        if !cstring.is_null() {
            drop(CString::from_raw(cstring));
        }
    }
}