
#[no_mangle]
pub extern fn get_hello_string() -> String {
    format!("Hello, world!")
}
