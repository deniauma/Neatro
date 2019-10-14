#![no_std]
use win32::{wglGetProcAddress, GetProcAddress, LoadLibraryW, HDC, HGLRC};
use simplealloc::{CString, Once};

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
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STATIC_DRAW: u32 = 0x88E4;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_FALSE: u8 = 0;
pub const GL_VERSION: u32 = 0x1F02;

pub type GLuint = u32;
pub type GLsizei = i32;
pub type GLchar = i8;
pub type GLint = i32;
pub type GLenum = u32;
pub type GLsizeiptr = isize;
pub type GLvoid = u8;
pub type GLboolean = u8;

pub type GETSTRINGPROC = extern "system" fn(GLenum) -> *const u8;
pub type CLEARCOLORPROC = extern "system" fn(f32, f32, f32, f32) -> ();
pub type CLEARPROC = extern "system" fn(u32) -> ();
pub type VIEWPORTPROC = extern "system" fn(i32, i32, i32, i32) -> ();
pub type CREATESHADERPROC = extern "system" fn(u32) -> u32;
pub type SHADERSOURCEPROC = extern "system" fn(u32, i32, *const *const GLchar, *const GLint) -> ();
pub type COMPILESHADERPROC = extern "system" fn(u32) -> ();
pub type ATTACHSHADERPROC = extern "system" fn(GLuint, GLuint) -> ();
pub type CREATEPROGRAMPROC = extern "system" fn() -> u32;
pub type LINKPROGRAMPROC = extern "system" fn(GLuint) -> ();
pub type DELETESHADERPROC = extern "system" fn(GLuint) -> ();
pub type GENVERTEXARRAYSPROC = extern "system" fn(GLsizei, *mut GLuint) -> ();
pub type GENBUFFERSPROC = extern "system" fn(GLsizei, *mut GLuint) -> ();
pub type BINDVERTEXARRAYPROC = extern "system" fn(GLuint) -> ();
pub type BINDBUFFERPROC = extern "system" fn(GLenum, GLuint) -> ();
pub type BUFFERDATAPROC = extern "system" fn(GLenum, GLsizeiptr, *const GLvoid, GLenum) -> ();
pub type VERTEXATTRIBPOINTERPROC = extern "system" fn(GLuint, GLint, GLenum, GLboolean, GLsizei, *const GLvoid) -> ();
pub type ENABLEVERTEXATTRIBARRAYPROC = extern "system" fn(GLuint) -> ();
pub type USEPROGRAMPROC = extern "system" fn(GLuint) -> ();
pub type DELETEVERTEXARRAYSPROC = extern "system" fn(GLsizei, *const GLuint) -> ();
pub type DELETEBUFFERSPROC = extern "system" fn(GLsizei, *const GLuint) -> ();
pub type WGLCHOOSEPIXELFORMATARBPROC = extern "system" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> bool;
pub type WGLCREATECONTEXTATTRIBSARBPROC = extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC;

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

#[allow(non_snake_case)]
pub fn glGetString(name: GLenum) -> *const u8 {
    // static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    // static ONCE: Once = Once::INIT;
    // ONCE.run_once(|| {
    //     unsafe { FUNC_PTR = get_gl_func_address("glGetString") }
    // });
    // unsafe { core::mem::transmute::<_, GETSTRINGPROC>(FUNC_PTR) (name) }
    unsafe {
        let func_ptr = get_gl_func_address("glGetString");
        core::mem::transmute::<_, GETSTRINGPROC>(func_ptr) (name)
    }
}

#[allow(non_snake_case)]
pub fn glCreateShader(shader_type: u32) -> u32 {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glCreateShader") }
    });
    unsafe { core::mem::transmute::<_, CREATESHADERPROC>(FUNC_PTR) (shader_type) }
}

#[allow(non_snake_case)]
pub fn glShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glShaderSource") }
    });
    unsafe { core::mem::transmute::<_, SHADERSOURCEPROC>(FUNC_PTR) (shader, count, string, length); }
}

#[allow(non_snake_case)]
pub fn glCompileShader(shader: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glCompileShader") }
    });
    unsafe { core::mem::transmute::<_, COMPILESHADERPROC>(FUNC_PTR) (shader); }
}

#[allow(non_snake_case)]
pub fn glAttachShader(program: GLuint, shader: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glAttachShader") }
    });
    unsafe { core::mem::transmute::<_, ATTACHSHADERPROC>(FUNC_PTR) (program, shader); }
}

#[allow(non_snake_case)]
pub fn glCreateProgram() -> u32 {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glAttachShader") }
    });
    unsafe { core::mem::transmute::<_, CREATEPROGRAMPROC>(FUNC_PTR) () }
}

