//=========================================================================
// public imports
//=========================================================================
pub use secret_collection::*;
pub use secret_item::*;
pub use secret_value::*;

//=========================================================================
// private imports
//=========================================================================
use std::ptr;
use std::collections::HashMap;
use glib::Error;
use glib::translate::*;
use SecretResult;
use ffi;
use glib_sys as glib_ffi;  // FIXME workaround for bug in glib 0.3.1
use std::mem;  // FIXME workaround for bug in glib 0.3.1
use gobject_sys as gobject_ffi;  // FIXME workaround for bug in glib 0.3.1

/// A SecretService object represents the Secret Service implementation which 
/// runs as a D-Bus service.
/// In order to securely transfer secrets to the Sercret Service, a session is 
/// established. This will automatically be done when calling 
/// `SecretService::get()`
/// To search for items, use the `search()` method.
/// Multiple collections can exist in the Secret Service, each of which 
/// contains secret items. To access the list of Collections, use 
/// `get_collections()`.
/// Certain actions on the Secret Service require user prompting to complete, 
/// such as creating a collection, or unlocking a collection. When such a 
/// prompt is necessary, then a SecretPrompt object is created by libsecret, 
/// and passed to the secret_service_prompt() method. In this way it is handled
/// automatically.
wrapper! {
    pub struct SecretService(Object<ffi::SecretService, ffi::SecretServiceClass>);

    match fn {
        type_ => || ffi::secret_service_get_type(),
    }
}

impl SecretService {

    /// Constructs a new SecretService which has established a session and whose
    /// collections are loaded.
    /// The underlying FFI object might be identical for multiple instances of 
    /// this struct.
    pub fn get() -> SecretResult<Self>{
        SecretService::with_flags(SECRET_SERVICE_OPEN_SESSION | SECRET_SERVICE_LOAD_COLLECTIONS)
    }

    fn with_flags(flags: i32) -> SecretResult<Self> {
        let mut err = ptr::null_mut();
        let ptr = unsafe {
            ffi::secret_service_get_sync(flags, ptr::null_mut(), &mut err)
        };
        if err.is_null() {
            Ok(unsafe{from_glib_full(ptr)})
        } else {
            Err(
                unsafe {
                    from_glib_full(err)
                }
            )
        }
    }

    /// Returns if a session to the SecretService is currently established.
    pub fn is_session_established(&self) -> bool {
        let flags = unsafe {
            ffi::secret_service_get_flags(self.to_glib_none().0)
        };
        flags & SECRET_SERVICE_OPEN_SESSION != 0
    }

    /// Returns if the Service's collections are loaded.
    pub fn are_collections_loaded(&self) -> bool {
        let flags = unsafe {
            ffi::secret_service_get_flags(self.to_glib_none().0)
        };
        flags & SECRET_SERVICE_LOAD_COLLECTIONS != 0
    }

    /// Get the set of algorithms being used to transfer secrets between this 
    /// secret service proxy and the Secret Service itself.
    /// The contained String has the format "algorithm-algorithm-algorithm-..."
    pub fn get_session_algorithms(&self) -> String {
        unsafe{
            let ptr = ffi::secret_service_get_session_algorithms(
                self.to_glib_none().0
                );
            from_glib_none(ptr)
        }
    }

    /// Get the collections of the Service.
    /// A collection contains multiple SecretItems.
    pub fn get_collections(&self) -> Vec<SecretCollection> {
        unsafe {
            let glist = ffi::secret_service_get_collections(
                self.to_glib_none().0
                );
            Vec::from_glib_full(glist)
        }
    }

    /// Search for items matching the attributes. All collections are searched.
    /// The attributes should be a table of string keys and string values.
    pub fn search(&self, attributes: &HashMap<String, String>) -> SecretResult<Vec<SecretItem>> {
        let mut err = ptr::null_mut();
        unsafe {
            let glist = ffi::secret_service_search_sync(
                self.to_glib_none().0,
                ptr::null(), attributes.to_glib_none().0,
                SECRET_SEARCH_ALL | SECRET_SEARCH_UNLOCK | SECRET_SEARCH_LOAD_SECRETS,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(Vec::from_glib_full(glist))
            } else {
                Err(
                    unsafe {
                        from_glib_full(err)
                    }
                )
            }
        }
    }

