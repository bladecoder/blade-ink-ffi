use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    rc::Rc,
};

use bladeink::{choice::Choice, story::Story};

use crate::{BINK_FAIL, BINK_FAIL_NULL_POINTER, BINK_OK};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_story_new(
    story: *mut *mut Story,
    json_string: *const c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || err_msg.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    unsafe {
        *story = std::ptr::null_mut();
        *err_msg = std::ptr::null_mut();
    }

    let c_str: &CStr = unsafe { CStr::from_ptr(json_string) };
    let str_slice: &str = c_str.to_str().unwrap();

    let result = Story::new(str_slice);

    match result {
        Ok(s) => unsafe {
            *story = Box::into_raw(Box::new(s));
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
pub extern "C" fn bink_story_free(story: *mut Story) {
    if !story.is_null() {
        unsafe {
            drop(Box::from_raw(story));
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_story_can_continue(story: *mut Story, can_continue: *mut bool) -> u32 {
    if story.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    unsafe {
        *can_continue = story.can_continue();
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_story_cont(
    story: *mut Story,
    line: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let result = story.cont();

    match result {
        Ok(l) => unsafe {
            *line = CString::new(l).unwrap().into_raw();
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
pub extern "C" fn bink_story_continue_maximally(
    story: *mut Story,
    lines: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let result = story.continue_maximally();

    match result {
        Ok(l) => unsafe {
            *lines = CString::new(l).unwrap().into_raw();
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
pub extern "C" fn bink_story_get_current_choices(
    story: *mut Story,
    choices: *mut *mut Vec<Rc<Choice>>,
    len: *mut usize,
) -> u32 {
    if story.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let result = Box::new(story.get_current_choices());

    unsafe {
        *len = result.len();
        *choices = Box::into_raw(result);
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_story_choose_choice_index(
    story: *mut Story,
    choice_index: usize,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let result = story.choose_choice_index(choice_index);

    match result {
        Ok(_) => BINK_OK,
        Err(e) => {
            unsafe {
                *err_msg = CString::new(e.to_string()).unwrap().into_raw();
            }
            BINK_FAIL
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_story_get_current_tags(
    story: *mut Story,
    tags: *mut *mut Vec<String>,
    len: *mut usize,
) -> u32 {
    if story.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let result = story.get_current_tags();

    match result {
        Ok(result) => unsafe {
            *len = result.len();
            *tags = Box::into_raw(Box::new(result));
        },
        Err(_) => return BINK_FAIL,
    }

    BINK_OK
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn bink_story_choose_path_string(
    story: *mut Story,
    path: *const c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if story.is_null() || path.is_null() {
        return BINK_FAIL_NULL_POINTER;
    }

    let story: &mut Story = unsafe { &mut *story };

    let path_c_str: &CStr = unsafe { CStr::from_ptr(path) };
    let path_str_slice: &str = path_c_str.to_str().unwrap();

    let result = story.choose_path_string(path_str_slice, true, None);

    if let Err(desc) = result {
        unsafe {
            *err_msg = CString::new(desc.to_string()).unwrap().into_raw();
        }

        return BINK_FAIL;
    }

    BINK_OK
}
