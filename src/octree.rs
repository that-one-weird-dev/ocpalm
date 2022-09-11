use crate::arena::{Arena, ArenaHandle};


#[repr(C)]
pub struct Octree<T: Copy> {
    pub(crate) arena: Arena<OctreeNode<T>>,
    root: ArenaHandle<OctreeNode<T>>,
    max_depth: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OctreeNode<T> {
    parent: ArenaHandle<OctreeNode<T>>,
    leaf: bool,
    children: [ArenaHandle<OctreeNode<T>>; 8],
    data: T,
}

impl<T: Default> Default for OctreeNode<T> {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            leaf: true,
            children: Default::default(),
            data: T::default(),
        }
    }
}

impl<T: Default + Copy + PartialEq> Octree<T> {
    pub fn new(max_depth: u32) -> Self {
        let mut arena = Arena::new();

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

        let mut current_node_handle = self.root;

        let mut middle_x = 0;
        let mut middle_y = 0;
        let mut middle_z = 0;

        loop {
            let xside = if x < middle_x { -1 } else { 1 };
            let yside = if y < middle_y { -1 } else { 1 };
            let zside = if z < middle_z { -1 } else { 1 };

            let current_node = self.arena.get(&current_node_handle);
            if current_node.leaf {
                let data = current_node.data;
                drop(current_node);

                // Subdivide the node
                // 1. Create the children
                let mut children = [ArenaHandle::<OctreeNode<T>>::default(); 8];
                for i in 0..8 {
                    children[i] = self.arena.store(OctreeNode::new_child(current_node_handle, data));
                }
                // 2. Update the new parent node
                let current_node = self.arena.get_mut(&current_node_handle);
                current_node.children = children;
                current_node.leaf = false;
            }

            current_node_handle = self.arena
                .get_mut(&current_node_handle)
                .children[((xside + 1) / 2 + (yside + 1) * 2 + (zside + 1)) as usize];

            if half_size == 1 { break }

            half_size /= 2;

            middle_x += half_size * xside;
            middle_y += half_size * yside;
            middle_z += half_size * zside;
        }

        // Setting the new value
        self.arena.get_mut(&current_node_handle).data = value;

        // Checking if compressable
        loop {
            let parent_handle = self.arena.get(&current_node_handle).parent;
            if parent_handle.is_null() { break }

            let parent = self.arena.get(&parent_handle);
            let compressable = parent.is_compressable(&self.arena);

            // If it can't compress then stop
            if let None = compressable { break }

            // Then compress the node
            // 1. Remove the children
            for child in parent.children {
                self.arena.remove(child);
            }
            // 2. Update state
            {
                let parent = self.arena.get_mut(&parent_handle);
                parent.leaf = true;
                parent.data = compressable.unwrap();
            }

            current_node_handle = parent_handle;
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> T {
        // TODO: Assert coords are inside the bounds

        // 2^(max_depth - 1) / 2
        let mut half_size = 1 << (self.max_depth - 2);

        let mut current_node = self.arena.get(&self.root);

        let mut middle_x = 0;
        let mut middle_y = 0;
        let mut middle_z = 0;

        loop {
            let xside = if x < middle_x { -1 } else { 1 };
            let yside = if y < middle_y { -1 } else { 1 };
            let zside = if z < middle_z { -1 } else { 1 };

            if current_node.leaf {
                break
            }

            current_node = self.arena.get(&current_node.children[((xside + 1) / 2 + (yside + 1) * 2 + (zside + 1)) as usize]);

            if half_size == 1 { break }

            half_size /= 2;

            middle_x += half_size * xside;
            middle_y += half_size * yside;
            middle_z += half_size * zside;
        }

        // Checking if parent is clearable

        current_node.data
    }
}

impl<T: Default + Copy + PartialEq> OctreeNode<T> {
    fn new_child(parent: ArenaHandle<OctreeNode<T>>, value: T) -> Self {
        Self {
            parent,
            leaf: true,
            children: Default::default(),
            data: value,
        }
    }

    // fn subdivide(&mut self, self_handle: ArenaHandle<OctreeNode<T>>, octree: &Octree<T>) {
    //     self.leaf = false;

    //     for i in 0..8 {
    //         self.children[i] = octree.arena.store(OctreeNode::new_child(self_handle, self.data));
    //     }
    // }

    fn is_compressable(&self, arena: &Arena<OctreeNode<T>>) -> Option<T> {
        let first = arena.get(&self.children[0]).data;
        
        for i in 1..8 {
            let child_data = arena.get(&self.children[i]).data;

            if child_data != first {
                return None
            }
        }

        Some(first)
    }

    // fn compress_if_possible(&mut self, octree: &Octree<T>) -> bool {
    //     let first = octree.arena.get(&self.children[0]).data;
        
    //     for i in 1..8 {
    //         let child_data = octree.arena.get(&self.children[i]).data;

    //         if child_data != first {
    //             return false
    //         }
    //     }

    //     // Otherwise compress
    //     for child in self.children {
    //         octree.arena.remove(child);
    //     }

    //     self.leaf = true;
    //     self.data = first;

    //     true
    // }
}