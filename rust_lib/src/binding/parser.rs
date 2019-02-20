use crate::resolver::parser::parse;
use crate::binding::puzzle::RVector;
use libc::c_char;
use std::ffi::{CStr, CString};
use std::ptr;

#[repr(C)]
pub struct Parser {
    state: *mut RVector,
    error: *mut c_char,
}

#[no_mangle]
pub extern "C" fn parser_new(filename: *const c_char) -> Parser {
    let c_filename = unsafe {
        assert!(!filename.is_null());
        CStr::from_ptr(filename)
    };
    let rust_filename = c_filename.to_str().unwrap();

    match parse(rust_filename) {
        Ok((mut r_values, size)) => {
            let c_error = CString::new("no error").unwrap();
            let c_values = r_values.as_mut_ptr();
            std::mem::forget(r_values);
			Parser {
				state: Box::into_raw(Box::new(RVector {
					values: c_values,
					size: u32::from(size * size),
				})),
				error: c_error.into_raw(),
			}
        }
        Err(err) => {
            let c_error = CString::new(err.to_string()).unwrap();
            Parser {
                state: ptr::null_mut(),
                error: c_error.into_raw(),
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn parser_free(parser: Parser) {
    unsafe {
        if !parser.error.is_null() {
            CString::from_raw(parser.error);
        }
        if !parser.state.is_null() {
            Box::from_raw(parser.state);
        }
    };
}
