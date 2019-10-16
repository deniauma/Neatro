#![no_std]
use core::ptr;

pub enum HWND_ {}
pub enum HANDLE_ {}
pub enum HMENU_ {}
pub enum HINSTANCE_ {}
pub enum LPVOID_ {}
pub enum HICON_ {}
pub enum HCURSOR_ {}
pub enum HBRUSH_ {}
pub enum HDC_ {}
pub enum HGLRC_ {}
pub enum FUNCTION_PTR_ {}
pub enum HMODULE_ {}

pub type DWORD = u32;
pub type LPCWSTR = *const u16;
pub type LPCSTR = *const i8;
pub type HWND = *mut HWND_;
pub type HANDLE = *mut HANDLE_;
pub type HMENU = *mut HMENU_;
pub type HINSTANCE = *mut HINSTANCE_;
pub type LPVOID = *mut LPVOID_;
pub type HICON = *mut HICON_;
pub type HCURSOR = *mut HCURSOR_;
pub type HBRUSH = *mut HBRUSH_;
pub type HDC = *mut HDC_;
pub type HGLRC = *mut HGLRC_;
pub type FUNCTION_PTR = *mut FUNCTION_PTR_;
pub type HMODULE = *mut HMODULE_;
pub type WNDPROC = unsafe extern "stdcall" fn(_: HWND, _: u32, _: usize, _: isize) -> isize;

pub const STD_INPUT_HANDLE: DWORD = -10i32 as u32;
pub const STD_OUTPUT_HANDLE: DWORD = -11i32 as u32;
pub const STD_ERROR_HANDLE: DWORD = -12i32 as u32;


#[repr(C)]
pub struct WNDCLASSW {
    pub style: u32,
    pub lpfnWndProc: Option<WNDPROC>,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

impl WNDCLASSW {
    pub fn new() -> Self {
        let class_name = &[b'n' as u16, b'i' as u16, b't' as u16, b'r' as u16, b'o' as u16, b'_' as u16, b'w' as u16, b'n' as u16, b'd' as u16, 0 as u16];
        Self {
            style : 0x0002 | 0x0001 | 0x0020,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: get_module_handle(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
        }
    }
}

#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32
}

#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: usize,
    pub lParam: isize,
    pub time: DWORD,
    pub pt: POINT,
}

impl MSG {
    pub fn new() -> Self {
        Self {
            hwnd: ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT {x: 0, y: 0}
        }
    }
}

pub enum WM {
    SIZE,
    QUIT,
    DESTROY,
    CLOSE,
    NULL
}

impl From<u32> for WM {
    fn from(msg: u32) -> Self {
        match msg {
            0x0002 => WM::DESTROY,
            0x0005 => WM::SIZE,
            0x0010 => WM::CLOSE,
            0x0012 => WM::QUIT,
            _ => WM::NULL
        }
    }
}

#[repr(C)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: DWORD,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: u8,
    pub bReserved: u8,
    pub dwLayerMask: DWORD,
    pub dwVisibleMask: DWORD,
    pub dwDamageMask: DWORD,
}

impl PIXELFORMATDESCRIPTOR {
    pub fn new() -> Self {
        Self {
            nSize: core::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: 0x00000004 | 0x00000020 | 0x00000001,//PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: 0,//PFD_TYPE_RGBA
            cColorBits: 32,
            cRedBits: 0,
            cRedShift: 0,
            cGreenBits: 0,
            cGreenShift: 0,
            cBlueBits: 0,
            cBlueShift: 0,
            cAlphaBits: 0,
            cAlphaShift: 0,
            cAccumBits: 0,
            cAccumRedBits: 0,
            cAccumGreenBits: 0,
            cAccumBlueBits: 0,
            cAccumAlphaBits: 0,
            cDepthBits: 24,
            cStencilBits: 8,
            cAuxBuffers: 0,
            iLayerType: 0,//PFD_MAIN_PLANE
            bReserved: 0,
            dwLayerMask: 0,
            dwVisibleMask: 0,
            dwDamageMask: 0,
        }
    }
}


#[link(name = "user32")]
extern "stdcall" {
    pub fn MessageBoxW(
        hWnd: HWND,
        lpText: *const u16,
        lpCaption: *const u16,
        uType: u32
    ) -> i32;

    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        X: i32,
        Y: i32,
        nWidth: i32,
        nHeight: i32,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID
    ) -> HWND;

    pub fn RegisterClassW(
        lpWndClass: *const WNDCLASSW
    ) -> u16;

    pub fn ShowWindow(
        hWnd: HWND,
        nCmdShow: i32
    ) -> i32;

    pub fn DefWindowProcW(
        window: HWND,
        msg: u32,
        wparam: usize,
        lparam: isize
    ) -> isize;

    pub fn PeekMessageW(
        lpMsg: *mut MSG, 
        hWnd: HWND, 
        wMsgFilterMin: u32, 
        wMsgFilterMax: u32, 
        wRemoveMsg: u32
    ) -> i32;

    pub fn TranslateMessage(lpmsg: *const MSG) -> i32;

    pub fn DispatchMessageW(lpmsg: *const MSG) -> isize;

    pub fn DestroyWindow(hWnd: HWND) -> i32;

    pub fn PostQuitMessage(nExitCode: i32);

    pub fn GetDC(hwnd: HWND) -> HDC;

    pub fn ReleaseDC(hWnd: HWND, hdc: HDC) -> i32;
}

