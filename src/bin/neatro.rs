#![allow(unused_variables, unused_imports)]

#![feature(core_intrinsics, lang_items, link_args, alloc_error_handler)]
#![no_std]
#![no_main]

#[link_args = "/NODEFAULTLIB /SUBSYSTEM:WINDOWS /SAFESEH:NO /DYNAMICBASE:NO /ENTRY:WinMainCRTStartup /LTCG vcruntime.lib"]
extern "C" {}

use core::intrinsics;
use core::panic::PanicInfo;
use core::ptr;
use win32::{ffi_message_box, SwapBuffers, exit_process};
use tinygl::*;
use simplealloc::{WinVec, CString};
use neatro::Window;

// These functions and traits are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"] extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}


#[no_mangle]
pub extern "C" fn WinMainCRTStartup() -> () {
	WinMain()
}

const vertex_shader_src: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos
    void main()
    {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }"#;

const fragment_shader_src: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
    "#;

#[no_mangle]
pub extern "C" fn WinMain() -> () {
    // ffi_message_box();
    //let win = Window::new(800, 600);
    let fake_window = Window::new_fake(800, 600);
    let real_window = Window::new_real(800, 600);
    fake_window.destroy();
    real_window.make_current();
    let vs = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vs, 1, &CString::from_str(vertex_shader_src).to_i8_str().as_ptr(), core::ptr::null());
    glCompileShader(vs);
    let fs = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fs, 1, &CString::from_str(fragment_shader_src).to_i8_str().as_ptr(), core::ptr::null());
    glCompileShader(fs);
    let program = glCreateProgram();
    glAttachShader(program, vs);
    glAttachShader(program, fs);
    glLinkProgram(program);
    let vertices: [f32;9] = [
        -0.5, -0.5, 0.0, // left  
        0.5, -0.5, 0.0, // right 
        0.0,  0.5, 0.0  // top  
    ];
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    glGenVertexArrays(1, &mut vao);
    glGenBuffers(1, &mut vbo);
    glBindVertexArray(vao);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, core::mem::size_of::<[f32;9]>() as isize, vertices.as_ptr() as *const u8, GL_STATIC_DRAW);

    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * core::mem::size_of::<f32>() as i32, ptr::null());
    glEnableVertexAttribArray(0);
    glBindBuffer(GL_ARRAY_BUFFER, 0); 
    glBindVertexArray(0);

    unsafe {glViewport(0, 0, 800, 600);
    glClearColor(0.2, 0.3, 0.3, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    while !real_window.message_loop() {
        glClear(GL_COLOR_BUFFER_BIT);

        glUseProgram(program);
        glBindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
        glDrawArrays(GL_TRIANGLES, 0, 3);

        SwapBuffers(real_window.dc);
    }}
    
    exit_process(0);
}

// Resolves a linker error when floating points are used
#[no_mangle]
pub static _fltused: i32 = 1;