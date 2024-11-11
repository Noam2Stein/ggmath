use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(f64(8));

impl Scalar for f64 {}

impl ScalarNeg for f64 {}
impl ScalarAdd<f64> for f64 {}
impl ScalarSub<f64> for f64 {}
impl ScalarMul<f64> for f64 {}
impl ScalarDiv<f64> for f64 {}
impl ScalarRem<f64> for f64 {}
impl ScalarAddAssign<f64> for f64 {}
impl ScalarSubAssign<f64> for f64 {}
impl ScalarMulAssign<f64> for f64 {}
impl ScalarDivAssign<f64> for f64 {}
impl ScalarRemAssign<f64> for f64 {}
