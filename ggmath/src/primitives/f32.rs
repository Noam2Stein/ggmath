use super::*;

primitive_aliases! { pub F => f32 }

impl Scalar for f32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;
}

impl<const N: usize, A: VecAlignment> Vector<N, f32, A>
where
    MaybeVecLen<N>: VecLen,
{
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);

    pub const NAN: Self = Self::splat(f32::NAN);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    pub const MIN: Self = Self::splat(f32::MIN);
    pub const MAX: Self = Self::splat(f32::MAX);

    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0.0)
    }
    pub fn is_negative(&self) -> Vector<N, bool, A> {
        self.map(|x| x < 0.0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0.0)
    }

    pub fn is_bin_positive(&self) -> Vector<N, bool, A> {
        self.map(f32::is_sign_positive)
    }
    pub fn is_bin_negative(&self) -> Vector<N, bool, A> {
        self.map(f32::is_sign_negative)
    }

    pub fn abs(self) -> Self {
        self.map(f32::abs)
    }
    pub fn neg_abs(self) -> Self {
        self.map(|x| -x.abs())
    }

    pub fn signumt(self) -> Self {
        self.map(|x| {
            if x > 0.0 {
                1.0
            } else if x < 0.0 {
                -1.0
            } else {
                0.0
            }
        })
    }
    pub fn bin_signum(self) -> Self {
        self.map(f32::signum)
    }

    pub fn abs_diff(self, rhs: Vector<N, f32, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }

    pub fn min(self, other: Vector<N, f32, impl VecAlignment>) -> Self {
        self.map_rhs(other, |a, b| if a < b { a } else { b })
    }
    pub fn max(self, other: Vector<N, f32, impl VecAlignment>) -> Self {
        self.map_rhs(other, |a, b| if a > b { a } else { b })
    }
    pub fn clamp(
        self,
        min: Vector<N, f32, impl VecAlignment>,
        max: Vector<N, f32, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    pub fn round(self) -> Self {
        self.map(f32::round)
    }
    pub fn floor(self) -> Self {
        self.map(f32::floor)
    }
    pub fn ceil(self) -> Self {
        self.map(f32::ceil)
    }
    pub fn trunc(self) -> Self {
        self.map(f32::trunc)
    }
    pub fn atrunc(self) -> Self {
        self.map(|x| {
            if x.is_sign_positive() {
                x.ceil()
            } else {
                x.floor()
            }
        })
    }

    pub fn sin(self) -> Self {
        self.map(f32::sin)
    }
    pub fn cos(self) -> Self {
        self.map(f32::cos)
    }
    pub fn tan(self) -> Self {
        self.map(f32::tan)
    }
    pub fn asin(self) -> Self {
        self.map(f32::asin)
    }
    pub fn acos(self) -> Self {
        self.map(f32::acos)
    }
    pub fn atan(self) -> Self {
        self.map(f32::atan)
    }
    pub fn sinh(self) -> Self {
        self.map(f32::sinh)
    }
    pub fn cosh(self) -> Self {
        self.map(f32::cosh)
    }
    pub fn tanh(self) -> Self {
        self.map(f32::tanh)
    }
    pub fn asinh(self) -> Self {
        self.map(f32::asinh)
    }
    pub fn acosh(self) -> Self {
        self.map(f32::acosh)
    }
    pub fn atanh(self) -> Self {
        self.map(f32::atanh)
    }
}
