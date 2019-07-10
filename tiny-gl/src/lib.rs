#![no_std]
use win32::{wglGetProcAddress, GetProcAddress, LoadLibraryW};
use simplealloc::{WinVec, CString, Once};

/* GL functions to import:
glClearColor
glClear
glViewport
 */
pub const GL_COLOR_BUFFER_BIT: u32 = 0x00004000;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x00000100;
pub const GL_STENCIL_BUFFER_BIT: u32 = 0x00000400;

pub type CLEARCOLORPROC = extern "system" fn(f32, f32, f32, f32) -> ();
pub type CLEARPROC = extern "system" fn(u32) -> ();
pub type VIEWPORTPROC = extern "system" fn(i32, i32, i32, i32) -> ();

pub fn get_gl_func_address(func_name: &str) -> win32::FUNCTION_PTR {
    // let name = &[b'g' as i8, b'l' as i8, b'C' as i8, b'l' as i8, b'e' as i8, b'a' as i8, b'r' as i8, 0 as i8];
    let name = CString::from_str(func_name).to_i8_str();
    let mut p;
    unsafe {
        p = wglGetProcAddress(name.as_ptr());
        if p.is_null() || p == 1 as win32::FUNCTION_PTR || p == 2 as win32::FUNCTION_PTR || p == 3 as win32::FUNCTION_PTR {
            // let dll = &[b'o' as u16, b'p' as u16, b'e' as u16, b'n' as u16, b'g' as u16, b'l' as u16, b'3' as u16, b'2' as u16, b'.' as u16, b'd' as u16, b'l' as u16, b'l' as u16, 0 as u16]; //"opengl32.dll"
            let dll = CString::from_str("opengl32.dll").to_u16_str();
            let module = LoadLibraryW(dll.as_ptr());
            p = GetProcAddress(module, name.as_ptr());
        }
    }
    p
}

pub struct GlLib {
    pub ptrs: WinVec<win32::FUNCTION_PTR>,
}

impl GlLib {
    pub fn new() -> Self {
        let functions = [
            "glClear",
            "glClearColor",
            "glViewport"
        ];

        let mut func_ptrs: WinVec<win32::FUNCTION_PTR> = WinVec::new();
        for &func in &functions {
            func_ptrs.push(get_gl_func_address(func));
        }

        Self {
            ptrs: func_ptrs
        }
    }
}


pub fn glClearColor(r: f32, g: f32, b: f32, a: f32) {
    unsafe { core::mem::transmute::<_, CLEARCOLORPROC>(get_gl_func_address("glClearColor")) (r, g, b, a); }
}

pub fn glClear(mask: u32) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glClear") }
    });
    unsafe { core::mem::transmute::<_, CLEARPROC>(FUNC_PTR) (mask); }
}

pub fn glViewport(x: i32, y: i32, width: i32, height: i32) {
    unsafe { core::mem::transmute::<_, VIEWPORTPROC>(get_gl_func_address("glViewport")) (x, y, width, height); }
}