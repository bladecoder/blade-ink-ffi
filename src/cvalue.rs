use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use bladeink::value_type::ValueType;

use crate::{BINK_FAIL, BINK_FAIL_NULL_POINTER, BINK_OK};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_new_bool(value: bool) -> *mut ValueType {
    let v = ValueType::Bool(value);

    let value_type = Box::into_raw(Box::new(v));

    return value_type;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_new_int(value: i32) -> *mut ValueType {
    let v = ValueType::Int(value);

    let value_type = Box::into_raw(Box::new(v));

    return value_type;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_new_float(value: f32) -> *mut ValueType {
    let v = ValueType::Float(value);

    let value_type = Box::into_raw(Box::new(v));

    return value_type;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_new_string(value: *const c_char) -> *mut ValueType {
    let value_c_str: &CStr = unsafe { CStr::from_ptr(value) };
    let value_str_slice: &str = value_c_str.to_str().unwrap();

    let v = ValueType::new_string(value_str_slice);

    let value_type = Box::into_raw(Box::new(v));

    return value_type;
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_get_bool(
    value_type: *const ValueType,
    value: *mut bool,
    err_msg: *mut *mut c_char,
) -> u32 {
    if value_type.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let v: &ValueType = unsafe { &*value_type };

    let result = v.coerce_to_bool();

    match result {
        Ok(v) => unsafe {
            *value = v;
            BINK_OK
        },
        Err(e) => unsafe {
            *err_msg = CString::new(e.to_string()).unwrap().into_raw();
            BINK_FAIL
        },
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_get_int(
    value_type: *const ValueType,
    value: *mut i32,
    err_msg: *mut *mut c_char,
) -> u32 {
    if value_type.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let v: &ValueType = unsafe { &*value_type };

    let result = v.coerce_to_int();

    match result {
        Ok(v) => unsafe {
            *value = v;
            BINK_OK
        },
        Err(e) => unsafe {
            *err_msg = CString::new(e.to_string()).unwrap().into_raw();
            BINK_FAIL
        },
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_get_float(
    value_type: *const ValueType,
    value: *mut f32,
    err_msg: *mut *mut c_char,
) -> u32 {
    if value_type.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let v: &ValueType = unsafe { &*value_type };

    let result = v.coerce_to_float();

    match result {
        Ok(v) => unsafe {
            *value = v;
            BINK_OK
        },
        Err(e) => unsafe {
            *err_msg = CString::new(e.to_string()).unwrap().into_raw();
            BINK_FAIL
        },
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_get_string(
    value_type: *const ValueType,
    value: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if value_type.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let v: &ValueType = unsafe { &*value_type };

    let result = v.coerce_to_string();

    match result {
        Ok(v) => unsafe {
            *value = CString::new(v).unwrap().into_raw();
            BINK_OK
        },
        Err(e) => unsafe {
            *err_msg = CString::new(e.to_string()).unwrap().into_raw();
            BINK_FAIL
        },
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_value_free(value: *mut ValueType) {
    if !value.is_null() {
        unsafe {
            drop(Box::from_raw(value));
        }
    }
}
