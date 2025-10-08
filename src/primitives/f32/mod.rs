use crate::impl_element_of_vector;

impl_element_of_vector!(for N = 0: impl for f32);
impl_element_of_vector!(for N = 1: impl for f32);

// FVec2 is only 64 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 2: impl for f32);

// TODO: Add SIMD optimizations to FVec3.
impl_element_of_vector!(for N = 3: impl for f32);

// TODO: Add SIMD optimizations to FVec4.
impl_element_of_vector!(for N = 4: impl for f32);

// TODO: Determine if FVec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for f32);

// TODO: Determine if FVec6 benefits from SIMD.
impl_element_of_vector!(for N = 6: impl for f32);

// TODO: Determine if FVec7 benefits from SIMD.
impl_element_of_vector!(for N = 7: impl for f32);

// TODO: Determine if FVec8 benefits from SIMD.
impl_element_of_vector!(for N = 8: impl for f32);
