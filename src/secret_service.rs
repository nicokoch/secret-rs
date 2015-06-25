use std::ptr;
use libc::{c_int, c_void};
use glib::Error;
use glib_sys::g_object_unref;
use ffi;

bitflags! {
    flags SecretServiceFlags: c_int {
        const SECRET_SERVICE_NONE               = 0,
        const SECRET_SERVICE_OPEN_SESSION       = 1 << 1,
        const SECRET_SERVICE_LOAD_COLLECTIONS   = 1 << 2,
    }
}

pub type SecretResult<T> = Result<T, Error>;

pub struct SecretService {
    ptr: *mut ffi::SecretServiceFFI,
}

impl SecretService {
    pub fn get() -> SecretResult<Self>{
        SecretService::get_with_flags(SECRET_SERVICE_NONE)
    }

    pub fn get_with_flags(flags: SecretServiceFlags) -> SecretResult<Self> {
        let ptr = unsafe {ffi::secret_service_get_sync(flags.bits(), ptr::null_mut(), ptr::null_mut())};
        Ok(SecretService{
            ptr: ptr,
        })
    }
}

impl Drop for SecretService{
    fn drop(&mut self){
        unsafe{
            g_object_unref(self.ptr as *mut c_void);
        }
        self.ptr = ptr::null_mut();
    }
}
