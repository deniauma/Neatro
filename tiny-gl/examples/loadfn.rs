use tinygl::*;
use win32::{Window, SwapBuffers};

fn main() {
    println!("Opening the window ...");
    let win = Window::new(800, 600);
    // match  tinygl::GlLib::new() {
    //     Ok(gl_lib) => {
    //         println!("Loading successful!");
    //         gl_lib.clear_color(0.2, 0.3, 0.3, 1.0);
    //         // glClear(GL_COLOR_BUFFER_BIT);
    //         while !win.message_loop() {
    //             gl_lib.clear(GL_COLOR_BUFFER_BIT);
    //             unsafe { SwapBuffers(win.dc) };
    //         }
    //     },
    //     Err(e) => println!("Error: {}", e)
    // }
    // let addr = get_gl_func_address("glClear");
    // println!("{:?}", addr);
    unsafe {glViewport(0, 0, 800, 600);
    glClearColor(0.2, 0.3, 0.3, 1.0);
    glClear(GL_COLOR_BUFFER_BIT);
    while !win.message_loop() {
        glClear(GL_COLOR_BUFFER_BIT);
        unsafe { SwapBuffers(win.dc) };
    }}
}