use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, f32, A>
where
    ScalarCount<N>: VecLen,
{
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
