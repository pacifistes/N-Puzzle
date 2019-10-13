use crate::resolver::generate::r_generate_random_state;
use crate::resolver::generate::r_generate_sorted_state;
use crate::binding::puzzle::RVector;

#[no_mangle]
pub extern "C" fn c_generate_random_state(size: u32) -> RVector {
    let mut r_values = r_generate_random_state(size as u8);
    let size = r_values.len() as u32;
    let c_values = r_values.as_mut_ptr();

    std::mem::forget(r_values);
    RVector {
        values: c_values,
        size,
    }
}

#[no_mangle]
pub extern "C" fn c_generate_sorted_state(size: u32) -> RVector {
    let mut r_values = r_generate_sorted_state(size as u8);
    let size = r_values.len() as u32;
    let c_values = r_values.as_mut_ptr();

    std::mem::forget(r_values);
    RVector {
        values: c_values,
        size,
    }
}