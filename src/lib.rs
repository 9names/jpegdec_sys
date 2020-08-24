#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" fn callback(_pdraw: *mut JPEGDRAW) {
    
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn decode() {
        // Need to let Rust trust us with some initialized memory to store our C data
        // We can do that by getting C to zero-init a struct and return it
        let image = Box(JPEG_ZeroInitJPEGIMAGE());
        let imgptr: *mut JPEGIMAGE = Box::into_raw(image);
        
        // include_bytes gives us an immutable slice, copy that into a mutable one
        let tulips_const = include_bytes!("tulips.jpg");
        let tulips: &mut [u8] = &mut tulips_const.clone();
        let tulipsmut = tulips.as_mut_ptr();

        // Must wrap our callback function
        let c: JPEG_DRAW_CALLBACK = Some(callback);
        unsafe {
            let opened = JPEG_openRAM(imgptr, tulipsmut, tulips.len() as i32, c);
            assert!(opened != 0, "Could not open file, {}", JPEG_getLastError(imgptr));
            if opened != 0 {
                let rc = JPEG_decode(imgptr, 0, 0, 0);
                let errstr = JPEG_getLastError(imgptr);
                JPEG_close(imgptr);
                assert!(rc == 1, "Could not decode file, {}", errstr);
            }     
        }
    }
}
