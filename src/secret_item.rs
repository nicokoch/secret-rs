use std::ptr;
use libc::{c_int};
use glib_sys::{GObject};
use glib::object::{Wrapper, Ref};
use glib::types::{StaticType, Type};
use glib::translate::{ToGlibPtr, FromGlib, FromGlibPtr};
use secret_service::SecretService;
use ffi;

pub struct SecretItem(Ref);

impl SecretItem {

    /// Delete this secret item.
    pub fn delete(&self) -> bool {
        let gbool = unsafe{ffi::secret_item_delete_sync(self.raw(), ptr::null_mut(), ptr::null_mut())};
        FromGlib::from_glib(gbool)
    }

    pub fn get_schema_name(&self) -> String {
        unsafe {
            let ptr = ffi::secret_item_get_schema_name(self.raw());
            FromGlibPtr::from_glib_full(ptr)
        }
    }

    /*
    pub fn get_attributes(&self) -> HashMap{ TODO

    }
    */

    pub fn get_created(&self) -> u64 {
        unsafe {ffi::secret_item_get_created(self.raw())}
    }

    pub fn get_modified(&self) -> u64 {
        unsafe {ffi::secret_item_get_modified(self.raw())}
    }

    pub fn get_label(&self) -> String {
        unsafe {
            let ptr = ffi::secret_item_get_label(self.raw());
            FromGlibPtr::from_glib_full(ptr)
        }
    }

    pub fn get_locked(&self) -> bool {
        let gbool = unsafe{ffi::secret_item_get_locked(self.raw())};
        FromGlib::from_glib(gbool)
    }

    pub fn get_service(&self) -> SecretService {
        unsafe {
            let ptr = ffi::secret_item_get_service(self.raw());
            SecretService::wrap(Ref::from_glib_full(ptr as *mut GObject))
        }
    }

    pub fn load_secret(&self) -> bool {
        let gbool = unsafe{ffi::secret_item_load_secret_sync(self.raw(), ptr::null_mut(), ptr::null_mut())};
        FromGlib::from_glib(gbool)
    }

    #[inline]
    fn raw(&self) -> *mut ffi::SecretItemFFI {
        self.0.to_glib_none() as *mut ffi::SecretItemFFI
    }

}

impl StaticType for SecretItem {
    fn static_type() -> Type{
        Type::BaseObject //TODO?
    }
}

impl Wrapper for SecretItem {
    type GlibType = ffi::SecretItemFFI;
    unsafe fn wrap(r: Ref) -> Self{
        SecretItem(r)
    }

    fn as_ref(&self) -> &Ref{
        &self.0
    }

    fn unwrap(self) -> Ref{
        self.0
    }
}
