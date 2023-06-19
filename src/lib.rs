mod arena;
mod octree;
mod utils;

pub use octree::Octree;

#[cfg(test)]
mod tests {
    use crate::{
        arena::Arena,
        octree::{Octree, OctreeNode},
        utils::first_zero_position,
    };

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

        assert_eq!(*arena.get(&handle), 10);
        assert_eq!(*arena.get(&handle2), 30);
    }

    #[test]
    fn arena_store_retrive_8_aligned() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store_8_aligned([1, 2, 3, 4, 5, 6, 7, 8]);
        arena.store_8_aligned([4, 3, 5, 6, 3, 0, 4, 3]);
        let handle2 = arena.store_8_aligned([5, 6, 5, 6, 8, 5, 2, 7]);

        assert_eq!(*arena.get(&handle), 1);
        assert_eq!(*arena.get(&handle.index(3)), 4);
        assert_eq!(*arena.get(&handle.index(7)), 8);

        assert_eq!(*arena.get(&handle2), 5);
        assert_eq!(*arena.get(&handle2.index(2)), 5);
        assert_eq!(*arena.get(&handle2.index(6)), 2);
    }

    #[test]
    fn arena_remove() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store(10);
        let handle2 = arena.store(20);
        let handle3 = arena.store(30);

        arena.remove(handle2);
        arena.remove(handle);

        assert!(*arena.get(&handle3) == 30)
    }

    #[test]
    fn arena_remove_8_aligned() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store_8_aligned([1, 2, 3, 4, 5, 6, 7, 8]);
        let handle2 = arena.store_8_aligned([2, 6, 7, 3, 6, 4, 3, 1]);

        arena.remove_8_aligned(handle);

        assert!(*arena.get(&handle2.index(2)) == 7)
    }

    #[test]
    fn arena_remove_reinsert() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store(10);

        assert_eq!(handle.index, 2);

        arena.store(20);
        arena.remove(handle);

        let handle3 = arena.store(30);

        assert_eq!(*arena.get(&handle3), 30);
        assert_eq!(handle3.index, 2);
    }

    #[test]
    fn arena_remove_reinsert_8_aligned() {
        let mut arena = Arena::<i32>::new();

        let handle = arena.store_8_aligned([1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(handle.index, 2);

        arena.store_8_aligned([5, 6, 2, 4, 5, 2, 5, 7]);
        arena.remove_8_aligned(handle);

        let handle3 = arena.store_8_aligned([8, 5, 1, 6, 0, 3, 5, 0]);

        assert_eq!(*arena.get(&handle3.index(2)), 1);
        assert_eq!(handle3.index, 2);
    }

    #[test]
    fn arena_alloc_count() {
        let mut arena = Arena::<i32>::new();

        arena.store(10);
        let handle = arena.store(20);
        arena.store(30);

        arena.remove(handle);

        assert_eq!(arena.get_allocation_count(), 2);
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

        assert_eq!(octree.arena.get_allocation_count(), 0);

        octree.set(0, 0, 0, 10);

        assert_eq!(octree.arena.get_allocation_count(), 16);

        octree.set(1, 0, 1, 10);
        octree.set(1, 0, 0, 10);
        octree.set(1, 1, 1, 10);
        octree.set(1, 1, 0, 10);
        octree.set(0, 1, 1, 10);
        octree.set(0, 1, 0, 10);
        octree.set(0, 0, 1, 10);

        assert_eq!(octree.arena.get_allocation_count(), 8);
    }

    #[test]
    fn as_bytes_slice() {
        let octree = Octree::<u32>::new(4);

        octree.as_byte_slice();
    }

    #[test]
    fn memory_alignment() {
        assert_eq!(std::mem::size_of::<OctreeNode<u32>>(), 12);
    }
}
