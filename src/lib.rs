#![warn(missing_docs)]
//! Rust bindings to libsecret.
//! Many unix/linux systems utilize a secret service to securely store and retrieve passwords. Examples for such a SecretService are gnome-keyring and kwallet.
//! This library provides methods to access the system's secret service in a platform independent matter. This is done by linking to [libsecret](https://developer.gnome.org/libsecret/0.18/), a library developed by the gnome project.

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
use glib::ffi::{GList};
use glib::object::{Wrapper};
use glib::translate::{ToGlibPtr, FromGlibPtrContainer, from_glib};
use glib::glib_container::GlibContainer;
use glib::types::StaticType;

/// A Result which may contain an error from the SecretService backend.
pub type SecretResult<T> = Result<T, Error>;

/// This Trait is implemented by objects which can be locked and unlocked
pub trait Lock {

    /// Lock the object.
    fn lock(&self) -> SecretResult<Vec<Self>>;

    /// Unlock the object
    fn unlock(&self) -> SecretResult<Vec<Self>>;

    /// Get if the object is currently locked.
    fn is_locked(&self) -> bool;
}

impl<W: Wrapper> Lock for W{
    fn lock(&self) -> SecretResult<Vec<Self>>{
        let my_type = W::static_type();
        assert!(my_type == SecretItem::static_type() || my_type == SecretCollection::static_type(), "Can only lock items or collections");
        let mut err = ptr::null_mut();
        let mut res = ptr::null_mut();
        let arr = [self];
        let slice = (&arr[..]).to_glib_none();
        unsafe {
            ffi::secret_service_lock_sync(ptr::null_mut(), slice.0 as *mut GList, ptr::null_mut(), &mut res, &mut err);
            if err.is_null() {
                Ok(Vec::from_glib_full(res))
            } else {
                Err(Error::wrap(err))
            }
        }
    }

    fn unlock(&self) -> SecretResult<Vec<Self>>{
        println!("Unlocking");
        let my_type = W::static_type();
        assert!(my_type == SecretItem::static_type() || my_type == SecretCollection::static_type(), "Can only unlock items or collections");
        let mut err = ptr::null_mut();
        let mut res = ptr::null_mut();
        let arr = [self];
        let slice = (&arr[..]).to_glib_none();
        unsafe {
            ffi::secret_service_unlock_sync(ptr::null_mut(), slice.0 as *mut GList, ptr::null_mut(), &mut res, &mut err);
            if err.is_null() {
                Ok(Vec::from_glib_full(res))
            } else {
                Err(Error::wrap(err))
            }
        }
    }

    fn is_locked(&self) -> bool {
        let my_type = W::static_type();
        let gbool = if my_type == SecretItem::static_type() {
            unsafe{ffi::secret_item_get_locked(self.as_ref().to_glib_none() as *mut ffi::SecretItem)}
        } else if my_type == SecretCollection::static_type() {
            unsafe{ffi::secret_collection_get_locked(self.as_ref().to_glib_none() as *mut ffi::SecretCollection)}
        } else {
            panic!("Only items and collections can be locked")
        };
        from_glib(gbool)
    }
}
