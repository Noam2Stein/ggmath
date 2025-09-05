use std::ops::*;

use crate::{Scalar, Usize, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Neg<Output: Scalar>, A: VecAlignment> Neg for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn neg(self) -> Vector<N, T::Output, A> {
        T::vec_neg(self)
    }
}

impl<const N: usize, T: Scalar + Neg<Output: Scalar>, A: VecAlignment> Neg for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn neg(self) -> Vector<N, T::Output, A> {
        (*self).neg()
    }
}

impl<const N: usize, T: Scalar + Not<Output: Scalar>, A: VecAlignment> Not for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn not(self) -> Vector<N, T::Output, A> {
        T::vec_not(self)
    }
}

impl<const N: usize, T: Scalar + Not<Output: Scalar>, A: VecAlignment> Not for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn not(self) -> Vector<N, T::Output, A> {
        (*self).not()
    }
}

impl<const N: usize, T: Scalar + Add<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Add<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn add(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_add(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Add<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Add<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn add(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).add(rhs)
    }
}

impl<const N: usize, T: Scalar + Add<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Add<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn add(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.add(*rhs)
    }
}

impl<const N: usize, T: Scalar + Add<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Add<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn add(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).add(*rhs)
    }
}

impl<const N: usize, T: Scalar + AddAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> AddAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_add_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + AddAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> AddAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.add_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + Sub<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Sub<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sub(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_sub(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Sub<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Sub<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sub(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).sub(rhs)
    }
}

impl<const N: usize, T: Scalar + Sub<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Sub<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sub(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.sub(*rhs)
    }
}

impl<const N: usize, T: Scalar + Sub<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Sub<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sub(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).sub(*rhs)
    }
}

impl<const N: usize, T: Scalar + SubAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> SubAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_sub_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + SubAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> SubAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.sub_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + Mul<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Mul<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn mul(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_mul(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Mul<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Mul<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn mul(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).mul(rhs)
    }
}

impl<const N: usize, T: Scalar + Mul<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Mul<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn mul(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.mul(*rhs)
    }
}

impl<const N: usize, T: Scalar + Mul<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Mul<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn mul(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).mul(*rhs)
    }
}

impl<const N: usize, T: Scalar + MulAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> MulAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_mul_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + MulAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> MulAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.mul_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + Div<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Div<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn div(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_div(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Div<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Div<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn div(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).div(rhs)
    }
}

impl<const N: usize, T: Scalar + Div<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Div<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn div(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.div(*rhs)
    }
}

impl<const N: usize, T: Scalar + Div<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Div<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn div(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).div(*rhs)
    }
}

impl<const N: usize, T: Scalar + DivAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> DivAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_div_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + DivAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> DivAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.div_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + Rem<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Rem<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn rem(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_rem(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Rem<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Rem<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn rem(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).rem(rhs)
    }
}

impl<const N: usize, T: Scalar + Rem<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Rem<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn rem(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.rem(*rhs)
    }
}

impl<const N: usize, T: Scalar + Rem<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Rem<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn rem(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).rem(*rhs)
    }
}

impl<const N: usize, T: Scalar + RemAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> RemAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_rem_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + RemAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> RemAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn rem_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.rem_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + Shl<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shl<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shl(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_shl(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Shl<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shl<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shl(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).shl(rhs)
    }
}

impl<const N: usize, T: Scalar + Shl<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shl<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shl(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.shl(*rhs)
    }
}

impl<const N: usize, T: Scalar + Shl<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shl<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shl(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).shl(*rhs)
    }
}

impl<const N: usize, T: Scalar + ShlAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> ShlAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shl_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_shl_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + ShlAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> ShlAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shl_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.shl_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + Shr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shr<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shr(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_shr(self, rhs)
    }
}

impl<const N: usize, T: Scalar + Shr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shr<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shr(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).shr(rhs)
    }
}

impl<const N: usize, T: Scalar + Shr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shr<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shr(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.shr(*rhs)
    }
}

impl<const N: usize, T: Scalar + Shr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> Shr<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn shr(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).shr(*rhs)
    }
}

impl<const N: usize, T: Scalar + ShrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> ShrAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shr_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_shr_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + ShrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> ShrAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn shr_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.shr_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitAnd<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitAnd<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitand(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_bitand(self, rhs)
    }
}

impl<const N: usize, T: Scalar + BitAnd<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitAnd<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitand(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).bitand(rhs)
    }
}

impl<const N: usize, T: Scalar + BitAnd<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitAnd<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitand(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.bitand(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitAnd<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitAnd<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitand(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).bitand(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitAndAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitAndAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_bitand_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + BitAndAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitAndAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.bitand_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitOr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitOr<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitor(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_bitor(self, rhs)
    }
}

impl<const N: usize, T: Scalar + BitOr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitOr<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitor(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).bitor(rhs)
    }
}

impl<const N: usize, T: Scalar + BitOr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitOr<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitor(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.bitor(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitOr<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitOr<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitor(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).bitor(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitOrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitOrAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_bitor_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + BitOrAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitOrAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.bitor_assign(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitXor<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitXor<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitxor(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        T::vec_bitxor(self, rhs)
    }
}

impl<const N: usize, T: Scalar + BitXor<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitXor<Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitxor(self, rhs: Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).bitxor(rhs)
    }
}

impl<const N: usize, T: Scalar + BitXor<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitXor<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitxor(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        self.bitxor(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitXor<T2, Output: Scalar>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitXor<&Vector<N, T2, A2>> for &Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn bitxor(self, rhs: &Vector<N, T2, A2>) -> Vector<N, T::Output, A> {
        (*self).bitxor(*rhs)
    }
}

impl<const N: usize, T: Scalar + BitXorAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitXorAssign<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Vector<N, T2, A2>) {
        T::vec_bitxor_assign(self, rhs)
    }
}

impl<const N: usize, T: Scalar + BitXorAssign<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment> BitXorAssign<&Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: &Vector<N, T2, A2>) {
        self.bitxor_assign(*rhs)
    }
}

