use glib::translate::{ToGlibPtr, FromGlibPtr};
use ffi;

pub struct SecretValue {
    ptr: *mut ffi::SecretValueFFI,
}

impl SecretValue{

    /// Create a SecretValue for the secret data passed in.
    /// This method will *NOT* create the actual secret in the backing Secret Service. (see SecretService.store() for that)
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
    pub fn get(&self) -> Option<String> {
        if self.get_content_type() == "text/plain" {
            unsafe{
                let secret = ffi::secret_value_get_text(self.ptr);
                Some(String::from_glib_none(secret))
            }
        } else {
            None
        }
    }

    /// Get the content type of the secret value, such as `text/plain`.
    pub fn get_content_type(&self) -> String {
        unsafe {
            let ptr = ffi::secret_value_get_content_type(self.ptr);
            String::from_glib_none(ptr)
        }
    }

    /// Workaround method, do not use :-)
    pub unsafe fn wrap(ptr: *mut ffi::SecretValueFFI) -> Self{
        SecretValue {
            ptr: ptr,
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
