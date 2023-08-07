use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use std::ffi::{c_char, CString};
use std::slice;

#[no_mangle]
pub fn generate_pdf(ptr: *const u8, len: usize) -> *const c_char {
    let tiff = unsafe { slice::from_raw_parts(ptr, len) };
    let pdf = libtiff_sys::generate_pdf(tiff).expect("failed to generate pdf.");
    let base64 = STANDARD_NO_PAD.encode(pdf);
    let c_str = CString::new(base64).expect("failed to create cstring.");
    c_str.into_raw()
}
