use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, f64, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[cfg(not(feature = "newnum"))]
    pub fn zero() -> Self {
        Self::splat(0.0)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn one() -> Self {
        Self::splat(1.0)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn neg_one() -> Self {
        Self::splat(-1.0)
    }

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
        self.map(f64::is_sign_positive)
    }
    pub fn is_bin_negative(&self) -> Vector<N, bool, A> {
        self.map(f64::is_sign_negative)
    }

    pub fn abs(self) -> Self {
        self.map(f64::abs)
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
        self.map(f64::signum)
    }

    pub fn abs_diff(self, rhs: Vector<N, f64, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }

    pub fn min(self, other: Vector<N, f64, impl VecAlignment>) -> Self {
        self.map_rhs(other, |a, b| if a < b { a } else { b })
    }
    pub fn max(self, other: Vector<N, f64, impl VecAlignment>) -> Self {
        self.map_rhs(other, |a, b| if a > b { a } else { b })
    }
    pub fn clamp(
        self,
        min: Vector<N, f64, impl VecAlignment>,
        max: Vector<N, f64, impl VecAlignment>,
    ) -> Self {
        self.min(max).max(min)
    }

    pub fn round(self) -> Self {
        self.map(f64::round)
    }
    pub fn floor(self) -> Self {
        self.map(f64::floor)
    }
    pub fn ceil(self) -> Self {
        self.map(f64::ceil)
    }
    pub fn trunc(self) -> Self {
        self.map(f64::trunc)
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
        self.map(f64::sin)
    }
    pub fn cos(self) -> Self {
        self.map(f64::cos)
    }
    pub fn tan(self) -> Self {
        self.map(f64::tan)
    }
    pub fn asin(self) -> Self {
        self.map(f64::asin)
    }
    pub fn acos(self) -> Self {
        self.map(f64::acos)
    }
    pub fn atan(self) -> Self {
        self.map(f64::atan)
    }
    pub fn sinh(self) -> Self {
        self.map(f64::sinh)
    }
    pub fn cosh(self) -> Self {
        self.map(f64::cosh)
    }
    pub fn tanh(self) -> Self {
        self.map(f64::tanh)
    }
    pub fn asinh(self) -> Self {
        self.map(f64::asinh)
    }
    pub fn acosh(self) -> Self {
        self.map(f64::acosh)
    }
    pub fn atanh(self) -> Self {
        self.map(f64::atanh)
    }
}
