use crate::vector::{Scalar, VecAlignment, Vector};

impl<T: Scalar, A: VecAlignment> Vector<2, T, A> {
    #[inline(always)]
    pub fn xx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<2, A, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<2, A, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<2, A, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<2, A, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<2, A, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 0, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<2, A, 1, 1, 1, 1>(self)
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    #[inline(always)]
    pub fn xx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<3, A, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<3, A, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 0, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 1, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<3, A, 2, 2, 2, 2>(self)
    }
}

impl<T: Scalar, A: VecAlignment> Vector<4, T, A> {
    #[inline(always)]
    pub fn xx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xw(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 0, 3>(self)
    }

    #[inline(always)]
    pub fn yx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yw(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 1, 3>(self)
    }

    #[inline(always)]
    pub fn zx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zw(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 2, 3>(self)
    }

    #[inline(always)]
    pub fn wx(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 3, 0>(self)
    }

    #[inline(always)]
    pub fn wy(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 3, 1>(self)
    }

    #[inline(always)]
    pub fn wz(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 3, 2>(self)
    }

    #[inline(always)]
    pub fn ww(self) -> Vector<2, T, A> {
        T::vec_swizzle2::<4, A, 3, 3>(self)
    }

    #[inline(always)]
    pub fn xxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xxw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 0, 3>(self)
    }

    #[inline(always)]
    pub fn xyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xyw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 1, 3>(self)
    }

    #[inline(always)]
    pub fn xzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xzw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 2, 3>(self)
    }

    #[inline(always)]
    pub fn xwx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 3, 0>(self)
    }

    #[inline(always)]
    pub fn xwy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 3, 1>(self)
    }

    #[inline(always)]
    pub fn xwz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 3, 2>(self)
    }

    #[inline(always)]
    pub fn xww(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 0, 3, 3>(self)
    }

    #[inline(always)]
    pub fn yxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yxw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 0, 3>(self)
    }

    #[inline(always)]
    pub fn yyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yyw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 1, 3>(self)
    }

    #[inline(always)]
    pub fn yzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yzw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 2, 3>(self)
    }

    #[inline(always)]
    pub fn ywx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 3, 0>(self)
    }

    #[inline(always)]
    pub fn ywy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 3, 1>(self)
    }

    #[inline(always)]
    pub fn ywz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 3, 2>(self)
    }

    #[inline(always)]
    pub fn yww(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 1, 3, 3>(self)
    }

    #[inline(always)]
    pub fn zxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zxw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 0, 3>(self)
    }

    #[inline(always)]
    pub fn zyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zyw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 1, 3>(self)
    }

    #[inline(always)]
    pub fn zzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zzw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 2, 3>(self)
    }

    #[inline(always)]
    pub fn zwx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 3, 0>(self)
    }

    #[inline(always)]
    pub fn zwy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 3, 1>(self)
    }

    #[inline(always)]
    pub fn zwz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 3, 2>(self)
    }

    #[inline(always)]
    pub fn zww(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 2, 3, 3>(self)
    }

    #[inline(always)]
    pub fn wxx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 0, 0>(self)
    }

    #[inline(always)]
    pub fn wxy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 0, 1>(self)
    }

    #[inline(always)]
    pub fn wxz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 0, 2>(self)
    }

    #[inline(always)]
    pub fn wxw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 0, 3>(self)
    }

    #[inline(always)]
    pub fn wyx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 1, 0>(self)
    }

    #[inline(always)]
    pub fn wyy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 1, 1>(self)
    }

    #[inline(always)]
    pub fn wyz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 1, 2>(self)
    }

    #[inline(always)]
    pub fn wyw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 1, 3>(self)
    }

    #[inline(always)]
    pub fn wzx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 2, 0>(self)
    }

    #[inline(always)]
    pub fn wzy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 2, 1>(self)
    }

    #[inline(always)]
    pub fn wzz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 2, 2>(self)
    }

    #[inline(always)]
    pub fn wzw(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 2, 3>(self)
    }

    #[inline(always)]
    pub fn wwx(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 3, 0>(self)
    }

    #[inline(always)]
    pub fn wwy(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 3, 1>(self)
    }

    #[inline(always)]
    pub fn wwz(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 3, 2>(self)
    }

    #[inline(always)]
    pub fn www(self) -> Vector<3, T, A> {
        T::vec_swizzle3::<4, A, 3, 3, 3>(self)
    }

    #[inline(always)]
    pub fn xxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xxxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 0, 3>(self)
    }

    #[inline(always)]
    pub fn xxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xxyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 1, 3>(self)
    }

    #[inline(always)]
    pub fn xxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xxzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 2, 3>(self)
    }

    #[inline(always)]
    pub fn xxwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 3, 0>(self)
    }

    #[inline(always)]
    pub fn xxwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 3, 1>(self)
    }

    #[inline(always)]
    pub fn xxwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 3, 2>(self)
    }

    #[inline(always)]
    pub fn xxww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 0, 3, 3>(self)
    }

    #[inline(always)]
    pub fn xyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xyxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 0, 3>(self)
    }

    #[inline(always)]
    pub fn xyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xyyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 1, 3>(self)
    }

    #[inline(always)]
    pub fn xyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xyzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 2, 3>(self)
    }

    #[inline(always)]
    pub fn xywx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 3, 0>(self)
    }

    #[inline(always)]
    pub fn xywy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 3, 1>(self)
    }

    #[inline(always)]
    pub fn xywz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 3, 2>(self)
    }

    #[inline(always)]
    pub fn xyww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 1, 3, 3>(self)
    }

    #[inline(always)]
    pub fn xzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xzxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 0, 3>(self)
    }

    #[inline(always)]
    pub fn xzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xzyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 1, 3>(self)
    }

    #[inline(always)]
    pub fn xzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xzzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 2, 3>(self)
    }

    #[inline(always)]
    pub fn xzwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 3, 0>(self)
    }

    #[inline(always)]
    pub fn xzwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 3, 1>(self)
    }

    #[inline(always)]
    pub fn xzwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 3, 2>(self)
    }

    #[inline(always)]
    pub fn xzww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 2, 3, 3>(self)
    }

    #[inline(always)]
    pub fn xwxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 0, 0>(self)
    }

    #[inline(always)]
    pub fn xwxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 0, 1>(self)
    }

    #[inline(always)]
    pub fn xwxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 0, 2>(self)
    }

    #[inline(always)]
    pub fn xwxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 0, 3>(self)
    }

    #[inline(always)]
    pub fn xwyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 1, 0>(self)
    }

    #[inline(always)]
    pub fn xwyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 1, 1>(self)
    }

    #[inline(always)]
    pub fn xwyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 1, 2>(self)
    }

    #[inline(always)]
    pub fn xwyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 1, 3>(self)
    }

    #[inline(always)]
    pub fn xwzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 2, 0>(self)
    }

    #[inline(always)]
    pub fn xwzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 2, 1>(self)
    }

    #[inline(always)]
    pub fn xwzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 2, 2>(self)
    }

    #[inline(always)]
    pub fn xwzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 2, 3>(self)
    }

    #[inline(always)]
    pub fn xwwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 3, 0>(self)
    }

    #[inline(always)]
    pub fn xwwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 3, 1>(self)
    }

    #[inline(always)]
    pub fn xwwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 3, 2>(self)
    }

    #[inline(always)]
    pub fn xwww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 0, 3, 3, 3>(self)
    }

    #[inline(always)]
    pub fn yxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yxxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 0, 3>(self)
    }

    #[inline(always)]
    pub fn yxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yxyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 1, 3>(self)
    }

    #[inline(always)]
    pub fn yxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yxzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 2, 3>(self)
    }

    #[inline(always)]
    pub fn yxwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 3, 0>(self)
    }

    #[inline(always)]
    pub fn yxwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 3, 1>(self)
    }

    #[inline(always)]
    pub fn yxwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 3, 2>(self)
    }

    #[inline(always)]
    pub fn yxww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 0, 3, 3>(self)
    }

    #[inline(always)]
    pub fn yyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yyxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 0, 3>(self)
    }

    #[inline(always)]
    pub fn yyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yyyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 1, 3>(self)
    }

    #[inline(always)]
    pub fn yyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yyzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 2, 3>(self)
    }

    #[inline(always)]
    pub fn yywx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 3, 0>(self)
    }

    #[inline(always)]
    pub fn yywy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 3, 1>(self)
    }

    #[inline(always)]
    pub fn yywz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 3, 2>(self)
    }

    #[inline(always)]
    pub fn yyww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 1, 3, 3>(self)
    }

    #[inline(always)]
    pub fn yzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn yzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn yzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn yzxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 0, 3>(self)
    }

    #[inline(always)]
    pub fn yzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn yzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn yzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn yzyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 1, 3>(self)
    }

    #[inline(always)]
    pub fn yzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn yzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn yzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn yzzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 2, 3>(self)
    }

    #[inline(always)]
    pub fn yzwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 3, 0>(self)
    }

    #[inline(always)]
    pub fn yzwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 3, 1>(self)
    }

    #[inline(always)]
    pub fn yzwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 3, 2>(self)
    }

    #[inline(always)]
    pub fn yzww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 2, 3, 3>(self)
    }

    #[inline(always)]
    pub fn ywxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 0, 0>(self)
    }

    #[inline(always)]
    pub fn ywxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 0, 1>(self)
    }

    #[inline(always)]
    pub fn ywxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 0, 2>(self)
    }

    #[inline(always)]
    pub fn ywxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 0, 3>(self)
    }

    #[inline(always)]
    pub fn ywyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 1, 0>(self)
    }

    #[inline(always)]
    pub fn ywyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 1, 1>(self)
    }

    #[inline(always)]
    pub fn ywyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 1, 2>(self)
    }

    #[inline(always)]
    pub fn ywyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 1, 3>(self)
    }

    #[inline(always)]
    pub fn ywzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 2, 0>(self)
    }

    #[inline(always)]
    pub fn ywzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 2, 1>(self)
    }

    #[inline(always)]
    pub fn ywzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 2, 2>(self)
    }

    #[inline(always)]
    pub fn ywzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 2, 3>(self)
    }

    #[inline(always)]
    pub fn ywwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 3, 0>(self)
    }

    #[inline(always)]
    pub fn ywwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 3, 1>(self)
    }

    #[inline(always)]
    pub fn ywwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 3, 2>(self)
    }

    #[inline(always)]
    pub fn ywww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 1, 3, 3, 3>(self)
    }

    #[inline(always)]
    pub fn zxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zxxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 0, 3>(self)
    }

    #[inline(always)]
    pub fn zxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zxyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 1, 3>(self)
    }

    #[inline(always)]
    pub fn zxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zxzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 2, 3>(self)
    }

    #[inline(always)]
    pub fn zxwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 3, 0>(self)
    }

    #[inline(always)]
    pub fn zxwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 3, 1>(self)
    }

    #[inline(always)]
    pub fn zxwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 3, 2>(self)
    }

    #[inline(always)]
    pub fn zxww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 0, 3, 3>(self)
    }

    #[inline(always)]
    pub fn zyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zyxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 0, 3>(self)
    }

    #[inline(always)]
    pub fn zyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zyyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 1, 3>(self)
    }

    #[inline(always)]
    pub fn zyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zyzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 2, 3>(self)
    }

    #[inline(always)]
    pub fn zywx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 3, 0>(self)
    }

    #[inline(always)]
    pub fn zywy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 3, 1>(self)
    }

    #[inline(always)]
    pub fn zywz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 3, 2>(self)
    }

    #[inline(always)]
    pub fn zyww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 1, 3, 3>(self)
    }

    #[inline(always)]
    pub fn zzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zzxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 0, 3>(self)
    }

    #[inline(always)]
    pub fn zzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zzyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 1, 3>(self)
    }

    #[inline(always)]
    pub fn zzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zzzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 2, 3>(self)
    }

    #[inline(always)]
    pub fn zzwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 3, 0>(self)
    }

    #[inline(always)]
    pub fn zzwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 3, 1>(self)
    }

    #[inline(always)]
    pub fn zzwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 3, 2>(self)
    }

    #[inline(always)]
    pub fn zzww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 2, 3, 3>(self)
    }

    #[inline(always)]
    pub fn zwxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 0, 0>(self)
    }

    #[inline(always)]
    pub fn zwxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 0, 1>(self)
    }

    #[inline(always)]
    pub fn zwxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 0, 2>(self)
    }

    #[inline(always)]
    pub fn zwxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 0, 3>(self)
    }

    #[inline(always)]
    pub fn zwyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 1, 0>(self)
    }

    #[inline(always)]
    pub fn zwyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 1, 1>(self)
    }

    #[inline(always)]
    pub fn zwyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 1, 2>(self)
    }

    #[inline(always)]
    pub fn zwyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 1, 3>(self)
    }

    #[inline(always)]
    pub fn zwzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 2, 0>(self)
    }

    #[inline(always)]
    pub fn zwzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 2, 1>(self)
    }

    #[inline(always)]
    pub fn zwzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 2, 2>(self)
    }

    #[inline(always)]
    pub fn zwzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 2, 3>(self)
    }

    #[inline(always)]
    pub fn zwwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 3, 0>(self)
    }

    #[inline(always)]
    pub fn zwwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 3, 1>(self)
    }

    #[inline(always)]
    pub fn zwwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 3, 2>(self)
    }

    #[inline(always)]
    pub fn zwww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 2, 3, 3, 3>(self)
    }

    #[inline(always)]
    pub fn wxxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 0, 0>(self)
    }

    #[inline(always)]
    pub fn wxxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 0, 1>(self)
    }

    #[inline(always)]
    pub fn wxxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 0, 2>(self)
    }

    #[inline(always)]
    pub fn wxxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 0, 3>(self)
    }

    #[inline(always)]
    pub fn wxyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 1, 0>(self)
    }

    #[inline(always)]
    pub fn wxyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 1, 1>(self)
    }

    #[inline(always)]
    pub fn wxyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 1, 2>(self)
    }

    #[inline(always)]
    pub fn wxyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 1, 3>(self)
    }

    #[inline(always)]
    pub fn wxzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 2, 0>(self)
    }

    #[inline(always)]
    pub fn wxzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 2, 1>(self)
    }

    #[inline(always)]
    pub fn wxzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 2, 2>(self)
    }

    #[inline(always)]
    pub fn wxzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 2, 3>(self)
    }

    #[inline(always)]
    pub fn wxwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 3, 0>(self)
    }

    #[inline(always)]
    pub fn wxwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 3, 1>(self)
    }

    #[inline(always)]
    pub fn wxwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 3, 2>(self)
    }

    #[inline(always)]
    pub fn wxww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 0, 3, 3>(self)
    }

    #[inline(always)]
    pub fn wyxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 0, 0>(self)
    }

    #[inline(always)]
    pub fn wyxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 0, 1>(self)
    }

    #[inline(always)]
    pub fn wyxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 0, 2>(self)
    }

    #[inline(always)]
    pub fn wyxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 0, 3>(self)
    }

    #[inline(always)]
    pub fn wyyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 1, 0>(self)
    }

    #[inline(always)]
    pub fn wyyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 1, 1>(self)
    }

    #[inline(always)]
    pub fn wyyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 1, 2>(self)
    }

    #[inline(always)]
    pub fn wyyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 1, 3>(self)
    }

    #[inline(always)]
    pub fn wyzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 2, 0>(self)
    }

    #[inline(always)]
    pub fn wyzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 2, 1>(self)
    }

    #[inline(always)]
    pub fn wyzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 2, 2>(self)
    }

    #[inline(always)]
    pub fn wyzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 2, 3>(self)
    }

    #[inline(always)]
    pub fn wywx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 3, 0>(self)
    }

    #[inline(always)]
    pub fn wywy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 3, 1>(self)
    }

    #[inline(always)]
    pub fn wywz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 3, 2>(self)
    }

    #[inline(always)]
    pub fn wyww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 1, 3, 3>(self)
    }

    #[inline(always)]
    pub fn wzxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 0, 0>(self)
    }

    #[inline(always)]
    pub fn wzxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 0, 1>(self)
    }

    #[inline(always)]
    pub fn wzxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 0, 2>(self)
    }

    #[inline(always)]
    pub fn wzxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 0, 3>(self)
    }

    #[inline(always)]
    pub fn wzyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 1, 0>(self)
    }

    #[inline(always)]
    pub fn wzyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 1, 1>(self)
    }

    #[inline(always)]
    pub fn wzyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 1, 2>(self)
    }

    #[inline(always)]
    pub fn wzyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 1, 3>(self)
    }

    #[inline(always)]
    pub fn wzzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 2, 0>(self)
    }

    #[inline(always)]
    pub fn wzzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 2, 1>(self)
    }

    #[inline(always)]
    pub fn wzzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 2, 2>(self)
    }

    #[inline(always)]
    pub fn wzzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 2, 3>(self)
    }

    #[inline(always)]
    pub fn wzwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 3, 0>(self)
    }

    #[inline(always)]
    pub fn wzwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 3, 1>(self)
    }

    #[inline(always)]
    pub fn wzwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 3, 2>(self)
    }

    #[inline(always)]
    pub fn wzww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 2, 3, 3>(self)
    }

    #[inline(always)]
    pub fn wwxx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 0, 0>(self)
    }

    #[inline(always)]
    pub fn wwxy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 0, 1>(self)
    }

    #[inline(always)]
    pub fn wwxz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 0, 2>(self)
    }

    #[inline(always)]
    pub fn wwxw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 0, 3>(self)
    }

    #[inline(always)]
    pub fn wwyx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 1, 0>(self)
    }

    #[inline(always)]
    pub fn wwyy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 1, 1>(self)
    }

    #[inline(always)]
    pub fn wwyz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 1, 2>(self)
    }

    #[inline(always)]
    pub fn wwyw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 1, 3>(self)
    }

    #[inline(always)]
    pub fn wwzx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 2, 0>(self)
    }

    #[inline(always)]
    pub fn wwzy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 2, 1>(self)
    }

    #[inline(always)]
    pub fn wwzz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 2, 2>(self)
    }

    #[inline(always)]
    pub fn wwzw(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 2, 3>(self)
    }

    #[inline(always)]
    pub fn wwwx(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 3, 0>(self)
    }

    #[inline(always)]
    pub fn wwwy(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 3, 1>(self)
    }

    #[inline(always)]
    pub fn wwwz(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 3, 2>(self)
    }

    #[inline(always)]
    pub fn wwww(self) -> Vector<4, T, A> {
        T::vec_swizzle4::<4, A, 3, 3, 3, 3>(self)
    }
}
