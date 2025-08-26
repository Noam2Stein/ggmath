use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
    T: PartialEq<T2>,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_eq(self, other)
    }

    #[inline(always)]
    fn ne(&self, other: &Vector<N, T2, A2>) -> bool {
        T::vec_ne(self, other)
    }
}

impl<const N: usize, T: Scalar + Eq, A: VecAlignment> Eq for Vector<N, T, A> where Usize<N>: VecLen {}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn eq_mask<T2: Scalar>(&self, rhs: &Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_eq_mask(self, rhs)
    }

    #[inline(always)]
    fn ne_mask<T2: Scalar>(&self, rhs: &Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialEq<T2>,
    {
        T::vec_ne_mask(self, rhs)
    }

    #[inline(always)]
    fn lt_mask<T2: Scalar>(&self, rhs: &Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_lt_mask(self, rhs)
    }

    #[inline(always)]
    fn le_mask<T2: Scalar>(&self, rhs: &Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_le_mask(self, rhs)
    }

    #[inline(always)]
    fn gt_mask<T2: Scalar>(&self, rhs: &Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_gt_mask(self, rhs)
    }

    #[inline(always)]
    fn ge_mask<T2: Scalar>(&self, rhs: &Vector<N, T2, impl VecAlignment>) -> Vector<N, bool, A>
    where
        T: PartialOrd<T2>,
    {
        T::vec_ge_mask(self, rhs)
    }
}
