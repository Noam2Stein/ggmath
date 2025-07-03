use super::*;

primitive_aliases! { pub I8 => i8 }

impl Scalar for i8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;
}

impl<const N: usize, A: VecAlignment> Vector<N, i8, A>
where
    MaybeVecLen<N>: VecLen,
{
    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const NEG_ONE: Self = Self::splat(-1);

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

    pub fn min(self, other: Vector<N, i8, impl VecAlignment>) -> Self {
        self.map_rhs(other, i8::min)
    }
    pub fn max(self, other: Vector<N, i8, impl VecAlignment>) -> Self {
        self.map_rhs(other, i8::max)
    }
    pub fn clamp(
        self,
        min: Vector<N, i8, impl VecAlignment>,
        max: Vector<N, i8, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    pub fn cmin(self) -> i8 {
        self.fold(i8::min)
    }
    pub fn cmax(self) -> i8 {
        self.fold(i8::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, i8, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}
