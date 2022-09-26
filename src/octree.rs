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
    children: ArenaHandle<OctreeNode<T>>,
    parent: ArenaHandle<OctreeNode<T>>,
    data: T,
}

impl<T: Default> Default for OctreeNode<T> {
    fn default() -> Self {
        Self {
            parent: Default::default(),
            children: Default::default(),
            data: T::default(),
        }
    }
}

impl<T: Default + Copy + PartialEq> Octree<T> {
    pub fn new(max_depth: u32) -> Self {
        let mut arena = Arena::new();

        let handle = ArenaHandle::new(1);
        arena.set(&handle, OctreeNode::default());

        Self {
            root: handle,
            arena,
            max_depth,
        }
    }

    pub fn set(&mut self, mut x: i32, mut y: i32, mut z: i32, value: T) {
        // TODO: Assert coords are inside the bounds

        // 2^(max_depth - 1) / 2
        let mut size = 1 << (self.max_depth - 1);
        let mut half_size = size / 2;

        let mut current_node_handle = self.root;

        loop {
            if size == 1 { break }

            size /= 2;
            half_size /= 2;

            let mut index = 0;

            if x >= 0 {
                index |= 0b100;
                x -= half_size;
            } else {
                x += half_size;
            }
            if y >= 0 {
                index |= 0b010;
                y -= half_size;
            } else {
                y += half_size;
            }
            if z >= 0 {
                index |= 0b001;
                z -= half_size;
            } else {
                z += half_size;
            }

            let current_node = self.arena.get(&current_node_handle);
            if current_node.leaf() {
                let data = current_node.data;

                // Subdivide the node
                // 1. Create the children
                let children = self.arena.store_8_aligned([OctreeNode::new_child(current_node_handle, data); 8]);

                // 2. Update the new parent node
                let current_node = self.arena.get_mut(&current_node_handle);

                current_node.children = children;
            }

            current_node_handle = self.arena
                .get_mut(&current_node_handle)
                .children.index(index);
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
            self.arena.remove_8_aligned(parent.children);

            // 2. Update state
            {
                let parent = self.arena.get_mut(&parent_handle);
                parent.data = compressable.unwrap();
            }

            current_node_handle = parent_handle;
        }
    }

    pub fn get(&self, mut x: i32, mut y: i32, mut z: i32) -> T {
        // TODO: Assert coords are inside the bounds

        let mut size = 1 << (self.max_depth - 1);
        let mut half_size = size / 2;

        let mut current_node = self.arena.get(&self.root);

        loop {
            if size == 1 { break }

            size /= 2;
            half_size /= 2;

            let mut index = 0;

            if x >= 0 {
                index |= 0b100;
                x -= half_size;
            } else {
                x += half_size;
            }
            if y >= 0 {
                index |= 0b010;
                y -= half_size;
            } else {
                y += half_size;
            }
            if z >= 0 {
                index |= 0b001;
                z -= half_size;
            } else {
                z += half_size;
            }

            if current_node.leaf() {
                break
            }

            current_node = self.arena.get(&current_node.children.index(index));
        }

        current_node.data
    }

    pub fn as_slice(&self) -> &[OctreeNode<T>] {
        self.arena.as_slice()
    }

    pub fn as_byte_slice(&self) -> &[u8] {
        unsafe {
            let slice = self.as_slice();

            let len = slice.len() * std::mem::size_of::<OctreeNode<T>>();
            let bytes = std::slice::from_raw_parts(slice.as_ptr() as *const u8, len);

            bytes
        }
    }
}

impl<T: Default + Copy + PartialEq> OctreeNode<T> {
    fn new_child(parent: ArenaHandle<OctreeNode<T>>, value: T) -> Self {
        Self {
            parent,
            children: Default::default(),
            data: value,
        }
    }

    #[inline]
    fn leaf(&self) -> bool {
        self.children.is_null()
    }

    fn is_compressable(&self, arena: &Arena<OctreeNode<T>>) -> Option<T> {
        let first = arena.get(&self.children.index(0)).data;
        
        for i in 1..8 {
            let child_data = arena.get(&self.children.index(i)).data;

            if child_data != first {
                return None
            }
        }

        Some(first)
    }
}