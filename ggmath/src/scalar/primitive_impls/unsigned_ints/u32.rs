use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::Scalar};

inner_vecs!(u32(4));

impl Scalar for u32 {}
