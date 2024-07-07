// https://www.sqlite.org/loadext.html
// https://github.com/jgallagher/rusqlite/issues/524#issuecomment-507787350

use std::os::raw::{c_char, c_int};
use rusqlite::{ffi, Connection};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub unsafe extern "C" fn sqlite3_chia_init(
    db: *mut ffi::sqlite3,
    _pz_err_msg: *mut *mut c_char,
    p_api: *mut ffi::sqlite3_api_routines,
) -> c_int {
    if p_api.is_null() {
        return ffi::SQLITE_ERROR;
    }
    if let Err(_err) = extension_init(db, p_api) {
        return ffi::SQLITE_ERROR;
    }
    ffi::SQLITE_OK
}

fn extension_init(db: *mut ffi::sqlite3, p_api: *mut ffi::sqlite3_api_routines) -> anyhow::Result<()> {
    let db = unsafe { Connection::extension_init2(db, p_api)? };
    crate::setup(&db)?;
    Ok(())
}
