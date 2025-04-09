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

    pub fn is_sign_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x.is_sign_positive())
    }
    pub fn is_sign_negative(&self) -> Vector<N, bool, A> {
        self.map(|x| x.is_sign_negative())
    }

    pub fn abs(self) -> Self {
        self.map(|x| x.abs())
    }
    pub fn neg_abs(self) -> Self {
        self.map(|x| -x.abs())
    }

    pub fn signum(self) -> Self {
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
    pub fn signumf(self) -> Self {
        self.map(|x| x.signum())
    }

    pub fn round(self) -> Self {
        self.map(|x| x.round())
    }
    pub fn floor(self) -> Self {
        self.map(|x| x.floor())
    }
    pub fn ceil(self) -> Self {
        self.map(|x| x.ceil())
    }
    pub fn trunc(self) -> Self {
        self.map(|x| x.trunc())
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
        self.map(|x| x.sin())
    }
    pub fn cos(self) -> Self {
        self.map(|x| x.cos())
    }
    pub fn tan(self) -> Self {
        self.map(|x| x.tan())
    }
    pub fn asin(self) -> Self {
        self.map(|x| x.asin())
    }
    pub fn acos(self) -> Self {
        self.map(|x| x.acos())
    }
    pub fn atan(self) -> Self {
        self.map(|x| x.atan())
    }
    pub fn sinh(self) -> Self {
        self.map(|x| x.sinh())
    }
    pub fn cosh(self) -> Self {
        self.map(|x| x.cosh())
    }
    pub fn tanh(self) -> Self {
        self.map(|x| x.tanh())
    }
    pub fn asinh(self) -> Self {
        self.map(|x| x.asinh())
    }
    pub fn acosh(self) -> Self {
        self.map(|x| x.acosh())
    }
    pub fn atanh(self) -> Self {
        self.map(|x| x.atanh())
    }
}
