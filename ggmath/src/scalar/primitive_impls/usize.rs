use super::*;

#[cfg(target_pointer_width = "32")]
inner_vectors!(usize(4));

#[cfg(target_pointer_width = "64")]
inner_vectors!(usize(8));

impl Scalar for usize {}

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
