pub mod arena;
mod utils;

pub struct Octree {
}

mod tests {
    use crate::{utils::first_zero_position, arena::Arena};

    #[test]
    fn first_zero_position_test() {
        assert!(first_zero_position(245) == 1);
    }

    #[test]
    fn arena_store_retrive() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store(10);
        arena.store(20);
        let handle2 = arena.store(30);

        assert!(*handle.get(&arena) == 10);
        assert!(*handle2.get(&arena) == 30);
    }

    #[test]
    fn arena_remove() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store(10);
        let handle2 = arena.store(20);
        let handle3 = arena.store(30);

        handle2.remove(&mut arena);
        handle.remove(&mut arena);

        assert!(*handle3.get(&arena) == 30)
    }

    #[test]
    fn arena_remove_reinsert() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store(10);
        arena.store(20);
        handle.remove(&mut arena);

        let handle3 = arena.store(30);

        assert!(*handle3.get(&arena) == 30);
        assert!(handle3.index == 0);
    }
}