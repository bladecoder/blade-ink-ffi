use std::{
    cell::RefCell,
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    rc::Rc,
};

use bladeink::{
    story::{variable_observer::VariableObserver, Story},
    value_type::ValueType,
};

use crate::{BINK_FAIL, BINK_FAIL_NULL_POINTER, BINK_OK};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_get(
    story: *mut Story,
    name: *const c_char,
    value: *mut *mut ValueType,
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
            *value = Box::into_raw(Box::new(v));
            BINK_OK
        },
        None => {
            unsafe {
                *err_msg = CString::new(format!("Variable '{name_str_slice}' not found."))
                    .unwrap()
                    .into_raw();
            }
            BINK_FAIL
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_var_set(
    story: *mut Story,
    name: *const c_char,
    value: *const ValueType,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let val: &ValueType = unsafe { &*value };
    let result = story.set_variable(name_str_slice, val);

    if let Err(err) = result {
        unsafe {
            *err_msg = CString::new(err.to_string()).unwrap().into_raw();
        }
        return BINK_FAIL;
    }

    BINK_OK
}

struct VObserver {
    callback: unsafe extern "C" fn(*const c_char, *const ValueType, *const c_void),
    user_data: *const c_void,
}

impl VariableObserver for VObserver {
    fn changed(&mut self, variable_name: &str, new_value: &ValueType) {
        let cname = CString::new(variable_name).unwrap().into_raw();
        let cnew_value = Box::into_raw(Box::new(new_value.clone()));
        unsafe {
            (self.callback)(cname, cnew_value, self.user_data);
            drop(CString::from_raw(cname));
            drop(Box::from_raw(cnew_value));
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_observe_variable(
    story: *mut Story,
    variable_name: *const c_char,
    observer: unsafe extern "C" fn(*const c_char, *const ValueType, *const c_void),
    user_data: *const c_void,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || variable_name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(variable_name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.observe_variable(
        name_str_slice,
        Rc::new(RefCell::new(VObserver {
            callback: observer,
            user_data,
        })),
    );

    if let Err(err) = result {
        unsafe {
            *err_msg = CString::new(err.to_string()).unwrap().into_raw();
        }
        return BINK_FAIL;
    }

    BINK_OK
}
