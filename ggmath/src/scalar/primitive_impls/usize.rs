use super::*;

#[cfg(target_pointer_width = "32")]
inner_vectors!(usize(4));

#[cfg(target_pointer_width = "64")]
inner_vectors!(usize(8));

impl Scalar for usize {}

impl ScalarDefault for usize {}
impl ScalarPartialEq for usize {}
impl ScalarPartialOrd for usize {}

impl ScalarAdd for usize {}
impl ScalarSub for usize {}
impl ScalarMul for usize {}
impl ScalarDiv for usize {}
impl ScalarRem for usize {}
impl ScalarAddAssign for usize {}
impl ScalarSubAssign for usize {}
impl ScalarMulAssign for usize {}
impl ScalarDivAssign for usize {}
impl ScalarRemAssign for usize {}

impl ScalarNot for usize {}
impl ScalarBitAnd for usize {}
impl ScalarBitOr for usize {}
impl ScalarBitXor for usize {}

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

impl ScalarAbsDiff for usize {}
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
impl ScalarDot for usize {}

impl ScalarRound for usize {
    #[inline(always)]
    fn ceil(self) -> Self {
        self
    }
    #[inline(always)]
    fn floor(self) -> Self {
        self
    }
    #[inline(always)]
    fn round(self) -> Self {
        self
    }
    fn trunc(self) -> Self {
        self
    }
}

impl ScalarNum for usize {}
