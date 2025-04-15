use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, u8, A>
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

    pub fn min(self, other: Vector<N, u8, impl VecAlignment>) -> Self {
        self.map_rhs(other, u8::min)
    }
    pub fn max(self, other: Vector<N, u8, impl VecAlignment>) -> Self {
        self.map_rhs(other, u8::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, u8, impl VecAlignment>,
        max: Vector<N, u8, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    #[cfg(not(feature = "newnum"))]
    pub fn cmin(self) -> u8 {
        self.fold(u8::min)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn cmax(self) -> u8 {
        self.fold(u8::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, u8, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
