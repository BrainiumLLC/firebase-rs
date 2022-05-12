use objc::{msg_send, runtime::Object, sel, sel_impl, Message};
use objc_foundation::INSObject;
use objc_id::{Id, Ownership};
use std::{
    fmt::{self, Display, Formatter},
    ops::Deref,
};

#[derive(Debug)]
pub struct InvalidClass(&'static str);

impl Display for InvalidClass {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "Conversion of raw objc pointer to incorrect class `{}`",
            self.0
        )
    }
}

pub trait FromCheckedPtr<T>: Sized {
    unsafe fn from_checked_retained_ptr(ptr: *mut T) -> Result<Self, InvalidClass>;
    unsafe fn from_checked_ptr(ptr: *mut T) -> Result<Self, InvalidClass>;
}

impl<T, O> FromCheckedPtr<T> for Id<T, O>
where
    T: Message + INSObject,
    O: Ownership,
{
    unsafe fn from_checked_retained_ptr(ptr: *mut T) -> Result<Self, InvalidClass> {
        assert!(
            !ptr.is_null(),
            "`Id::from_checked_ptr` received a null pointer"
        );
        let t = &mut *ptr;
        if t.is_kind_of(T::class()) {
            Ok(Id::from_retained_ptr(ptr))
        } else {
            Err(InvalidClass(T::class().name()))
        }
    }

    unsafe fn from_checked_ptr(ptr: *mut T) -> Result<Self, InvalidClass> {
        assert!(
            !ptr.is_null(),
            "`Id::from_checked_ptr` received a null pointer"
        );
        let t = &mut *ptr;
        if t.is_kind_of(T::class()) {
            Ok(Id::from_ptr(ptr))
        } else {
            Err(InvalidClass(T::class().name()))
        }
    }
}

pub type ObjcId = *mut Object;
pub const NIL: ObjcId = std::ptr::null_mut();
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct IdRef(ObjcId);

impl IdRef {
    pub fn new(inner: ObjcId) -> Self {
        Self(inner)
    }

    pub fn non_nil(self) -> Option<Self> {
        (self.0 != NIL).then(|| self)
    }
}

impl Drop for IdRef {
    fn drop(&mut self) {
        if self.0 != NIL {
            let () = unsafe { msg_send![self.0, release] };
        }
    }
}

impl Deref for IdRef {
    type Target = ObjcId;

    fn deref<'a>(&'a self) -> &'a ObjcId {
        &self.0
    }
}

impl Clone for IdRef {
    fn clone(&self) -> Self {
        if self.0 != NIL {
            let _: ObjcId = unsafe { msg_send![self.0, retain] };
        }
        Self(self.0)
    }
}
