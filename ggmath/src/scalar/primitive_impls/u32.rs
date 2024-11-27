use super::*;

inner_vectors!(u32(4));

impl Scalar for u32 {}

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
