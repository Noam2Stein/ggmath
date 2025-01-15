use super::*;

scalar_inner_vectors!(f64(8));

impl Scalar for f64 {}

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
