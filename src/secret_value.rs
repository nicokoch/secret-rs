use glib::types::{StaticType, Type};
use glib::translate::*;
use ffi;

/// A SecretValue contains a password or other secret value.
/// Use `SecretValue::get()` to get the actual secret data, such as a password.
/// This library only supports content type `text/plain` for now.
pub struct SecretValue {
    ptr: *mut ffi::SecretValue
}

impl SecretValue{

    /// Create a SecretValue for the secret data passed in.
    /// This method will *NOT* create the actual secret in the backing Secret 
    /// Service. (see SecretService.store() for that)
    /// The secret data is copied into non-pageable 'secure' memory.
    pub fn new(secret: &str) -> Self {
        let content_type = "text/plain";
        unsafe {
            let ptr = ffi::secret_value_new(
                secret.to_glib_none().0,
                -1,
                content_type.to_glib_none().0
                );
            SecretValue::from_glib_full(ptr)
        }
    }

    /// Transfer ownership of a raw SecretValue pointer to rust.
    pub fn from_glib_full(ptr: *mut ffi::SecretValue) -> Self {
        assert!(!ptr.is_null());
        //debug_assert!(types::instance_of::<T>(ptr as *const _));
        SecretValue {
            ptr: ptr
        }
    }

    /// None-transfer to glib.
    pub fn to_glib_none(&self) -> *mut ffi::SecretValue {
        self.ptr
    }

    /// Get the secret data in the SecretValue.
    /// For now, this method only supports String values.
    /// Returns None, if the content type is npt text/plain
    pub fn get(&self) -> Option<String> {
        if self.get_content_type() == "text/plain" {
            unsafe{
                let secret = ffi::secret_value_get_text(self.to_glib_none());
                Some(from_glib_none(secret))
            }
        } else {
            None
        }
    }

    /// Get the content type of the secret value, such as `text/plain`.
    pub fn get_content_type(&self) -> String {
        unsafe {
            let ptr = ffi::secret_value_get_content_type(self.to_glib_none());
            from_glib_none(ptr)
        }
    }
}

impl StaticType for SecretValue {
    fn static_type() -> Type{
        unsafe {
            from_glib(ffi::secret_value_get_type())
        }
    }
}

impl Clone for SecretValue {
    fn clone(&self) -> Self {
        unsafe {
            let ptr = ffi::secret_value_ref(self.ptr);
            SecretValue::from_glib_full(ptr)
        }
    }
}

impl Drop for SecretValue {
    fn drop(&mut self){
        unsafe {
            ffi::secret_value_unref(self.ptr as *mut _)
        }
    }
}
