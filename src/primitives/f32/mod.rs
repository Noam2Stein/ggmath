use crate::{F32ElementOfVector, Simd, impl_element_of_vector};

#[cfg(target_feature = "sse")]
mod sse;

#[cfg(not(target_feature = "sse"))]
mod fallback;

impl_element_of_vector!(for N = 0: impl for f32);
impl F32ElementOfVector<0, Simd> for f32 {}

impl_element_of_vector!(for N = 1: impl for f32);
impl F32ElementOfVector<1, Simd> for f32 {}

// TODO: Determine if FVec2 benefits from alignment.
impl_element_of_vector!(for N = 2: impl for f32);
impl F32ElementOfVector<2, Simd> for f32 {}

// TODO: Determine if FVec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for f32);
impl F32ElementOfVector<5, Simd> for f32 {}

// TODO: Determine if FVec6 benefits from SIMD.
impl_element_of_vector!(for N = 6: impl for f32);
impl F32ElementOfVector<6, Simd> for f32 {}

// TODO: Determine if FVec7 benefits from SIMD.
impl_element_of_vector!(for N = 7: impl for f32);
impl F32ElementOfVector<7, Simd> for f32 {}

// TODO: Determine if FVec8 benefits from SIMD.
impl_element_of_vector!(for N = 8: impl for f32);
impl F32ElementOfVector<8, Simd> for f32 {}
