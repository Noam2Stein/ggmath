use crate::impl_element_of_vector;

// i8 is only 8 bits, so none of its supported vectors (0..=8) benefit from SIMD.
impl_element_of_vector!(impl for i8);
