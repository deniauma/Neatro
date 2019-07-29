use tinygl::*;
use win32::{Window, SwapBuffers, wglGetProcAddress, GetProcAddress, LoadLibraryW};
use simplealloc::{WinVec, CString, Once};

pub fn get_gl_func(func_name: &str) -> win32::FUNCTION_PTR {
    // let name = &[b'g' as i8, b'l' as i8, b'C' as i8, b'l' as i8, b'e' as i8, b'a' as i8, b'r' as i8, 0 as i8];
    let name = CString::from_str(func_name).to_i8_str();
    let mut p;
    unsafe {
        // let dll = &[b'o' as u16, b'p' as u16, b'e' as u16, b'n' as u16, b'g' as u16, b'l' as u16, b'3' as u16, b'2' as u16, b'.' as u16, b'd' as u16, b'l' as u16, b'l' as u16, 0 as u16]; //"opengl32.dll"
        let dll = CString::from_str("opengl32.dll").to_u16_str();
        let module = LoadLibraryW(dll.as_ptr());
        p = GetProcAddress(module, name.as_ptr());
    }
    p
}

fn main() {
    println!("Opening the window ...");
    let win = Window::new(800, 600);
    let functions = [
            "glClear",
            "glClearColor",
            "glViewport"
        ];
    println!("GL context: {:?}", unsafe { win32::wglGetCurrentContext() });
    for &func in &functions {
        println!("{}: {:?}", func, get_gl_func(func));
    }
}