use super::*;

impl<const N: usize, T: Scalar + PartialOrd, A: VecAlignment> MinMax for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        self.clamp(min, max)
    }
}