    /// Store a secret value in the secret service.
    /// The `attributes` should be a set of key and value string pairs.
    /// If the attributes match a secret item already stored in the collection,
    /// then the item will be updated with these new values.
    /// `collection` is a collection alias, or `None` to store the value in the
    /// default collection (TODO: What about session storage?)
    /// `label` specifies a label for the secret.
    /// `value` is the actual secret to store. This can be created with 
    /// `SecretValue::new()`.
    pub fn store(&self, attributes: &HashMap<String, String>, collection: Option<&str>, label: &str, value: &SecretValue) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_service_store_sync(
                self.to_glib_none().0,
                ptr::null(),
                attributes.to_glib_none().0,
                collection.to_glib_none().0,
                label.to_glib_none().0,
                value.to_glib_none(),
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(
                    unsafe {
                        from_glib_full(err)
                    }
                )
            }
        }
    }

    /*
    pub fn lookup () -> SecretValue {

    }
    */

    /// Remove unlocked items which match the attributes from the secret service.
    /// The attributes should be a set of key and value string pairs.
    pub fn clear(&self, attributes: &HashMap<String, String>) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_service_clear_sync(
                self.to_glib_none().0,
                ptr::null(),
                attributes.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(
                    unsafe {
                        from_glib_full(err)
                    }
                )
            }
        }
    }

    /// Ensures that a session is established.
    pub fn ensure_session(&self) -> SecretResult<()> {
        unsafe {
            let mut err = ptr::null_mut();
            ffi::secret_service_ensure_session_sync(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(
                    unsafe {
                        from_glib_full(err)
                    }
                )
            }
        }
    }

    /// Ensures that the collections are loaded.
    pub fn load_collections(&self) -> SecretResult<()> {
        unsafe {
            let mut err = ptr::null_mut();
            ffi::secret_service_load_collections_sync(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(
                    unsafe {
                        from_glib_full(err)
                    }
                )
            }
        }
    }
}

#[allow(dead_code)]
const SECRET_SERVICE_NONE: i32              = 0;
const SECRET_SERVICE_OPEN_SESSION: i32      = 1 << 1;
const SECRET_SERVICE_LOAD_COLLECTIONS: i32  = 1 << 2;

#[allow(dead_code)]
const SECRET_SEARCH_NONE: i32               = 0;
const SECRET_SEARCH_ALL: i32                = 1 << 1;
const SECRET_SEARCH_UNLOCK: i32             = 1 << 2;
const SECRET_SEARCH_LOAD_SECRETS: i32       = 1 << 3;

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use glib::types::{StaticType, Type};
    use secret_value::SecretValue;
    use super::SecretService;

    #[test]
    pub fn test_ss_get() {
        let res = SecretService::get();
        assert!(res.is_ok());
        let ss = res.unwrap();
        assert!(ss.is_session_established());
        assert!(ss.are_collections_loaded());
    }

    #[test]
    pub fn test_ss_session_algorithms() {
        let ss = SecretService::get().ok().unwrap();
        assert!(ss.get_session_algorithms() == "plain" || ss.get_session_algorithms() == "dh-ietf1024-sha256-aes128-cbc-pkcs7");
    }

    #[test]
    #[ignore = "glib 0.14.x no longer uses an enum Type"]
    pub fn test_ss_static_type() {
        //match SecretService::static_type() {
        //    Type::Other(_) => {},
        //    _ => panic!("Expected Type::Other")
        //}
    }

    #[test]
    pub fn test_ss_store_search_clear() {
        let ss = SecretService::get().unwrap();
        let mut attrs: HashMap<String, String> = HashMap::new();
        attrs.insert("application".into(), "secret-rs-unit-test".into());

        let sv = SecretValue::new("password123");

        ss.store(&attrs, None, "mylabel", &sv).unwrap();
        let search_results = ss.search(&attrs).unwrap();
        assert!(search_results.len() == 1);
        let item = search_results.get(0).unwrap();
        assert_eq!(item.get_label(), "mylabel");
        assert_eq!(item.get_attributes().get("application").unwrap(), "secret-rs-unit-test");
        assert_eq!(item.get_secret().unwrap().get().unwrap(), "password123");
        ss.clear(&attrs).unwrap();
        let search_results = ss.search(&attrs).unwrap();
        assert!(search_results.len() == 0);
    }
}
