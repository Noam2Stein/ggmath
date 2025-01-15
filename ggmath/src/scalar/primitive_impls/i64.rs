use super::*;

scalar_inner_vectors!(i64(8));

impl Scalar for i64 {}

impl ScalarSigned for i64 {
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
