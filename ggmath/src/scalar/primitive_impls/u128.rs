use super::*;

scalar_inner_vectors!(u128(16));

impl Scalar for u128 {}

impl ScalarRound for u128 {
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

impl ScalarNum for u128 {}
