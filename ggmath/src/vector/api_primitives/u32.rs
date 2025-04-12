use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, u32, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0)
    }

    pub fn signum(self) -> Self {
        self.map(|x| if x > 0 { 1 } else { 0 })
    }

    pub fn min(self, other: Vector<N, u32, impl VecAlignment>) -> Self {
        self.map_rhs(other, |a, b| a.min(b))
    }
    pub fn max(self, other: Vector<N, u32, impl VecAlignment>) -> Self {
        self.map_rhs(other, |a, b| a.max(b))
    }
    pub fn clamp(
        self,
        min: Vector<N, u32, impl VecAlignment>,
        max: Vector<N, u32, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    #[cfg(not(feature = "newnum"))]
    pub fn cmin(self) -> u32 {
        self.fold(u32::min)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn cmax(self) -> u32 {
        self.fold(u32::max)
    }
}
