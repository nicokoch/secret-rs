use std::ptr;
use libc::{c_int};
use glib::ffi::{GObject};
use glib::object::{Wrapper, Ref};
use glib::types::{StaticType, Type};
use glib::translate::{ToGlibPtr, FromGlib, FromGlibPtr};
use secret_service::SecretService;
use ffi;

bitflags! {
    flags SecretCollectionFlags: c_int {
        const SECRET_COLLECTION_NONE        = 0,
        const SECRET_COLLECTION_LOAD_ITEMS  = 1 << 1,
    }
}

pub struct SecretCollection(Ref);

impl SecretCollection {

    /// Lookup which collection is assigned to this alias.
    /// Aliases help determine well known collections, such as 'default'.
    /// Returns the collection, or NULL if none assigned to the alias.
    pub fn for_alias(alias: &str, flags: SecretCollectionFlags) -> Option<SecretCollection>{
        let ptr = unsafe{ffi::secret_collection_for_alias_sync(ptr::null_mut(), alias.to_glib_none().0, flags.bits(), ptr::null_mut(), ptr::null_mut())};
        if ptr.is_null(){
            None
        } else {
            Some(SecretCollection(Ref::from_glib_full(ptr as *mut GObject)))
        }
    }

    /// Create a new collection in the secret service.
    /// If you specify an alias and a collection with that alias already exists, then a new collection will not be created. The previous one will be returned instead.
    /// Return the creates Collection.
    pub fn create(label: &str, alias: Option<&str>) -> SecretCollection {
        let ptr = unsafe{ffi::secret_collection_create_sync(ptr::null_mut(), label.to_glib_none().0, alias.to_glib_none().0, 0 as c_int, ptr::null_mut(), ptr::null_mut())};
        SecretCollection(Ref::from_glib_full(ptr as *mut GObject))
    }

    /// Get the created date and time of the collection.
    /// The return value is the number of seconds since the unix epoch, January 1st 1970.
    pub fn get_created(&self) -> u64 {
        unsafe {ffi::secret_collection_get_created(self.raw())}
    }

    /// Get the modified date and time of the collection.
    /// The return value is the number of seconds since the unix epoch, January 1st 1970.
    pub fn get_modified(&self) -> u64 {
        unsafe {ffi::secret_collection_get_modified(self.raw())}
    }

    /// Get the Secret Service object that this collection was created with.
    pub fn get_service(&self) -> SecretService { //TODO find out if this can return null
        unsafe {
            let ptr = ffi::secret_collection_get_service(self.raw());
            SecretService::wrap(Ref::from_glib_full(ptr as *mut GObject))
        }
    }

    /// Get the flags representing what features of the SecretCollection have been initialized.
    pub fn get_flags(&self) -> SecretCollectionFlags {
        let flags = unsafe {ffi::secret_collection_get_flags(self.raw())};
        SecretCollectionFlags::from_bits(flags).unwrap()
    }

    /// Get the label of this collection.
    pub fn get_label(&self) -> String {
        unsafe{
            let ptr = ffi::secret_collection_get_label(self.raw());
            FromGlibPtr::from_glib_full(ptr)
        }
    }

    /// Get whether the collection is locked or not.
    pub fn get_locked(&self) -> bool {
        let gbool = unsafe{ffi:: secret_collection_get_locked(self.raw())};
        FromGlib::from_glib(gbool)
    }


    #[inline]
    fn raw(&self) -> *mut ffi::SecretCollectionFFI {
        self.0.to_glib_none() as *mut ffi::SecretCollectionFFI
    }

    /// Ensure that the SecretCollection proxy has loaded all the items present in the Secret Service.
    pub fn load_items(&self) -> bool {
        unsafe {
            let loaded = ffi::secret_collection_load_items_sync(self.raw(), ptr::null_mut(), ptr::null_mut());
            FromGlib::from_glib(loaded)
        }
    }

    //TODO set_alias from ss
}

impl StaticType for SecretCollection {
    fn static_type() -> Type{
        Type::BaseObject
    }
}

impl Wrapper for SecretCollection {
    type GlibType = ffi::SecretCollectionFFI;
    unsafe fn wrap(r: Ref) -> Self{
        SecretCollection(r)
    }

    fn as_ref(&self) -> &Ref{
        &self.0
    }

    fn unwrap(self) -> Ref{
        self.0
    }
}
