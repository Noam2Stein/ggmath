use super::*;

inner_vectors!(i16(2));

impl Scalar for i16 {}

impl ScalarAbsDiff for i16 {}
impl AbsDiff for i16 {
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

impl ScalarRound for i16 {
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

impl ScalarNum for i16 {}

impl ScalarSigned for i16 {
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
