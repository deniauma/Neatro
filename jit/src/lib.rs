use win32::{VirtualAlloc, VirtualFree, VirtualProtect};
use core::ptr;

const PAGE_SIZE: usize = 4096;
const MEM_COMMIT: u32 = 0x00001000;
const MEM_RESERVE: u32 = 0x00002000;
const MEM_RELEASE: u32 = 0x00008000;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_EXECUTE_READ: u32 = 0x20;

pub struct AsmBuf {
    addr: *mut u8,
    len: isize
}

impl AsmBuf {
    pub fn new() -> Self {
        let buf = unsafe { VirtualAlloc(ptr::null_mut(), PAGE_SIZE, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) };

        Self {
            addr: buf as *mut u8,
            len: 0
        }
    }

    pub fn finalize(&self) {
        let mut old: win32::DWORD = 0;
        unsafe { VirtualProtect(self.addr as win32::LPVOID, PAGE_SIZE, PAGE_EXECUTE_READ, &mut old) };
    }

    pub fn push_instruct_byte(&mut self, byte: u8) {
        unsafe { self.addr.offset(self.len).write(byte) };
        self.len += 1;
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
        let mut asmbuf = AsmBuf::new();
        assert_ne!(asmbuf.addr, ptr::null_mut());
        asmbuf.push_instruct_byte(0x48);
        // asmbuf.push_instruct_byte(0x89);
        // asmbuf.push_instruct_byte(0xf8);
        asmbuf.push_instruct_byte(0xc7);
        asmbuf.push_instruct_byte(0xc0);
        asmbuf.push_instruct_byte(0x03);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0x00);
        // assert_eq!(asmbuf.len, 3);
        asmbuf.push_instruct_byte(0xc3);
        asmbuf.finalize(); //asm: mov rax, 0x03; ret
        let res = unsafe { core::mem::transmute::<_, fn() -> i32>(asmbuf.addr)() };
        assert_eq!(res, 3);
    }

    #[test]
    fn asmbuf_echo_fn() {
        let mut asmbuf = AsmBuf::new();
        assert_ne!(asmbuf.addr, ptr::null_mut());
        asmbuf.push_instruct_byte(0x48);
        asmbuf.push_instruct_byte(0x89);
        asmbuf.push_instruct_byte(0xc8);
        assert_eq!(asmbuf.len, 3);
        asmbuf.push_instruct_byte(0xc3);
        asmbuf.finalize(); //asm: mov rax, rcx; ret
        let res = unsafe { core::mem::transmute::<_, fn(i32) -> i32>(asmbuf.addr)(3) };
        assert_eq!(res, 3);
    }
}
