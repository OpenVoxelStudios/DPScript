#![feature(box_into_inner)]

use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectRef<T> {
    index: usize,
    _ty: PhantomData<T>,
}

impl<T> Clone for ObjectRef<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            _ty: self._ty,
        }
    }
}

impl<T> Copy for ObjectRef<T> {}

#[repr(C)]
pub struct ObjectInfo<T> {
    ptr: *mut T,
    occupied: bool,
}

impl<T> ObjectInfo<T> {
    /// Create an unallocated [`ObjectInfo<T>`].
    ///
    /// # Safety
    ///
    /// This function is inherently unsafe, as it creates an [`ObjectInfo<T>`] with an
    /// intentionally dangling pointer ([`core::ptr::dangling_mut`]). However, the [`Self::occupied`]
    /// flag is set to false, so readers should know not to read from the pointer.
    pub unsafe fn unallocated() -> Self {
        Self {
            ptr: core::ptr::dangling_mut(),
            occupied: false,
        }
    }

    #[must_use]
    pub fn take(self) -> Option<T> {
        if self.occupied {
            Some(Box::into_inner(unsafe { Box::from_raw(self.ptr) }))
        } else {
            None
        }
    }

    #[must_use]
    pub fn get(&self) -> Option<&T> {
        if self.occupied {
            unsafe { self.ptr.as_ref() }
        } else {
            None
        }
    }

    /// Get a mutable reference to the underlying data.
    ///
    /// # Clippy
    ///
    /// This returns an `&mut T` value, even though this function operates on `&self`.
    /// This is because this holds a pointer, which won't change.
    #[must_use]
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> Option<&mut T> {
        if self.occupied {
            unsafe { self.ptr.as_mut() }
        } else {
            None
        }
    }
}

pub struct ObjectPool<T> {
    objects: Vec<ObjectInfo<T>>,
    free_indices: Vec<usize>,
}

impl<T> ObjectPool<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            free_indices: Vec::new(),
        }
    }

    pub fn add(&mut self, object: T) -> ObjectRef<T> {
        let idx = if !self.free_indices.is_empty() {
            self.free_indices.remove(0)
        } else {
            self.objects.len()
        };

        let obj = Box::leak(Box::new(object));

        let info = ObjectInfo {
            ptr: obj as *mut T,
            occupied: true,
        };

        if idx >= self.objects.len() {
            self.objects.push(info);
        } else {
            self.objects[idx] = info;
        }

        ObjectRef {
            index: idx,
            _ty: PhantomData,
        }
    }

    pub fn remove(&mut self, obj_ref: ObjectRef<T>) -> Option<T> {
        let mut obj = unsafe { ObjectInfo::unallocated() };

        core::mem::swap(&mut self.objects[obj_ref.index], &mut obj);

        if !self.free_indices.contains(&obj_ref.index) {
            self.free_indices.push(obj_ref.index);
        }

        obj.take()
    }

    #[must_use]
    pub fn get(&self, obj: &ObjectRef<T>) -> Option<&T> {
        // I actually didn't know how to use and_then() before, thanks Clippy! :D Clippy is awesome.
        self.objects.get(obj.index).and_then(|it| it.get())
    }

    #[must_use]
    pub fn get_mut(&self, obj: &ObjectRef<T>) -> Option<&mut T> {
        self.objects.get(obj.index).and_then(|it| it.get_mut())
    }

    /// Free all objects in this pool.
    ///
    /// # Safety
    ///
    /// This function is safe, however it is marked as unsafe as it can leave dangling
    /// [`ObjectRef`]s with nothing to point to, and adding new objects into this pool
    /// could make them point to an unintended place. This function is only intended for
    /// cleanup before dropping the pool, like in its [`Drop`] implementation.
    pub unsafe fn free(&mut self) {
        for obj in core::mem::take(&mut self.objects) {
            let _ = obj.take();
        }

        self.free_indices.clear();
    }
}

impl<T> Default for ObjectPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ObjectPool<T> {
    fn drop(&mut self) {
        unsafe {
            self.free();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ObjectPool, ObjectRef};

    #[test]
    pub fn test_obj_pool_static() {
        let mut pool = ObjectPool::new();

        let ref1 = pool.add("Hello, world! 1");
        let ref2 = pool.add("Hello, world! 2");
        let ref3 = pool.add("Hello, world! 3");
        let ref4 = pool.add("Hello, world! 4");

        assert_eq!(*pool.get(&ref1).unwrap(), "Hello, world! 1");
        assert_eq!(*pool.get(&ref2).unwrap(), "Hello, world! 2");
        assert_eq!(*pool.get(&ref3).unwrap(), "Hello, world! 3");
        assert_eq!(*pool.get(&ref4).unwrap(), "Hello, world! 4");
    }

    #[test]
    pub fn test_obj_pool_alloc() {
        let mut pool = ObjectPool::new();

        let ref1 = pool.add(String::from("Hello, world! 1"));
        let ref2 = pool.add(String::from("Hello, world! 2"));
        let ref3 = pool.add(String::from("Hello, world! 3"));
        let ref4 = pool.add(String::from("Hello, world! 4"));

        assert_eq!(*pool.get(&ref1).unwrap(), "Hello, world! 1");
        assert_eq!(*pool.get(&ref2).unwrap(), "Hello, world! 2");
        assert_eq!(*pool.get(&ref3).unwrap(), "Hello, world! 3");
        assert_eq!(*pool.get(&ref4).unwrap(), "Hello, world! 4");
    }

    #[test]
    pub fn test_obj_pool_remove() {
        let mut pool = ObjectPool::new();

        let ref1 = pool.add(String::from("Hello, world!"));

        assert_eq!(pool.remove(ref1).as_deref(), Some("Hello, world!"));
    }

    #[test]
    pub fn test_obj_pool_reuse_removed_slot() {
        let mut pool = ObjectPool::new();

        let ref1 = pool.add(String::from("First"));
        let ref2 = pool.add(String::from("Second"));

        assert_eq!(pool.remove(ref1).as_deref(), Some("First"));

        let ref3 = pool.add(String::from("Third"));

        assert_eq!(*pool.get(&ref2).unwrap(), "Second");
        assert_eq!(*pool.get(&ref3).unwrap(), "Third");
    }

    #[test]
    pub fn test_obj_pool_get_mut() {
        let mut pool = ObjectPool::new();

        let ref1 = pool.add(String::from("Hello"));

        pool.get_mut(&ref1).unwrap().push_str(", world!");

        assert_eq!(*pool.get(&ref1).unwrap(), "Hello, world!");
    }

    #[test]
    pub fn test_obj_pool_removed_returns_none() {
        let mut pool = ObjectPool::<String>::new();

        let ref1 = pool.add(String::from("Hello"));
        let _ = pool.remove(ref1);

        assert!(pool.get(&ref1).is_none());
    }

    #[test]
    pub fn test_obj_pool_remove_twice_returns_none() {
        let mut pool = ObjectPool::new();

        let ref1 = pool.add(String::from("Hello"));

        assert_eq!(pool.remove(ref1).as_deref(), Some("Hello"));

        let ref2 = ObjectRef {
            index: 0,
            _ty: core::marker::PhantomData,
        };

        assert!(pool.remove(ref2).is_none());
    }
}
