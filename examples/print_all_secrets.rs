extern crate secret;

use secret::{SecretService, Lock};

fn main() {
    let secret_service = SecretService::get().ok().unwrap();
    println!("Session algorithms: {}", secret_service.get_session_algorithms());
    println!("Number of collections: {}\n", secret_service.get_collections().len());
    for secret_collection in secret_service.get_collections() {
        println!("Label for collection: {}\n", secret_collection.get_label());
        if secret_collection.is_locked() {
            println!("Collection is locked");
            secret_collection.unlock().ok().unwrap();
        }
        let all_items = secret_collection.get_items();

        for secret_item in all_items {
            println!("Label for item: {}", secret_item.get_label());
            secret_item.load_secret().ok().unwrap();
            let secret_value = secret_item.get_secret().unwrap();
            println!("ContentType for item: {}", secret_value.get_content_type());
            println!("Attributes for item:");
            for (key, val) in secret_item.get_attributes() {
                println!("({}, {})", key, val);
            }
            println!("SecretValue for item: {}\n", secret_value.get().unwrap());
        }
    }
}
