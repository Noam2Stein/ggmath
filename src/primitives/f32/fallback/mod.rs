use crate::{impl_element_of_vector, impl_float_element_of_vector};

impl_element_of_vector!(for N = 3: impl for f32);
impl_float_element_of_vector!(for N = 3: impl for f32);

impl_element_of_vector!(for N = 4: impl for f32);
impl_float_element_of_vector!(for N = 4: impl for f32);
