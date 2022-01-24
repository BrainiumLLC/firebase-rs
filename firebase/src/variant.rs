use crate::util::convert_to_c_string;
use firebase_sys::*;
use std::ffi::CString;

#[derive(Debug, Clone)]
pub enum Variant {
    I64(i64),
    F64(f64),
    Bool(bool),
    Str(CString),
}

impl Variant {
    pub fn new_i64(value: i64) -> Self {
        Self::I64(value)
    }

    pub fn new_f64(value: f64) -> Self {
        Self::F64(value)
    }

    pub fn new_bool(value: bool) -> Self {
        Self::Bool(value)
    }

    pub fn new_str(value: &str) -> Self {
        let c_value = convert_to_c_string(value);
        Self::Str(c_value)
    }

    pub(crate) fn raw(&self) -> firebase_Variant {
        firebase_Variant {
            type_: self.internal_type(),
            value_: self.raw_value(),
        }
    }

    fn internal_type(&self) -> firebase_Variant_InternalType {
        match self {
            Self::I64(_) => firebase_Variant_InternalType_kInternalTypeInt64,
            Self::F64(_) => firebase_Variant_InternalType_kInternalTypeDouble,
            Self::Bool(_) => firebase_Variant_InternalType_kInternalTypeBool,
            Self::Str(_) => firebase_Variant_InternalType_kInternalTypeStaticString,
        }
    }

    fn raw_value(&self) -> firebase_Variant_Value {
        match self {
            Self::I64(val) => firebase_Variant_Value { int64_value: *val },
            Self::F64(val) => firebase_Variant_Value { double_value: *val },
            Self::Bool(val) => firebase_Variant_Value { bool_value: *val },
            Self::Str(val) => firebase_Variant_Value {
                static_string_value: val.as_ptr(),
            },
        }
    }
}
