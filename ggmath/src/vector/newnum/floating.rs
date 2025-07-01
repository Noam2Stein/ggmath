use super::*;

impl<const N: usize, T: Scalar + FloatingEquivalent<Floating: Scalar>, A: VecAlignment>
    FloatingEquivalent for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    type Floating = Vector<N, T::Floating, A>;

    fn float(self) -> Self::Floating {
        self.map(FloatingEquivalent::float)
    }
}
