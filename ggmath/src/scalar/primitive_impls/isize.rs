use super::*;

#[cfg(target_pointer_width = "32")]
scalar_inner_vectors!(isize(4));

#[cfg(target_pointer_width = "64")]
scalar_inner_vectors!(isize(8));

impl Scalar for isize {}

impl ScalarSigned for isize {
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
