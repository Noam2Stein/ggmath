use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(bool(1));

impl Scalar for bool {}

impl ScalarNot for bool {}
impl ScalarBitAnd<bool> for bool {}
impl ScalarBitOr<bool> for bool {}
impl ScalarBitXor<bool> for bool {}
