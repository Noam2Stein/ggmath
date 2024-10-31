use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::Scalar};

#[cfg(target_pointer_width = "32")]
inner_vecs!(isize(4));

#[cfg(target_pointer_width = "64")]
inner_vecs!(isize(8));

impl Scalar for isize {}