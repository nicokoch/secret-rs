use std::ptr;
use glib::Error;
use glib::translate::*;
use secret_service::SecretService;
use secret_item::SecretItem;
use SecretResult;
use util::{lock_object, unlock_object};
use Lock;
use ffi;
use glib_sys as glib_ffi;  // FIXME workaround for bug in glib 0.3.1
use std::mem;  // FIXME workaround for bug in glib 0.3.1
use gobject_sys as gobject_ffi;  // FIXME workaround for bug in glib 0.3.1

/// SecretCollection represents a collection of secret items stored in the
/// Secret Service.
/// A collection can be in a locked or unlocked state. Use `Lock::lock()` or 
/// `Lock::unlock()` to lock or unlock the collection.
/// Use `get_items()` to lookup the items in the collection. There may not be 
/// any items exposed when the collection is locked.
glib_wrapper! {
    pub struct SecretCollection(Object<ffi::SecretCollection, SecretCollectionClass>);

    match fn {
        get_type => || ffi::secret_collection_get_type(),
    }
}

impl SecretCollection {

    /// Lookup which collection is assigned to this alias.
    /// Aliases help determine well known collections, such as 'default'.
    /// Returns the collection, or NULL if none assigned to the alias.
    pub fn for_alias(alias: &str) -> SecretResult<SecretCollection>{
        let mut err = ptr::null_mut();
        let ptr = unsafe {
            ffi::secret_collection_for_alias_sync(
                ptr::null_mut(),
                alias.to_glib_none().0,
                SECRET_COLLECTION_LOAD_ITEMS,
                ptr::null_mut(),
                &mut err
                )
        };
        if err.is_null(){
            Ok(
                unsafe {
                    from_glib_full(ptr)
                }
              )
        } else {
            Err(Error::wrap(err))
        }
    }

    /// Create a new collection in the secret service.
    /// If you specify an alias and a collection with that alias already exists,
    /// then a new collection will not be created. The previous one will be 
    /// returned instead.
    /// Returns the created Collection.
    pub fn create(label: &str, alias: Option<&str>) -> SecretResult<SecretCollection> {
        let mut err = ptr::null_mut();
        let ptr = unsafe {
            ffi::secret_collection_create_sync(
                ptr::null_mut(),
                label.to_glib_none().0,
                alias.to_glib_none().0,
                0,
                ptr::null_mut(),
                &mut err
                )
        };
        if err.is_null(){
            Ok(
                unsafe {
                    from_glib_full(ptr)
                }
            )
        } else {
            Err(Error::wrap(err))
        }
    }

    /// Delete this collection.
    /// The Secret Service may prompt the user.
    pub fn delete(&self) -> SecretResult<()>{
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_collection_delete_sync(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                )
        };
        if err.is_null(){
            Ok(())
        } else {
            Err(Error::wrap(err))
        }
    }

    /// Get the created date and time of the collection.
    /// The return value is the number of seconds since the unix epoch, 
    /// January 1st 1970.
    pub fn get_created(&self) -> u64 {
        unsafe {
            ffi::secret_collection_get_created(
                self.to_glib_none().0
                )
        }
    }

    /// Get the modified date and time of the collection.
    /// The return value is the number of seconds since the unix epoch, 
    /// January 1st 1970.
    pub fn get_modified(&self) -> u64 {
        unsafe {
            ffi::secret_collection_get_modified(
                self.to_glib_none().0
                )
        }
    }

    /// Get the Secret Service object that this collection was created with.
    pub fn get_service(&self) -> SecretService {
        unsafe {
            let ptr = ffi::secret_collection_get_service(
                self.to_glib_none().0
                );
            from_glib_none(ptr)
        }
    }

    /// Get if the items are currently loaded. Use `load_items()` to load them.
    pub fn are_items_loaded(&self) -> bool {
        let flags = unsafe {
            ffi::secret_collection_get_flags(
                self.to_glib_none().0
                )
        };
        flags & SECRET_COLLECTION_LOAD_ITEMS != 0
    }

    /// Get the label of this collection.
    pub fn get_label(&self) -> String {
        unsafe{
            let ptr = ffi::secret_collection_get_label(
                self.to_glib_none().0
                );
            from_glib_full(ptr)
        }
    }

    /// Get the SecretItems of the collection
    pub fn get_items(&self) -> Vec<SecretItem> {
        unsafe {
            let glist = ffi::secret_collection_get_items(
                self.to_glib_none().0
                );
            Vec::from_glib_full(glist)
        }
    }

    /// Ensure that the SecretCollection proxy has loaded all the items present
    /// in the Secret Service.
    pub fn load_items(&self) -> SecretResult<()>{
        unsafe {
            let mut err = ptr::null_mut();
            ffi::secret_collection_load_items_sync(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(Error::wrap(err))
            }
        }
    }

    /// Assign the collection to this alias. Aliases help determine well known
    /// collections, such as 'default'.
    pub fn set_alias(&self, alias: &str) -> SecretResult<()>{
        unsafe {
            let mut err = ptr::null_mut();
            ffi::secret_service_set_alias_sync(
                ptr::null_mut(),
                alias.to_glib_none().0,
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(Error::wrap(err))
            }
        }
    }

    /// Check if the collection is currently locked.
    pub fn is_locked(&self) -> bool {
        let gbool = unsafe {
            ffi::secret_collection_get_locked(
                self.to_glib_none().0
                )
        };
        from_glib(gbool)
    }
}

impl Lock<SecretCollection> for SecretCollection {

    fn lock(&self) -> SecretResult<Vec<SecretCollection>>{
        lock_object::<SecretCollection>(self)
    }

    fn unlock(&self) -> SecretResult<Vec<SecretCollection>>{
        unlock_object::<SecretCollection>(self)
    }
}

#[allow(dead_code)]
const SECRET_COLLECTION_NONE: i32        = 0;
const SECRET_COLLECTION_LOAD_ITEMS: i32  = 1 << 1;


#[cfg(test)]
mod test {
    use glib::types::{StaticType, Type};
    use super::SecretCollection;

    /*
    #[test]
    fn test_sc_create_delete() {
        let sc = SecretCollection::create("cool_label", None).unwrap();
        assert!(!sc.is_locked());
        assert!(sc.are_items_loaded());
        assert_eq!(sc.get_label(), "cool_label");
        assert_eq!(sc.get_items().len(), 0);
        sc.delete().ok().unwrap();
    } */

    #[test]
    pub fn test_sc_static_type() {
        match SecretCollection::static_type() {
            Type::Other(_) => {},
            _ => panic!("Expected Type::Other")
        }
    }
}
