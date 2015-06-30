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

use glib::Error;

pub type SecretResult<T> = Result<T, Error>;

/// This Trait is implemented by objects which can be locked and unlocked
pub trait Lock {
    fn lock(&self) -> Option<Vec<Self>>;
    fn unlock(&self) -> Option<Vec<Self>>;
}

#[test]
fn it_works() {}
