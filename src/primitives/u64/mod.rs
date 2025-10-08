use crate::impl_element_of_vector;

impl_element_of_vector!(for N = 0: impl for u64);
impl_element_of_vector!(for N = 1: impl for u64);

// TODO: Add SIMD optimizations to U64Vec2.
impl_element_of_vector!(for N = 2: impl for u64);

// TODO: Add SIMD optimizations to U64Vec3.
impl_element_of_vector!(for N = 3: impl for u64);

// TODO: Add SIMD optimizations to U64Vec4.
impl_element_of_vector!(for N = 4: impl for u64);

// TODO: Determine if U64Vec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for u64);

// TODO: Determine if U64Vec6 benefits from SIMD.
impl_element_of_vector!(for N = 6: impl for u64);

// TODO: Determine if U64Vec7 benefits from SIMD.
impl_element_of_vector!(for N = 7: impl for u64);

// TODO: Determine if U64Vec8 benefits from SIMD.
impl_element_of_vector!(for N = 8: impl for u64);
