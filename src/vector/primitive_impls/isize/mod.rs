use crate::{Alignment, Length, ScalarBackend, ScalarWrapper, SupportedLength, Vector};

impl<const N: usize, A: Alignment> ScalarBackend<N, A> for isize
where
    Length<N>: SupportedLength,
{
    #[cfg(target_pointer_width = "16")]
    type VectorRepr = Vector<N, i16, A>;

    #[cfg(target_pointer_width = "32")]
    type VectorRepr = Vector<N, i32, A>;

    #[cfg(target_pointer_width = "64")]
    type VectorRepr = Vector<N, i64, A>;

    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, A>, other: Vector<N, Self, A>) -> bool {
        vec.repr() == other.repr()
    }

    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, A>, other: Vector<N, Self, A>) -> bool {
        vec.repr() != other.repr()
    }

    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(-vec.repr())
    }

    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(!vec.repr())
    }

    #[inline(always)]
    fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() + rhs.repr())
    }

    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() - rhs.repr())
    }

    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() * rhs.repr())
    }

    #[inline(always)]
    fn vec_div(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() / rhs.repr())
    }

    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() % rhs.repr())
    }

    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() << rhs.repr())
    }

    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() >> rhs.repr())
    }

    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() & rhs.repr())
    }

    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() | rhs.repr())
    }

    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
        Vector::from_repr(vec.repr() ^ rhs.repr())
    }
}

#[cfg(target_pointer_width = "16")]
unsafe impl ScalarWrapper<i16> for isize {}

#[cfg(target_pointer_width = "32")]
unsafe impl ScalarWrapper<i32> for isize {}

#[cfg(target_pointer_width = "64")]
unsafe impl ScalarWrapper<i64> for isize {}
