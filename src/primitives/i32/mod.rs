use crate::impl_element_of_vector;

impl_element_of_vector!(for N = 0: impl for i32);
impl_element_of_vector!(for N = 1: impl for i32);

// IVec2 is only 64 bits, so it doesn't benefit from SIMD.
impl_element_of_vector!(for N = 2: impl for i32);

// TODO: Add SIMD optimizations to IVec3.
impl_element_of_vector!(for N = 3: impl for i32);

// TODO: Add SIMD optimizations to IVec4.
impl_element_of_vector!(for N = 4: impl for i32);

// TODO: Determine if IVec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for i32);

// TODO: Determine if IVec6 benefits from SIMD.
impl_element_of_vector!(for N = 6: impl for i32);

// TODO: Determine if IVec7 benefits from SIMD.
impl_element_of_vector!(for N = 7: impl for i32);

// TODO: Determine if IVec8 benefits from SIMD.
impl_element_of_vector!(for N = 8: impl for i32);
