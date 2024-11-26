use super::*;

#[cfg(target_pointer_width = "32")]
inner_vectors!(usize(4));

#[cfg(target_pointer_width = "64")]
inner_vectors!(usize(8));

impl Scalar for usize {}

impl ScalarPartialEq for usize {}
impl ScalarPartialOrd for usize {}

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
