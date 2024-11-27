use super::*;

inner_vectors!(f64(8));

impl Scalar for f64 {}

impl ScalarRound for f64 {
    #[inline(always)]
    fn ceil(self) -> Self {
        self.ceil()
    }
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor()
    }
    #[inline(always)]
    fn round(self) -> Self {
        self.round()
    }
    fn trunc(self) -> Self {
        self.trunc()
    }
}

impl ScalarNum for f64 {}

impl ScalarSigned for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
    fn is_negative(self) -> bool {
        self.is_sign_negative()
    }
    fn is_positive(self) -> bool {
        self.is_sign_positive()
    }
    fn signum(self) -> Self {
        self.signum()
    }
}
