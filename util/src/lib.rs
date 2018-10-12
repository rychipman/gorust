extern crate libc;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn get_hello_string() -> *const c_char {
    to_extern_string("Hello, world!")
}

fn to_extern_string(s: &str) -> *const c_char {
    // build a new nul-terminated string
    let c_str = CString::new(s).unwrap();

    // turn it into a pointer
    let c_str_ptr = c_str.as_ptr();

    // tell rust not to free the string when this func ends
    std::mem::forget(c_str);

    c_str_ptr
}

#[no_mangle]
pub extern "C" fn print_hello_string(name: *const libc::c_char) {
    let buf_name = unsafe { CStr::from_ptr(name).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello to {} from rust!", str_name);
}
