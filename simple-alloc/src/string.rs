use crate::vec::CVec;
use crate::alloc::Win32HeapAllocator;

pub struct CString {
    inner: CVec<u8, Win32HeapAllocator>
}

impl CString {
    pub fn new() -> Self {
        Self {
            inner: CVec::new()
        }
    }

    pub fn from_str(src: &str) -> Self {
        let mut res = CString::new();
        for &c in src.as_bytes() {
            res.inner.push(c);
        }
        res
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.inner.as_ptr()
    }

    pub fn to_i8_str(&self) -> CVec<i8, Win32HeapAllocator> {
        let mut i8_str: CVec<i8, Win32HeapAllocator> = CVec::new();
        for c in &self.inner {
            i8_str.push(c as i8);
        }

        i8_str
    }

    pub fn to_u16_str(&self) -> CVec<u16, Win32HeapAllocator> {
        let mut u16_str: CVec<u16, Win32HeapAllocator> = CVec::new();
        for c in &self.inner {
            u16_str.push(c as u16);
        }

        u16_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wstring() {
        let func_name = CString::from_str("glClearColor");
        // func_name.as_ptr();
    }
}