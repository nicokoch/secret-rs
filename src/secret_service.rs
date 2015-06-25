use std::ptr;
use libc::{c_int, c_void};
use glib::{Error};
use glib::translate::FromGlibPtr;
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
    pub fn get() -> Option<Self>{
        SecretService::with_flags(SECRET_SERVICE_OPEN_SESSION)
    }

    pub fn with_flags(flags: SecretServiceFlags) -> Option<Self> {
        let ptr = unsafe {ffi::secret_service_get_sync(flags.bits(), ptr::null_mut(), ptr::null_mut())};
        if ptr.is_null(){
            None
        } else {
            Some(SecretService{
                ptr: ptr,
            })
        }
    }

    pub fn get_flags(&self) -> SecretServiceFlags {
        let flags = unsafe {ffi::secret_service_get_flags(self.ptr)};
        SecretServiceFlags::from_bits(flags).unwrap()
    }

    pub fn get_session_algorithms(&self) -> String {
        let res_c = unsafe {ffi::secret_service_get_session_algorithms(self.ptr)};
        unsafe {FromGlibPtr::from_glib_none(res_c)}
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
