pub trait Signed {
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
    fn signum(self) -> Self;
    fn abs(self) -> Self;
}
