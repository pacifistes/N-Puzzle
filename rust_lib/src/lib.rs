mod resolver;

use resolver::generate::generate_random_puzzle;
use resolver::generate::generate_sorted_puzzle;
use resolver::generate::generate_state_index;
use resolver::heuristic::*;
use resolver::parser::parse;
use resolver::puzzle::*;
use resolver::resolver::*;

use libc::{c_char, uint32_t, uint8_t, size_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::iter;
use std::slice;
use std::convert::From;
use std::ptr;

#[no_mangle]
pub extern fn how_many_characters(s: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}

#[repr(C)]
pub struct Parser {
    puzzle: *mut Puzzle,
    error: *mut c_char,
}


#[no_mangle]
pub extern fn parser_new(filename: *const c_char) -> Parser {
	let c_filename = unsafe {
        assert!(!filename.is_null());
        CStr::from_ptr(filename)
    };
    let rust_filename = c_filename.to_str().unwrap();
	match parse(rust_filename) {
		Ok(puzzle) => {
			let c_error = CString::new("no error").unwrap();
			Parser {
				puzzle: Box::into_raw(Box::new(puzzle)),
				error: c_error.into_raw()
			}
		},
		Err(err) => {
			let c_error = CString::new(err.to_string()).unwrap();
			Parser {
				puzzle: ptr::null_mut(),
				error: c_error.into_raw(),
			}
		}
	}
}

// #[no_mangle]
// pub extern fn parser_free(ptr: *mut ZipCodeDatabase) {
//     if ptr.is_null() { return }
//     unsafe { Box::from_raw(ptr); }
// }