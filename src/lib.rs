#![no_std]
use core::ptr;
use win32::*;
use tinygl::*;

pub struct Window {
    pub width: usize,
    pub height: usize,
    pub dc: HDC,
    pub hwnd: HWND,
    pub hglrc: HGLRC,
}

impl Window {
    pub fn new_fake(width: usize, height: usize) -> Self {
        let class = WNDCLASSW::new();
        let window_name = &[b'N' as u16, b'i' as u16, b't' as u16, b'r' as u16, b'o' as u16, 0 as u16];
        let flags: DWORD = 0x00000000 | 0x00C00000 | 0x00080000 | 0x00040000 | 0x00020000 | 0x00010000;
        let dc: HDC;
        let handle: HWND;
        let hglrc: HGLRC;
        unsafe {
            RegisterClassW(&class);
            handle = CreateWindowExW(0, class.lpszClassName, window_name.as_ptr(), flags, 100, 100, width as i32, height as i32, ptr::null_mut(), ptr::null_mut(), class.hInstance, ptr::null_mut());

            //Get GL context
            dc = GetDC(handle);
            let pfd = PIXELFORMATDESCRIPTOR::new();
            let pixel_format = ChoosePixelFormat(dc, &pfd);
            SetPixelFormat(dc, pixel_format, &pfd);

            hglrc = wglCreateContext(dc);
            wglMakeCurrent(dc, hglrc);

            //create real window
            // let attrib_list: [i32; 15] = [
            //     0x2001, 1,
            //     0x2010, 1,
            //     0x2011, 1,
            //     0x2013, 0x202B,
            //     0x2014, 32,
            //     0x2022, 24,
            //     0x2023, 8,
            //     0
            // ];
            // let mut pixelFormat: i32 = 0;
            // let mut numFormats: u32 = 0;
            // let status = wglChoosePixelFormatARB(dc, attrib_list.as_ptr(), ptr::null(), 1, &mut pixelFormat, &mut numFormats);

            //ShowWindow(handle, 1);
        }
        let window = Window {
            width: width,
            height: height,
            dc: dc,
            hwnd: handle,
            hglrc: hglrc
        };

        window
    }

    pub fn new_real(width: usize, height: usize) -> Self {
        let class = WNDCLASSW::new();
        let window_name = &[b'N' as u16, b'i' as u16, b't' as u16, b'r' as u16, b'o' as u16, 0 as u16];
        let flags: DWORD = 0x00000000 | 0x00C00000 | 0x00080000 | 0x00040000 | 0x00020000 | 0x00010000;
        let dc: HDC;
        let handle: HWND;
        let hglrc: HGLRC;
        unsafe {
            RegisterClassW(&class);
            handle = CreateWindowExW(0, class.lpszClassName, window_name.as_ptr(), flags, 100, 100, width as i32, height as i32, ptr::null_mut(), ptr::null_mut(), class.hInstance, ptr::null_mut());

            //Get GL context
            dc = GetDC(handle);
            let attrib_list: [i32; 15] = [
                0x2001, 1,
                0x2010, 1,
                0x2011, 1,
                0x2013, 0x202B,
                0x2014, 32,
                0x2022, 24,
                0x2023, 8,
                0
            ];
            let mut pixelFormat: i32 = 0;
            let mut numFormats: u32 = 0;
            let status = wglChoosePixelFormatARB(dc, attrib_list.as_ptr(), ptr::null(), 1, &mut pixelFormat, &mut numFormats);
            let mut pfd = PIXELFORMATDESCRIPTOR::new();
            DescribePixelFormat(dc, pixelFormat, core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32, &mut pfd);
            let status2 = SetPixelFormat(dc, pixelFormat, &pfd);
            let gl_version_major: i32 = 4;
            let gl_version_minor: i32 = 5;
            let context_attribs = [
                0x2091, gl_version_major,
                0x2092, gl_version_minor,
                0x9126, 0x00000001,
                0
            ];
            hglrc = wglCreateContextAttribsARB(dc, ptr::null_mut(), context_attribs.as_ptr());
            //ShowWindow(handle, 1);
        }

        Window {
            width: width,
            height: height,
            dc: dc,
            hwnd: handle,
            hglrc: hglrc
        }
    }

    pub fn make_current(&self) {
        unsafe {
            wglMakeCurrent(self.dc, self.hglrc);
        }
    }

    pub fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, 1);
        }
    }

    pub fn destroy(&self) {
        unsafe {
            wglMakeCurrent(ptr::null_mut(), ptr::null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hwnd, self.dc);
            DestroyWindow(self.hwnd);
        }
    }

    pub fn message_loop(&self) -> bool {
        unsafe {
            let mut msg = MSG::new();
            let mut quit = false;
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, 1) != 0 {
                match WM::from(msg.message) {
                    WM::QUIT => quit = true,
                    _ => {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            }
            quit
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            wglMakeCurrent(ptr::null_mut(), ptr::null_mut());
            wglDeleteContext(self.hglrc);
            ReleaseDC(self.hwnd, self.dc);
            DestroyWindow(self.hwnd);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_gl_context() {
        
    }
}
