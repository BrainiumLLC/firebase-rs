#[cfg(feature = "analytics")]
pub mod analytics;
pub mod app;
#[cfg(target_os = "ios")]
mod ffi;
#[cfg(feature = "remote_config")]
pub mod future;
#[cfg(feature = "remote_config")]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub mod remote_config;
mod util;
#[cfg(any(feature = "analytics", feature = "remote_config"))]
pub mod variant;

use thiserror::Error;

/// # Safety
///
/// This function is unsafe. Spooky!
pub unsafe fn configure() {
    #[cfg(target_os = "ios")]
    {
        use objc::{class, msg_send, sel, sel_impl};

        let cls = class!(FIRApp);
        let _: *const () = msg_send![cls, configure];
    }
}

pub fn log(message: &str) {
    #[cfg(target_os = "ios")]
    {
        use objc::{class, msg_send, rc::autoreleasepool, runtime::Object, sel, sel_impl};
        autoreleasepool(|| unsafe {
            let crashlytics: *mut Object = msg_send![class!(FIRCrashlytics), crashlytics];
            let ns_message = ffi::NSStringRust::from_str(message);
            let _: *const () = msg_send![crashlytics, log: ns_message];
        });
    }
    // TODO: Other platforms/Android
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to convert string: {0}")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error("failed to convert integer into C integer type")]
    IntegerConversionError,
}

// TODO: Remove this later after testing
/// # Safety
///
/// This function is unsafe. Spooky!
pub unsafe fn force_crash() {
    firebase_sys::force_crash();
}
