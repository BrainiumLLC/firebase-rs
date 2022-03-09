use firebase_sys::*;
use std::os::raw::c_void;

#[derive(Debug, Copy, Clone)]
pub struct FutureError {
    status: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct FutureOk {
    status: i32,
}

pub enum FirebaseErrorStatus {
    Error(FutureError),
    Ok(FutureOk),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FutureReceipt {
    status: firebase_FutureStatus,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FutureStatus {
    Complete(FutureReceipt),
    Pending,
    Invalid,
}

impl FutureStatus {
    fn from_internal_type(status: firebase_FutureStatus) -> Self {
        #![allow(non_upper_case_globals)]
        match status {
            firebase_FutureStatus_kFutureStatusComplete => Self::Complete(FutureReceipt { status }),
            firebase_FutureStatus_kFutureStatusPending => Self::Pending,
            firebase_FutureStatus_kFutureStatusInvalid => Self::Invalid,
            _ => unreachable!(),
        }
    }
}

pub struct Future {
    raw: firebase_Future,
}

impl Future {
    pub(crate) fn from_raw(raw: firebase_Future) -> Self {
        Self { raw }
    }

    pub fn status(&self) -> FutureStatus {
        unsafe { FutureStatus::from_internal_type(self.raw._base.status()) }
    }

    pub fn on_completion<F: FnMut(*const firebase_FutureBase) + Send + Sync + 'static>(
        &self,
        callback: F,
    ) {
        let data = Box::into_raw(Box::new(callback));
        unsafe {
            firebase_FutureBase_OnCompletion(
                &self.raw._base as *const _,
                Some(call_closure::<F>),
                data as *mut _,
            );
        }
    }

    pub fn on_success_or_error<
        F1: FnMut() + Send + Sync + 'static,
        F2: FnMut() + Send + Sync + 'static,
    >(
        &self,
        mut on_success: F1,
        mut on_error: F2,
    ) {
        self.on_completion(move |raw_future| {
            let error_status = unsafe { firebase_FutureBase_error(raw_future) };
            if error_status == 0 {
                on_success();
            } else {
                log::error!("{}", Future::error_message(raw_future));
                on_error();
            }
        });
    }

    pub(crate) fn error_message(raw_base: *const firebase_FutureBase) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(firebase_FutureBase_error_message(raw_base))
                .to_string_lossy()
                .into_owned()
        }
    }
}

extern "C" fn call_closure<F: FnMut(*const firebase_FutureBase)>(
    result: *const firebase_FutureBase,
    data: *mut c_void,
) {
    let callback_ptr = data as *mut F;
    let callback = unsafe { &mut *callback_ptr };
    callback(result);
    unsafe { Box::from_raw(callback_ptr) };
}
