use ggmath_proc_macros::inner_vecs;

use crate::{self as ggmath, scalar::*};

inner_vecs!(f32(4));

impl Scalar for f32 {}

impl ScalarDefault for f32 {}

impl ScalarNeg for f32 {}
impl ScalarAdd<f32> for f32 {}
impl ScalarSub<f32> for f32 {}
impl ScalarMul<f32> for f32 {}
impl ScalarDiv<f32> for f32 {}
impl ScalarRem<f32> for f32 {}
impl ScalarAddAssign<f32> for f32 {}
impl ScalarSubAssign<f32> for f32 {}
impl ScalarMulAssign<f32> for f32 {}
impl ScalarDivAssign<f32> for f32 {}
impl ScalarRemAssign<f32> for f32 {}
