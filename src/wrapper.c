// Rust won't let us easily zero init a struct, so that's all this function does
#include "../JPEGDEC/src/JPEGDEC.h"
JPEGIMAGE JPEG_ZeroInitJPEGIMAGE(){
    JPEGIMAGE j = {0};
    return j;
}
