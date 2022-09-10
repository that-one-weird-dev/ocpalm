use crate::arena::{Arena, ArenaHandle};


#[repr(C)]
pub struct Octree<T> {
    arena: Arena<OctreeNode<T>>,
    root: ArenaHandle<OctreeNode<T>>,
    radius: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OctreeNode<T> {
    leaf: bool,
    children: [ArenaHandle<OctreeNode<T>>; 8],
    data: T,
}

impl<T: Default> Default for OctreeNode<T> {
    fn default() -> Self {
        Self {
            leaf: true,
            children: Default::default(),
            data: T::default(),
        }
    }
}

impl<T: Default + Copy + PartialEq> Octree<T> {
    pub fn new(size_radius: usize) -> Self {
        let mut arena = Arena::new();

        Self {
            root: arena.store(OctreeNode::default()),
            arena,
            radius: size_radius as i32,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, value: T) {
        todo!()
        // if x > self.radius || x < -self.radius
        // || y > self.radius || y < -self.radius
        // || z > self.radius || z < -self.radius {
        //     return;
        // }

        // let mut current_node = self.root.get_mut(&self.arena);
        // let mut i = 0;

        // loop {
        //     if current_node.data == value { break }

        //     i += 1;
        // }
    }

    pub fn get(&mut self, x: i32, y: i32, z: i32) -> T {
        todo!()
    }
}