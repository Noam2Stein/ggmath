use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, i128, A>
where
    ScalarCount<N>: VecLen,
{
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

    pub fn min(self, other: Vector<N, i128, impl VecAlignment>) -> Self {
        self.map_rhs(other, i128::min)
    }
    pub fn max(self, other: Vector<N, i128, impl VecAlignment>) -> Self {
        self.map_rhs(other, i128::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, i128, impl VecAlignment>,
        max: Vector<N, i128, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    #[cfg(not(feature = "newnum"))]
    pub fn cmin(self) -> i128 {
        self.fold(i128::min)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn cmax(self) -> i128 {
        self.fold(i128::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, i128, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
