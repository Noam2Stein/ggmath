pub trait Builder {
    type Output;

    fn build(self) -> Self::Output;
}
