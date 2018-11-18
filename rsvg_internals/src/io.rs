use data_url;
use glib_sys;
use libc;

use glib::translate::*;
use std::ptr;

use error::{set_gerror, LoadingError};
use handle::BinaryData;
use util::utf8_cstr;

fn decode_data_uri(uri: &str) -> Result<BinaryData, LoadingError> {
    let data_url = data_url::DataUrl::process(uri).map_err(|_| LoadingError::BadDataUrl)?;

    let mime_type = data_url.mime_type().to_string();

    let (bytes, _fragment_id) = data_url
        .decode_to_vec()
        .map_err(|_| LoadingError::BadDataUrl)?;

    Ok(BinaryData {
        data: bytes,
        content_type: Some(mime_type),
    })
}

#[no_mangle]
pub fn rsvg_decode_data_uri(
    uri: *const libc::c_char,
    out_mime_type: *mut *mut libc::c_char,
    out_size: *mut usize,
    error: *mut *mut glib_sys::GError,
) -> *mut libc::c_char {
    unsafe {
        assert!(!out_mime_type.is_null());
        assert!(!out_size.is_null());

        let uri = utf8_cstr(uri);

        match decode_data_uri(uri) {
            Ok(binary_data) => {
                *out_mime_type = binary_data.content_type.to_glib_full();
                *out_size = binary_data.data.len();

                if !error.is_null() {
                    *error = ptr::null_mut();
                }

                ToGlibContainerFromSlice::to_glib_full_from_slice(&binary_data.data)
                    as *mut libc::c_char
            }

            Err(_) => {
                *out_mime_type = ptr::null_mut();
                *out_size = 0;

                set_gerror(error, 0, "could not decode data: URL");

                ptr::null_mut()
            }
        }
    }
}