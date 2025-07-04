use super::*;

primitive_aliases! { pub U8 => u8 }

impl Scalar for u8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;
}

impl<const N: usize, A: VecAlignment> Vector<N, u8, A>
where
    MaybeVecLen<N>: VecLen,
{
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);

    pub const MIN: Self = Self::splat(u8::MIN);
    pub const MAX: Self = Self::splat(u8::MAX);

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

    pub fn cmin(self) -> u8 {
        self.fold(u8::min)
    }
    pub fn cmax(self) -> u8 {
        self.fold(u8::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, u8, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
