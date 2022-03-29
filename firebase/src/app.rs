#[cfg(target_os = "android")]
use ndk_glue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(target_os = "android")]
    #[error("failed to create Java VM: {0}")]
    JavaVmCreationFailed(#[from] jni::errors::Error),
}

pub struct App {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    raw: *mut firebase_sys::firebase_App,
}

impl App {
    /// # Safety
    ///
    /// This function is unsafe. Spooky!
    #[cfg(not(target_os = "android"))]
    pub unsafe fn create() -> Self {
        #[cfg(target_os = "ios")]
        {
            Self {
                raw: firebase_sys::firebase_App_Create(),
            }
        }
        #[cfg(not(target_os = "ios"))]
        {
            Self {}
        }
    }

    /// # Safety
    ///
    /// This function is unsafe. Spooky!
    #[cfg(target_os = "android")]
    pub unsafe fn create_from_jni(
        activity: &'static ndk::native_activity::NativeActivity,
        jni_env: *mut jni::sys::JNIEnv,
    ) -> Self {
        Self {
            raw: firebase_sys::firebase_App_Create(jni_env as _, activity.activity() as _),
        }
    }

    #[cfg(any(target_os = "android", target_os = "ios"))]
    pub(crate) fn raw(&self) -> *mut firebase_sys::firebase_App {
        self.raw
    }
}

impl std::fmt::Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirebaseApp").finish()
    }
}
