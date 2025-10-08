use crate::impl_element_of_vector;

impl_element_of_vector!(for N = 0: impl for u16);
impl_element_of_vector!(for N = 1: impl for u16);

// U16Vec2 is only 32 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 2: impl for u16);

// U16Vec3 is only 48 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 3: impl for u16);

// U16Vec4 is only 64 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 4: impl for u16);

// TODO: Determine if U16Vec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for u16);

// TODO: Add SIMD optimizations to U16Vec6.
impl_element_of_vector!(for N = 6: impl for u16);

// TODO: Add SIMD optimizations to U16Vec7.
impl_element_of_vector!(for N = 7: impl for u16);

// TODO: Add SIMD optimizations to U16Vec8.
impl_element_of_vector!(for N = 8: impl for u16);
