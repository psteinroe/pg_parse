use pg_parse::parse;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

/// Returns 1 if the SQL is valid, 0 if invalid
#[no_mangle]
pub extern "C" fn pg_parse_is_valid_sql(query: *const c_char) -> c_int {
    if query.is_null() {
        return 0;
    }

    let c_str = unsafe { CStr::from_ptr(query) };
    let query_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    match parse(query_str) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}



