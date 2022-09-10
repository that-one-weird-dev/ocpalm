use crate::arena::{Arena, ArenaHandle};


#[repr(C)]
pub struct Octree<T: Copy> {
    arena: Arena<OctreeNode<T>>,
    root: ArenaHandle<OctreeNode<T>>,
    max_depth: u32,
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
    pub fn new(max_depth: u32) -> Self {
        let arena = Arena::new();

        Self {
            root: arena.store(OctreeNode::default()),
            arena,
            max_depth,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, value: T) {
        // TODO: Assert coords are inside the bounds

        // 2^(max_depth - 1) / 2
        let mut half_size = 1 << (self.max_depth - 2);

        let mut current_node = self.root.get_mut(&self.arena);

        let mut middle_x = 0;
        let mut middle_y = 0;
        let mut middle_z = 0;

        loop {
            let xside = if x < middle_x { -1 } else { 1 };
            let yside = if y < middle_y { -1 } else { 1 };
            let zside = if z < middle_z { -1 } else { 1 };

            if current_node.leaf {
                current_node.subdivide(&self);
            }

            current_node = current_node.children[((xside + 1) / 2 + (yside + 1) * 2 + (zside + 1)) as usize].get_mut(&self.arena);

            if half_size == 1 { break }

            half_size /= 2;

            middle_x += half_size * xside;
            middle_y += half_size * yside;
            middle_z += half_size * zside;
        }

        current_node.data = value;
    }

    pub fn get(&mut self, _x: i32, _y: i32, _z: i32) -> T {
        todo!()
    }
}

impl<T: Default + Copy> OctreeNode<T> {
    fn new(value: T) -> Self {
        Self {
            leaf: true,
            children: Default::default(),
            data: value,
        }
    }

    fn subdivide(&mut self, octree: &Octree<T>) {
        self.leaf = false;

        for i in 0..8 {
            self.children[i] = octree.arena.store(OctreeNode::new(self.data));
        }
    }
}