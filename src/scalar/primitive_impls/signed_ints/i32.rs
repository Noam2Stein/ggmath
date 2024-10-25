use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::Scalar};

inner_vecs!(i32(4));

impl Scalar for i32 {}