#[link(name = "Kernel32")]
extern "stdcall" {
    pub fn GetModuleHandleA(
        lpModuleName: *const i8
    ) -> HINSTANCE;

    pub fn ExitProcess(uExitCode: u32);

    pub fn GetProcessHeap() -> HANDLE;

    pub fn HeapAlloc(
        hHeap: HANDLE, 
        dwFlags: DWORD, 
        dwBytes: usize) -> LPVOID;

    pub fn HeapFree(
        hHeap: HANDLE, 
        dwFlags: DWORD, 
        lpMem: LPVOID
    ) -> i32;

    pub fn GetProcAddress(
        hModule: HMODULE, 
        lpProcName: LPCSTR
    ) -> FUNCTION_PTR;

    pub fn LoadLibraryW(
        lpLibFileName: LPCWSTR
    ) -> HMODULE;

    pub fn VirtualAlloc(
        lpAddress: LPVOID,
        dwSize: usize,
        flAllocationType: DWORD, 
        flProtect: DWORD
    ) -> LPVOID;

    pub fn VirtualProtect(
        lpAddress: LPVOID, 
        dwSize: usize, 
        flNewProtect: DWORD, 
        lpflOldProtect: *mut DWORD
    ) -> i32;

    pub fn VirtualFree(
        lpAddress: LPVOID, 
        dwSize: usize, 
        dwFreeType: DWORD
    ) -> i32;

    pub fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    pub fn WriteConsoleA(hConsoleOutput: HANDLE, lpBuffer: *const u8, nNumberOfCharsToWrite: DWORD, lpNumberOfCharsWritten: *mut DWORD, lpReserved: LPVOID) -> bool;
    pub fn GetLastError() -> DWORD;
}

#[link(name = "Gdi32")]
extern "stdcall" {
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
    pub fn DescribePixelFormat(hdc: HDC, iPixelFormat: i32, nBytes: u32, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32;
    pub fn SetPixelFormat(
        hdc: HDC, 
        iPixelFormat: i32, 
        ppfd: *const PIXELFORMATDESCRIPTOR
    ) -> bool;

    pub fn SwapBuffers(hdc: HDC) -> i32;
}

#[link(name = "Opengl32")]
extern "stdcall" {
    pub fn wglCreateContext(hdc: HDC) -> HGLRC;
    pub fn wglDeleteContext(hglrc: HGLRC) -> i32;
    pub fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> i32;
    pub fn wglGetCurrentContext() -> HGLRC;
    pub fn wglGetProcAddress(arg: LPCSTR) -> FUNCTION_PTR;
}

pub fn get_module_handle() -> HINSTANCE {
    unsafe{ GetModuleHandleA(ptr::null()) }
}

pub fn ffi_message_box() {
    let title_u16 = &[b'N' as u16, b'i' as u16, b't' as u16, b'r' as u16, b'o' as u16, 0 as u16];
    unsafe {
        MessageBoxW(ptr::null_mut(), ptr::null_mut(),  title_u16.as_ptr(), 0);
    }
}

pub fn exit_process(exit_code: u32) {
    unsafe { ExitProcess(exit_code)}
}


unsafe extern "stdcall" fn wnd_proc(window: HWND,
                                   msg: u32,
                                   wparam: usize,
                                   lparam: isize)
-> isize {
    match WM::from(msg) {
        WM::SIZE => return 0,
        WM::CLOSE => {
            DestroyWindow(window);
            return 0;
        }
        WM::DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        _ => DefWindowProcW(window, msg, wparam, lparam)
    }
}


pub struct Window {
    pub width: usize,
    pub height: usize,
    pub dc: HDC,
    pub hwnd: HWND,
    pub hglrc: HGLRC,
}

impl Window {
    pub fn new(width: usize, height: usize) -> Self {
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
            let pixelFormat: i32;
            let numFormats: u32;

            ShowWindow(handle, 1);
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
    fn test_write_console() {
        let std_out = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
        let mut written: DWORD = 0;
        let text = &[b'H' as u8, b'e' as u8, b'l' as u8, b'l' as u8, b'o' as u8, b'!' as u8];
        unsafe { WriteConsoleA(std_out, text.as_ptr(), text.len() as u32, &mut written, ptr::null_mut()) };
    }
}