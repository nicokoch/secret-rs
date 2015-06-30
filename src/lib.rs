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



#[test]
fn it_works() {}
