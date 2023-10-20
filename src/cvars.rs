use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use bladeink::{story::Story, value_type::ValueType};

use crate::{BINK_FAIL, BINK_FAIL_NULL_POINTER, BINK_OK};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_get_int(
    story: *mut Story,
    name: *const c_char,
    value: *mut i32,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.get_variable(name_str_slice);

    match result {
        Some(v) => unsafe {
            match v.coerce_to_int() {
                Ok(v) => *value = v,
                Err(msg) => {
                    *err_msg = CString::new(msg.to_string()).unwrap().into_raw();
                    return BINK_FAIL;
                }
            }
        },
        None => {
            unsafe {
                *err_msg = CString::new(format!("Variable '{name_str_slice}' not found."))
                    .unwrap()
                    .into_raw();
            }
            return BINK_FAIL;
        }
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_set_int(
    story: *mut Story,
    name: *const c_char,
    value: i32,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let val = ValueType::Int(value);
    let result = story.set_variable(name_str_slice, &val);

    if let Err(err) = result {
        unsafe {
            *err_msg = CString::new(err.to_string()).unwrap().into_raw();
        }
        return BINK_FAIL;
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_get_float(
    story: *mut Story,
    name: *const c_char,
    value: *mut f32,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.get_variable(name_str_slice);

    match result {
        Some(v) => unsafe {
            match v.coerce_to_float() {
                Ok(v) => *value = v,
                Err(msg) => {
                    *err_msg = CString::new(msg.to_string()).unwrap().into_raw();
                    return BINK_FAIL;
                }
            }
        },
        None => {
            unsafe {
                *err_msg = CString::new(format!("Variable '{name_str_slice}' not found."))
                    .unwrap()
                    .into_raw();
            }
            return BINK_FAIL;
        }
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_set_float(
    story: *mut Story,
    name: *const c_char,
    value: f32,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let val = ValueType::Float(value);
    let result = story.set_variable(name_str_slice, &val);

    if let Err(err) = result {
        unsafe {
            *err_msg = CString::new(err.to_string()).unwrap().into_raw();
        }
        return BINK_FAIL;
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_get_bool(
    story: *mut Story,
    name: *const c_char,
    value: *mut bool,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.get_variable(name_str_slice);

    match result {
        Some(v) => unsafe {
            match v.coerce_to_bool() {
                Ok(v) => *value = v,
                Err(msg) => {
                    *err_msg = CString::new(msg.to_string()).unwrap().into_raw();
                    return BINK_FAIL;
                }
            }
        },
        None => {
            unsafe {
                *err_msg = CString::new(format!("Variable '{name_str_slice}' not found."))
                    .unwrap()
                    .into_raw();
            }
            return BINK_FAIL;
        }
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_set_bool(
    story: *mut Story,
    name: *const c_char,
    value: bool,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let val = ValueType::Bool(value);
    let result = story.set_variable(name_str_slice, &val);

    if let Err(err) = result {
        unsafe {
            *err_msg = CString::new(err.to_string()).unwrap().into_raw();
        }
        return BINK_FAIL;
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_get_string(
    story: *mut Story,
    name: *const c_char,
    value: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.get_variable(name_str_slice);

    match result {
        Some(v) => unsafe {
            match v.coerce_to_string() {
                Ok(v) => *value = CString::new(v).unwrap().into_raw(),
                Err(msg) => {
                    *err_msg = CString::new(msg.to_string()).unwrap().into_raw();
                    return BINK_FAIL;
                }
            }
        },
        None => {
            unsafe {
                *err_msg = CString::new(format!("Variable '{name_str_slice}' not found."))
                    .unwrap()
                    .into_raw();
            }
            return BINK_FAIL;
        }
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_set_string(
    story: *mut Story,
    name: *const c_char,
    value: *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let value_c_str: &CStr = unsafe { CStr::from_ptr(value) };
    let value_str_slice: &str = value_c_str.to_str().unwrap();

    let val = ValueType::new_string(value_str_slice);
    let result = story.set_variable(name_str_slice, &val);

    if let Err(err) = result {
        unsafe {
            *err_msg = CString::new(err.to_string()).unwrap().into_raw();
        }
        return BINK_FAIL;
    }

    BINK_OK
}
