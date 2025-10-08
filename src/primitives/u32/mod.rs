use crate::impl_element_of_vector;

impl_element_of_vector!(for N = 0: impl for u32);
impl_element_of_vector!(for N = 1: impl for u32);

// UVec2 is only 64 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 2: impl for u32);

// TODO: Add SIMD optimizations to UVec3.
impl_element_of_vector!(for N = 3: impl for u32);

// TODO: Add SIMD optimizations to UVec4.
impl_element_of_vector!(for N = 4: impl for u32);

// TODO: Determine if UVec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for u32);

// TODO: Determine if UVec6 benefits from SIMD.
impl_element_of_vector!(for N = 6: impl for u32);

// TODO: Determine if UVec7 benefits from SIMD.
impl_element_of_vector!(for N = 7: impl for u32);

// TODO: Determine if UVec8 benefits from SIMD.
impl_element_of_vector!(for N = 8: impl for u32);
