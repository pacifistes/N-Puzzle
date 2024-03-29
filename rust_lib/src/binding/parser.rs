use crate::resolver::parser::parse;
use crate::binding::puzzle::RVector;
use libc::c_char;
use std::io;
use std::ffi::{CStr, CString};
use std::ptr;

#[repr(C)]
pub struct CreatedPuzzle {
    state: *mut RVector,
    error: *mut c_char,
}

pub fn create_puzzle(result: Result<Vec<u8>, io::Error>) -> CreatedPuzzle {
    match result {
        Ok(mut r_values) => {
            let c_error = CString::new("no error").unwrap();
            let c_values = r_values.as_mut_ptr();
            let size: u32 = r_values.len() as u32;
            std::mem::forget(r_values);
			CreatedPuzzle {
				state: Box::into_raw(Box::new(RVector {
					values: c_values,
					size,
				})),
				error: c_error.into_raw(),
			}
        }
        Err(err) => {
            let c_error = CString::new(err.to_string()).unwrap();
            CreatedPuzzle {
                state: ptr::null_mut(),
                error: c_error.into_raw(),
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn parser_new(filename: *const c_char) -> CreatedPuzzle {
    let c_filename = unsafe {
        assert!(!filename.is_null());
        CStr::from_ptr(filename)
    };
    let rust_filename = c_filename.to_str().unwrap();
    create_puzzle(parse(rust_filename))
}

#[no_mangle]
pub extern "C" fn created_puzzle_free(created_puzzle: CreatedPuzzle) {
    unsafe {
        if !created_puzzle.error.is_null() {
            CString::from_raw(created_puzzle.error);
        }
        if !created_puzzle.state.is_null() {
            let state = Box::from_raw(created_puzzle.state);
            if !state.values.is_null() {
                let size: usize = state.size as usize;
                Vec::from_raw_parts(state.values, size, size);
            }
        }
    };
}
