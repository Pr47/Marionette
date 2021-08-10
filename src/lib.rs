use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::error::Error;

#[repr(C)]
pub struct CStrPair {
    key: *const c_char,
    value: *const c_char
}

impl CStrPair {
    pub fn new(key: *const c_char, value: *const c_char) -> Self {
        Self { key, value }
    }

    pub fn null() -> Self {
        Self {
            key: std::ptr::null(),
            value: std::ptr::null()
        }
    }
}

pub struct HttpRequest {
    pub query_path: String,
    pub headers: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
    pub body: String
}

unsafe fn raw_c_str_to_string(raw_c_str: *const c_char) -> Result<String, Box<dyn Error + 'static>> {
    Ok(CStr::from_ptr(raw_c_str).to_str()?.to_string())
}

impl HttpRequest {
    pub unsafe fn from_dcgi_pack(
        raw_query_path: *const c_char,
        raw_headers: *const CStrPair,
        raw_params: *const CStrPair,
        raw_body: *const c_char
    ) -> Result<Self, Box<dyn Error + 'static>> {
        let query_path: String = raw_c_str_to_string(raw_query_path)?;
        let mut headers: Vec<(String, String)> = vec![];
        {
            let mut i: isize = 0;
            let mut raw_header: &CStrPair = &*raw_headers.offset(i);
            while !raw_header.key.is_null() {
                let key: String = raw_c_str_to_string(raw_header.key)?;
                let value: String = raw_c_str_to_string(raw_header.value)?;
                headers.push((key, value));
                i += 1;
                raw_header = &*raw_headers.offset(i);
            }
        }
        let mut params: Vec<(String, String)> = vec![];
        {
            let mut i: isize = 0;
            let mut raw_param: &CStrPair = &*raw_params.offset(i);
            while !raw_param.key.is_null() {
                let key: String = raw_c_str_to_string(raw_param.key)?;
                let value: String = raw_c_str_to_string(raw_param.value)?;
                params.push((key, value));
                i += 1;
                raw_param = &*raw_params.offset(i);
            }
        }
        let body: String = raw_c_str_to_string(raw_body)?;

        Ok(Self { query_path, headers, params, body })
    }
}

pub struct HttpResponse {
    headers: Vec<(String, String)>,
    body: String
}

impl HttpResponse {
    pub fn new(headers: Vec<(String, String)>, body: String) -> Self {
        Self { headers, body }
    }

    pub unsafe fn write_to_dcgi_pack(
        &self,
        header_dest: *mut *mut CStrPair,
        body_dest: *mut *mut c_char
    ) {
        let headers: Vec<CStrPair> =
            self.headers
                .iter()
                .map(|(key, value): &(String, String)| {
                    CStrPair::new(
                        CString::new(key.as_str()).unwrap().into_raw(),
                        CString::new(value.as_str()).unwrap().into_raw()
                    )
                })
                .chain([CStrPair::null()])
                .collect::<Vec<_>>();
        let raw_headers: *mut CStrPair = headers.leak().as_mut_ptr();
        std::ptr::write(header_dest, raw_headers);
        let raw_body: *mut c_char = CString::new(self.body.as_str()).unwrap().into_raw();
        std::ptr::write(body_dest, raw_body);
    }
}

#[no_mangle]
pub extern "C" fn dcgi_main(
    _query_path: *const c_char,
    _headers: *const CStrPair,
    _params: *const CStrPair,
    _body: *const c_char,
    header_dest: *mut *mut CStrPair,
    data_dest: *mut *mut c_char,
    _err_dest: *mut *mut c_char
) -> c_int {
    let data: CString = CString::new("Hello, DCGI!\n").unwrap();
    let data: *mut c_char = data.into_raw();
    let header: Box<CStrPair> = Box::new(CStrPair {
        key: std::ptr::null(), value: std::ptr::null()
    });
    let header: *mut CStrPair = Box::into_raw(header);
    unsafe {
        std::ptr::write(data_dest, data);
        std::ptr::write(header_dest, header);
    }
    0
}
