pub trait Builder<O> {
    fn build(self) -> O;
}

pub trait FromBuilder<I: Builder<Self>>: Sized {
    #[inline(always)]
    fn from_builder(builder: I) -> Self {
        builder.build()
    }
}
impl<I: Builder<O>, O> FromBuilder<I> for O {}
