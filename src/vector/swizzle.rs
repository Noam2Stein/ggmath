use super::*;

impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x(self) -> T {
        self.index(0)
    }

    /// Returns the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y(self) -> T {
        self.index(1)
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a vector with the `x` and `x` (1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x` and `y` (1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y` and `x` (2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y` and `y` (2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(1)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a vector with the `x`, `x` and `x` (1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `x` and `y` (1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `y` and `x` (1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `y` and `y` (1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `x` and `x` (2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `x` and `y` (2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `y` and `x` (2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `y` and `y` (2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a vector with the `x`, `x`, `x` and `x` (1st, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `x` and `y` (1st, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `x` (1st, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `y` (1st, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `x` (1st, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `y` (1st, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `x` (1st, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `y` (1st, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `x` (2nd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `y` (2nd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `x` (2nd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `y` (2nd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `x` (2nd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `y` (2nd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `x` (2nd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `y` (2nd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(1)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x(self) -> T {
        self.index(0)
    }

    /// Returns the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y(self) -> T {
        self.index(1)
    }

    /// Returns the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn z(self) -> T {
        self.index(2)
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a vector with the `x` and `x` (1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x` and `y` (1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x` and `z` (1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y` and `x` (2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y` and `y` (2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y` and `z` (2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z` and `x` (3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z` and `y` (3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z` and `z` (3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(2)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a vector with the `x`, `x` and `x` (1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `x` and `y` (1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `x` and `z` (1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `y` and `x` (1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `y` and `y` (1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `y` and `z` (1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `z` and `x` (1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `z` and `y` (1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `z` and `z` (1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `x` and `x` (2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `x` and `y` (2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `x` and `z` (2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `y` and `x` (2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `y` and `y` (2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `y` and `z` (2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `z` and `x` (2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `z` and `y` (2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `z` and `z` (2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `x` and `x` (3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `x` and `y` (3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `x` and `z` (3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `y` and `x` (3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `y` and `y` (3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `y` and `z` (3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `z` and `x` (3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `z` and `y` (3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `z` and `z` (3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a vector with the `x`, `x`, `x` and `x` (1st, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `x` and `y` (1st, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `x` and `z` (1st, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `x` (1st, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `y` (1st, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `z` (1st, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `x` (1st, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `y` (1st, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `z` (1st, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `x` (1st, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `y` (1st, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `z` (1st, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `x` (1st, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `y` (1st, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `z` (1st, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `x` (1st, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `y` (1st, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `z` (1st, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `x` (1st, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `y` (1st, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `z` (1st, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `x` (1st, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `y` (1st, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `z` (1st, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `x` (1st, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `y` (1st, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `z` (1st, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `x` (2nd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `y` (2nd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `z` (2nd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `x` (2nd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `y` (2nd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `z` (2nd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `x` (2nd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `y` (2nd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `z` (2nd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `x` (2nd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `y` (2nd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `z` (2nd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `x` (2nd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `y` (2nd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `z` (2nd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `x` (2nd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `y` (2nd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `z` (2nd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `x` (2nd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `y` (2nd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `z` (2nd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `x` (2nd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `y` (2nd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `z` (2nd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `x` (2nd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `y` (2nd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `z` (2nd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `x` (3rd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `y` (3rd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `z` (3rd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `x` (3rd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `y` (3rd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `z` (3rd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `x` (3rd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `y` (3rd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `z` (3rd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `x` (3rd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `y` (3rd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `z` (3rd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `x` (3rd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `y` (3rd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `z` (3rd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `x` (3rd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `y` (3rd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `z` (3rd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `x` (3rd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `y` (3rd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `z` (3rd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `x` (3rd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `y` (3rd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `z` (3rd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `x` (3rd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `y` (3rd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `z` (3rd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(2)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x(self) -> T {
        self.index(0)
    }

    /// Returns the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y(self) -> T {
        self.index(1)
    }

    /// Returns the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn z(self) -> T {
        self.index(2)
    }

    /// Returns the `w` (4th) component of the vector.
    #[inline(always)]
    pub const fn w(self) -> T {
        self.index(3)
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a vector with the `x` and `x` (1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x` and `y` (1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x` and `z` (1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x` and `w` (1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xw(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(0), self.index(3)])
    }

    /// Returns a vector with the `y` and `x` (2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y` and `y` (2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y` and `z` (2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y` and `w` (2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yw(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(1), self.index(3)])
    }

    /// Returns a vector with the `z` and `x` (3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z` and `y` (3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z` and `z` (3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z` and `w` (3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zw(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(2), self.index(3)])
    }

    /// Returns a vector with the `w` and `x` (4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wx(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(3), self.index(0)])
    }

    /// Returns a vector with the `w` and `y` (4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wy(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(3), self.index(1)])
    }

    /// Returns a vector with the `w` and `z` (4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wz(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(3), self.index(2)])
    }

    /// Returns a vector with the `w` and `w` (4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ww(self) -> Vector<2, T, A> {
        Vector::from_array([self.index(3), self.index(3)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a vector with the `x`, `x` and `x` (1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `x` and `y` (1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `x` and `z` (1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `x` and `w` (1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `x`, `y` and `x` (1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `y` and `y` (1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `y` and `z` (1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `y` and `w` (1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `x`, `z` and `x` (1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `z` and `y` (1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `z` and `z` (1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `z` and `w` (1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `x`, `w` and `x` (1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `x`, `w` and `y` (1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `x`, `w` and `z` (1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `x`, `w` and `w` (1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xww(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `y`, `x` and `x` (2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `x` and `y` (2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `x` and `z` (2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `x` and `w` (2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `y`, `y` and `x` (2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `y` and `y` (2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `y` and `z` (2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `y` and `w` (2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `y`, `z` and `x` (2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `z` and `y` (2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `z` and `z` (2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `z` and `w` (2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `y`, `w` and `x` (2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `y`, `w` and `y` (2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `y`, `w` and `z` (2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `y`, `w` and `w` (2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yww(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `z`, `x` and `x` (3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `x` and `y` (3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `x` and `z` (3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `x` and `w` (3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `z`, `y` and `x` (3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `y` and `y` (3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `y` and `z` (3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `y` and `w` (3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `z`, `z` and `x` (3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `z` and `y` (3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `z` and `z` (3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `z` and `w` (3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `z`, `w` and `x` (3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `z`, `w` and `y` (3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `z`, `w` and `z` (3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `z`, `w` and `w` (3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zww(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `w`, `x` and `x` (4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `w`, `x` and `y` (4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `w`, `x` and `z` (4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `w`, `x` and `w` (4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `w`, `y` and `x` (4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `w`, `y` and `y` (4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `w`, `y` and `z` (4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `w`, `y` and `w` (4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `w`, `z` and `x` (4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `w`, `z` and `y` (4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `w`, `z` and `z` (4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `w`, `z` and `w` (4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzw(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `w`, `w` and `x` (4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwx(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `w`, `w` and `y` (4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwy(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `w`, `w` and `z` (4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwz(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `w`, `w` and `w` (4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn www(self) -> Vector<3, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(3)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a vector with the `x`, `x`, `x` and `x` (1st, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `x` and `y` (1st, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `x` and `z` (1st, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `x`, `x` and `w` (1st, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `x` (1st, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `y` (1st, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `z` (1st, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `x`, `y` and `w` (1st, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `x` (1st, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `y` (1st, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `z` (1st, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `x`, `z` and `w` (1st, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `x`, `x`, `w` and `x` (1st, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xxwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `x`, `x`, `w` and `y` (1st, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xxwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `x`, `x`, `w` and `z` (1st, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xxwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `x`, `x`, `w` and `w` (1st, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xxww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(0), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `x` (1st, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `y` (1st, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `z` (1st, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `x` and `w` (1st, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `x` (1st, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `y` (1st, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `z` (1st, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `y` and `w` (1st, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `x` (1st, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `y` (1st, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `z` (1st, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `z` and `w` (1st, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `x`, `y`, `w` and `x` (1st, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xywx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `x`, `y`, `w` and `y` (1st, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xywy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `x`, `y`, `w` and `z` (1st, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xywz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `x`, `y`, `w` and `w` (1st, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xyww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(1), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `x` (1st, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `y` (1st, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `z` (1st, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `x` and `w` (1st, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `x` (1st, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `y` (1st, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `z` (1st, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `y` and `w` (1st, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `x` (1st, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `y` (1st, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `z` (1st, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `z` and `w` (1st, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `x`, `z`, `w` and `x` (1st, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xzwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `x`, `z`, `w` and `y` (1st, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xzwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `x`, `z`, `w` and `z` (1st, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xzwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `x`, `z`, `w` and `w` (1st, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xzww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(2), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `x`, `w`, `x` and `x` (1st, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `x`, `w`, `x` and `y` (1st, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `x`, `w`, `x` and `z` (1st, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `x`, `w`, `x` and `w` (1st, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `x`, `w`, `y` and `x` (1st, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `x`, `w`, `y` and `y` (1st, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `x`, `w`, `y` and `z` (1st, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `x`, `w`, `y` and `w` (1st, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `x`, `w`, `z` and `x` (1st, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `x`, `w`, `z` and `y` (1st, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `x`, `w`, `z` and `z` (1st, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `x`, `w`, `z` and `w` (1st, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `x`, `w`, `w` and `x` (1st, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn xwwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `x`, `w`, `w` and `y` (1st, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn xwwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `x`, `w`, `w` and `z` (1st, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn xwwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `x`, `w`, `w` and `w` (1st, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn xwww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(0), self.index(3), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `x` (2nd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `y` (2nd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `z` (2nd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `x` and `w` (2nd, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `x` (2nd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `y` (2nd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `z` (2nd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `y` and `w` (2nd, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `x` (2nd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `y` (2nd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `z` (2nd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `z` and `w` (2nd, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `y`, `x`, `w` and `x` (2nd, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yxwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `y`, `x`, `w` and `y` (2nd, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yxwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `y`, `x`, `w` and `z` (2nd, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yxwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `y`, `x`, `w` and `w` (2nd, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yxww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(0), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `x` (2nd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `y` (2nd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `z` (2nd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `x` and `w` (2nd, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `x` (2nd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `y` (2nd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `z` (2nd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `y` and `w` (2nd, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `x` (2nd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `y` (2nd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `z` (2nd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `z` and `w` (2nd, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `y`, `y`, `w` and `x` (2nd, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yywx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `y`, `y`, `w` and `y` (2nd, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yywy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `y`, `y`, `w` and `z` (2nd, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yywz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `y`, `y`, `w` and `w` (2nd, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yyww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(1), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `x` (2nd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `y` (2nd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `z` (2nd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `x` and `w` (2nd, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `x` (2nd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `y` (2nd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `z` (2nd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `y` and `w` (2nd, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `x` (2nd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `y` (2nd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `z` (2nd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `z` and `w` (2nd, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `y`, `z`, `w` and `x` (2nd, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn yzwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `y`, `z`, `w` and `y` (2nd, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn yzwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `y`, `z`, `w` and `z` (2nd, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn yzwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `y`, `z`, `w` and `w` (2nd, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn yzww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(2), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `y`, `w`, `x` and `x` (2nd, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `y`, `w`, `x` and `y` (2nd, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `y`, `w`, `x` and `z` (2nd, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `y`, `w`, `x` and `w` (2nd, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `y`, `w`, `y` and `x` (2nd, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `y`, `w`, `y` and `y` (2nd, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `y`, `w`, `y` and `z` (2nd, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `y`, `w`, `y` and `w` (2nd, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `y`, `w`, `z` and `x` (2nd, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `y`, `w`, `z` and `y` (2nd, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `y`, `w`, `z` and `z` (2nd, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `y`, `w`, `z` and `w` (2nd, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `y`, `w`, `w` and `x` (2nd, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn ywwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `y`, `w`, `w` and `y` (2nd, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn ywwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `y`, `w`, `w` and `z` (2nd, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn ywwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `y`, `w`, `w` and `w` (2nd, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn ywww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(1), self.index(3), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `x` (3rd, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `y` (3rd, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `z` (3rd, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `x` and `w` (3rd, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `x` (3rd, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `y` (3rd, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `z` (3rd, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `y` and `w` (3rd, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `x` (3rd, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `y` (3rd, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `z` (3rd, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `z` and `w` (3rd, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `z`, `x`, `w` and `x` (3rd, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zxwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `z`, `x`, `w` and `y` (3rd, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zxwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `z`, `x`, `w` and `z` (3rd, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zxwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `z`, `x`, `w` and `w` (3rd, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zxww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(0), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `x` (3rd, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `y` (3rd, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `z` (3rd, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `x` and `w` (3rd, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `x` (3rd, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `y` (3rd, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `z` (3rd, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `y` and `w` (3rd, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `x` (3rd, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `y` (3rd, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `z` (3rd, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `z` and `w` (3rd, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `z`, `y`, `w` and `x` (3rd, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zywx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `z`, `y`, `w` and `y` (3rd, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zywy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `z`, `y`, `w` and `z` (3rd, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zywz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `z`, `y`, `w` and `w` (3rd, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zyww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(1), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `x` (3rd, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `y` (3rd, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `z` (3rd, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `x` and `w` (3rd, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `x` (3rd, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `y` (3rd, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `z` (3rd, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `y` and `w` (3rd, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `x` (3rd, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `y` (3rd, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `z` (3rd, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `z` and `w` (3rd, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `z`, `z`, `w` and `x` (3rd, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zzwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `z`, `z`, `w` and `y` (3rd, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zzwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `z`, `z`, `w` and `z` (3rd, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zzwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `z`, `z`, `w` and `w` (3rd, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zzww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(2), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `z`, `w`, `x` and `x` (3rd, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `z`, `w`, `x` and `y` (3rd, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `z`, `w`, `x` and `z` (3rd, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `z`, `w`, `x` and `w` (3rd, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `z`, `w`, `y` and `x` (3rd, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `z`, `w`, `y` and `y` (3rd, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `z`, `w`, `y` and `z` (3rd, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `z`, `w`, `y` and `w` (3rd, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `z`, `w`, `z` and `x` (3rd, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `z`, `w`, `z` and `y` (3rd, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `z`, `w`, `z` and `z` (3rd, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `z`, `w`, `z` and `w` (3rd, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `z`, `w`, `w` and `x` (3rd, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn zwwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `z`, `w`, `w` and `y` (3rd, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn zwwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `z`, `w`, `w` and `z` (3rd, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn zwwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `z`, `w`, `w` and `w` (3rd, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn zwww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(2), self.index(3), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `w`, `x`, `x` and `x` (4th, 1st, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `w`, `x`, `x` and `y` (4th, 1st, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `w`, `x`, `x` and `z` (4th, 1st, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `w`, `x`, `x` and `w` (4th, 1st, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `w`, `x`, `y` and `x` (4th, 1st, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `w`, `x`, `y` and `y` (4th, 1st, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `w`, `x`, `y` and `z` (4th, 1st, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `w`, `x`, `y` and `w` (4th, 1st, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `w`, `x`, `z` and `x` (4th, 1st, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `w`, `x`, `z` and `y` (4th, 1st, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `w`, `x`, `z` and `z` (4th, 1st, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `w`, `x`, `z` and `w` (4th, 1st, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `w`, `x`, `w` and `x` (4th, 1st, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wxwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `w`, `x`, `w` and `y` (4th, 1st, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wxwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `w`, `x`, `w` and `z` (4th, 1st, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wxwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `w`, `x`, `w` and `w` (4th, 1st, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wxww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(0), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `w`, `y`, `x` and `x` (4th, 2nd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `w`, `y`, `x` and `y` (4th, 2nd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `w`, `y`, `x` and `z` (4th, 2nd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `w`, `y`, `x` and `w` (4th, 2nd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `w`, `y`, `y` and `x` (4th, 2nd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `w`, `y`, `y` and `y` (4th, 2nd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `w`, `y`, `y` and `z` (4th, 2nd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `w`, `y`, `y` and `w` (4th, 2nd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `w`, `y`, `z` and `x` (4th, 2nd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wyzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `w`, `y`, `z` and `y` (4th, 2nd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wyzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `w`, `y`, `z` and `z` (4th, 2nd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wyzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `w`, `y`, `z` and `w` (4th, 2nd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `w`, `y`, `w` and `x` (4th, 2nd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wywx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `w`, `y`, `w` and `y` (4th, 2nd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wywy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `w`, `y`, `w` and `z` (4th, 2nd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wywz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `w`, `y`, `w` and `w` (4th, 2nd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wyww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(1), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `w`, `z`, `x` and `x` (4th, 3rd, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `w`, `z`, `x` and `y` (4th, 3rd, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `w`, `z`, `x` and `z` (4th, 3rd, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `w`, `z`, `x` and `w` (4th, 3rd, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `w`, `z`, `y` and `x` (4th, 3rd, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `w`, `z`, `y` and `y` (4th, 3rd, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `w`, `z`, `y` and `z` (4th, 3rd, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `w`, `z`, `y` and `w` (4th, 3rd, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `w`, `z`, `z` and `x` (4th, 3rd, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `w`, `z`, `z` and `y` (4th, 3rd, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `w`, `z`, `z` and `z` (4th, 3rd, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `w`, `z`, `z` and `w` (4th, 3rd, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `w`, `z`, `w` and `x` (4th, 3rd, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wzwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `w`, `z`, `w` and `y` (4th, 3rd, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wzwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `w`, `z`, `w` and `z` (4th, 3rd, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wzwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `w`, `z`, `w` and `w` (4th, 3rd, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wzww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(2), self.index(3), self.index(3)])
    }

    /// Returns a vector with the `w`, `w`, `x` and `x` (4th, 4th, 1st and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwxx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(0), self.index(0)])
    }

    /// Returns a vector with the `w`, `w`, `x` and `y` (4th, 4th, 1st and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwxy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(0), self.index(1)])
    }

    /// Returns a vector with the `w`, `w`, `x` and `z` (4th, 4th, 1st and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwxz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(0), self.index(2)])
    }

    /// Returns a vector with the `w`, `w`, `x` and `w` (4th, 4th, 1st and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwxw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(0), self.index(3)])
    }

    /// Returns a vector with the `w`, `w`, `y` and `x` (4th, 4th, 2nd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwyx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(1), self.index(0)])
    }

    /// Returns a vector with the `w`, `w`, `y` and `y` (4th, 4th, 2nd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwyy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(1), self.index(1)])
    }

    /// Returns a vector with the `w`, `w`, `y` and `z` (4th, 4th, 2nd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwyz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(1), self.index(2)])
    }

    /// Returns a vector with the `w`, `w`, `y` and `w` (4th, 4th, 2nd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwyw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(1), self.index(3)])
    }

    /// Returns a vector with the `w`, `w`, `z` and `x` (4th, 4th, 3rd and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwzx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(2), self.index(0)])
    }

    /// Returns a vector with the `w`, `w`, `z` and `y` (4th, 4th, 3rd and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwzy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(2), self.index(1)])
    }

    /// Returns a vector with the `w`, `w`, `z` and `z` (4th, 4th, 3rd and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwzz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(2), self.index(2)])
    }

    /// Returns a vector with the `w`, `w`, `z` and `w` (4th, 4th, 3rd and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwzw(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(2), self.index(3)])
    }

    /// Returns a vector with the `w`, `w`, `w` and `x` (4th, 4th, 4th and 1st) components of the input vector.
    #[inline(always)]
    pub const fn wwwx(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(3), self.index(0)])
    }

    /// Returns a vector with the `w`, `w`, `w` and `y` (4th, 4th, 4th and 2nd) components of the input vector.
    #[inline(always)]
    pub const fn wwwy(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(3), self.index(1)])
    }

    /// Returns a vector with the `w`, `w`, `w` and `z` (4th, 4th, 4th and 3rd) components of the input vector.
    #[inline(always)]
    pub const fn wwwz(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(3), self.index(2)])
    }

    /// Returns a vector with the `w`, `w`, `w` and `w` (4th, 4th, 4th and 4th) components of the input vector.
    #[inline(always)]
    pub const fn wwww(self) -> Vector<4, T, A> {
        Vector::from_array([self.index(3), self.index(3), self.index(3), self.index(3)])
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a reference to the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x_ref(&self) -> &T {
        &self.as_array_ref()[0]
    }

    /// Returns a reference to the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y_ref(&self) -> &T {
        &self.as_array_ref()[1]
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a reference to the `x` and `y` (1st and 2nd) components part of the vector.
    #[inline(always)]
    pub const fn xy_ref(&self) -> &Vector<2, T, VecPacked> {
        self.get_2_ref(0).unwrap()
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a reference to the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x_ref(&self) -> &T {
        &self.as_array_ref()[0]
    }

    /// Returns a reference to the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y_ref(&self) -> &T {
        &self.as_array_ref()[1]
    }

    /// Returns a reference to the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn z_ref(&self) -> &T {
        &self.as_array_ref()[2]
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a reference to the `x` and `y` (1st and 2nd) components part of the vector.
    #[inline(always)]
    pub const fn xy_ref(&self) -> &Vector<2, T, VecPacked> {
        self.get_2_ref(0).unwrap()
    }

    /// Returns a reference to the `y` and `z` (2nd and 3rd) components part of the vector.
    #[inline(always)]
    pub const fn yz_ref(&self) -> &Vector<2, T, VecPacked> {
        self.get_2_ref(1).unwrap()
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a reference to the `x`, `y` and `z` (1st, 2nd and 3rd) components part of the vector.
    #[inline(always)]
    pub const fn xyz_ref(&self) -> &Vector<3, T, VecPacked> {
        self.get_3_ref(0).unwrap()
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a reference to the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x_ref(&self) -> &T {
        &self.as_array_ref()[0]
    }

    /// Returns a reference to the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y_ref(&self) -> &T {
        &self.as_array_ref()[1]
    }

    /// Returns a reference to the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn z_ref(&self) -> &T {
        &self.as_array_ref()[2]
    }

    /// Returns a reference to the `w` (4th) component of the vector.
    #[inline(always)]
    pub const fn w_ref(&self) -> &T {
        &self.as_array_ref()[3]
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a reference to the `x` and `y` (1st and 2nd) components part of the vector.
    #[inline(always)]
    pub const fn xy_ref(&self) -> &Vector<2, T, VecPacked> {
        self.get_2_ref(0).unwrap()
    }

    /// Returns a reference to the `y` and `z` (2nd and 3rd) components part of the vector.
    #[inline(always)]
    pub const fn yz_ref(&self) -> &Vector<2, T, VecPacked> {
        self.get_2_ref(1).unwrap()
    }

    /// Returns a reference to the `z` and `w` (3rd and 4th) components part of the vector.
    #[inline(always)]
    pub const fn zw_ref(&self) -> &Vector<2, T, VecPacked> {
        self.get_2_ref(2).unwrap()
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a reference to the `x`, `y` and `z` (1st, 2nd and 3rd) components part of the vector.
    #[inline(always)]
    pub const fn xyz_ref(&self) -> &Vector<3, T, VecPacked> {
        self.get_3_ref(0).unwrap()
    }

    /// Returns a reference to the `y`, `z` and `w` (2nd, 3rd and 4th) components part of the vector.
    #[inline(always)]
    pub const fn yzw_ref(&self) -> &Vector<3, T, VecPacked> {
        self.get_3_ref(1).unwrap()
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a reference to the `x`, `y`, `z` and `w` (1st, 2nd, 3rd and 4th) components part of the vector.
    #[inline(always)]
    pub const fn xyzw_ref(&self) -> &Vector<4, T, VecPacked> {
        self.get_4_ref(0).unwrap()
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns a reference to the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[0]
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component.
    #[inline(always)]
    pub const fn x_y_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
            )
        }
    }

    /// Returns a reference to the `x``y` (1st2nd) components part of the vector.
    #[inline(always)]
    pub const fn xy_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
        self.get_2_mut(0).unwrap()
    }

    /// Returns a reference to the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[1]
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns a reference to the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[0]
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component.
    #[inline(always)]
    pub const fn x_y_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` and `z` (2nd and 3rd) components.
    #[inline(always)]
    pub const fn x_yz_mut(&mut self) -> (&mut T, &mut Vector<2, T, VecPacked>) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *(self.as_mut_ptr().add(1) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn x_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Returns a reference to the `x``y` (1st2nd) components part of the vector.
    #[inline(always)]
    pub const fn xy_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
        self.get_2_mut(0).unwrap()
    }

    /// Splits the vector into these mutable references:
    /// - the `x` and `y` (1st and 2nd) components,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn xy_z_mut(&mut self) -> (&mut Vector<2, T, VecPacked>, &mut T) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(0) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Returns a reference to the `x``y``z` (1st2nd3rd) components part of the vector.
    #[inline(always)]
    pub const fn xyz_mut(&mut self) -> &mut Vector<3, T, VecPacked> {
        self.get_3_mut(0).unwrap()
    }

    /// Returns a reference to the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[1]
    }

    /// Splits the vector into these mutable references:
    /// - the `y` (2nd) component,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn y_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Returns a reference to the `y``z` (2nd3rd) components part of the vector.
    #[inline(always)]
    pub const fn yz_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
        self.get_2_mut(1).unwrap()
    }

    /// Returns a reference to the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[2]
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns a reference to the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn x_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[0]
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component.
    #[inline(always)]
    pub const fn x_y_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn x_y_z_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component,
    /// - the `z` (3rd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn x_y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(2),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component,
    /// - the `z` and `w` (3rd and 4th) components.
    #[inline(always)]
    pub const fn x_y_zw_mut(&mut self) -> (&mut T, &mut T, &mut Vector<2, T, VecPacked>) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
                &mut *(self.as_mut_ptr().add(2) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` (2nd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn x_y_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` and `z` (2nd and 3rd) components.
    #[inline(always)]
    pub const fn x_yz_mut(&mut self) -> (&mut T, &mut Vector<2, T, VecPacked>) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *(self.as_mut_ptr().add(1) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y` and `z` (2nd and 3rd) components,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn x_yz_w_mut(&mut self) -> (&mut T, &mut Vector<2, T, VecPacked>, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *(self.as_mut_ptr().add(1) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `y`, `z` and `w` (2nd, 3rd and 4th) components.
    #[inline(always)]
    pub const fn x_yzw_mut(&mut self) -> (&mut T, &mut Vector<3, T, VecPacked>) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *(self.as_mut_ptr().add(1) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn x_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `z` (3rd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn x_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(2),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `z` and `w` (3rd and 4th) components.
    #[inline(always)]
    pub const fn x_zw_mut(&mut self) -> (&mut T, &mut Vector<2, T, VecPacked>) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *(self.as_mut_ptr().add(2) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` (1st) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn x_w_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(0),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Returns a reference to the `x``y` (1st2nd) components part of the vector.
    #[inline(always)]
    pub const fn xy_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
        self.get_2_mut(0).unwrap()
    }

    /// Splits the vector into these mutable references:
    /// - the `x` and `y` (1st and 2nd) components,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn xy_z_mut(&mut self) -> (&mut Vector<2, T, VecPacked>, &mut T) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(0) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` and `y` (1st and 2nd) components,
    /// - the `z` (3rd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn xy_z_w_mut(&mut self) -> (&mut Vector<2, T, VecPacked>, &mut T, &mut T) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(0) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(2),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` and `y` (1st and 2nd) components,
    /// - the `z` and `w` (3rd and 4th) components.
    #[inline(always)]
    pub const fn xy_zw_mut(
        &mut self,
    ) -> (&mut Vector<2, T, VecPacked>, &mut Vector<2, T, VecPacked>) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(0) as *mut Vector<_, _, _>),
                &mut *(self.as_mut_ptr().add(2) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `x` and `y` (1st and 2nd) components,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn xy_w_mut(&mut self) -> (&mut Vector<2, T, VecPacked>, &mut T) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(0) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Returns a reference to the `x``y``z` (1st2nd3rd) components part of the vector.
    #[inline(always)]
    pub const fn xyz_mut(&mut self) -> &mut Vector<3, T, VecPacked> {
        self.get_3_mut(0).unwrap()
    }

    /// Splits the vector into these mutable references:
    /// - the `x`, `y` and `z` (1st, 2nd and 3rd) components,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn xyz_w_mut(&mut self) -> (&mut Vector<3, T, VecPacked>, &mut T) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(0) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Returns a reference to the `x``y``z``w` (1st2nd3rd4th) components part of the vector.
    #[inline(always)]
    pub const fn xyzw_mut(&mut self) -> &mut Vector<4, T, VecPacked> {
        self.get_4_mut(0).unwrap()
    }

    /// Returns a reference to the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn y_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[1]
    }

    /// Splits the vector into these mutable references:
    /// - the `y` (2nd) component,
    /// - the `z` (3rd) component.
    #[inline(always)]
    pub const fn y_z_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(2),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `y` (2nd) component,
    /// - the `z` (3rd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn y_z_w_mut(&mut self) -> (&mut T, &mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(2),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `y` (2nd) component,
    /// - the `z` and `w` (3rd and 4th) components.
    #[inline(always)]
    pub const fn y_zw_mut(&mut self) -> (&mut T, &mut Vector<2, T, VecPacked>) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(1),
                &mut *(self.as_mut_ptr().add(2) as *mut Vector<_, _, _>),
            )
        }
    }

    /// Splits the vector into these mutable references:
    /// - the `y` (2nd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn y_w_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(1),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Returns a reference to the `y``z` (2nd3rd) components part of the vector.
    #[inline(always)]
    pub const fn yz_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
        self.get_2_mut(1).unwrap()
    }

    /// Splits the vector into these mutable references:
    /// - the `y` and `z` (2nd and 3rd) components,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn yz_w_mut(&mut self) -> (&mut Vector<2, T, VecPacked>, &mut T) {
        unsafe {
            (
                &mut *(self.as_mut_ptr().add(1) as *mut Vector<_, _, _>),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Returns a reference to the `y``z``w` (2nd3rd4th) components part of the vector.
    #[inline(always)]
    pub const fn yzw_mut(&mut self) -> &mut Vector<3, T, VecPacked> {
        self.get_3_mut(1).unwrap()
    }

    /// Returns a reference to the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn z_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[2]
    }

    /// Splits the vector into these mutable references:
    /// - the `z` (3rd) component,
    /// - the `w` (4th) component.
    #[inline(always)]
    pub const fn z_w_mut(&mut self) -> (&mut T, &mut T) {
        unsafe {
            (
                &mut *self.as_mut_ptr().add(2),
                &mut *self.as_mut_ptr().add(3),
            )
        }
    }

    /// Returns a reference to the `z``w` (3rd4th) components part of the vector.
    #[inline(always)]
    pub const fn zw_mut(&mut self) -> &mut Vector<2, T, VecPacked> {
        self.get_2_mut(2).unwrap()
    }

    /// Returns a reference to the `w` (4th) component of the vector.
    #[inline(always)]
    pub const fn w_mut(&mut self) -> &mut T {
        &mut self.as_array_mut()[3]
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Sets the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn set_x(&mut self, value: T) {
        self.as_array_mut()[0] = value;
    }

    /// Sets the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn set_y(&mut self, value: T) {
        self.as_array_mut()[1] = value;
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Sets the `x` and `y` (1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xy(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
    }

    /// Sets the `y` and `x` (2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_yx(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Sets the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn set_x(&mut self, value: T) {
        self.as_array_mut()[0] = value;
    }

    /// Sets the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn set_y(&mut self, value: T) {
        self.as_array_mut()[1] = value;
    }

    /// Sets the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn set_z(&mut self, value: T) {
        self.as_array_mut()[2] = value;
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Sets the `x` and `y` (1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xy(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
    }

    /// Sets the `x` and `z` (1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xz(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
    }

    /// Sets the `y` and `x` (2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_yx(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
    }

    /// Sets the `y` and `z` (2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_yz(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
    }

    /// Sets the `z` and `x` (3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zx(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
    }

    /// Sets the `z` and `y` (3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zy(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Sets the `x`, `y` and `z` (1st, 2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xyz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `x`, `z` and `y` (1st, 3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xzy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `y`, `x` and `z` (2nd, 1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_yxz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `y`, `z` and `x` (2nd, 3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_yzx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `z`, `x` and `y` (3rd, 1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zxy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `z`, `y` and `x` (3rd, 2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zyx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Sets the `x` (1st) component of the vector.
    #[inline(always)]
    pub const fn set_x(&mut self, value: T) {
        self.as_array_mut()[0] = value;
    }

    /// Sets the `y` (2nd) component of the vector.
    #[inline(always)]
    pub const fn set_y(&mut self, value: T) {
        self.as_array_mut()[1] = value;
    }

    /// Sets the `z` (3rd) component of the vector.
    #[inline(always)]
    pub const fn set_z(&mut self, value: T) {
        self.as_array_mut()[2] = value;
    }

    /// Sets the `w` (4th) component of the vector.
    #[inline(always)]
    pub const fn set_w(&mut self, value: T) {
        self.as_array_mut()[3] = value;
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Sets the `x` and `y` (1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xy(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
    }

    /// Sets the `x` and `z` (1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xz(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
    }

    /// Sets the `x` and `w` (1st and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_xw(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
    }

    /// Sets the `y` and `x` (2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_yx(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
    }

    /// Sets the `y` and `z` (2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_yz(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
    }

    /// Sets the `y` and `w` (2nd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_yw(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
    }

    /// Sets the `z` and `x` (3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zx(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
    }

    /// Sets the `z` and `y` (3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zy(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
    }

    /// Sets the `z` and `w` (3rd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_zw(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
    }

    /// Sets the `w` and `x` (4th and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_wx(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
    }

    /// Sets the `w` and `y` (4th and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_wy(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
    }

    /// Sets the `w` and `z` (4th and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_wz(&mut self, value: Vector<2, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Sets the `x`, `y` and `z` (1st, 2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xyz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `x`, `y` and `w` (1st, 2nd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_xyw(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
    }

    /// Sets the `x`, `z` and `y` (1st, 3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xzy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `x`, `z` and `w` (1st, 3rd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_xzw(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
    }

    /// Sets the `x`, `w` and `y` (1st, 4th and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xwy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `x`, `w` and `z` (1st, 4th and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xwz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `y`, `x` and `z` (2nd, 1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_yxz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `y`, `x` and `w` (2nd, 1st and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_yxw(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
    }

    /// Sets the `y`, `z` and `x` (2nd, 3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_yzx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `y`, `z` and `w` (2nd, 3rd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_yzw(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
    }

    /// Sets the `y`, `w` and `x` (2nd, 4th and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_ywx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `y`, `w` and `z` (2nd, 4th and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_ywz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `z`, `x` and `y` (3rd, 1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zxy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `z`, `x` and `w` (3rd, 1st and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_zxw(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
    }

    /// Sets the `z`, `y` and `x` (3rd, 2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zyx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `z`, `y` and `w` (3rd, 2nd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_zyw(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
    }

    /// Sets the `z`, `w` and `x` (3rd, 4th and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zwx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `z`, `w` and `y` (3rd, 4th and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zwy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `w`, `x` and `y` (4th, 1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_wxy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }

    /// Sets the `w`, `x` and `z` (4th, 1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_wxz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `w`, `y` and `x` (4th, 2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_wyx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `w`, `y` and `z` (4th, 2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_wyz(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
    }

    /// Sets the `w`, `z` and `x` (4th, 3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_wzx(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
    }

    /// Sets the `w`, `z` and `y` (4th, 3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_wzy(&mut self, value: Vector<3, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Sets the `x`, `y`, `z` and `w` (1st, 2nd, 3rd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_xyzw(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
        self.as_array_mut()[3] = value.to_array()[3];
    }

    /// Sets the `x`, `y`, `w` and `z` (1st, 2nd, 4th and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xywz(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
        self.as_array_mut()[2] = value.to_array()[3];
    }

    /// Sets the `x`, `z`, `y` and `w` (1st, 3rd, 2nd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_xzyw(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
        self.as_array_mut()[3] = value.to_array()[3];
    }

    /// Sets the `x`, `z`, `w` and `y` (1st, 3rd, 4th and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xzwy(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
        self.as_array_mut()[1] = value.to_array()[3];
    }

    /// Sets the `x`, `w`, `y` and `z` (1st, 4th, 2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_xwyz(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
        self.as_array_mut()[2] = value.to_array()[3];
    }

    /// Sets the `x`, `w`, `z` and `y` (1st, 4th, 3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_xwzy(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[0] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
        self.as_array_mut()[1] = value.to_array()[3];
    }

    /// Sets the `y`, `x`, `z` and `w` (2nd, 1st, 3rd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_yxzw(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
        self.as_array_mut()[3] = value.to_array()[3];
    }

    /// Sets the `y`, `x`, `w` and `z` (2nd, 1st, 4th and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_yxwz(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
        self.as_array_mut()[2] = value.to_array()[3];
    }

    /// Sets the `y`, `z`, `x` and `w` (2nd, 3rd, 1st and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_yzxw(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
        self.as_array_mut()[3] = value.to_array()[3];
    }

    /// Sets the `y`, `z`, `w` and `x` (2nd, 3rd, 4th and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_yzwx(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
        self.as_array_mut()[0] = value.to_array()[3];
    }

    /// Sets the `y`, `w`, `x` and `z` (2nd, 4th, 1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_ywxz(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
        self.as_array_mut()[2] = value.to_array()[3];
    }

    /// Sets the `y`, `w`, `z` and `x` (2nd, 4th, 3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_ywzx(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[1] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
        self.as_array_mut()[0] = value.to_array()[3];
    }

    /// Sets the `z`, `x`, `y` and `w` (3rd, 1st, 2nd and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_zxyw(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
        self.as_array_mut()[3] = value.to_array()[3];
    }

    /// Sets the `z`, `x`, `w` and `y` (3rd, 1st, 4th and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zxwy(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
        self.as_array_mut()[1] = value.to_array()[3];
    }

    /// Sets the `z`, `y`, `x` and `w` (3rd, 2nd, 1st and 4th) components of the vector.
    #[inline(always)]
    pub const fn set_zyxw(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
        self.as_array_mut()[3] = value.to_array()[3];
    }

    /// Sets the `z`, `y`, `w` and `x` (3rd, 2nd, 4th and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zywx(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[3] = value.to_array()[2];
        self.as_array_mut()[0] = value.to_array()[3];
    }

    /// Sets the `z`, `w`, `x` and `y` (3rd, 4th, 1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_zwxy(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
        self.as_array_mut()[1] = value.to_array()[3];
    }

    /// Sets the `z`, `w`, `y` and `x` (3rd, 4th, 2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_zwyx(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[2] = value.to_array()[0];
        self.as_array_mut()[3] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
        self.as_array_mut()[0] = value.to_array()[3];
    }

    /// Sets the `w`, `x`, `y` and `z` (4th, 1st, 2nd and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_wxyz(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
        self.as_array_mut()[2] = value.to_array()[3];
    }

    /// Sets the `w`, `x`, `z` and `y` (4th, 1st, 3rd and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_wxzy(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[0] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
        self.as_array_mut()[1] = value.to_array()[3];
    }

    /// Sets the `w`, `y`, `x` and `z` (4th, 2nd, 1st and 3rd) components of the vector.
    #[inline(always)]
    pub const fn set_wyxz(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
        self.as_array_mut()[2] = value.to_array()[3];
    }

    /// Sets the `w`, `y`, `z` and `x` (4th, 2nd, 3rd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_wyzx(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[1] = value.to_array()[1];
        self.as_array_mut()[2] = value.to_array()[2];
        self.as_array_mut()[0] = value.to_array()[3];
    }

    /// Sets the `w`, `z`, `x` and `y` (4th, 3rd, 1st and 2nd) components of the vector.
    #[inline(always)]
    pub const fn set_wzxy(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[0] = value.to_array()[2];
        self.as_array_mut()[1] = value.to_array()[3];
    }

    /// Sets the `w`, `z`, `y` and `x` (4th, 3rd, 2nd and 1st) components of the vector.
    #[inline(always)]
    pub const fn set_wzyx(&mut self, value: Vector<4, T, impl VecAlignment>) {
        self.as_array_mut()[3] = value.to_array()[0];
        self.as_array_mut()[2] = value.to_array()[1];
        self.as_array_mut()[1] = value.to_array()[2];
        self.as_array_mut()[0] = value.to_array()[3];
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns the input vector with the `x` (1st) component modified.
    #[inline(always)]
    pub const fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.set_x(value);
        result
    }

    /// Returns the input vector with the `y` (2nd) component modified.
    #[inline(always)]
    pub const fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.set_y(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    /// Returns the input vector with the `x` and `y` (1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xy(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xy(value);
        result
    }

    /// Returns the input vector with the `y` and `x` (2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_yx(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yx(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the input vector with the `x` (1st) component modified.
    #[inline(always)]
    pub const fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.set_x(value);
        result
    }

    /// Returns the input vector with the `y` (2nd) component modified.
    #[inline(always)]
    pub const fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.set_y(value);
        result
    }

    /// Returns the input vector with the `z` (3rd) component modified.
    #[inline(always)]
    pub const fn with_z(self, value: T) -> Self {
        let mut result = self;
        result.set_z(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the input vector with the `x` and `y` (1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xy(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xy(value);
        result
    }

    /// Returns the input vector with the `x` and `z` (1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xz(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xz(value);
        result
    }

    /// Returns the input vector with the `y` and `x` (2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_yx(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yx(value);
        result
    }

    /// Returns the input vector with the `y` and `z` (2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_yz(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yz(value);
        result
    }

    /// Returns the input vector with the `z` and `x` (3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_zx(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zx(value);
        result
    }

    /// Returns the input vector with the `z` and `y` (3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zy(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zy(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the input vector with the `x`, `y` and `z` (1st, 2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xyz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xyz(value);
        result
    }

    /// Returns the input vector with the `x`, `z` and `y` (1st, 3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xzy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xzy(value);
        result
    }

    /// Returns the input vector with the `y`, `x` and `z` (2nd, 1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_yxz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yxz(value);
        result
    }

    /// Returns the input vector with the `y`, `z` and `x` (2nd, 3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_yzx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yzx(value);
        result
    }

    /// Returns the input vector with the `z`, `x` and `y` (3rd, 1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zxy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zxy(value);
        result
    }

    /// Returns the input vector with the `z`, `y` and `x` (3rd, 2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_zyx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zyx(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns the input vector with the `x` (1st) component modified.
    #[inline(always)]
    pub const fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.set_x(value);
        result
    }

    /// Returns the input vector with the `y` (2nd) component modified.
    #[inline(always)]
    pub const fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.set_y(value);
        result
    }

    /// Returns the input vector with the `z` (3rd) component modified.
    #[inline(always)]
    pub const fn with_z(self, value: T) -> Self {
        let mut result = self;
        result.set_z(value);
        result
    }

    /// Returns the input vector with the `w` (4th) component modified.
    #[inline(always)]
    pub const fn with_w(self, value: T) -> Self {
        let mut result = self;
        result.set_w(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns the input vector with the `x` and `y` (1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xy(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xy(value);
        result
    }

    /// Returns the input vector with the `x` and `z` (1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xz(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xz(value);
        result
    }

    /// Returns the input vector with the `x` and `w` (1st and 4th) components modified.
    #[inline(always)]
    pub const fn with_xw(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xw(value);
        result
    }

    /// Returns the input vector with the `y` and `x` (2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_yx(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yx(value);
        result
    }

    /// Returns the input vector with the `y` and `z` (2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_yz(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yz(value);
        result
    }

    /// Returns the input vector with the `y` and `w` (2nd and 4th) components modified.
    #[inline(always)]
    pub const fn with_yw(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yw(value);
        result
    }

    /// Returns the input vector with the `z` and `x` (3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_zx(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zx(value);
        result
    }

    /// Returns the input vector with the `z` and `y` (3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zy(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zy(value);
        result
    }

    /// Returns the input vector with the `z` and `w` (3rd and 4th) components modified.
    #[inline(always)]
    pub const fn with_zw(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zw(value);
        result
    }

    /// Returns the input vector with the `w` and `x` (4th and 1st) components modified.
    #[inline(always)]
    pub const fn with_wx(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wx(value);
        result
    }

    /// Returns the input vector with the `w` and `y` (4th and 2nd) components modified.
    #[inline(always)]
    pub const fn with_wy(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wy(value);
        result
    }

    /// Returns the input vector with the `w` and `z` (4th and 3rd) components modified.
    #[inline(always)]
    pub const fn with_wz(self, value: Vector<2, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wz(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns the input vector with the `x`, `y` and `z` (1st, 2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xyz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xyz(value);
        result
    }

    /// Returns the input vector with the `x`, `y` and `w` (1st, 2nd and 4th) components modified.
    #[inline(always)]
    pub const fn with_xyw(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xyw(value);
        result
    }

    /// Returns the input vector with the `x`, `z` and `y` (1st, 3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xzy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xzy(value);
        result
    }

    /// Returns the input vector with the `x`, `z` and `w` (1st, 3rd and 4th) components modified.
    #[inline(always)]
    pub const fn with_xzw(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xzw(value);
        result
    }

    /// Returns the input vector with the `x`, `w` and `y` (1st, 4th and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xwy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xwy(value);
        result
    }

    /// Returns the input vector with the `x`, `w` and `z` (1st, 4th and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xwz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xwz(value);
        result
    }

    /// Returns the input vector with the `y`, `x` and `z` (2nd, 1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_yxz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yxz(value);
        result
    }

    /// Returns the input vector with the `y`, `x` and `w` (2nd, 1st and 4th) components modified.
    #[inline(always)]
    pub const fn with_yxw(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yxw(value);
        result
    }

    /// Returns the input vector with the `y`, `z` and `x` (2nd, 3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_yzx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yzx(value);
        result
    }

    /// Returns the input vector with the `y`, `z` and `w` (2nd, 3rd and 4th) components modified.
    #[inline(always)]
    pub const fn with_yzw(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yzw(value);
        result
    }

    /// Returns the input vector with the `y`, `w` and `x` (2nd, 4th and 1st) components modified.
    #[inline(always)]
    pub const fn with_ywx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_ywx(value);
        result
    }

    /// Returns the input vector with the `y`, `w` and `z` (2nd, 4th and 3rd) components modified.
    #[inline(always)]
    pub const fn with_ywz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_ywz(value);
        result
    }

    /// Returns the input vector with the `z`, `x` and `y` (3rd, 1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zxy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zxy(value);
        result
    }

    /// Returns the input vector with the `z`, `x` and `w` (3rd, 1st and 4th) components modified.
    #[inline(always)]
    pub const fn with_zxw(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zxw(value);
        result
    }

    /// Returns the input vector with the `z`, `y` and `x` (3rd, 2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_zyx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zyx(value);
        result
    }

    /// Returns the input vector with the `z`, `y` and `w` (3rd, 2nd and 4th) components modified.
    #[inline(always)]
    pub const fn with_zyw(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zyw(value);
        result
    }

    /// Returns the input vector with the `z`, `w` and `x` (3rd, 4th and 1st) components modified.
    #[inline(always)]
    pub const fn with_zwx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zwx(value);
        result
    }

    /// Returns the input vector with the `z`, `w` and `y` (3rd, 4th and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zwy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zwy(value);
        result
    }

    /// Returns the input vector with the `w`, `x` and `y` (4th, 1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_wxy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wxy(value);
        result
    }

    /// Returns the input vector with the `w`, `x` and `z` (4th, 1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_wxz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wxz(value);
        result
    }

    /// Returns the input vector with the `w`, `y` and `x` (4th, 2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_wyx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wyx(value);
        result
    }

    /// Returns the input vector with the `w`, `y` and `z` (4th, 2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_wyz(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wyz(value);
        result
    }

    /// Returns the input vector with the `w`, `z` and `x` (4th, 3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_wzx(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wzx(value);
        result
    }

    /// Returns the input vector with the `w`, `z` and `y` (4th, 3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_wzy(self, value: Vector<3, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wzy(value);
        result
    }
}
impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    /// Returns the input vector with the `x`, `y`, `z` and `w` (1st, 2nd, 3rd and 4th) components modified.
    #[inline(always)]
    pub const fn with_xyzw(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xyzw(value);
        result
    }

    /// Returns the input vector with the `x`, `y`, `w` and `z` (1st, 2nd, 4th and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xywz(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xywz(value);
        result
    }

    /// Returns the input vector with the `x`, `z`, `y` and `w` (1st, 3rd, 2nd and 4th) components modified.
    #[inline(always)]
    pub const fn with_xzyw(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xzyw(value);
        result
    }

    /// Returns the input vector with the `x`, `z`, `w` and `y` (1st, 3rd, 4th and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xzwy(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xzwy(value);
        result
    }

    /// Returns the input vector with the `x`, `w`, `y` and `z` (1st, 4th, 2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_xwyz(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xwyz(value);
        result
    }

    /// Returns the input vector with the `x`, `w`, `z` and `y` (1st, 4th, 3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_xwzy(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_xwzy(value);
        result
    }

    /// Returns the input vector with the `y`, `x`, `z` and `w` (2nd, 1st, 3rd and 4th) components modified.
    #[inline(always)]
    pub const fn with_yxzw(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yxzw(value);
        result
    }

    /// Returns the input vector with the `y`, `x`, `w` and `z` (2nd, 1st, 4th and 3rd) components modified.
    #[inline(always)]
    pub const fn with_yxwz(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yxwz(value);
        result
    }

    /// Returns the input vector with the `y`, `z`, `x` and `w` (2nd, 3rd, 1st and 4th) components modified.
    #[inline(always)]
    pub const fn with_yzxw(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yzxw(value);
        result
    }

    /// Returns the input vector with the `y`, `z`, `w` and `x` (2nd, 3rd, 4th and 1st) components modified.
    #[inline(always)]
    pub const fn with_yzwx(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_yzwx(value);
        result
    }

    /// Returns the input vector with the `y`, `w`, `x` and `z` (2nd, 4th, 1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_ywxz(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_ywxz(value);
        result
    }

    /// Returns the input vector with the `y`, `w`, `z` and `x` (2nd, 4th, 3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_ywzx(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_ywzx(value);
        result
    }

    /// Returns the input vector with the `z`, `x`, `y` and `w` (3rd, 1st, 2nd and 4th) components modified.
    #[inline(always)]
    pub const fn with_zxyw(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zxyw(value);
        result
    }

    /// Returns the input vector with the `z`, `x`, `w` and `y` (3rd, 1st, 4th and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zxwy(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zxwy(value);
        result
    }

    /// Returns the input vector with the `z`, `y`, `x` and `w` (3rd, 2nd, 1st and 4th) components modified.
    #[inline(always)]
    pub const fn with_zyxw(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zyxw(value);
        result
    }

    /// Returns the input vector with the `z`, `y`, `w` and `x` (3rd, 2nd, 4th and 1st) components modified.
    #[inline(always)]
    pub const fn with_zywx(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zywx(value);
        result
    }

    /// Returns the input vector with the `z`, `w`, `x` and `y` (3rd, 4th, 1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_zwxy(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zwxy(value);
        result
    }

    /// Returns the input vector with the `z`, `w`, `y` and `x` (3rd, 4th, 2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_zwyx(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_zwyx(value);
        result
    }

    /// Returns the input vector with the `w`, `x`, `y` and `z` (4th, 1st, 2nd and 3rd) components modified.
    #[inline(always)]
    pub const fn with_wxyz(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wxyz(value);
        result
    }

    /// Returns the input vector with the `w`, `x`, `z` and `y` (4th, 1st, 3rd and 2nd) components modified.
    #[inline(always)]
    pub const fn with_wxzy(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wxzy(value);
        result
    }

    /// Returns the input vector with the `w`, `y`, `x` and `z` (4th, 2nd, 1st and 3rd) components modified.
    #[inline(always)]
    pub const fn with_wyxz(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wyxz(value);
        result
    }

    /// Returns the input vector with the `w`, `y`, `z` and `x` (4th, 2nd, 3rd and 1st) components modified.
    #[inline(always)]
    pub const fn with_wyzx(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wyzx(value);
        result
    }

    /// Returns the input vector with the `w`, `z`, `x` and `y` (4th, 3rd, 1st and 2nd) components modified.
    #[inline(always)]
    pub const fn with_wzxy(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wzxy(value);
        result
    }

    /// Returns the input vector with the `w`, `z`, `y` and `x` (4th, 3rd, 2nd and 1st) components modified.
    #[inline(always)]
    pub const fn with_wzyx(self, value: Vector<4, T, impl VecAlignment>) -> Self {
        let mut result = self;
        result.set_wzyx(value);
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swizzle() {
        assert_eq!(vec2!(1, 2).xxy(), vec3!(1, 1, 2));
        assert_eq!(vec3!(1, 2, 3).xyzx(), vec4!(1, 2, 3, 1));
        assert_eq!(vec4!(1, 2, 3, 4).xyzw(), vec4!(1, 2, 3, 4));
        assert_eq!(vec2!(1, 2).y(), 2);

        assert_eq!(vec2p!(false, true).xyx(), vec3p!(false, true, false));
        assert_eq!(vec2p!(false, true).y(), true);
    }

    #[test]
    fn test_swizzle_ref() {
        assert_eq!(vec2!(1, 2).x_ref(), &1);
        assert_eq!(vec3p!(1, 2, 3).yz_ref(), &vec2p!(2, 3));
    }

    #[test]
    fn test_swizzle_mut() {
        assert_eq!(vec2!(1, 2).x_mut(), &mut 1);
        assert_eq!(vec3p!(1, 2, 3).yz_mut(), &mut vec2p!(2, 3));
        assert_eq!(vec4p!(1, 2, 3, 4).xy_z_mut(), (&mut vec2p!(1, 2), &mut 3));
    }

    #[test]
    fn test_swizzle_set() {
        let mut vec = vec2!(1, 2);
        vec.set_x(3);
        assert_eq!(vec, vec2!(3, 2));

        let mut vec = vec3p!(1, 2, 3);
        vec.set_zx(vec2!(4, 5));
        assert_eq!(vec, vec3p!(5, 2, 4));

        let mut vec = vec4p!(1, 2, 3, 4);
        vec.set_xzyw(vec4p!(5, 6, 7, 8));
        assert_eq!(vec, vec4p!(5, 7, 6, 8));
    }

    #[test]
    fn test_swizzle_with() {
        assert_eq!(vec2!(1, 2).with_x(3), vec2!(3, 2));
        assert_eq!(vec3p!(1, 2, 3).with_y(4), vec3p!(1, 4, 3));
        assert_eq!(vec4!(1, 2, 3, 4).with_zx(vec2!(5, 6)), vec4p!(6, 2, 5, 4));
    }
}
