use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, usize, A>
where
    ScalarCount<N>: VecLen,
{
    #[cfg(not(feature = "newnum"))]
    pub fn zero() -> Self {
        Self::splat(0)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn one() -> Self {
        Self::splat(1)
    }

    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0)
    }

    pub fn signumt(self) -> Self {
        self.map(|x| if x > 0 { 1 } else { 0 })
    }

    pub fn min(self, other: Vector<N, usize, impl VecAlignment>) -> Self {
        self.map_rhs(other, usize::min)
    }
    pub fn max(self, other: Vector<N, usize, impl VecAlignment>) -> Self {
        self.map_rhs(other, usize::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, usize, impl VecAlignment>,
        max: Vector<N, usize, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    #[cfg(not(feature = "newnum"))]
    pub fn cmin(self) -> usize {
        self.fold(usize::min)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn cmax(self) -> usize {
        self.fold(usize::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, usize, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
