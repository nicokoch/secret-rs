use glib::object::{Wrapper, Ref};
use glib::types::{StaticType, Type};
use ffi;

pub struct SecretCollection(Ref);

impl SecretCollection {

    #[inline]
    fn raw(&self) -> *mut ffi::SecretCollectionFFI {
        self.0.to_glib_none() as *mut ffi::SecretCollectionFFI
    }
}

impl StaticType for SecretCollection {
    fn static_type() -> Type{
        Type::BaseObject
    }
}

impl Wrapper for SecretCollection {
    type GlibType = ffi::SecretCollectionFFI;

    /// Wraps a `Ref`.
    unsafe fn wrap(r: Ref) -> Self{
        SecretCollection(r)
    }

    /// Returns a reference to the inner `Ref`.
    fn as_ref(&self) -> &Ref{
        &self.0
    }
    /// Transforms into the inner `Ref`.
    fn unwrap(self) -> Ref{
        self.0
    }
}
