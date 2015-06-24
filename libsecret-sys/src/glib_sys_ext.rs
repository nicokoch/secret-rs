//Provides bindings which are not included in extern crate glib-sys (and will not be merged by project maintainers)
#![allow(non_camel_case_types)]
use libc::{c_uint, c_char, c_int, c_ulong, c_long};
use glib_sys::{gboolean, gconstpointer, gpointer, GList};

pub type gchar = c_char;
pub type gint = c_int;
pub type guint = c_uint;
pub type guint64 = c_ulong;
pub type gssize = c_long;

pub type GHashFunc = unsafe extern "C" fn(v: gconstpointer) -> c_uint;
pub type GEqualFunc = unsafe extern "C" fn(v1: gconstpointer, v2: gconstpointer) -> gboolean;

#[repr(C)] pub struct GHashTable;
#[repr(C)] pub struct GCancellable;

#[link(name="glib-2.0")]
extern "C" {
    //=========================================================================
    // GHashTable
    //=========================================================================
    pub fn g_hash_table_new         (hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
//  pub fn g_hash_table_new_full    (hash_func: GHashFunc, key_equal_func: GEqualFunc, .. ) -> *mut GHashTable;
    pub fn g_hash_table_insert      (hash_table: *mut GHashTable, key: gpointer, value: gpointer) -> gboolean;
    pub fn g_hash_table_replace     (hash_table: *mut GHashTable, key: gpointer, value: gpointer) -> gboolean;
    pub fn g_hash_table_add         (hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    pub fn g_hash_table_contains    (hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    pub fn g_hash_table_size        (hash_table: *mut GHashTable) -> c_uint;
    pub fn g_hash_table_lookup      (hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    pub fn g_hash_table_lookup_extended (hash_table: *mut GHashTable, lookup_key: gconstpointer, orig_key: gpointer, value: gpointer) -> gboolean;
//  pub fn g_hash_table_foreach     ();
//  pub fn g_hash_table_find        ();
    pub fn g_hash_table_remove      (hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    pub fn g_hash_table_steal       (hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
//  pub fn g_hash_table_foreach_remove() -> c_uint;
//  pub fn g_hash_table_foreach_steal() -> c_uint;
    pub fn g_hash_table_remove_all  (hash_table: *mut GHashTable);
    pub fn g_hash_table_steal_all   (hash_table: *mut GHashTable);
    pub fn g_hash_table_get_keys    (hash_table: *mut GHashTable) -> *mut GList;
    pub fn g_hash_table_get_values  (hash_table: *mut GHashTable) -> *mut GList;
    pub fn g_hash_table_get_keys_as_array(hash_table: *mut GHashTable, length: *mut c_uint) -> gpointer;
    pub fn g_hash_table_destroy     (hash_table: *mut GHashTable);
    pub fn g_hash_table_ref         (hash_table: *mut GHashTable) -> *mut GHashTable;
    pub fn g_hash_table_unref       (hash_table: *mut GHashTable);
//  skipped g_hash_table_iter functions (TODO?)
    pub fn g_direct_equal           (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_direct_hash            (v: gconstpointer) -> c_uint;
    pub fn g_int_equal              (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_int_hash               (v: gconstpointer) -> c_uint;
    pub fn g_int64_equal            (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_int64_hash             (v: gconstpointer) -> c_uint;
    pub fn g_double_equal           (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_double_hash            (v: gconstpointer) -> c_uint;
    pub fn g_str_equal              (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_str_hash               (v: gconstpointer) -> c_uint;
}

#[test]
fn test_g_hash_table() {
    use std::ffi::{CString, CStr};
    use libc::{c_void, c_char};
    use glib_sys::GTRUE;
    unsafe{
        let ht = g_hash_table_new(g_str_hash, g_str_equal);
        assert!(g_hash_table_size(ht) == 0 as c_uint);
        let key = CString::new("hello").unwrap();
        let val = CString::new("world").unwrap();
        g_hash_table_insert(ht, key.as_ptr() as *mut c_void, val.as_ptr() as *mut c_void);
        let key = CString::new("foo").unwrap();
        let val = CString::new("bar").unwrap();
        g_hash_table_insert(ht, key.as_ptr() as *mut c_void, val.as_ptr() as *mut c_void);
        assert!(g_hash_table_size(ht) == 2 as c_uint);
        assert!(g_hash_table_contains(ht, key.as_ptr() as *const c_void) == GTRUE);
        let ret_val = g_hash_table_lookup(ht, key.as_ptr() as *const c_void);
        let ret_val_str = String::from_utf8_lossy(CStr::from_ptr(ret_val as *const c_char).to_bytes());
        assert!(ret_val_str == "bar");
        assert!(g_hash_table_remove(ht, key.as_ptr() as *const c_void) == GTRUE);
        assert!(g_hash_table_size(ht) == 1 as c_uint);
        g_hash_table_destroy(ht);
    }
}
