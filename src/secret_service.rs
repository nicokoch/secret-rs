
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
use glib::object::{Ref, Wrapper};
use glib::types::{StaticType, Type};
use glib::translate::{FromGlib, FromGlibPtr, FromGlibPtrContainer};
use glib::ffi::{GObject};
use ffi;

pub struct SecretService(Ref);

impl SecretService {

    /// Constructs a new SecretService which has established a session and whose collections are loaded.
    /// The underlying FFI object might be identical for multiple instances of this struct.
    pub fn get() -> Option<Self>{
        SecretService::with_flags(SECRET_SERVICE_OPEN_SESSION | SECRET_SERVICE_LOAD_COLLECTIONS)
    }

    fn with_flags(flags: i32) -> Option<Self> {
        let ptr = unsafe {ffi::secret_service_get_sync(flags, ptr::null_mut(), ptr::null_mut())};
        if ptr.is_null(){
            None
        } else {
            Some(SecretService(Ref::from_glib_none(ptr as *mut GObject)))
        }
    }

    #[inline]
    fn raw(&self) -> *mut ffi::SecretServiceFFI {
        self.0.to_glib_none() as *mut ffi::SecretServiceFFI
    }

    pub fn is_session_established(&self) -> bool {
        let flags = unsafe {ffi::secret_service_get_flags(self.raw())};
        flags & SECRET_SERVICE_OPEN_SESSION != 0
    }

    pub fn are_collections_loaded(&self) -> bool {
        let flags = unsafe {ffi::secret_service_get_flags(self.raw())};
        flags & SECRET_SERVICE_LOAD_COLLECTIONS != 0
    }

    /// Get the set of algorithms being used to transfer secrets between this secret service proxy and the Secret Service itself.
    /// Returns `None` if no session has been established yet.
    /// The contained String has the format "algorithm-algorithm-algorithm-..."
    pub fn get_session_algorithms(&self) -> Option<String> {
        unsafe{
            let res_c = ffi::secret_service_get_session_algorithms(self.raw());
            if res_c.is_null(){
                None
            } else {
                Some(FromGlibPtr::from_glib_none(res_c))
            }
        }
    }

    pub fn get_collections(&self) -> Option<Vec<SecretCollection>> {
        unsafe {
            let glist = ffi::secret_service_get_collections(self.raw());
            if glist.is_null(){
                None
            } else {
                Some(FromGlibPtrContainer::from_glib_none(glist))
            }
        }
    }

    /*
    pub fn search() -> Vec<SecretItem> {
        unimplemented!()
    }
    */

    /*
    pub fn lock () -> Vec<SecretItem> {

    }
    */

    /*
    pub fn unlock () -> Vec<SecretItem> {

    }
    */

    /*
    pub fn store () -> bool {

    }
    */

    /*
    pub fn lookup () -> SecretValue {

    }
    */

    /*
    pub fn clear () -> bool {

    }
    */

    /*
    pub fn set_alias(&str alias, ) -> bool {
        //FIXME: actually we should put this into SecretCollection
    }
    */

    /// Ensures that a session is established.
    /// This function should rarely be needed. Construct a SecretService with the `SECRET_SERVICE_OPEN_SESSION` flag instead.
    /// Returns true if a session has been established, false otherwise.
    pub fn ensure_session(&self) -> bool {
        unsafe {
            let established = ffi::secret_service_ensure_session_sync(self.raw(), ptr::null_mut(), ptr::null_mut());
            FromGlib::from_glib(established)
        }
    }

    /// Ensures that the collections are loaded.
    /// This function should rarely be needed. Construct a SecretService with the `SECRET_SERVICE_LOAD_COLLECTIONS` flag instead.
    /// Returns true if a session has been established, false otherwise.
    pub fn load_collections(&self) -> bool {
        unsafe {
            let loaded = ffi::secret_service_load_collections_sync(self.raw(), ptr::null_mut(), ptr::null_mut());
            FromGlib::from_glib(loaded)
        }
    }
}

impl StaticType for SecretService {
    fn static_type() -> Type{
        Type::BaseObject //TODO get this from libsecret
    }
}

impl Wrapper for SecretService {
    type GlibType = ffi::SecretServiceFFI;

    unsafe fn wrap(r: Ref) -> Self{
        SecretService(r)
    }

    fn as_ref(&self) -> &Ref{
        &self.0
    }

    fn unwrap(self) -> Ref{
        self.0
    }
}

#[allow(dead_code)]
const SECRET_SERVICE_NONE: i32              = 0;
const SECRET_SERVICE_OPEN_SESSION: i32      = 1 << 1;
const SECRET_SERVICE_LOAD_COLLECTIONS: i32  = 1 << 2;

#[allow(dead_code)]
const SECRET_SEARCH_NONE: i32               = 0;
#[allow(dead_code)]
const SECRET_SEARCH_ALL: i32                = 1 << 1;
#[allow(dead_code)]
const SECRET_SEARCH_UNLOCK: i32             = 1 << 2;
#[allow(dead_code)]
const SECRET_SEARCH_LOAD_SECRETS: i32       = 1 << 3;
