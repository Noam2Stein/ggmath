use super::*;

#[cfg(target_pointer_width = "32")]
inner_vectors!(isize(4));

#[cfg(target_pointer_width = "64")]
inner_vectors!(isize(8));

impl Scalar for isize {}

impl ScalarDefault for isize {}
impl ScalarPartialEq<isize> for isize {}
impl ScalarPartialOrd for isize {}

impl ScalarNeg for isize {}
impl ScalarAdd<isize> for isize {}
impl ScalarSub<isize> for isize {}
impl ScalarMul<isize> for isize {}
impl ScalarDiv<isize> for isize {}
impl ScalarRem<isize> for isize {}
impl ScalarAddAssign<isize> for isize {}
impl ScalarSubAssign<isize> for isize {}
impl ScalarMulAssign<isize> for isize {}
impl ScalarDivAssign<isize> for isize {}
impl ScalarRemAssign<isize> for isize {}

impl ScalarNot for isize {}
impl ScalarBitAnd<isize> for isize {}
impl ScalarBitOr<isize> for isize {}
impl ScalarBitXor<isize> for isize {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for isize {}
            impl ScalarShr<$rhs> for isize {}
            impl ScalarShlAssign<$rhs> for isize {}
            impl ScalarShrAssign<$rhs> for isize {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<isize> for isize {}
impl AbsDiff for isize {
    type Output = Self;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Self) -> Self::Output {
        if self > rhs {
            self - rhs
        } else {
            rhs - self
        }
    }
}

impl ScalarCSum for isize {}
impl ScalarDot<isize> for isize {}

impl ScalarNum for isize {}

impl ScalarSigned for isize {
    fn abs(self) -> Self {
        self.abs()
    }
    fn is_negative(self) -> bool {
        self.is_negative()
    }
    fn is_positive(self) -> bool {
        self.is_positive()
    }
    fn signum(self) -> Self {
        self.signum()
    }
}
