use std::ptr;
//use glib_sys::GList;
//use glib::translate::FromGlibPtrContainer;
//use glib::translate::ToGlibContainerFromSlice;
use glib::object::ObjectType;
use glib::translate::{ToGlibPtr, GlibPtrDefault};
use glib::translate::from_glib_full;
use glib::types::StaticType;
use secret_item::SecretItem;
use secret_collection::SecretCollection;
use ffi;
use SecretResult;

//pub fn lock_object<'a, W: ObjectType + StaticType + GlibPtrDefault + ToGlibContainerFromSlice<'a, *mut <W as GlibPtrDefault>::GlibType>>(obj: &W) -> SecretResult<Vec<W>>{
//pub fn lock_object<'a, W: ObjectType + StaticType + GlibPtrDefault + ToGlibContainerFromSlice<'a, *mut GList>>(obj: &W) -> SecretResult<Vec<W>>{
pub fn lock_object<'a, W: ObjectType + StaticType + GlibPtrDefault + ToGlibPtr<'a, <W as GlibPtrDefault>::GlibType>>(obj: &'a W) -> SecretResult<Vec<W>>{
    debug_assert!(W::static_type() == SecretItem::static_type() || W::static_type() == SecretCollection::static_type(), "Can only lock items or collections");
    let mut err = ptr::null_mut();
    let mut res = ptr::null_mut();
    let _arr = [obj];
    //let arr = [obj];
    //let slice: (*mut GList, <&W as ToGlibContainerFromSlice<'a, *mut GList>>::Storage) = ToGlibContainerFromSlice::to_glib_none_from_slice(&arr[..]);
    unsafe {
        ffi::secret_service_lock_sync(
            ptr::null_mut(),
            //slice.0 as *mut GList,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut res,
            &mut err
            );
        //if err.is_null() {
        //    Ok(Vec::from_glib_full(res))
        //} else {
            Err(
                from_glib_full(err)
            )
        //}
    }
}

pub fn unlock_object<W: ObjectType + StaticType + GlibPtrDefault>(obj: &W) -> SecretResult<Vec<W>>{
    debug_assert!(W::static_type() == SecretItem::static_type() || W::static_type() == SecretCollection::static_type(), "Can only unlock items or collections");
    let mut err = ptr::null_mut();
    let mut res = ptr::null_mut();
    let _arr = [obj];
    //let arr = [obj];
    //let slice = (&arr[..]).to_glib_none();
    unsafe {
        ffi::secret_service_unlock_sync(
            ptr::null_mut(),
            //slice.0 as *mut GList,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut res,
            &mut err
            );
        //if err.is_null() {
        //    Ok(Vec::from_glib_full(res))
        //} else {
            Err(
                from_glib_full(err)
            )
        //}
    }
}
