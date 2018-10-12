extern crate libc;
use std::ffi::CStr;

#[no_mangle]
pub extern fn get_hello_string() -> String {
    format!("Hello, world!")
}

pub extern fn print_hello_string(name: *const libc::c_char) {
    let buf_name = unsafe {
        CStr::from_ptr(name).to_bytes()
    };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    println!("Hello to {} from rust!", str_name);
}
