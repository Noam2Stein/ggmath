pub trait Builder<O> {
    fn build(self) -> O;
}
impl<T> Builder<T> for T {
    #[inline(always)]
    fn build(self) -> T {
        self
    }
}

pub trait FromBuilder<I: Builder<Self>>: Sized {
    #[inline(always)]
    fn from_builder(builder: I) -> Self {
        builder.build()
    }
}
impl<I: Builder<O>, O> FromBuilder<I> for O {}
