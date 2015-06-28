extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate glib;
extern crate libsecret_sys;

pub use libsecret_sys as ffi;

pub mod secret_service;
pub mod secret_collection;
pub mod secret_item;

#[test]
fn it_works() {
    use self::secret_service::SecretService;
    let ss = SecretService::get().unwrap();
    println!("Session algorithms: {}", ss.get_session_algorithms().unwrap());
    println!("No collections: {}", ss.get_collections().unwrap().len());
}
