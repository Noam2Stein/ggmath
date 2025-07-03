use super::*;

primitive_aliases! { pub U128 => u128 }

impl Scalar for u128 {
    type Vec2Alignment = Align<32>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;
}

impl<const N: usize, A: VecAlignment> Vector<N, u128, A>
where
    MaybeVecLen<N>: VecLen,
{
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);

    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0)
    }

    pub fn signumt(self) -> Self {
        self.map(|x| if x > 0 { 1 } else { 0 })
    }

    pub fn min(self, other: Vector<N, u128, impl VecAlignment>) -> Self {
        self.map_rhs(other, u128::min)
    }
    pub fn max(self, other: Vector<N, u128, impl VecAlignment>) -> Self {
        self.map_rhs(other, u128::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, u128, impl VecAlignment>,
        max: Vector<N, u128, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    pub fn cmin(self) -> u128 {
        self.fold(u128::min)
    }
    pub fn cmax(self) -> u128 {
        self.fold(u128::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, u128, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
