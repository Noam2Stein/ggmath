use super::*;

#[cfg(target_pointer_width = "32")]
scalar_inner_vectors!(isize(4));

#[cfg(target_pointer_width = "64")]
scalar_inner_vectors!(isize(8));

impl Scalar for isize {}
