
GGMath is a generic-graphics-math crate.
So types like ```Vector``` and ```Matrix``` are generic over ```T: Scalar```.

You can implement your own scalar type and override the implementations of vector fns for that scalar type to make optimizations.
These custom fn implementations can have bugs, so this crate helps you test them.

