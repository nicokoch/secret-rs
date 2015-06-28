#![allow(non_camel_case_types)]
extern crate libc;
extern crate glib;

use libc::{c_uint, c_char, c_int, c_ulong, c_long};
use glib::ffi::{GError, GType, GList, gboolean, gsize, gpointer, GHashTable};

//glib stuff not provided by glib-rs
pub type gchar = c_char;
pub type gint = c_int;
pub type guint = c_uint;
pub type guint64 = c_ulong;
pub type gssize = c_long;
#[repr(C)] pub struct GCancellable;

//libsecret

#[repr(C)] pub struct SecretServiceFFI;
#[repr(C)] pub struct SecretSchemaFFI;
#[repr(C)] pub struct SecretCollectionFFI;
#[repr(C)] pub struct SecretItemFFI;
#[repr(C)] pub struct SecretValueFFI;

#[link(name="secret-1")]
extern "C" {
    //=========================================================================
    // SecretService
    //=========================================================================
    pub fn secret_service_get_sync              (flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut SecretServiceFFI;
    pub fn secret_service_disconnect            ();
    pub fn secret_service_open_sync             (service_gtype: GType, service_bus_name: *const gchar, flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut SecretServiceFFI;
    pub fn secret_service_get_collections       (secret_service: *mut SecretServiceFFI) -> *mut GList;
    pub fn secret_service_get_flags             (secret_service: *mut SecretServiceFFI) -> c_int;
    pub fn secret_service_get_session_algorithms(secret_service: *mut SecretServiceFFI) -> *const gchar;
    pub fn secret_service_ensure_session_sync   (secret_service: *mut SecretServiceFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_service_load_collections_sync (secret_service: *mut SecretServiceFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_service_search_sync           (secret_service: *mut SecretServiceFFI, secret_schema: *const SecretSchemaFFI, attributes: *mut GHashTable, flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut GList;
    pub fn secret_service_lock_sync             (secret_service: *mut SecretServiceFFI, objects: *mut GList, cancellable: *mut GCancellable, locked: *mut *mut GList, error: *mut *mut GError);
    pub fn secret_service_unlock_sync           (secret_service: *mut SecretServiceFFI, objects: *mut GList, cancellable: *mut GCancellable, unlocked: *mut *mut GList, error: *mut *mut GError) -> gint;
    pub fn secret_service_store_sync            (secret_service: *mut SecretServiceFFI, schema: *const SecretSchemaFFI, attributes: *mut GHashTable, collection: *const gchar, label: *const gchar, value: *mut SecretValueFFI, cancellable: *mut GCancellable, error: *mut *mut GError);
    pub fn secret_service_lookup_sync           (secret_service: *mut SecretServiceFFI, schema: *const SecretSchemaFFI, attributes: *mut GHashTable, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut SecretValueFFI;
    pub fn secret_service_clear_sync            (secret_service: *mut SecretServiceFFI, schema: *const SecretSchemaFFI, attributes: *mut GHashTable, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
//  pub fn secret_service_prompt_sync           ();
    pub fn secret_service_set_alias_sync        (secret_service: *mut SecretServiceFFI, alias: *const gchar, collection: *mut SecretCollectionFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_service_get_collection_gtype  (secret_service: *mut SecretServiceFFI) -> GType;
    pub fn secret_service_get_item_gtype        (secret_service: *mut SecretServiceFFI) -> GType;

    //=========================================================================
    // SecretCollection
    //=========================================================================
    pub fn secret_collection_for_alias_sync (secret_service: *mut SecretServiceFFI, alias: *const gchar, flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut SecretCollectionFFI;
    pub fn secret_collection_load_items_sync(secret_collection: *mut SecretCollectionFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_collection_create_sync    (secret_service: *mut SecretServiceFFI, label: *const gchar, alias: *const gchar, flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut SecretCollectionFFI;
    pub fn secret_collection_search_sync    (secret_collection: *mut SecretCollectionFFI, schema: *const SecretSchemaFFI, atrributes: *mut GHashTable, flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut GList;
    pub fn secret_collection_delete_sync    (secret_collection: *mut SecretCollectionFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_collection_get_created    (secret_collection: *mut SecretCollectionFFI) -> guint64;
    pub fn secret_collection_get_service    (secret_collection: *mut SecretCollectionFFI) -> *mut SecretServiceFFI;
    pub fn secret_collection_get_flags      (secret_collection: *mut SecretCollectionFFI) -> c_int;
    pub fn secret_collection_get_items      (secret_collection: *mut SecretCollectionFFI) -> *mut GList;
    pub fn secret_collection_get_label      (secret_collection: *mut SecretCollectionFFI) -> *mut gchar;
    pub fn secret_collection_set_label_sync (secret_collection: *mut SecretCollectionFFI, label: *const gchar, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_collection_get_locked     (secret_collection: *mut SecretCollectionFFI) -> gboolean;
    pub fn secret_collection_get_modified   (secret_collection: *mut SecretCollectionFFI) -> guint64;
    pub fn secret_collection_refresh        (secret_collection: *mut SecretCollectionFFI);

    //=========================================================================
    // SecretItem
    //=========================================================================
    pub fn secret_item_create_sync          (secret_collection: *mut SecretCollectionFFI, schema: *const SecretSchemaFFI, attributes: *mut GHashTable, label: *const gchar, value: *mut SecretValueFFI, flags: c_int, cancellable: *mut GCancellable, error: *mut *mut GError) -> *mut SecretItemFFI;
    pub fn secret_item_delete_sync          (secret_item: *mut SecretItemFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_item_get_schema_name      (secret_item: *mut SecretItemFFI) -> *mut gchar;
    pub fn secret_item_get_attributes       (secret_item: *mut SecretItemFFI) -> *mut GHashTable;
    pub fn secret_item_set_attributes_sync  (secret_item: *mut SecretItemFFI, schema: *const SecretSchemaFFI, attributes: *mut GHashTable, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_item_get_created          (secret_item: *mut SecretItemFFI) -> guint64;
    pub fn secret_item_get_label            (secret_item: *mut SecretItemFFI) -> *mut gchar;
    pub fn secret_item_set_label_sync       (secret_item: *mut SecretItemFFI, label: *const gchar, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_item_get_flags            (secret_item: *mut SecretItemFFI) -> c_int;
    pub fn secret_item_get_locked           (secret_item: *mut SecretItemFFI) -> gboolean;
    pub fn secret_item_get_modified         (secret_item: *mut SecretItemFFI) -> guint64;
    pub fn secret_item_get_service          (secret_item: *mut SecretItemFFI) -> *mut SecretServiceFFI;
    pub fn secret_item_get_secret           (secret_item: *mut SecretItemFFI) -> *mut SecretValueFFI;
    pub fn secret_item_load_secret_sync     (secret_item: *mut SecretItemFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_item_load_secrets_sync    (items: *mut GList, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_item_set_secret_sync      (secret_item: *mut SecretItemFFI, value: *mut SecretValueFFI, cancellable: *mut GCancellable, error: *mut *mut GError) -> gboolean;
    pub fn secret_item_refresh              (secret_item: *mut SecretItemFFI);


    //=========================================================================
    // SecretValue
    //=========================================================================
    pub fn secret_value_new             (secret: *const gchar, length: gssize, content_type: *const gchar) -> *mut SecretValueFFI;
//  pub fn secret_value_new_full        ()
    pub fn secret_value_get             (secret_value: *mut SecretValueFFI, length: *mut gsize) -> *const gchar;
    pub fn secret_value_get_text        (secret_value: *mut SecretValueFFI) -> *const gchar;
    pub fn secret_value_get_content_type(secret_value: *mut SecretValueFFI) -> *const gchar;
    pub fn secret_value_ref             (secret_value: *mut SecretValueFFI) -> *mut SecretValueFFI;
    pub fn secret_value_unref           (value: gpointer);
}
