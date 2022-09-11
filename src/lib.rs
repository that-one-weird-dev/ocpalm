pub mod octree;
mod arena;
mod utils;


#[cfg(test)]
mod tests {
    use crate::{utils::first_zero_position, arena::Arena, octree::Octree};

    #[test]
    fn first_zero_position_test() {
        assert!(first_zero_position(245) == 1);
    }

    #[test]
    fn arena_store_retrive() {
        let arena = Arena::<i32>::new();

        let handle = arena.store(10);
        arena.store(20);
        let handle2 = arena.store(30);

        assert!(arena.get(&handle) == 10);
        assert!(arena.get(&handle2) == 30);
    }

    #[test]
    fn arena_remove() {
        let arena = Arena::<i32>::new();

        let handle = arena.store(10);
        let handle2 = arena.store(20);
        let handle3 = arena.store(30);

        arena.remove(handle2);
        arena.remove(handle);

        assert!(arena.get(&handle3) == 30)
    }

    #[test]
    fn arena_remove_reinsert() {
        let arena = Arena::<i32>::new();

        let handle = arena.store(10);
        arena.store(20);
        arena.remove(handle);

        let handle3 = arena.store(30);

        assert!(arena.get(&handle3) == 30);
        assert!(handle3.index == 0);
    }

    #[test]
    fn arena_alloc_count() {
        let arena = Arena::<i32>::new();

        arena.store(10);
        let handle = arena.store(20);
        arena.store(30);

        arena.remove(handle);

        assert!(arena.get_allocation_count() == 2);
    }

    #[test]
    fn octree_store_retrive() {
        let mut octree = Octree::<u16>::new(8);

        octree.set(10, 20, 30, 56);
        octree.set(0, 0, 0, 12);

        assert!(octree.get(0, 0, 0) == 12);
        assert!(octree.get(10, 20, 30) == 56);
    }

    #[test]
    fn octree_allocation() {
        let mut octree = Octree::<u32>::new(3);

        assert!(octree.arena.get_allocation_count() == 1);

        octree.set(0, 0, 0, 10);

        assert!(octree.arena.get_allocation_count() == 17);

        octree.set(1, 0, 1, 10);
        octree.set(1, 0, 0, 10);
        octree.set(1, 1, 1, 10);
        octree.set(1, 1, 0, 10);
        octree.set(0, 1, 1, 10);
        octree.set(0, 1, 0, 10);
        octree.set(0, 0, 1, 10);

        assert!(octree.arena.get_allocation_count() == 9);
    }
}