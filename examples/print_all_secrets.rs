extern crate secret;

use secret::SecretService;

fn main() {
    let secret_service = SecretService::get().ok().unwrap();
    println!("Session algorithms: {}", secret_service.get_session_algorithms().unwrap());
    println!("Number of collections: {}\n", secret_service.get_collections().unwrap().len());
    for secret_collection in secret_service.get_collections().unwrap() {
        println!("Label for collection: {}\n", secret_collection.get_label());
        if secret_collection.get_locked() {
            println!("Collection is locked");
        }
        let all_items = match secret_collection.get_items(){
            Some(items) => items,
            None => continue
        };

        for secret_item in all_items {
            println!("Label for item: {}", secret_item.get_label());
            secret_item.load_secret();
            let secret_value = secret_item.get_secret().unwrap();
            println!("ContentType for item: {}", secret_value.get_content_type());
            println!("SecretValue for item: {}\n", secret_value.get().unwrap());
        }
    }
}
