use crate::alloc::*;
use core::ffi::c_void;
use core::ptr;
use core::ops::{Index, IndexMut};

pub struct RawVec<T, A: Allocator> {
    pub ptr: *mut T,
    pub capacity: usize,
    alloc: A,
    // _phantom: PhantomData<T>,
}

impl<T, A: Allocator> RawVec<T, A> {
    pub fn new(alloc: A) -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            capacity: 0,
            alloc: alloc
        }
    }

    pub fn reserve(&mut self, new_capacity: usize) {
        if new_capacity <= self.capacity {
            return;
        }

        let layout = Layout::from_array_type::<T>(new_capacity);
        let ptr = unsafe { self.alloc.alloc(layout).expect("Allcation error") as *mut T };

        if self.capacity > 0 {
            unsafe {
                ptr.copy_from(self.ptr, self.capacity);
                self.alloc.dealloc(self.ptr as *mut c_void);
            }
        }

        self.ptr = ptr;
        self.capacity = new_capacity;
        
    }
}

impl<T, A: Allocator> Drop for RawVec<T, A>
{
    fn drop(&mut self)
    {
        if !self.ptr.is_null()
        {
            unsafe {
                self.alloc.dealloc(self.ptr as *mut c_void);
            }
        }
    }
}

pub struct CVec<T, A: Allocator> {
    length: usize,
    buffer: RawVec<T, A>
}

impl<T, A: Allocator + Default> CVec<T, A> {
    pub fn new() -> Self {
        Self {
            length: 0,
            buffer: RawVec::new(A::default())
        }
    }

    fn grow(&mut self) {
        if self.length == 0 {
            self.buffer.reserve(1);
        } else {
            self.buffer.reserve(self.length *2);
        }
    }

    pub fn push(&mut self, value: T) {
        if self.length == self.buffer.capacity {
            self.grow();
        }

        unsafe {
            self.buffer.ptr.offset(self.length as isize).write(value);
        }

        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            let value = unsafe {
                self.buffer.ptr.offset((self.length - 1) as isize).read()
            };
            self.length -= 1;
            Some(value)
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.length {
            unsafe {
                ptr::drop_in_place(self.buffer.ptr.offset(i as isize));
            }
        }
        self.length = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn as_ptr(&self) -> *const T {
        self.buffer.ptr
    }
}

impl<T, A: Allocator> Index<usize> for CVec<T, A> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        unsafe { self.buffer.ptr.offset(i as isize).as_ref().expect("Null pointer") }
    }
}

impl<T, A: Allocator> IndexMut<usize> for CVec<T, A> {

    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        unsafe { self.buffer.ptr.offset(i as isize).as_mut().expect("Null pointer") }
    }
}

pub struct VecIntoIter<T, A: Allocator>
{
    inner: CVec<T, A>,
    current: usize,
    size: usize,
}

impl<T, A: Allocator> Iterator for VecIntoIter<T, A>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        if self.current >= self.size
        {
            None
        }
        else
        {
            unsafe {
                let index = self.current;
                self.current += 1;
                Some(ptr::read(self.inner.buffer.ptr.offset(index as isize)))
            }
        }
    }
}

impl<T, A: Allocator> Drop for VecIntoIter<T, A>
{
    fn drop(&mut self)
    {
        // Drop the remaining elements if we didn't iter
        // until the end.
        if core::mem::needs_drop::<T>()
        {
            unsafe {
                for i in self.current..self.size
                {
                    ptr::drop_in_place(self.inner.buffer.ptr.offset(i as isize));
                }
            }
        }

        // Set size of array to 0 so it doesn't drop anything else
        self.inner.length = 0;
    }
}

impl<T, A: Allocator> IntoIterator for CVec<T, A>
{
    type Item = T;
    type IntoIter = VecIntoIter<T, A>;

    fn into_iter(self) -> Self::IntoIter
    {
        VecIntoIter {
            size: self.length,
            inner: self,
            current: 0,
        }
    }
}

pub struct RefVecIntoIter<'a, T, A: Allocator>
{
    inner: &'a CVec<T, A>,
    current: usize,
    size: usize,
}

impl<'a, T, A: Allocator> Iterator for RefVecIntoIter<'a, T, A>
{
    type Item = T;

    fn next(&mut self) -> Option<T>
    {
        if self.current >= self.size
        {
            None
        }
        else
        {
            unsafe {
                let index = self.current;
                self.current += 1;
                Some(ptr::read(self.inner.buffer.ptr.offset(index as isize)))
            }
        }
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a CVec<T, A>
{
    type Item = T;
    type IntoIter = RefVecIntoIter<'a, T, A>;

    fn into_iter(self) -> Self::IntoIter
    {
        RefVecIntoIter {
            size: self.length,
            inner: self,
            current: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let mut cvec: CVec<usize, Win32HeapAllocator> = CVec::new();
        assert_eq!(cvec.length, 0);
        cvec.push(12);
        assert_eq!(cvec.length, 1);
        cvec.push(6);
        assert_eq!(cvec.pop(), Some(6));
        assert_eq!(cvec[0], 12);
        cvec[0] = 89;
        assert_eq!(cvec[0], 89);
    }
}