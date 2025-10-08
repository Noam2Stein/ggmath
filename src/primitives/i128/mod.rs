use crate::impl_element_of_vector;

// there are currently no SIMD instructions for i128 arithmetic.
impl_element_of_vector!(impl for i128);
