use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::slice;

static BUF: Global<ManuallyDrop<Vec<u8>>> = Global(RefCell::new(None));

struct Global<T>(RefCell<Option<T>>);

unsafe impl<T> Sync for Global<T> {}

#[no_mangle]
pub fn generate_pdf(ptr: *const u8, len: usize) -> *const u8 {
    let tiff = unsafe { slice::from_raw_parts(ptr, len) };
    let pdf = libtiff_sys::generate_pdf(tiff).expect("failed to generate pdf.");
    let pdf = ManuallyDrop::new(pdf);
    let ptr = pdf.as_ptr();
    *BUF.0.borrow_mut() = Some(pdf);
    ptr
}

#[no_mangle]
pub fn buf_len() -> usize {
    BUF.0.borrow().as_ref().unwrap().len()
}

#[no_mangle]
pub fn free_buf() {
    if let Some(ref mut buf) = *BUF.0.borrow_mut() {
        unsafe { ManuallyDrop::drop(buf) };
    }
    *BUF.0.borrow_mut() = None;
}
