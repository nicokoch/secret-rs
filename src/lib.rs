#![warn(missing_docs)]
extern crate libc;
extern crate glib;
extern crate libsecret_sys;

mod secret_service;
mod secret_collection;
mod secret_item;
mod secret_value;

pub use self::libsecret_sys as ffi;
pub use self::secret_service::SecretService;
pub use self::secret_collection::SecretCollection;
pub use self::secret_item::SecretItem;
pub use self::secret_value::SecretValue;

use std::ptr;
use glib::Error;
use libc::{c_void};
use glib::ffi::{GList};
use glib::object::{Wrapper};
use glib::translate::{ToGlibPtr, FromGlibPtrContainer};
use glib::glib_container::GlibContainer;

pub type SecretResult<T> = Result<T, Error>;

/// This Trait is implemented by objects which can be locked and unlocked
pub trait Lock {
    /// Lock the object.
    fn lock(&self) -> SecretResult<Vec<Self>>;
    /// Unlock the object
    fn unlock(&self) -> SecretResult<Vec<Self>>;
}

impl<W: Wrapper> Lock for W{
    fn lock(&self) -> SecretResult<Vec<Self>>{
        let mut err = ptr::null_mut();
        let mut res = ptr::null_mut();
        //TODO: We can definitely solve this with ToGlibPtrContainer somehow
        let mut arr = GList{
            data: self.as_ref().to_glib_none() as *mut c_void,
            next: ptr::null_mut(),
            prev: ptr::null_mut()
        };
        unsafe {
            ffi::secret_service_lock_sync(ptr::null_mut(), &mut arr, ptr::null_mut(), &mut res, &mut err);
            if err.is_null() {
                Ok(FromGlibPtrContainer::from_glib_none(res))
            } else {
                Err(Error::wrap(err))
            }
        }
    }

    fn unlock(&self) -> SecretResult<Vec<Self>>{
        let mut err = ptr::null_mut();
        let mut res = ptr::null_mut();
        //TODO: We can definitely solve this with ToGlibPtrContainer somehow
        let mut arr = GList{
            data: self.as_ref().to_glib_none() as *mut c_void,
            next: ptr::null_mut(),
            prev: ptr::null_mut()
        };
        unsafe {
            ffi::secret_service_unlock_sync(ptr::null_mut(), &mut arr, ptr::null_mut(), &mut res, &mut err);
            if err.is_null() {
                Ok(FromGlibPtrContainer::from_glib_none(res))
            } else {
                Err(Error::wrap(err))
            }
        }
    }
}
