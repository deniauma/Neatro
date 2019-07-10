use core::mem::{size_of, align_of};
use core::ffi::c_void;
use win32::{HANDLE, GetProcessHeap, HeapAlloc, HeapFree};

pub struct Layout
{
    pub size: usize,
    pub align: usize,
}

impl Layout
{
    pub fn new(size: usize) -> Self
    {
        Self
        {
            size,
            align: 4,
        }
    }

    pub fn from_type<T>() -> Self
    {
        Self
        {
            size: size_of::<T>(),
            align: align_of::<T>(),
        }
    }

    pub fn from_array_type<T>(length: usize) -> Self
    {
        Self
        {
            size: size_of::<T>() * length,
            align: align_of::<T>(),
        }
    }
}

pub trait Allocator
{
    unsafe fn alloc(&mut self, layout: Layout) -> Option<*mut c_void>;
    unsafe fn dealloc(&mut self, ptr: *mut c_void);
}

pub struct Win32HeapAllocator
{
    heap: HANDLE,
}

impl Default for Win32HeapAllocator {
    fn default() -> Self
    {
        Self
        {
            heap: unsafe { GetProcessHeap() },
        }
    }
}

impl Allocator for Win32HeapAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Option<*mut c_void>
    {
        let ptr = HeapAlloc(self.heap, 0, layout.size);

        if ptr.is_null()
        {
            None
        }
        else
        {
            Some(ptr as *mut c_void)
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut c_void)
    {
        HeapFree(self.heap, 0, ptr as win32::LPVOID);
    }
}
