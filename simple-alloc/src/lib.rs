#![no_std]
pub mod alloc;
pub mod vec;
pub mod string;
pub mod hashmap;
pub mod once;

pub use alloc::*;
pub use vec::CVec;
pub use string::CString;
pub use once::Once;

pub type WinVec<T> = CVec<T, Win32HeapAllocator>;