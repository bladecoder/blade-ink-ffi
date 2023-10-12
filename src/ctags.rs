use std::{ffi::CString, os::raw::c_char};

use crate::{BINK_FAIL, BINK_FAIL_NULL_POINTER, BINK_OK};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_tags_get(
    tags: *const Vec<String>,
    idx: usize,
    tag: *mut *mut c_char,
) -> u32 {
    if tags.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let tags: &Vec<String> = unsafe { &*tags };

    let t = tags.get(idx);

    match t {
        Some(t) => unsafe {
            *tag = CString::new(t.as_str()).unwrap_or_default().into_raw();
        },
        None => {
            return BINK_FAIL;
        }
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_tags_free(tags: *mut Vec<String>) {
    if !tags.is_null() {
        unsafe {
            drop(Box::from_raw(tags));
        }
    }
}
