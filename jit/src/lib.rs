#![no_std]
use win32::{VirtualAlloc, VirtualFree, VirtualProtect};
use core::ptr;

const PAGE_SIZE: usize = 4096;
const MEM_COMMIT: u32 = 0x00001000;
const MEM_RESERVE: u32 = 0x00002000;
const MEM_RELEASE: u32 = 0x00008000;
const PAGE_READWRITE: u32 = 0x04;
const PAGE_EXECUTE_READ: u32 = 0x20;

pub struct JitMem {
    addr: *mut u8,
    size: usize,
    offset: isize,
    fn_offset: isize
}

impl JitMem {
    pub fn new() -> Self {
        let buf = unsafe { VirtualAlloc(ptr::null_mut(), PAGE_SIZE, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) };

        Self {
            addr: buf as *mut u8,
            size: PAGE_SIZE,
            offset: 0,
            fn_offset: 0
        }
    }

    pub fn finalize(&self) {
        let mut old: win32::DWORD = 0;
        unsafe { VirtualProtect(self.addr as win32::LPVOID, PAGE_SIZE, PAGE_EXECUTE_READ, &mut old) };
    }

    pub fn push_instruct_byte(&mut self, byte: u8) {
        unsafe { self.addr.offset(self.offset).write(byte) };
        self.offset += 1;
    }

    pub fn push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        for &b in &bytes {
            self.push_instruct_byte(b);
        }
    }

    pub fn push_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        for &b in &bytes {
            self.push_instruct_byte(b);
        }
    }

    pub fn push_u64(&mut self, value: u64) {
        let bytes = value.to_le_bytes();
        for &b in &bytes {
            self.push_instruct_byte(b);
        }
    }

    pub fn set_jit_fn(&mut self) -> *mut u8 {
        let res = unsafe { self.addr.offset(self.fn_offset) };
        self.fn_offset += self.offset;
        res
    }
}

impl Drop for JitMem {
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
        let mut asmbuf = JitMem::new();
        assert_ne!(asmbuf.addr, ptr::null_mut());
        asmbuf.push_instruct_byte(0x48);
        asmbuf.push_instruct_byte(0xc7);
        asmbuf.push_instruct_byte(0xc0);
        asmbuf.push_instruct_byte(0x03);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0xc3);
        asmbuf.finalize(); //asm: mov rax, 0x03; ret
        let res = unsafe { core::mem::transmute::<_, fn() -> i32>(asmbuf.addr)() };
        assert_eq!(res, 3);
    }

    #[test]
    fn asmbuf_echo_fn() {
        let mut asmbuf = JitMem::new();
        assert_ne!(asmbuf.addr, ptr::null_mut());
        asmbuf.push_instruct_byte(0x48);
        asmbuf.push_instruct_byte(0x89);
        asmbuf.push_instruct_byte(0xc8);
        assert_eq!(asmbuf.offset, 3);
        asmbuf.push_instruct_byte(0xc3);
        let fn_addr = asmbuf.set_jit_fn();
        asmbuf.finalize(); //asm: mov rax, rcx; ret
        let res = unsafe { core::mem::transmute::<_, fn(i32) -> i32>(fn_addr)(3) };
        assert_eq!(res, 3);
    }

    #[test]
    fn jitmem_2_fn() {
        let mut asmbuf = JitMem::new();
        assert_ne!(asmbuf.addr, ptr::null_mut());
        // fn 1
        asmbuf.push_instruct_byte(0x48);
        asmbuf.push_instruct_byte(0xc7);
        asmbuf.push_instruct_byte(0xc0);
        asmbuf.push_instruct_byte(0x03);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0x00);
        asmbuf.push_instruct_byte(0xc3);
        let fn1 = asmbuf.set_jit_fn();
        //fn 2
        asmbuf.push_instruct_byte(0x48);
        asmbuf.push_instruct_byte(0x89);
        asmbuf.push_instruct_byte(0xc8);
        asmbuf.push_instruct_byte(0xc3);
        let fn2 = asmbuf.set_jit_fn();
        asmbuf.finalize(); //asm: mov rax, 0x03; ret
        let res1 = unsafe { core::mem::transmute::<_, fn() -> i32>(fn1)() };
        assert_eq!(res1, 3);
        let res2 = unsafe { core::mem::transmute::<_, fn(i32) -> i32>(fn2)(5) };
        assert_eq!(res2, 5);
    }

    pub extern "sysv64" fn test_from_jit() -> i32 {
        3+2
    }

    #[test]
    fn jitmem_rust_fn() {
        let mut asmbuf = JitMem::new();
        let fn_addr: u64 = unsafe { core::mem::transmute(test_from_jit as extern "sysv64" fn() -> i32) };
        asmbuf.push_instruct_byte(0x48);
        asmbuf.push_instruct_byte(0xb8);
        asmbuf.push_u64(fn_addr);
        asmbuf.push_instruct_byte(0xff);
        asmbuf.push_instruct_byte(0xd0);
        asmbuf.push_instruct_byte(0xc3);
        asmbuf.finalize();
        let res = unsafe { core::mem::transmute::<_, fn() -> i32>(asmbuf.set_jit_fn())() };
        assert_eq!(res, 5);
    }
}
