# Ocpalm
A rust octree data structure implementation

## Why?
This crate was created to fulfill my need of a library that could store voxel data inside of an octree, with the goal of being streamed to the GPU with 0 to no serialization time.
Well this library has NONE, the data is already stored inside of an arena that can be easely made into a slice, without any copy of the data.

## Documentation
### WIP
