use std::ptr;
use pcre2_sys::{
    pcre2_match_8, pcre2_compile_8,
    PCRE2_UCP, PCRE2_UTF,
    pcre2_match_data_create_from_pattern_8,
    pcre2_code_free_8, pcre2_match_data_free_8,
    pcre2_get_ovector_pointer_8,
};

fn main() {
    let pattern = r"(?<=\d{4})[^\d\s]{3,11}(?=.)";
    let text = r"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";

    let mut error_code = 0;
    let mut error_offset = 0;
    let code = unsafe {
        pcre2_compile_8(
            pattern.as_ptr(),
            pattern.len() as u64,
            PCRE2_UCP | PCRE2_UTF,
            &mut error_code,
            &mut error_offset,
            ptr::null_mut(),
        )
    };
    if code.is_null() {
        panic!("compile failed, error code: {:?}, offset: {:?}", error_code, error_offset);
    }

    let match_data = unsafe {
        pcre2_match_data_create_from_pattern_8(code, ptr::null_mut())
    };
    if match_data.is_null() {
        unsafe { pcre2_code_free_8(code); }
        panic!("could not allocate match_data");
    }

    let ovector = unsafe { pcre2_get_ovector_pointer_8(match_data) };
    if ovector.is_null() {
        unsafe {
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
        }
        panic!("could not get ovector");
    }

    let rc = unsafe {
        pcre2_match_8(
            code,
            text.as_ptr(),
            text.len() as u64,
            0,
            0,
            match_data,
            ptr::null_mut(),
        )
    };
    if rc <= 0 {
        unsafe {
            pcre2_match_data_free_8(match_data);
            pcre2_code_free_8(code);
        }
        panic!("error executing match");
    }

    let (s, e) = unsafe {
        (*ovector.offset(0) as usize, *ovector.offset(1) as usize)
    };
    unsafe {
        pcre2_match_data_free_8(match_data);
        pcre2_code_free_8(code);
    }

    let result = &text[s..e];
    println!("match: {:?}", result);
}
