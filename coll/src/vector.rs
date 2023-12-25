use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::{drop_in_place, read, NonNull};

const DEFAULT_CAPACITY: usize = 4;

pub struct Iter<'a, T> {
    src: &'a Vector<T>,
    index: usize,
}

pub struct IterMut<'a, T> {
    src: &'a Vector<T>,
    index: usize,
}

pub struct Vector<T> {
    cap: usize,
    len: usize,
    ptr: Option<NonNull<T>>,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            len: 0,
            cap: capacity,
            ptr: None,
        }
    }

    fn need_to_grow(&self) -> bool {
        !(self.len() < self.capacity())
    }

    fn allocate(&mut self) {
        let layout = Layout::array::<T>(self.capacity()).expect("Could not allocate memory");
        self.ptr = NonNull::new(unsafe { alloc(layout) } as *mut T);
    }

    fn reallocate(&mut self) {
        assert!(
            (!self.is_empty() && self.ptr.is_some()),
            "to reallocate you must have full lengthed vector first"
        );

        if let Some(p) = self.ptr {
            // TODO: check new capacity overflow case;
            let new_capacity = self.capacity() * 2;
            unsafe {
                let layout = Layout::array::<T>(new_capacity).expect("Could not reallocate memory");
                let ptr = realloc(p.as_ptr() as *mut u8, layout, new_capacity);
                self.ptr = Some(NonNull::new(ptr as *mut T).expect("Could not reallocate memory"));
            }
            self.cap = new_capacity;
        }
    }

    fn ensure_allocation(&mut self) {
        match self.ptr {
            None => {
                self.allocate();
            }
            _ => {
                if self.need_to_grow() {
                    self.reallocate();
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn peek(&self) -> Option<&T> {
        self.get(self.len() - 1)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() {
            return None;
        }
        match self.ptr {
            Some(p) => Some(unsafe { &*p.as_ptr().add(index) }),
            None => None,
        }
    }

    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        if index >= self.len() {
            return None;
        }
        match self.ptr {
            Some(p) => Some(unsafe { &mut *p.as_ptr().add(index) }),
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            src: self,
            index: 0,
        }
    }

    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            src: self,
            index: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.ensure_allocation();

        self.ptr
            .as_ref()
            .map(|p| unsafe { p.as_ptr().add(self.len()).write(item) });
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if let Some(p) = self.ptr {
            unsafe {
                self.len -= 1;
                Some(read(p.as_ptr().add(self.len())))
            }
        } else {
            None
        }
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if let Some(p) = self.ptr {
            unsafe {
                for i in 0..self.len() {
                    let ptr = p.as_ptr().add(i);
                    drop_in_place(ptr);
                }

                let layout =
                    Layout::array::<T>(self.capacity()).expect("Could not reallocate memory");
                dealloc(p.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.src.is_empty() {
            return None;
        }
        let item = self.src.get(self.index);
        self.index += 1;
        item
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.src.is_empty() {
            return None;
        }
        let item = self.src.get_mut(self.index);
        self.index += 1;
        item
    }
}

#[cfg(test)]
mod tests {
    use super::Vector;
    use crate::vector::DEFAULT_CAPACITY;

    #[test]
    fn test_vector_iter() {
        let cap: usize = 10;
        let mut vec = Vector::<i32>::with_capacity(cap);
        for i in 0..cap {
            vec.push(i as i32);
        }

        let mut i = 0;
        for item in vec.iter() {
            assert_eq!(item, &i);
            i += 1;
        }
    }

    #[test]
    fn test_vector_iter_mut() {
        let cap: usize = 10;
        let mut vec = Vector::<i32>::with_capacity(cap);
        for i in 0..cap {
            vec.push(i as i32);
        }

        let mut i = 0;
        for item in vec.iter_mut() {
            assert_eq!(item, &i);
            *item += 1;
            i += 1;
        }

        let mut i = 0;
        for item in vec.iter_mut() {
            assert_eq!(item, &(i + 1));
            i += 1;
        }
    }

    #[test]
    fn test_vector_creation() {
        let vec = Vector::<i32>::new();
        assert_eq!(vec.capacity(), DEFAULT_CAPACITY);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.need_to_grow(), false);
    }

    #[test]
    fn test_vector_creation_with_custom_capacity() {
        let capacity = 20;
        let other = Vector::<i32>::with_capacity(capacity);
        assert_eq!(other.capacity(), capacity);
        assert_eq!(other.len(), 0);
        assert_eq!(other.need_to_grow(), false);
    }
    #[test]
    fn test_vector_get() {
        let mut vec = Vector::<i32>::new();
        assert_eq!(vec.get(0), None);
        assert_eq!(vec.get(10), None);
        assert_eq!(vec.len(), 0);
        vec.push(10);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_vector_reallocation() {
        let count = 10;
        let capacity: usize = 2;
        let mut vec: Vector<i32> = Vector::with_capacity(capacity);
        assert_eq!(vec.capacity(), capacity);
        for i in 0..count {
            let value = (i * count) as i32;
            vec.push(value);
            assert_eq!(vec.len(), i + 1);
            assert_eq!(vec.get(i), Some(&value));
        }
        assert_eq!(vec.capacity(), 16);
    }

    #[test]
    fn test_vector_push() {
        let mut vec: Vector<i32> = Vector::new();
        assert_eq!(vec.capacity(), DEFAULT_CAPACITY);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.need_to_grow(), false);
        vec.push(10);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.need_to_grow(), false);
        assert_eq!(vec.peek(), Some(&10));
        vec.push(12);
        assert_eq!(vec.len(), 2);
        assert_eq!(vec.need_to_grow(), false);

        assert_eq!(vec.get(0), Some(&10));
        assert_eq!(vec.get(1), Some(&12));
        assert_eq!(vec.get(10), None);
    }
}
