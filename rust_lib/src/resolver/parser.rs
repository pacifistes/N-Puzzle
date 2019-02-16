#![crate_type = "staticlib"]

use crate::resolver::generate::generate_state_index;
use crate::resolver::puzzle::*;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

// For Binding
use libc::{c_char, uint32_t, uint8_t, size_t};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use std::iter;
use std::slice;
use std::convert::From;
use std::ptr;

fn get_size(line: &str) -> Result<u8, io::Error> {
    match line.parse::<u8>() {
        Ok(num) if num > 1 && num < 17 => Ok(num),
        _ => Err(Error::new(
            ErrorKind::InvalidInput,
            "The first no-comment line should be the size of the puzzle (between 2 and 10)",
        )),
    }
}

fn add_to_state(mut start_state: Vec<u8>, line: &str, size: u8) -> Result<Vec<u8>, io::Error> {
    match line
        .split_whitespace()
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
    {
        Ok(numbers) => {
            for number in numbers {
                match start_state.contains(&number) || number >= (size * size) as u8 {
                    true => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "The numbers must be unique and between 0 and {}",
                                size * size - 1
                            ),
                        ));
                    }
                    false => start_state.push(number),
                }
            }
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("The line must contain {} numbers", size),
            ));
        }
    }
    Ok(start_state)
}

pub fn parse(filename: &str) -> Result<(Vec<u8>, u8), io::Error> {
    let file = File::open(filename)?;
    let mut size: u8 = 0;
    let mut start_state: Vec<u8> = Vec::new();

    for line in BufReader::new(file).lines() {
        let line = line?.split('#').collect::<Vec<_>>()[0]
            .trim()
            .to_string();
        match line.is_empty() {
            true => (),
            false => match size == 0 {
                true => size = get_size(&line)?,
                false => start_state = add_to_state(start_state, &line, size)?,
            },
        }
    }
    if start_state.len() != size as usize * size as usize {
        return Err(Error::new(ErrorKind::InvalidData, "Missing some lines"));
    }
    // let start_state_index: Vec<u8> = generate_state_index(&start_state);
    Ok((start_state, size))
}

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
pub struct CPuzzle {
	state: *mut u8,
	size: u8
}

#[repr(C)]
pub struct Parser {
    puzzle: *mut CPuzzle,
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
		Ok((tmp_state, size)) => {
			let c_error = CString::new("no error").unwrap();
			unsafe {
				Parser {
					// puzzle: Box::into_raw(Box::new(puzzle)),
					puzzle:Box::into_raw(Box::new( CPuzzle  {
						state: tmp_state.into_boxed_slice().as_mut_ptr(),
						size,
					})),
					error: c_error.into_raw()
				}
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

#[no_mangle]
pub extern fn parser_free(parser: Parser) {
    unsafe {
        if !parser.error.is_null() {
	        CString::from_raw(parser.error);
		}
		if !parser.puzzle.is_null() {
			Box::from_raw(parser.puzzle);// TO remove after mayve
		}
    };
}