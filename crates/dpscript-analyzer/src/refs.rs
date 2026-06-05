use std::mem::ManuallyDrop;

pub struct StaticRef {
    owned: ManuallyDrop<String>,
    ptr: *const str,
}

impl StaticRef {
    pub fn new(owned: String) -> Self {
        Self {
            ptr: owned.as_str() as *const str,
            owned: ManuallyDrop::new(owned),
        }
    }

    pub fn get<'t>(&self) -> &'t str {
        unsafe { &*self.ptr }
    }

    pub fn free(mut self) {
        unsafe { ManuallyDrop::drop(&mut self.owned) };
    }
}
