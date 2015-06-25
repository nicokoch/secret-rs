extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate glib;
extern crate glib_sys;
extern crate libsecret_sys;

pub use libsecret_sys as ffi;

pub mod secret_service;

use glib::Error;

#[test]
fn it_works() {
    use self::secret_service::SecretService;
    let ss = SecretService::get().ok().unwrap();
}
