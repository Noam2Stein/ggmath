use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, i128, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn is_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x > 0)
    }
    pub fn is_negative(&self) -> Vector<N, bool, A> {
        self.map(|x| x < 0)
    }
    pub fn is_zero(&self) -> Vector<N, bool, A> {
        self.map(|x| x == 0)
    }

    pub fn is_sign_positive(&self) -> Vector<N, bool, A> {
        self.map(|x| x >= 0)
    }
    pub fn is_sign_negative(&self) -> Vector<N, bool, A> {
        self.map(|x| x < 0)
    }

    pub fn abs(self) -> Self {
        self.map(|x| x.abs())
    }
    pub fn neg_abs(self) -> Self {
        self.map(|x| -x.abs())
    }

    pub fn signum(self) -> Self {
        self.map(|x| x.signum())
    }
    pub fn signumf(self) -> Self {
        self.map(|x| if x >= 0 { 1 } else { -1 })
    }
}
