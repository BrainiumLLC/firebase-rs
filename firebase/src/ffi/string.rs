use crate::ffi::id_mod::{IdRef, ObjcId};
use objc::{class, msg_send, sel, sel_impl};
use std::os::raw::c_void;

#[repr(transparent)]
pub struct NSStringRust(IdRef);

impl NSStringRust {
    pub fn from_str(string: &str) -> Self {
        unsafe {
            const NS_UTF8_STRING_ENCODING: usize = 4;
            let object: ObjcId = msg_send![class!(NSString), alloc];
            let object = msg_send![
                object,
                initWithBytes: string.as_ptr() as *const c_void
                length: string.len()
                encoding: NS_UTF8_STRING_ENCODING
            ];
            Self(IdRef::new(object))
        }
    }
}