#[allow(non_snake_case)]
pub fn glLinkProgram(program: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glLinkProgram") }
    });
    unsafe { core::mem::transmute::<_, LINKPROGRAMPROC>(FUNC_PTR) (program); }
}

#[allow(non_snake_case)]
pub fn glDeleteShader(shader: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glLinkProgram") }
    });
    unsafe { core::mem::transmute::<_, DELETESHADERPROC>(FUNC_PTR) (shader); }
}

#[allow(non_snake_case)]
pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glGenVertexArrays") }
    });
    unsafe { core::mem::transmute::<_, GENVERTEXARRAYSPROC>(FUNC_PTR) (n, arrays); }
}

#[allow(non_snake_case)]
pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glGenBuffers") }
    });
    unsafe { core::mem::transmute::<_, GENBUFFERSPROC>(FUNC_PTR) (n, buffers); }
}

#[allow(non_snake_case)]
pub fn glBindBuffer(target: GLenum, buffer: GLuint) -> () {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glBindBuffer") }
    });
    unsafe { core::mem::transmute::<_, BINDBUFFERPROC>(FUNC_PTR) (target, buffer); }
}

#[allow(non_snake_case)]
pub fn glBindVertexArray(array: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glBindVertexArray") }
    });
    unsafe { core::mem::transmute::<_, BINDVERTEXARRAYPROC>(FUNC_PTR) (array); }
}

#[allow(non_snake_case)]
pub fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glBufferData") }
    });
    unsafe { core::mem::transmute::<_, BUFFERDATAPROC>(FUNC_PTR) (target, size, data, usage); }
}

#[allow(non_snake_case)]
pub fn glVertexAttribPointer(index: GLuint, size: GLint, kind: GLenum, normalized: GLboolean, stride: GLsizei, pointer: *const GLvoid) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glVertexAttribPointer") }
    });
    unsafe { core::mem::transmute::<_, VERTEXATTRIBPOINTERPROC>(FUNC_PTR) (index, size, kind, normalized, stride, pointer); }
}

#[allow(non_snake_case)]
pub fn glEnableVertexAttribArray(index: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glEnableVertexAttribArray") }
    });
    unsafe { core::mem::transmute::<_, ENABLEVERTEXATTRIBARRAYPROC>(FUNC_PTR) (index); }
}

#[allow(non_snake_case)]
pub fn glUseProgram(program: GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glUseProgram") }
    });
    unsafe { core::mem::transmute::<_, USEPROGRAMPROC>(FUNC_PTR) (program); }
}

#[allow(non_snake_case)]
pub fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glDeleteVertexArrays") }
    });
    unsafe { core::mem::transmute::<_, DELETEVERTEXARRAYSPROC>(FUNC_PTR) (n, arrays); }
}

#[allow(non_snake_case)]
pub fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glDeleteBuffers") }
    });
    unsafe { core::mem::transmute::<_, DELETEBUFFERSPROC>(FUNC_PTR) (n, buffers); }
}

#[allow(non_snake_case)]
pub fn glClear2(mask: u32) {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("glClear") }
    });
    unsafe { core::mem::transmute::<_, CLEARPROC>(FUNC_PTR) (mask); }
}

#[allow(non_snake_case)]
pub fn wglChoosePixelFormatARB(hdc: HDC, piAttribIList: *const i32, pfAttribFList: *const f32, nMaxFormats: u32, piFormats: *mut i32, nNumFormats: *mut u32) -> bool {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("wglChoosePixelFormatARB") }
    });
    unsafe { core::mem::transmute::<_, WGLCHOOSEPIXELFORMATARBPROC>(FUNC_PTR) (hdc, piAttribIList, pfAttribFList, nMaxFormats, piFormats, nNumFormats) }
}

#[allow(non_snake_case)]
pub fn wglCreateContextAttribsARB(hdc: HDC, hglrc: HGLRC, attribList: *const i32) -> HGLRC {
    static mut FUNC_PTR: win32::FUNCTION_PTR = core::ptr::null_mut();
    static ONCE: Once = Once::INIT;
    ONCE.run_once(|| {
        unsafe { FUNC_PTR = get_gl_func_address("wglCreateContextAttribsARB") }
    });
    unsafe { core::mem::transmute::<_, WGLCREATECONTEXTATTRIBSARBPROC>(FUNC_PTR) (hdc, hglrc, attribList) }
}

#[link(name = "Opengl32")]
extern "stdcall" {
    pub fn glClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn glClear(mask: u32);
    pub fn glViewport(x: i32, y: i32, width: i32, height: i32);
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
}