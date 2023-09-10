use std::ptr;
use std::collections::HashMap;
use glib::translate::*;
use glib::wrapper;
use secret_service::SecretService;
use secret_collection::SecretCollection;
use secret_value::SecretValue;
use SecretResult;
use ffi;
use util::{lock_object, unlock_object};
use Lock;

wrapper! {
    /// SecretItem represents a secret item stored in the Secret Service.
    /// Each item has a value, represented by a SecretValue, which can be retrieved
    /// by `get_secret()` or set by `set_secret()`. The item is only available when
    /// the item is not locked.
    /// Items can be locked or unlocked using the `Lock::lock()` or `Lock::unlock()`
    /// functions. The Lock trait is implemented by SecretItem. The Secret Service
    /// may not be able to unlock individual items, and may unlock an entire 
    /// collection when a single item is unlocked.
    /// Each item has a set of attributes, which are used to locate the item later.
    /// These are not stored or transferred in a secure manner. Each attribute has
    /// a string name and a string value. Use `SecretService::search()` to search 
    /// for items based on their attributes, and `set_attributes()` to change the 
    /// attributes associated with an item.
    /// Items can be created with `create()` or `SecretService::store()`.
    ///
    pub struct SecretItem(Object<ffi::SecretItem, ffi::SecretItemClass>);

    match fn {
        type_ => || ffi::secret_item_get_type(),
    }
}

impl SecretItem {

    /// Create a new item in the secret service.
    /// collection: a secret collection to create this item in
    /// attributes: attributes for the new item.
    /// label: label for the new item
    /// value: secret value for the new item
    pub fn create(collection: &SecretCollection, attributes: &HashMap<String, String>, label: &str, value: &SecretValue) -> SecretResult<SecretItem> {
        let mut err = ptr::null_mut();
        unsafe {
            let item = ffi::secret_item_create_sync(
                collection.to_glib_none().0,
                ptr::null(),
                attributes.to_glib_none().0,
                label.to_glib_none().0,
                value.to_glib_none(),
                SECRET_ITEM_CREATE_NONE,
                ptr::null_mut(),
                &mut err
                );

            if err.is_null() { //TODO for all patterns like this: This if does not need to be in the unsafe block. Fix pls.
                Ok(from_glib_full(item))
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    /// Delete this secret item.
    pub fn delete(&self) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_item_delete_sync(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                )
        };
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

    /// Get the name of the attribute schema.
    pub fn get_schema_name(&self) -> String {
        unsafe {
            let ptr = ffi::secret_item_get_schema_name(self.to_glib_none().0);
            from_glib_full(ptr)
        }
    }

    /// Get the created date and time of the item.
    /// The return value is the number of seconds since the unix epoch, January 1st 1970.
    pub fn get_created(&self) -> u64 {
        unsafe {
            ffi::secret_item_get_created(self.to_glib_none().0)
        }
    }

    /// Get the modified date and time of the item.
    /// The return value is the number of seconds since the unix epoch, January 1st 1970.
    pub fn get_modified(&self) -> u64 {
        unsafe {
            ffi::secret_item_get_modified(self.to_glib_none().0)
        }
    }

    /// Get the label of the item.
    pub fn get_label(&self) -> String {
        unsafe {
            let ptr = ffi::secret_item_get_label(self.to_glib_none().0);
            from_glib_full(ptr)
        }
    }

    /// Get the SecretService this item was created with.
    pub fn get_service(&self) -> SecretService {
        unsafe {
            let ptr = ffi::secret_item_get_service(self.to_glib_none().0);
            from_glib_none(ptr)
        }
    }

    /// Ensure that the SecretValue of this item is loaded.
    pub fn load_secret(&self) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_item_load_secret_sync(
                self.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                )
        };
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

    /// Get the SecretValue of this item. The item must be unlocked and the 
    /// value must be loaded.
    pub fn get_secret(&self) -> Option<SecretValue> {
        unsafe {
            let ptr = ffi::secret_item_get_secret(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(SecretValue::from_glib_full(ptr))
            }
        }
    }

    /// Set the secret value of this item.
    /// Each item has a single secret which might be a password or some other 
    /// secret binary value (not supported yet).
    pub fn set_secret(&self, value: &SecretValue) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_item_set_secret_sync(
                self.to_glib_none().0,
                value.to_glib_none(),
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    /// Get the attributes of this item.
    pub fn get_attributes(&self) -> HashMap<String, String> {
        unsafe {
            let ptr = ffi::secret_item_get_attributes(self.to_glib_none().0);
            HashMap::from_glib_full(ptr)
        }
    }

    /// Set the attributes of this item.
    pub fn set_attributes(&self, attributes: &HashMap<String, String>) -> SecretResult<()> {
        let mut err = ptr::null_mut();
        unsafe {
            ffi::secret_item_set_attributes_sync(
                self.to_glib_none().0,
                ptr::null(),
                attributes.to_glib_none().0,
                ptr::null_mut(),
                &mut err
                );
            if err.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    /// Check if the item is currently locked.
    pub fn is_locked(&self) -> bool {
        let gbool = unsafe {
            ffi::secret_item_get_locked(self.to_glib_none().0)
        };
        unsafe { from_glib(gbool) }
    }
}

impl Lock<SecretItem> for SecretItem {

    fn lock(&self) -> SecretResult<Vec<SecretItem>>{
        lock_object::<SecretItem>(self)
    }

    fn unlock(&self) -> SecretResult<Vec<SecretItem>>{
        unlock_object::<SecretItem>(self)
    }
}

const SECRET_ITEM_CREATE_NONE: i32        = 0;
#[allow(dead_code)]
const SECRET_ITEM_CREATE_REPLACE: i32     = 1 << 1;
