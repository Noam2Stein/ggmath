use super::*;

inner_vectors!(u32(4));

impl Scalar for u32 {}

impl ScalarPartialEq for u32 {}
impl ScalarPartialOrd for u32 {}

impl ScalarAbsDiff for u32 {}
impl AbsDiff for u32 {
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

impl ScalarRound for u32 {
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

impl ScalarNum for u32 {}
