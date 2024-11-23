use super::*;

#[cfg(target_pointer_width = "32")]
inner_vectors!(usize(4));

#[cfg(target_pointer_width = "64")]
inner_vectors!(usize(8));

impl Scalar for usize {}

impl ScalarDefault for usize {}
impl ScalarPartialEq<usize> for usize {}
impl ScalarPartialOrd for usize {}

impl ScalarAdd<usize> for usize {}
impl ScalarSub<usize> for usize {}
impl ScalarMul<usize> for usize {}
impl ScalarDiv<usize> for usize {}
impl ScalarRem<usize> for usize {}
impl ScalarAddAssign<usize> for usize {}
impl ScalarSubAssign<usize> for usize {}
impl ScalarMulAssign<usize> for usize {}
impl ScalarDivAssign<usize> for usize {}
impl ScalarRemAssign<usize> for usize {}

impl ScalarNot for usize {}
impl ScalarBitAnd<usize> for usize {}
impl ScalarBitOr<usize> for usize {}
impl ScalarBitXor<usize> for usize {}

macro_rules! impl_sh {
    ($($rhs:ident)*) => {
        $(
            impl ScalarShl<$rhs> for usize {}
            impl ScalarShr<$rhs> for usize {}
            impl ScalarShlAssign<$rhs> for usize {}
            impl ScalarShrAssign<$rhs> for usize {}
        )*
    };
}
impl_sh!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

impl ScalarAbsDiff<usize> for usize {}
impl AbsDiff for usize {
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

impl ScalarCSum for usize {}

impl ScalarNum for usize {}
