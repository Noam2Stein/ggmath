use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::Scalar};

inner_vecs!(bool(1));

impl Scalar for bool {}
