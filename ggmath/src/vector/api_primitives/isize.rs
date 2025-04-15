use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, isize, A>
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
    #[cfg(not(feature = "newnum"))]
    pub fn neg_one() -> Self {
        Self::splat(-1)
    }

    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0)
    }
    pub fn is_negative(&self) -> Vector<N, bool, A> {
        self.map(|x| x < 0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0)
    }

    pub fn is_bin_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x >= 0)
    }
    pub fn is_bin_negative(&self) -> Vector<N, bool, A> {
        self.map(|x| x < 0)
    }

    pub fn abs(self) -> Self {
        self.map(|x| x.abs())
    }
    pub fn neg_abs(self) -> Self {
        self.map(|x| -x.abs())
    }

    pub fn signumt(self) -> Self {
        self.map(|x| x.signum())
    }
    pub fn bin_signum(self) -> Self {
        self.map(|x| if x >= 0 { 1 } else { -1 })
    }

    pub fn min(self, other: Vector<N, isize, impl VecAlignment>) -> Self {
        self.map_rhs(other, isize::min)
    }
    pub fn max(self, other: Vector<N, isize, impl VecAlignment>) -> Self {
        self.map_rhs(other, isize::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, isize, impl VecAlignment>,
        max: Vector<N, isize, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    #[cfg(not(feature = "newnum"))]
    pub fn cmin(self) -> isize {
        self.fold(isize::min)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn cmax(self) -> isize {
        self.fold(isize::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, isize, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
