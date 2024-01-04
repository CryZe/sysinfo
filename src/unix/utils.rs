// Take a look at the license at the top of the repository in the LICENSE file.

use std::{
    ffi::{CStr, OsStr},
    os::unix::ffi::OsStrExt,
};

use libc::c_char;

pub(crate) unsafe fn cstr_to_utf8_str<'a>(c: *const c_char) -> Option<&'a str> {
    cstr_to_os_str_with_size(c, None)?.to_str()
}

pub(crate) unsafe fn cstr_to_os_str_with_size<'a>(
    c: *const c_char,
    size: Option<usize>,
) -> Option<&'a OsStr> {
    if c.is_null() {
        return None;
    }
    let mut bytes = CStr::from_ptr(c).to_bytes();
    if let Some(size) = size {
        bytes = &bytes[..size.min(bytes.len())];
    }
    Some(OsStr::from_bytes(bytes))
}
