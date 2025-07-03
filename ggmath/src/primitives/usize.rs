use super::*;

primitive_aliases! { pub Usize => usize }

impl Scalar for usize {
    type Vec2Alignment = Align<{ size_of::<Self>() * 2 }>;
    type Vec3Alignment = Align<{ size_of::<Self>() * 4 }>;
    type Vec4Alignment = Align<{ size_of::<Self>() * 4 }>;
}

impl<const N: usize, A: VecAlignment> Vector<N, usize, A>
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

    pub fn cmin(self) -> usize {
        self.fold(usize::min)
    }
    pub fn cmax(self) -> usize {
        self.fold(usize::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, usize, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
