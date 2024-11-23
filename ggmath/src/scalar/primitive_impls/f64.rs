use super::*;

inner_vectors!(f64(8));

impl Scalar for f64 {}

impl ScalarDefault for f64 {}
impl ScalarPartialEq<f64> for f64 {}
impl ScalarPartialOrd for f64 {}

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

impl ScalarAbsDiff<f64> for f64 {}
impl AbsDiff for f64 {
    type Output = Self;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Self) -> Self::Output {
        (self - rhs).abs()
    }
}

impl ScalarCSum for f64 {}

impl ScalarNum for f64 {}
