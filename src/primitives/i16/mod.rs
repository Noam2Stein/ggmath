use crate::impl_element_of_vector;

impl_element_of_vector!(for N = 0: impl for i16);
impl_element_of_vector!(for N = 1: impl for i16);

// I16Vec2 is only 32 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 2: impl for i16);

// I16Vec3 is only 48 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 3: impl for i16);

// I16Vec4 is only 64 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 4: impl for i16);

// TODO: Determine if I16Vec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for i16);

// TODO: Add SIMD optimizations to I16Vec6.
impl_element_of_vector!(for N = 6: impl for i16);

// TODO: Add SIMD optimizations to I16Vec7.
impl_element_of_vector!(for N = 7: impl for i16);

// TODO: Add SIMD optimizations to I16Vec8.
impl_element_of_vector!(for N = 8: impl for i16);
