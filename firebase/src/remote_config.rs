use crate::{app::App, future::Future, variant::Variant};
use firebase_sys::*;
use std::{convert::TryInto, ffi::CString};

#[derive(Debug, Clone)]
pub struct ConfigKeyValueVariant {
    key: CString,
    value: Variant,
}

impl ConfigKeyValueVariant {
    pub fn new(key: &str, variant: Variant) -> Self {
        let key = CString::new(key).expect("could not convert key to C string");
        Self {
            key,
            value: variant,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LastFetchStatus {
    Success,
    Failure,
    Pending,
}

impl LastFetchStatus {
    fn from_internal_type(value: firebase_remote_config_LastFetchStatus) -> Self {
        #![allow(non_upper_case_globals)]
        match value {
            firebase_remote_config_LastFetchStatus_kLastFetchStatusSuccess => Self::Success,
            firebase_remote_config_LastFetchStatus_kLastFetchStatusFailure => Self::Failure,
            firebase_remote_config_LastFetchStatus_kLastFetchStatusPending => Self::Pending,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FetchFailureReason {
    Invalid,
    Throttled,
    Error,
}

impl FetchFailureReason {
    fn from_internal_type(value: firebase_remote_config_LastFetchStatus) -> Self {
        #![allow(non_upper_case_globals)]
        match value {
            firebase_remote_config_FetchFailureReason_kFetchFailureReasonInvalid => Self::Invalid,
            firebase_remote_config_FetchFailureReason_kFetchFailureReasonThrottled => {
                Self::Throttled
            }
            firebase_remote_config_FetchFailureReason_kFetchFailureReasonError => Self::Error,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct ConfigInfo {
    fetch_time: u64,
    last_fecth_status: LastFetchStatus,
    failure_reason: FetchFailureReason,
    throttled_end_time: u64,
}

impl ConfigInfo {
    fn from_internal_type(value: firebase_remote_config_ConfigInfo) -> Self {
        Self {
            fetch_time: value.fetch_time,
            last_fecth_status: LastFetchStatus::from_internal_type(value.last_fetch_status),
            failure_reason: FetchFailureReason::from_internal_type(value.last_fetch_failure_reason),
            throttled_end_time: value.throttled_end_time,
        }
    }

    pub fn fetch_time(&self) -> u64 {
        self.fetch_time
    }

    pub fn last_fecth_status(&self) -> LastFetchStatus {
        self.last_fecth_status
    }

    pub fn failure_reason(&self) -> FetchFailureReason {
        self.failure_reason
    }

    pub fn throttled_end_time(&self) -> u64 {
        self.throttled_end_time
    }
}

#[derive(Debug)]
pub struct ConfigSettings {
    fetch_timeout_in_milliseconds: u64,
    minimum_fetch_interval_in_milliseconds: u64,
}

impl ConfigSettings {
    fn from_internal_type(settings: firebase_remote_config_ConfigSettings) -> Self {
        Self {
            fetch_timeout_in_milliseconds: settings.fetch_timeout_in_milliseconds,
            minimum_fetch_interval_in_milliseconds: settings.minimum_fetch_interval_in_milliseconds,
        }
    }

    pub fn fetch_timeout_in_milliseconds(&self) -> u64 {
        self.fetch_timeout_in_milliseconds
    }

    pub fn minimum_fetch_interval_in_milliseconds(&self) -> u64 {
        self.minimum_fetch_interval_in_milliseconds
    }
}

pub struct RemoteConfig {
    raw: *mut firebase_remote_config_RemoteConfig,
}

impl RemoteConfig {
    pub unsafe fn new(app: &App) -> Self {
        let raw = firebase_remote_config_RemoteConfig_GetInstance(app.raw());
        Self { raw }
    }

    pub fn ensure_initialized<
        F1: FnMut() + Send + Sync + 'static,
        F2: FnMut() + Send + Sync + 'static,
    >(
        &self,
        on_success: F1,
        on_error: F2,
    ) {
        let future = unsafe {
            Future::from_raw(firebase_remote_config_RemoteConfig_EnsureInitialized(
                self.raw,
            ))
        };
        future.on_success_or_error(on_success, on_error);
    }

    pub fn info(&self) -> ConfigInfo {
        unsafe {
            ConfigInfo::from_internal_type(firebase_remote_config_RemoteConfig_GetInfo(self.raw))
        }
    }

    pub fn settings(&self) -> ConfigSettings {
        unsafe {
            ConfigSettings::from_internal_type(
                firebase_remote_config_RemoteConfig_GetConfigSettings(self.raw),
            )
        }
    }

    pub fn fetch<F1: FnMut() + Send + Sync + 'static, F2: FnMut() + Send + Sync + 'static>(
        &self,
        on_success: F1,
        on_error: F2,
    ) {
        let future =
            unsafe { Future::from_raw(firebase_remote_config_RemoteConfig_Fetch(self.raw)) };
        future.on_success_or_error(on_success, on_error);
    }

    pub fn activate<F1: FnMut() + Send + Sync + 'static, F2: FnMut() + Send + Sync + 'static>(
        &self,
        on_success: F1,
        on_error: F2,
    ) {
        let future =
            unsafe { Future::from_raw(firebase_remote_config_RemoteConfig_Activate(self.raw)) };
        future.on_success_or_error(on_success, on_error);
    }

    pub fn fetch_and_activate<
        F1: FnMut() + Send + Sync + 'static,
        F2: FnMut() + Send + Sync + 'static,
    >(
        &self,
        on_success: F1,
        on_error: F2,
    ) {
        let future = unsafe {
            Future::from_raw(firebase_remote_config_RemoteConfig_FetchAndActivate(
                self.raw,
            ))
        };
        future.on_success_or_error(on_success, on_error);
    }

    pub fn set_defaults<
        F1: FnMut() + Send + Sync + 'static,
        F2: FnMut() + Send + Sync + 'static,
    >(
        &self,
        defaults: &[ConfigKeyValueVariant],
        on_success: F1,
        on_error: F2,
    ) {
        let key_value_raws = defaults
            .iter()
            .map(|config| firebase_remote_config_ConfigKeyValueVariant {
                key: config.key.as_ptr(),
                value: config.value.raw(),
            })
            .collect::<Vec<_>>();
        #[cfg(target_os = "ios")]
        let future = unsafe {
            Future::from_raw(firebase_remote_config_RemoteConfig_SetDefaults(
                self.raw,
                key_value_raws.as_ptr(),
                defaults.len().try_into().unwrap(),
            ))
        };
        #[cfg(target_os = "android")]
        let future = unsafe {
            Future::from_raw(firebase_remote_config_RemoteConfig_SetDefaults1(
                self.raw,
                key_value_raws.as_ptr(),
                defaults.len().try_into().unwrap(),
            ))
        };
        future.on_success_or_error(on_success, on_error);
    }

    pub fn get_bool(&self, key: &str) -> bool {
        let key = CString::new(key).expect("could not convert key to C string");
        unsafe { firebase_remote_config_RemoteConfig_GetBoolean(self.raw, key.as_ptr()) }
    }

    pub fn get_long(&self, key: &str) -> i64 {
        let key = CString::new(key).expect("could not convert key to C string");
        unsafe { firebase_remote_config_RemoteConfig_GetLong(self.raw, key.as_ptr()) }
    }

    pub fn get_double(&self, key: &str) -> f64 {
        let key = CString::new(key).expect("could not convert key to C string");
        unsafe { firebase_remote_config_RemoteConfig_GetDouble(self.raw, key.as_ptr()) }
    }

    pub fn get_string(&self, key: &str) -> String {
        let key = CString::new(key).expect("could not convert key to C string");
        unsafe {
            let c_ptr = get_string(self.raw, key.as_ptr());
            let ret = std::ffi::CStr::from_ptr(c_ptr)
                .to_string_lossy()
                .into_owned();
            free_string(c_ptr);
            ret
        }
    }
}

impl std::fmt::Debug for RemoteConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteConfig").finish()
    }
}
