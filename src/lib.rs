extern crate libc;
#[macro_use]
extern crate bitflags;
extern crate glib;
extern crate libsecret_sys;

pub use libsecret_sys as ffi;

pub mod secret_service;
pub mod secret_collection;
pub mod secret_item;
pub mod secret_value;

#[test]
fn it_works() {
    use self::secret_service::SecretService;
    let secret_service = SecretService::get().unwrap();
    println!("Session algorithms: {}", secret_service.get_session_algorithms().unwrap());
    println!("No collections: {}", secret_service.get_collections().unwrap().len());
    for secret_collection in secret_service.get_collections().unwrap() {
        println!("Label for collection: {}", secret_collection.get_label());
        if secret_collection.get_locked() {
            println!("Collection is locked");
        }

        if secret_collection.load_items(){
            println!("loaded items");
            let all_items = match secret_collection.get_items(){
                Some(items) => items,
                None => continue
            };

            for secret_item in all_items {
                println!("Label for item: {}", secret_item.get_label());
                secret_item.load_secret();
                let secret_value = secret_item.get_secret().unwrap();
                println!("CT for item: {}", secret_value.get_content_type());
                println!("SecretValue for item: {}", secret_value.get().unwrap());
            }
        }
    }
}
