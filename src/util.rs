use std::ptr;
use glib::Error;
use glib::ffi::{GList};
use glib::object::{Wrapper};
use glib::translate::{ToGlibPtr, FromGlibPtrContainer};
use glib::glib_container::GlibContainer;
use glib::types::StaticType;
use secret_item::SecretItem;
use secret_collection::SecretCollection;
use ffi;
use SecretResult;

pub fn lock_object<W: Wrapper>(obj: &W) -> SecretResult<Vec<W>>{
    debug_assert!(W::static_type() == SecretItem::static_type() || W::static_type() == SecretCollection::static_type(), "Can only lock items or collections");
    let mut err = ptr::null_mut();
    let mut res = ptr::null_mut();
    let arr = [obj];
    let slice = (&arr[..]).to_glib_none();
    unsafe {
        ffi::secret_service_lock_sync(
            ptr::null_mut(),
            slice.0 as *mut GList,
            ptr::null_mut(),
            &mut res,
            &mut err
            );
        if err.is_null() {
            Ok(Vec::from_glib_full(res))
        } else {
            Err(Error::wrap(err))
        }
    }
}

pub fn unlock_object<W: Wrapper>(obj: &W) -> SecretResult<Vec<W>>{
    debug_assert!(W::static_type() == SecretItem::static_type() || W::static_type() == SecretCollection::static_type(), "Can only unlock items or collections");
    let mut err = ptr::null_mut();
    let mut res = ptr::null_mut();
    let arr = [obj];
    let slice = (&arr[..]).to_glib_none();
    unsafe {
        ffi::secret_service_unlock_sync(
            ptr::null_mut(),
            slice.0 as *mut GList,
            ptr::null_mut(),
            &mut res,
            &mut err
            );
        if err.is_null() {
            Ok(Vec::from_glib_full(res))
        } else {
            Err(Error::wrap(err))
        }
    }
}
