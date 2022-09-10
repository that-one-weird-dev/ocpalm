use std::marker::PhantomData;

use crate::utils::first_zero_position;


pub struct Arena<
    T,
    const SIZE: usize = 1024,
    const FREE_SIZE: usize = 128
> {
    data: [T; SIZE],
    free_space: [u8; FREE_SIZE],
}

impl<
    T: Default + Copy,
    const SIZE: usize,
    const FREE_SIZE: usize
> Arena<T, SIZE, FREE_SIZE> {

    pub fn new() -> Self {
        Self {
            data: [T::default(); SIZE],
            free_space: [0; FREE_SIZE]
        }
    }

    pub fn store(&mut self, value: T) -> ArenaHandle<T> {
        let mut found = false;
        let mut first_free = 0;

        for (i, slot) in self.free_space.iter_mut().enumerate() {
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

    pub fn set(&mut self, handle: &ArenaHandle<T>, new_value: T) {
        self.data[handle.index as usize] = new_value;
    }

    pub fn get(&self, handle: &ArenaHandle<T>) -> &T {
        &self.data[handle.index as usize]
    }

    pub fn get_mut(&mut self, handle: &ArenaHandle<T>) -> &mut T {
        &mut self.data[handle.index as usize]
    }

    pub fn remove(&mut self, handle: ArenaHandle<T>) {
        let slot = handle.index / 8;
        let pos = handle.index % 8;

        self.free_space[slot as usize] -= 2u8.pow(pos as u32);

        self.data[handle.index as usize] = T::default();
    }
}

#[repr(C)]
pub struct ArenaHandle<T> {
    pub(crate) index: u32,
    _phantom: PhantomData<T>,
}

impl<T: Default + Copy> ArenaHandle<T> {
    fn new(index: u32) -> Self {
        Self {
            index,
            _phantom: PhantomData::default(),
        }
    }

    pub fn get<'a>(&self, arena: &'a Arena<T>) -> &'a T {
        arena.get(self)
    }

    pub fn get_mut<'a>(&self, arena: &'a mut Arena<T>) -> &'a mut T {
        arena.get_mut(self)
    }

    pub fn set(&self, arena: &mut Arena<T>, new_value: T) {
        arena.set(self, new_value)
    }

    pub fn remove(self, arena: &mut Arena<T>) {
        arena.remove(self)
    }
}