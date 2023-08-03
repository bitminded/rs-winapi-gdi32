extern crate winapi_typedefs;

use std::os::raw::*;
use winapi_typedefs::windef::*;
use winapi_typedefs::wingdi::*;

#[allow(non_snake_case)]
#[link(name = "gdi32")]
extern "system"
{
    fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
    fn SetPixelFormat(hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;
    fn SwapBuffers(hdc: HDC) -> BOOL;
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/dd318284(v=vs.85).aspx
pub fn choose_pixel_format(h_dc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int
{
    unsafe
    {
        ChoosePixelFormat(h_dc, ppfd)
    }
}

/// https://msdn.microsoft.com/de-de/library/windows/desktop/dd369049(v=vs.85).aspx
pub fn set_pixel_format(
    h_dc: HDC,
    i_pixel_format: c_int,
    ppfd: *const PIXELFORMATDESCRIPTOR) -> bool
{
    unsafe
    {
        if SetPixelFormat(h_dc, i_pixel_format, ppfd) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}

/// https://docs.microsoft.com/en-us/windows/desktop/api/wingdi/nf-wingdi-swapbuffers
pub fn swap_buffers(hdc: HDC) -> bool
{
    unsafe
    {
        if SwapBuffers(hdc) == TRUE
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}
