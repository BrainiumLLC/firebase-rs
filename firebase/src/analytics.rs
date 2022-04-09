use crate::{app::App, util::convert_to_c_string, variant::Variant};
use firebase_sys::*;
use std::{convert::TryInto, ffi::CString};

/// # Safety
///
/// This function is unsafe. Spooky!
pub unsafe fn initialize(app: &App) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        firebase_analytics_Initialize(app.raw());
    }
}

pub fn log_event_string(event_name: &str, parameter_name: &str, parameter_value: &str) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        unsafe {
            let c_event_name = convert_to_c_string(event_name);
            let c_parameter_name = convert_to_c_string(parameter_name);
            let c_parameter_value = convert_to_c_string(parameter_value);
            firebase_analytics_LogEvent(
                c_event_name.as_ptr(),
                c_parameter_name.as_ptr(),
                c_parameter_value.as_ptr(),
            );
        }
    }
}

/// # Safety
///
/// This function is unsafe. Spooky!
pub fn log_event_f64(event_name: &str, parameter_name: &str, parameter_value: f64) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        unsafe {
            let c_event_name = convert_to_c_string(event_name);
            let c_parameter_name = convert_to_c_string(parameter_name);
            firebase_analytics_LogEvent1(
                c_event_name.as_ptr(),
                c_parameter_name.as_ptr(),
                parameter_value,
            );
        }
    }
}

pub fn log_event_i64(event_name: &str, parameter_name: &str, parameter_value: i64) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        unsafe {
            let c_event_name = convert_to_c_string(event_name);
            let c_parameter_name = convert_to_c_string(parameter_name);
            firebase_analytics_LogEvent2(
                c_event_name.as_ptr(),
                c_parameter_name.as_ptr(),
                parameter_value,
            );
        }
    }
}

/// We are making the assumption here that c_int = i32. This will almost always be true (always in contexts we care about)
/// But if you are runing this on a very esoteric system, that might not be the case. This would likely be the least of your concerns then.
pub fn log_event_i32(event_name: &str, parameter_name: &str, parameter_value: i32) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        unsafe {
            let c_event_name = convert_to_c_string(event_name);
            let c_parameter_name = convert_to_c_string(parameter_name);
            firebase_analytics_LogEvent3(
                c_event_name.as_ptr(),
                c_parameter_name.as_ptr(),
                parameter_value,
            );
        }
    }
}

pub fn log_event(event_name: &str) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        unsafe {
            let c_event_name = convert_to_c_string(event_name);
            firebase_analytics_LogEvent4(c_event_name.as_ptr());
        }
    }
}

#[repr(C)]
pub struct Parameter {
    name: CString,
    variant: Variant,
}

impl Parameter {
    pub fn new(name: &str, variant: Variant) -> Self {
        Self {
            name: convert_to_c_string(name),
            variant,
        }
    }

    fn name(&self) -> &CString {
        &self.name
    }

    fn variant(&self) -> &Variant {
        &self.variant
    }
}

impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Logging for variant as well
        f.debug_struct("Parameter")
            .field("name", &self.name)
            .finish()
    }
}

pub fn log_event_parameters(event_name: &str, parameters: &[Parameter]) {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        unsafe {
            let c_event_name = convert_to_c_string(event_name);
            let firebase_params = parameters
                .iter()
                .map(|p| firebase_analytics_Parameter {
                    name: p.name().as_ptr(),
                    value: p.variant().raw(),
                })
                .collect::<Vec<_>>();
            firebase_analytics_LogEvent5(
                c_event_name.as_ptr(),
                firebase_params.as_ptr() as *const firebase_analytics_Parameter,
                parameters
                    .len()
                    .try_into()
                    .expect("unable to convert length of parameters 'slice' to type expect by C++"),
            );
            log::info!("log event: {}, parameters: {:?}", event_name, parameters);
        }
    }
}
