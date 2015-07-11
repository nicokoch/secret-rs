extern crate secret;

use std::collections::HashMap;
use secret::SecretService;

fn main() {
    let secret_service = SecretService::get().ok().unwrap();

    let mut search_attrs = HashMap::new();
    search_attrs.insert("application".to_string(), "chrome-966050".to_string());
    let items_chrome = secret_service.search(&search_attrs).ok().unwrap();
    println!("Number of searchresults: {}\n", items_chrome.len());
}
