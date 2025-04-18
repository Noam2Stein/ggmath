use super::*;

impl<const N: usize, A: VecAlignment> Vector<N, u8, A>
where
    ScalarCount<N>: VecLen,
{
    #[cfg(not(feature = "newnum"))]
    pub fn zero() -> Self {
        Self::splat(0)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn one() -> Self {
        Self::splat(1)
    }

    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_right")]
    pub fn right() -> Self {
        Self::zero().with_x(1)
    }
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_left")]
    pub fn left() -> Self {
        Self::zero().with_x(1)
    }
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_up")]
    pub fn up() -> Self {
        Self::zero().with_y(1)
    }
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_down")]
    pub fn down() -> Self {
        Self::zero().with_y(1)
    }

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

    #[cfg(not(feature = "newnum"))]
    pub fn cmin(self) -> u8 {
        self.fold(u8::min)
    }
    #[cfg(not(feature = "newnum"))]
    pub fn cmax(self) -> u8 {
        self.fold(u8::max)
    }

    pub fn abs_diff(self, rhs: Vector<N, u8, impl VecAlignment>) -> Self {
        self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
    }
}

#[cfg(any(feature = "positive_forward", feature = "positive_backward"))]
impl<A: VecAlignment> Vector<3, u8, A> {
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_forward")]
    pub fn forward() -> Self {
        Self::zero().with_z(1)
    }
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_backward")]
    pub fn backward() -> Self {
        Self::zero().with_z(1)
    }
}

#[cfg(any(feature = "positive_forward", feature = "positive_backward"))]
impl<A: VecAlignment> Vector<4, u8, A> {
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_forward")]
    pub fn forward() -> Self {
        Self::zero().with_z(1)
    }
    #[cfg(not(feature = "newnum"))]
    #[cfg(feature = "positive_backward")]
    pub fn backward() -> Self {
        Self::zero().with_z(1)
    }
}
