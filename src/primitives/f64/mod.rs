use crate::{F64ElementOfVector, Simd, impl_element_of_vector};

impl_element_of_vector!(for N = 0: impl for f64);
impl F64ElementOfVector<0, Simd> for f64 {}

impl_element_of_vector!(for N = 1: impl for f64);
impl F64ElementOfVector<1, Simd> for f64 {}

// TODO: Add SIMD optimizations to DVec2.
impl_element_of_vector!(for N = 2: impl for f64);
impl F64ElementOfVector<2, Simd> for f64 {}

// TODO: Add SIMD optimizations to DVec3.
impl_element_of_vector!(for N = 3: impl for f64);
impl F64ElementOfVector<3, Simd> for f64 {}

// TODO: Add SIMD optimizations to DVec4.
impl_element_of_vector!(for N = 4: impl for f64);
impl F64ElementOfVector<4, Simd> for f64 {}

// TODO: Determine if DVec5 benefits from SIMD.
impl_element_of_vector!(for N = 5: impl for f64);
impl F64ElementOfVector<5, Simd> for f64 {}

// TODO: Determine if DVec6 benefits from SIMD.
impl_element_of_vector!(for N = 6: impl for f64);
impl F64ElementOfVector<6, Simd> for f64 {}

// TODO: Determine if DVec7 benefits from SIMD.
impl_element_of_vector!(for N = 7: impl for f64);
impl F64ElementOfVector<7, Simd> for f64 {}

// TODO: Determine if DVec8 benefits from SIMD.
impl_element_of_vector!(for N = 8: impl for f64);
impl F64ElementOfVector<8, Simd> for f64 {}
