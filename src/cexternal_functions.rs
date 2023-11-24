use std::{
    cell::RefCell,
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
    rc::Rc,
};

use bladeink::{
    story::{external_functions::ExternalFunction, Story},
    value_type::ValueType,
};

use crate::{BINK_FAIL, BINK_FAIL_NULL_POINTER, BINK_OK};

struct ExtFun {
    callback: unsafe extern "C" fn(
        *const c_char,
        *const Vec<ValueType>,
        *const c_void,
    ) -> *const ValueType,
    user_data: *const c_void,
}

impl ExternalFunction for ExtFun {
    fn call(&mut self, fun_name: &str, args: Vec<ValueType>) -> Option<ValueType> {
        let cname = CString::new(fun_name).unwrap().into_raw();
        let cargs = Box::into_raw(Box::new(args));
        unsafe {
            let ret = (self.callback)(cname, cargs, self.user_data);
            drop(CString::from_raw(cname));
            drop(Box::from_raw(cargs));

            if ret.is_null() {
                None
            } else {
                Some(*Box::from_raw(ret as *mut ValueType))
            }
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_bind_external_function(
    story: *mut Story,
    func_name: *const c_char,
    function: unsafe extern "C" fn(
        *const c_char,
        *const Vec<ValueType>,
        *const c_void,
    ) -> *const ValueType,
    user_data: *const c_void,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || func_name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(func_name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.bind_external_function(
        name_str_slice,
        Rc::new(RefCell::new(ExtFun {
            callback: function,
            user_data,
        })),
        false,
    );

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
pub extern "C" fn bink_unbind_external_function(
    story: *mut Story,
    func_name: *const c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || func_name.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let name_c_str: &CStr = unsafe { CStr::from_ptr(func_name) };
    let name_str_slice: &str = name_c_str.to_str().unwrap();

    let result = story.unbind_external_function(name_str_slice);

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
pub extern "C" fn bink_fun_args_count(args: *const Vec<ValueType>) -> usize {
    let args: &Vec<ValueType> = unsafe { &*args };
    args.len()
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_fun_args_get(
    args: *const Vec<ValueType>,
    idx: usize,
    value: *mut *mut ValueType,
    err_msg: *mut *mut c_char,
) -> u32 {
    let args: &Vec<ValueType> = unsafe { &*args };
    if idx >= args.len() {
        unsafe {
            *err_msg = CString::new(format!("Argument {} not found.", idx))
                .unwrap()
                .into_raw();
        }
        return BINK_FAIL;
    }

    unsafe {
        *value = Box::into_raw(Box::new(args[idx].clone()));
    }

    BINK_OK
}
