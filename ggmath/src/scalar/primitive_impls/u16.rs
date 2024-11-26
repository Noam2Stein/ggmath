use super::*;

inner_vectors!(u16(2));

impl Scalar for u16 {}

impl ScalarPartialEq for u16 {}
impl ScalarPartialOrd for u16 {}

impl ScalarAbsDiff for u16 {}
impl AbsDiff for u16 {
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

impl ScalarRound for u16 {
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

impl ScalarNum for u16 {}
