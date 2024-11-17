pub trait AbsDiff<Rhs = Self> {
    type Output;

    fn abs_diff(&self, rhs: &Rhs) -> Self::Output;
}
