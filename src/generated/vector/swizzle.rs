use crate::vector::{Scalar, VecAlignment, Vector};

impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a new vector with the `x` and `x` (1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `x` and `y` (1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `y` and `x` (2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `y` and `y` (2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `x` and `x` (1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `x` and `y` (1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `y` and `x` (1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `y` and `y` (1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `x` and `x` (2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `x` and `y` (2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `y` and `x` (2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `y` and `y` (2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `x` (1st, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `y` (1st, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `x` (1st, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `y` (1st, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `x` (1st, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `y` (1st, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `x` (1st, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `y` (1st, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `x` (2nd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `y` (2nd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `x` (2nd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `y` (2nd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `x` (2nd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `y` (2nd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `x` (2nd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `y` (2nd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a new vector with the `x` and `x` (1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `x` and `y` (1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `x` and `z` (1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `y` and `x` (2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `y` and `y` (2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `y` and `z` (2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `z` and `x` (3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `z` and `y` (3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `z` and `z` (3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `x` and `x` (1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `x` and `y` (1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `x` and `z` (1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `y` and `x` (1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `y` and `y` (1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `y` and `z` (1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `z` and `x` (1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `z` and `y` (1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `z` and `z` (1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `x` and `x` (2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `x` and `y` (2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `x` and `z` (2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `y` and `x` (2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `y` and `y` (2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `y` and `z` (2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `z` and `x` (2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `z` and `y` (2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `z` and `z` (2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `x` and `x` (3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `x` and `y` (3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `x` and `z` (3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `y` and `x` (3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `y` and `y` (3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `y` and `z` (3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `z` and `x` (3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `z` and `y` (3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `z` and `z` (3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `x` (1st, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `y` (1st, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `z` (1st, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `x` (1st, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `y` (1st, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `z` (1st, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `x` (1st, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `y` (1st, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `z` (1st, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `x` (1st, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `y` (1st, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `z` (1st, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `x` (1st, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `y` (1st, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `z` (1st, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `x` (1st, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `y` (1st, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `z` (1st, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `x` (1st, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `y` (1st, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `z` (1st, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `x` (1st, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `y` (1st, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `z` (1st, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `x` (1st, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `y` (1st, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `z` (1st, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `x` (2nd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `y` (2nd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `z` (2nd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `x` (2nd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `y` (2nd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `z` (2nd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `x` (2nd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `y` (2nd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `z` (2nd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `x` (2nd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `y` (2nd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `z` (2nd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `x` (2nd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `y` (2nd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `z` (2nd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `x` (2nd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `y` (2nd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `z` (2nd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `x` (2nd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `y` (2nd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `z` (2nd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `x` (2nd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `y` (2nd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `z` (2nd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `x` (2nd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `y` (2nd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `z` (2nd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `x` (3rd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `y` (3rd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `z` (3rd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `x` (3rd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `y` (3rd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `z` (3rd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `x` (3rd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `y` (3rd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `z` (3rd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `x` (3rd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `y` (3rd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `z` (3rd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `x` (3rd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `y` (3rd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `z` (3rd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `x` (3rd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `y` (3rd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `z` (3rd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `x` (3rd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `y` (3rd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `z` (3rd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `x` (3rd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `y` (3rd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `z` (3rd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `x` (3rd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `y` (3rd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `z` (3rd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }
}

impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a new vector with the `x` and `x` (1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `x` and `y` (1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `x` and `z` (1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `x` and `w` (1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xw(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[3]])
    }

    /// Returns a new vector with the `y` and `x` (2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `y` and `y` (2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `y` and `z` (2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `y` and `w` (2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yw(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[3]])
    }

    /// Returns a new vector with the `z` and `x` (3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `z` and `y` (3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `z` and `z` (3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `z` and `w` (3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zw(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[3]])
    }

    /// Returns a new vector with the `w` and `x` (4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wx(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[0]])
    }

    /// Returns a new vector with the `w` and `y` (4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wy(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[1]])
    }

    /// Returns a new vector with the `w` and `z` (4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wz(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[2]])
    }

    /// Returns a new vector with the `w` and `w` (4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ww(self) -> Vector<2, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[3]])
    }

    /// Returns a new vector with the `x`, `x` and `x` (1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `x` and `y` (1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `x` and `z` (1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `x` and `w` (1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[0], self.as_array()[3]])
    }

    /// Returns a new vector with the `x`, `y` and `x` (1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `y` and `y` (1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `y` and `z` (1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `y` and `w` (1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[1], self.as_array()[3]])
    }

    /// Returns a new vector with the `x`, `z` and `x` (1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `z` and `y` (1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `z` and `z` (1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `z` and `w` (1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[2], self.as_array()[3]])
    }

    /// Returns a new vector with the `x`, `w` and `x` (1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[3], self.as_array()[0]])
    }

    /// Returns a new vector with the `x`, `w` and `y` (1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[3], self.as_array()[1]])
    }

    /// Returns a new vector with the `x`, `w` and `z` (1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[3], self.as_array()[2]])
    }

    /// Returns a new vector with the `x`, `w` and `w` (1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xww(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[0], self.as_array()[3], self.as_array()[3]])
    }

    /// Returns a new vector with the `y`, `x` and `x` (2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `x` and `y` (2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `x` and `z` (2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `x` and `w` (2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[0], self.as_array()[3]])
    }

    /// Returns a new vector with the `y`, `y` and `x` (2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `y` and `y` (2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `y` and `z` (2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `y` and `w` (2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[1], self.as_array()[3]])
    }

    /// Returns a new vector with the `y`, `z` and `x` (2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `z` and `y` (2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `z` and `z` (2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `z` and `w` (2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[2], self.as_array()[3]])
    }

    /// Returns a new vector with the `y`, `w` and `x` (2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[3], self.as_array()[0]])
    }

    /// Returns a new vector with the `y`, `w` and `y` (2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[3], self.as_array()[1]])
    }

    /// Returns a new vector with the `y`, `w` and `z` (2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[3], self.as_array()[2]])
    }

    /// Returns a new vector with the `y`, `w` and `w` (2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yww(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[1], self.as_array()[3], self.as_array()[3]])
    }

    /// Returns a new vector with the `z`, `x` and `x` (3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `x` and `y` (3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `x` and `z` (3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `x` and `w` (3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[0], self.as_array()[3]])
    }

    /// Returns a new vector with the `z`, `y` and `x` (3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `y` and `y` (3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `y` and `z` (3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `y` and `w` (3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[1], self.as_array()[3]])
    }

    /// Returns a new vector with the `z`, `z` and `x` (3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `z` and `y` (3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `z` and `z` (3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `z` and `w` (3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[2], self.as_array()[3]])
    }

    /// Returns a new vector with the `z`, `w` and `x` (3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[3], self.as_array()[0]])
    }

    /// Returns a new vector with the `z`, `w` and `y` (3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[3], self.as_array()[1]])
    }

    /// Returns a new vector with the `z`, `w` and `z` (3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[3], self.as_array()[2]])
    }

    /// Returns a new vector with the `z`, `w` and `w` (3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zww(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[2], self.as_array()[3], self.as_array()[3]])
    }

    /// Returns a new vector with the `w`, `x` and `x` (4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[0], self.as_array()[0]])
    }

    /// Returns a new vector with the `w`, `x` and `y` (4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[0], self.as_array()[1]])
    }

    /// Returns a new vector with the `w`, `x` and `z` (4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[0], self.as_array()[2]])
    }

    /// Returns a new vector with the `w`, `x` and `w` (4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[0], self.as_array()[3]])
    }

    /// Returns a new vector with the `w`, `y` and `x` (4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[1], self.as_array()[0]])
    }

    /// Returns a new vector with the `w`, `y` and `y` (4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[1], self.as_array()[1]])
    }

    /// Returns a new vector with the `w`, `y` and `z` (4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[1], self.as_array()[2]])
    }

    /// Returns a new vector with the `w`, `y` and `w` (4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[1], self.as_array()[3]])
    }

    /// Returns a new vector with the `w`, `z` and `x` (4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[2], self.as_array()[0]])
    }

    /// Returns a new vector with the `w`, `z` and `y` (4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[2], self.as_array()[1]])
    }

    /// Returns a new vector with the `w`, `z` and `z` (4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[2], self.as_array()[2]])
    }

    /// Returns a new vector with the `w`, `z` and `w` (4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[2], self.as_array()[3]])
    }

    /// Returns a new vector with the `w`, `w` and `x` (4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwx(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[3], self.as_array()[0]])
    }

    /// Returns a new vector with the `w`, `w` and `y` (4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwy(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[3], self.as_array()[1]])
    }

    /// Returns a new vector with the `w`, `w` and `z` (4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwz(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[3], self.as_array()[2]])
    }

    /// Returns a new vector with the `w`, `w` and `w` (4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn www(self) -> Vector<3, T, A> {
        Vector::from_array([self.as_array()[3], self.as_array()[3], self.as_array()[3]])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `x` (1st, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `y` (1st, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `z` (1st, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `x` and `w` (1st, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `x` (1st, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `y` (1st, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `z` (1st, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `y` and `w` (1st, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `x` (1st, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `y` (1st, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `z` (1st, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `z` and `w` (1st, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `w` and `x` (1st, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `w` and `y` (1st, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `w` and `z` (1st, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `x`, `w` and `w` (1st, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `x` (1st, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `y` (1st, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `z` (1st, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `x` and `w` (1st, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `x` (1st, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `y` (1st, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `z` (1st, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `y` and `w` (1st, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `x` (1st, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `y` (1st, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `z` (1st, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `z` and `w` (1st, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `w` and `x` (1st, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xywx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `w` and `y` (1st, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xywy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `w` and `z` (1st, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xywz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `y`, `w` and `w` (1st, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `x` (1st, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `y` (1st, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `z` (1st, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `x` and `w` (1st, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `x` (1st, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `y` (1st, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `z` (1st, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `y` and `w` (1st, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `x` (1st, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `y` (1st, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `z` (1st, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `z` and `w` (1st, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `w` and `x` (1st, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `w` and `y` (1st, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `w` and `z` (1st, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `z`, `w` and `w` (1st, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `x` and `x` (1st, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `x` and `y` (1st, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `x` and `z` (1st, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `x` and `w` (1st, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `y` and `x` (1st, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `y` and `y` (1st, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `y` and `z` (1st, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `y` and `w` (1st, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `z` and `x` (1st, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `z` and `y` (1st, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `z` and `z` (1st, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `z` and `w` (1st, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `w` and `x` (1st, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `w` and `y` (1st, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `w` and `z` (1st, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `x`, `w`, `w` and `w` (1st, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `x` (2nd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `y` (2nd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `z` (2nd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `x` and `w` (2nd, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `x` (2nd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `y` (2nd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `z` (2nd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `y` and `w` (2nd, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `x` (2nd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `y` (2nd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `z` (2nd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `z` and `w` (2nd, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `w` and `x` (2nd, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `w` and `y` (2nd, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `w` and `z` (2nd, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `x`, `w` and `w` (2nd, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `x` (2nd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `y` (2nd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `z` (2nd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `x` and `w` (2nd, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `x` (2nd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `y` (2nd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `z` (2nd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `y` and `w` (2nd, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `x` (2nd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `y` (2nd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `z` (2nd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `z` and `w` (2nd, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `w` and `x` (2nd, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yywx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `w` and `y` (2nd, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yywy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `w` and `z` (2nd, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yywz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `y`, `w` and `w` (2nd, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `x` (2nd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `y` (2nd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `z` (2nd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `x` and `w` (2nd, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `x` (2nd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `y` (2nd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `z` (2nd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `y` and `w` (2nd, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `x` (2nd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `y` (2nd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `z` (2nd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `z` and `w` (2nd, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `w` and `x` (2nd, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `w` and `y` (2nd, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `w` and `z` (2nd, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `z`, `w` and `w` (2nd, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `x` and `x` (2nd, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `x` and `y` (2nd, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `x` and `z` (2nd, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `x` and `w` (2nd, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `y` and `x` (2nd, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `y` and `y` (2nd, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `y` and `z` (2nd, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `y` and `w` (2nd, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `z` and `x` (2nd, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `z` and `y` (2nd, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `z` and `z` (2nd, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `z` and `w` (2nd, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `w` and `x` (2nd, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `w` and `y` (2nd, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `w` and `z` (2nd, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `y`, `w`, `w` and `w` (2nd, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `x` (3rd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `y` (3rd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `z` (3rd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `x` and `w` (3rd, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `x` (3rd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `y` (3rd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `z` (3rd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `y` and `w` (3rd, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `x` (3rd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `y` (3rd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `z` (3rd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `z` and `w` (3rd, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `w` and `x` (3rd, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `w` and `y` (3rd, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `w` and `z` (3rd, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `x`, `w` and `w` (3rd, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `x` (3rd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `y` (3rd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `z` (3rd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `x` and `w` (3rd, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `x` (3rd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `y` (3rd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `z` (3rd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `y` and `w` (3rd, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `x` (3rd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `y` (3rd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `z` (3rd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `z` and `w` (3rd, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `w` and `x` (3rd, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zywx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `w` and `y` (3rd, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zywy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `w` and `z` (3rd, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zywz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `y`, `w` and `w` (3rd, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `x` (3rd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `y` (3rd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `z` (3rd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `x` and `w` (3rd, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `x` (3rd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `y` (3rd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `z` (3rd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `y` and `w` (3rd, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `x` (3rd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `y` (3rd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `z` (3rd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `z` and `w` (3rd, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `w` and `x` (3rd, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `w` and `y` (3rd, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `w` and `z` (3rd, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `z`, `w` and `w` (3rd, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `x` and `x` (3rd, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `x` and `y` (3rd, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `x` and `z` (3rd, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `x` and `w` (3rd, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `y` and `x` (3rd, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `y` and `y` (3rd, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `y` and `z` (3rd, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `y` and `w` (3rd, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `z` and `x` (3rd, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `z` and `y` (3rd, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `z` and `z` (3rd, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `z` and `w` (3rd, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `w` and `x` (3rd, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `w` and `y` (3rd, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `w` and `z` (3rd, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `z`, `w`, `w` and `w` (3rd, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `x` and `x` (4th, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `x` and `y` (4th, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `x` and `z` (4th, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `x` and `w` (4th, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `y` and `x` (4th, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `y` and `y` (4th, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `y` and `z` (4th, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `y` and `w` (4th, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `z` and `x` (4th, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `z` and `y` (4th, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `z` and `z` (4th, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `z` and `w` (4th, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `w` and `x` (4th, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `w` and `y` (4th, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `w` and `z` (4th, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `x`, `w` and `w` (4th, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `x` and `x` (4th, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `x` and `y` (4th, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `x` and `z` (4th, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `x` and `w` (4th, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `y` and `x` (4th, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `y` and `y` (4th, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `y` and `z` (4th, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `y` and `w` (4th, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `z` and `x` (4th, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `z` and `y` (4th, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `z` and `z` (4th, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `z` and `w` (4th, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `w` and `x` (4th, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wywx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `w` and `y` (4th, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wywy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `w` and `z` (4th, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wywz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `y`, `w` and `w` (4th, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `x` and `x` (4th, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `x` and `y` (4th, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `x` and `z` (4th, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `x` and `w` (4th, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `y` and `x` (4th, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `y` and `y` (4th, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `y` and `z` (4th, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `y` and `w` (4th, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `z` and `x` (4th, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `z` and `y` (4th, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `z` and `z` (4th, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `z` and `w` (4th, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `w` and `x` (4th, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `w` and `y` (4th, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `w` and `z` (4th, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `z`, `w` and `w` (4th, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `x` and `x` (4th, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwxx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `x` and `y` (4th, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwxy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `x` and `z` (4th, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwxz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `x` and `w` (4th, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwxw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `y` and `x` (4th, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwyx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `y` and `y` (4th, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwyy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `y` and `z` (4th, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwyz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `y` and `w` (4th, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwyw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `z` and `x` (4th, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwzx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `z` and `y` (4th, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwzy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `z` and `z` (4th, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwzz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `z` and `w` (4th, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwzw(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
            self.as_array()[3],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `w` and `x` (4th, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwwx(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[0],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `w` and `y` (4th, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwwy(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[1],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `w` and `z` (4th, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwwz(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[2],
        ])
    }

    /// Returns a new vector with the `w`, `w`, `w` and `w` (4th, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwww(self) -> Vector<4, T, A> {
        Vector::from_array([
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
            self.as_array()[3],
        ])
    }
}
