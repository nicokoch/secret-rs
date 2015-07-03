use std::ptr;
use glib::Error;
use glib::glib_container::GlibContainer;
use glib::object::{Wrapper, Ref, Object, Upcast};
use glib::types::{StaticType, Type};
use glib::translate::*;
use secret_service::SecretService;
use secret_value::SecretValue;
use SecretResult;
use ffi;

pub struct SecretItem(Ref);

impl SecretItem {

    /// Delete this secret item.
    pub fn delete(&self) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe{ffi::secret_item_delete_sync(self.to_glib_none().0, ptr::null_mut(), &mut err)};
        if err.is_null() {
            Ok(())
        } else {
            Err(Error::wrap(err))
        }
    }

    pub fn get_schema_name(&self) -> String {
        unsafe {
            let ptr = ffi::secret_item_get_schema_name(self.to_glib_none().0);
            from_glib_full(ptr)
        }
    }

    /*
    pub fn get_attributes(&self) -> HashMap{ TODO

    }
    */

    pub fn get_created(&self) -> u64 {
        unsafe {ffi::secret_item_get_created(self.to_glib_none().0)}
    }

    pub fn get_modified(&self) -> u64 {
        unsafe {ffi::secret_item_get_modified(self.to_glib_none().0)}
    }

    pub fn get_label(&self) -> String {
        unsafe {
            let ptr = ffi::secret_item_get_label(self.to_glib_none().0);
            from_glib_full(ptr)
        }
    }

    pub fn get_locked(&self) -> bool {
        let gbool = unsafe{ffi::secret_item_get_locked(self.to_glib_none().0)};
        from_glib(gbool)
    }

    pub fn get_service(&self) -> SecretService {
        unsafe {
            let ptr = ffi::secret_item_get_service(self.to_glib_none().0);
            from_glib_none(ptr)
        }
    }

    pub fn load_secret(&self) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe{ffi::secret_item_load_secret_sync(self.to_glib_none().0, ptr::null_mut(), &mut err)};
        if err.is_null() {
            Ok(())
        } else {
            Err(Error::wrap(err))
        }
    }

    pub fn get_secret(&self) -> Option<SecretValue> {
        unsafe {
            let ptr = ffi::secret_item_get_secret(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(SecretValue::wrap(ptr))
            }
        }
    }
}

impl StaticType for SecretItem {
    fn static_type() -> Type{
        Type::BaseObject //TODO?
    }
}

unsafe impl Upcast<Object> for SecretItem { }

impl Wrapper for SecretItem {
    type GlibType = ffi::SecretItem;
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
