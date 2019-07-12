use win32::{VirtualAlloc, VirtualFree, VirtualProtect};
use core::ptr;

const PAGE_SIZE: usize = 4096;
const MEM_COMMIT: u32 = 0x00001000;
const MEM_RESERVE: u32 = 0x00002000;
const MEM_RELEASE: u32 = 0x00008000;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_EXECUTE_READ: u32 = 0x20;

pub struct AsmBuf {
    addr: *mut u8
}

impl AsmBuf {
    pub fn new() -> Self {
        let buf = unsafe { VirtualAlloc(ptr::null_mut(), PAGE_SIZE, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) };
        let mut old: win32::DWORD = 0;
        unsafe { VirtualProtect(buf, PAGE_SIZE, PAGE_EXECUTE_READ, &mut old) };

        Self {
            addr: buf as *mut u8
        }
    }
}

impl Drop for AsmBuf {
    fn drop(&mut self) {
        unsafe {
            VirtualFree(self.addr as win32::LPVOID, 0, MEM_RELEASE);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asmbuf_create() {
        let asmbuf = AsmBuf::new();
        assert_ne!(asmbuf.addr, ptr::null_mut());
    }
}
