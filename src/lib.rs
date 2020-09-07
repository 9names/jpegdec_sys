#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode() {
        // Need a callback function for JPEG_openRAM
        extern "C" fn callback(_pdraw: *mut JPEGDRAW) {
            // This is a stub, it does nothing
        }

        // Need a bit of casting to pass this to C
        let mut image = unsafe { JPEG_ZeroInitJPEGIMAGE() };
        let imgptr: *mut JPEGIMAGE = &mut image as *mut JPEGIMAGE;

        let tulips = include_bytes!("tulips.jpg");

        // Must wrap our callback function
        let c: JPEG_DRAW_CALLBACK = Some(callback);
        unsafe {
            let opened = JPEG_openRAM(imgptr, tulips.as_ptr(), tulips.len() as i32, c);
            assert!(
                opened != 0,
                "Could not open file, {}",
                JPEG_getLastError(imgptr)
            );
            if opened != 0 {
                let rc = JPEG_decode(imgptr, 0, 0, 0);
                let errstr = JPEG_getLastError(imgptr);
                JPEG_close(imgptr);
                assert!(rc == 1, "Could not decode file, {}", errstr);
            }
        }
    }
}
