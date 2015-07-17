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
mod util;

pub use self::libsecret_sys as ffi;
pub use self::secret_service::SecretService;
pub use self::secret_collection::SecretCollection;
pub use self::secret_item::SecretItem;
pub use self::secret_value::SecretValue;

use glib::Error;

/// A Result which may contain an error from the SecretService backend.
pub type SecretResult<T> = Result<T, Error>;

/// This Trait is implemented by objects which can be locked and unlocked
pub trait Lock<T> {

    /// Lock the object.
    fn lock(&self) -> SecretResult<Vec<T>>;

    /// Unlock the object
    fn unlock(&self) -> SecretResult<Vec<T>>;
}
