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
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;

pub type CLEARCOLORPROC = extern "system" fn(f32, f32, f32, f32) -> ();
pub type CLEARPROC = extern "system" fn(u32) -> ();
pub type VIEWPORTPROC = extern "system" fn(i32, i32, i32, i32) -> ();
pub type CREATESHADERPROC = extern "system" fn(u32) -> u32;
pub type SHADERSOURCEPROC = extern "system" fn() -> ();

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
    pub fn new() -> Result<Self, u8> {
        let functions = [
            "glClear",
            "glClearColor",
            "glViewport"
        ];

        let mut func_ptrs: WinVec<win32::FUNCTION_PTR> = WinVec::new();
        let mut i = 10;
        let gl_context = unsafe { win32::wglGetCurrentContext() };
        if gl_context.is_null() {
            return Err(100);
        }
        for &func in &functions {
            let fn_ptr = get_gl_func_address(func);
            if fn_ptr.is_null() {
                return Err(i);
            }
            func_ptrs.push(fn_ptr);
            i += 1;
        }

        Ok(Self {
            ptrs: func_ptrs
        })
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        let function = self.ptrs[1] as *const();
        unsafe { core::mem::transmute::<*const(), CLEARCOLORPROC>(function) (r, g, b, a); }
    }

    pub fn clear(&self, mask: u32) {
        let function = self.ptrs[0] as *const();
        unsafe { core::mem::transmute::<*const(), CLEARPROC>(function) (mask); }
    }
}

/*
pub fn glClearColor(r: f32, g: f32, b: f32, a: f32) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glClearColor") }
    });
    unsafe { core::mem::transmute::<_, CLEARCOLORPROC>(FUNC_PTR) (r, g, b, a); }
}

pub fn glClear(mask: u32) {
    static mut FUNC_CLEAR_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_CLEAR_PTR = get_gl_func_address("glClear") }
    });
    unsafe { core::mem::transmute::<_, CLEARPROC>(FUNC_CLEAR_PTR) (mask); }
}

pub fn glViewport(x: i32, y: i32, width: i32, height: i32) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glViewport") }
    });
    unsafe { core::mem::transmute::<_, VIEWPORTPROC>(FUNC_PTR) (x, y, width, height); }
}*/

pub fn glCreateShader(shader_type: u32) -> u32 {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glCreateShader") }
    });
    unsafe { core::mem::transmute::<_, CREATESHADERPROC>(FUNC_PTR) (shader_type) }
}

#[link(name = "Opengl32")]
extern "stdcall" {
    pub fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn glClear(mask: u32);
    pub fn glViewport(x: i32, y: i32, width: i32, height: i32);
}