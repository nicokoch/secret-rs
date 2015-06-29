use std::marker::PhantomData;
use glib::translate::{ToGlibPtr, FromGlib, FromGlibPtr};
use ffi;

pub struct SecretValue {
    ptr: *mut ffi::SecretValueFFI,
}

impl SecretValue{

    /// Create a SecretValue for the secret data passed in.
    /// This method will create the actual secret in the backing Secret Service.
    /// The secret data is copied into non-pageable 'secure' memory.
    pub fn new(secret: &str) -> SecretValue {
        let content_type = "text/plain";
        unsafe {
            let ptr = ffi::secret_value_new(secret.to_glib_none().0, -1, content_type.to_glib_none().0);
            SecretValue {
                ptr: ptr,
            }
        }
    }

    /// Get the secret data in the SecretValue.
    /// For now, this method only supports String values.
    /// Returns None, if the content type is npt text/plain
    pub fn get_secret(&self) -> Option<String> {
        if self.get_content_type() == "text/plain" {
            unsafe{
                let secret = ffi::secret_value_get_text(self.ptr);
                Some(String::from_glib_full(secret))
            }
        } else {
            None
        }
    }

    /// Get the content type of the secret value, such as `text/plain`.
    pub fn get_content_type(&self) -> String {
        unsafe {
            let ptr = ffi::secret_value_get_content_type(self.ptr);
            String::from_glib_full(ptr)
        }
    }
}

/*
impl Clone<T> for SecretValue<T> {
    fn clone(&self) -> SecretValue<T>{

    }
}*/

impl Drop for SecretValue {
    fn drop(&mut self) {
        unsafe {
            ffi::secret_value_unref(self.ptr as *mut _);
        }
    }
}
