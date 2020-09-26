#define __LINUX__
#include "../JPEGDEC/src/JPEGDEC.h"
// Rust won't let us easily zero init a struct, so that's all this function does
JPEGIMAGE JPEG_ZeroInitJPEGIMAGE();