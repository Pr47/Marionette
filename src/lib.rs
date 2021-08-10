use std::os::raw::{c_char, c_int};
use std::ffi::CString;

#[repr(C)]
pub struct StringPair {
    key: *const c_char,
    value: *const c_char
}

#[no_mangle]
pub extern "C" fn dcgi_main(
    _query_path: *const c_char,
    _headers: *const StringPair,
    _params: *const StringPair,
    _body: *const c_char,
    header_dest: *mut *mut StringPair,
    data_dest: *mut *mut c_char,
    _err_dest: *mut *mut c_char
) -> c_int {
    let data: CString = CString::new("Hello, DCGI!\n").unwrap();
    let data: *mut c_char = data.into_raw();
    let header: Box<StringPair> = Box::new(StringPair{
        key: std::ptr::null(), value: std::ptr::null()
    });
    let header: *mut StringPair = Box::into_raw(header);
    unsafe {
        std::ptr::write(data_dest, data);
        std::ptr::write(header_dest, header);
    }
    0
}
