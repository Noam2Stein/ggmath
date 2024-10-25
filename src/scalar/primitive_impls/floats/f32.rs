use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::Scalar};

inner_vecs!(f32(4));

impl Scalar for f32 {}
