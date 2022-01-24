#[cfg(feature = "analytics")]
pub mod analytics;
pub mod app;
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

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to convert string: {0}")]
    StringConversionError(#[from] std::ffi::NulError),
    #[error("failed to convert integer into C integer type")]
    IntegerConversionError,
}
