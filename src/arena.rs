use std::{marker::PhantomData, cell::RefCell};

use crate::utils::first_zero_position;


pub struct Arena<
    T: Copy,
    const SIZE: usize = 1024,
    const FREE_SIZE: usize = 128
> {
    data: RefCell<[T; SIZE]>,
    free_space: RefCell<[u8; FREE_SIZE]>,
}

impl<
    T: Default + Copy,
    const SIZE: usize,
    const FREE_SIZE: usize
> Arena<T, SIZE, FREE_SIZE> {

    pub fn new() -> Self {
        Self {
            data: RefCell::new([T::default(); SIZE]),
            free_space: RefCell::new([0; FREE_SIZE]),
        }
    }

    pub fn store(&self, value: T) -> ArenaHandle<T> {
        let mut found = false;
        let mut first_free = 0;

        for (i, slot) in self.free_space.borrow_mut().iter_mut().enumerate() {
            if *slot == 255 { continue }

            let pos = first_zero_position(*slot);

            *slot += 2u8.pow(pos as u32);

            first_free = i * 8 + pos as usize;
            found = true;

            break
        }

        if !found { todo!("Increment space") }

        let handle = ArenaHandle::new(first_free as u32);

        self.set(&handle, value);

        handle
    }

    pub fn set(&self, handle: &ArenaHandle<T>, new_value: T) {
        if handle.is_null() {
            eprintln!("Received null handle, skipping operation (set)");
            return
        }

        self.data.borrow_mut()[handle.index as usize] = new_value;
    }

    pub fn get(&self, handle: &ArenaHandle<T>) -> T {
        if handle.is_null() {
            eprintln!("Received null handle, skipping operation (get)");
            return T::default()
        }

        self.data.borrow()[handle.index as usize]
    }

    pub fn get_mut(&self, handle: &ArenaHandle<T>) -> &mut T {
        if handle.is_null() {
            panic!("Received null handle, cannot proceed (get_mut)")
        }

        // FIXME: Implement check for memory safety
        unsafe {
            (&self.data.borrow()[handle.index as usize] as *const T as *mut T).as_mut().unwrap()
        }
    }

    pub fn remove(&self, handle: ArenaHandle<T>) {
        let slot = handle.index / 8;
        let pos = handle.index % 8;

        self.free_space.borrow_mut()[slot as usize] -= 2u8.pow(pos as u32);

        self.data.borrow_mut()[handle.index as usize] = T::default();
    }

    #[allow(dead_code)]
    pub fn get_allocation_count(&self) -> u32 {
        let mut count = 0;

        for slot in self.free_space.borrow().iter() {
            for i in 0..8 {
                let mask = 1 << i;

                if slot & mask == mask {
                    count += 1;
                }
            }
        }

        count
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ArenaHandle<T> {
    pub(crate) index: u32,
    _phantom: PhantomData<T>,
}

impl<T> Default for ArenaHandle<T> {
    fn default() -> Self {
        Self {
            index: std::u32::MAX,
            _phantom: Default::default(),
        }
    }
}

impl<T: Default + Copy> ArenaHandle<T> {
    fn new(index: u32) -> Self {
        Self {
            index,
            _phantom: PhantomData::default(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.index == std::u32::MAX
    }
}