use std::ffi::CString;

#[allow(dead_code)]
pub(crate) fn convert_to_c_string(val: &str) -> CString {
    CString::new(val).unwrap_or_else(|_| {
        debug_assert!(false);
        CString::new("invalid string argument").unwrap()
    })
}
