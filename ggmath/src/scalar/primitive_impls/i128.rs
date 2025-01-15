use super::*;

scalar_inner_vectors!(i128(16));

impl Scalar for i128 {}

impl ScalarSigned for i128 {
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
