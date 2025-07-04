use super::*;

primitive_aliases! { pub U16 => u16 }

impl Scalar for u16 {
    type Vec2Alignment = Align<4>;
    type Vec3Alignment = Align<8>;
    type Vec4Alignment = Align<8>;
}

impl<const N: usize, A: VecAlignment> Vector<N, u16, A>
where
    MaybeVecLen<N>: VecLen,
{
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);

    pub const MIN: Self = Self::splat(u16::MIN);
    pub const MAX: Self = Self::splat(u16::MAX);

    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0)
    }

    pub fn signumt(self) -> Self {
        self.map(|x| if x > 0 { 1 } else { 0 })
    }

    pub fn min(self, other: Vector<N, u16, impl VecAlignment>) -> Self {
        self.map_rhs(other, u16::min)
    }
    pub fn max(self, other: Vector<N, u16, impl VecAlignment>) -> Self {
        self.map_rhs(other, u16::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, u16, impl VecAlignment>,
        max: Vector<N, u16, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    pub fn cmin(self) -> u16 {
        self.fold(u16::min)
    }
    pub fn cmax(self) -> u16 {
        self.fold(u16::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, u16, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
