use crate::ffi::id_mod::ObjcId;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CString;
use std::os::raw::c_void;

#[repr(transparent)]
pub struct NSStringRust(ObjcId);

impl NSStringRust {
    pub fn from_str(string: &str) -> Self {
        unsafe {
            let object: ObjcId = msg_send![class!(NSString), stringWithUTF8String: CString::new(string).unwrap_or_default().into_raw()];
            Self(object)
        }
    }
}
