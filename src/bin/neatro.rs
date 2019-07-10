#![allow(unused_variables, unused_imports)]

#![feature(core_intrinsics, lang_items, link_args, alloc_error_handler)]
#![no_std]
#![no_main]


use core::intrinsics;
use core::panic::PanicInfo;
use core::ptr;
use win32::{Window, ffi_message_box, SwapBuffers, exit_process};
use tinygl::*;

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

#[no_mangle]
pub extern "C" fn WinMain() -> () {
    // ffi_message_box();
    let win = Window::new(800, 600);
    // let mut func_ptrs: simplealloc::WinVec<win32::FUNCTION_PTR> = simplealloc::WinVec::new();
    while !win.message_loop() {
        
        unsafe { SwapBuffers(win.dc) };
    }

    exit_process(0);
}

// Resolves a linker error when floating points are used
#[no_mangle]
pub static _fltused: i32 = 1;