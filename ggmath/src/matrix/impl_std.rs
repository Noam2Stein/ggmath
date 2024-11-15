use std::fmt::{self, Debug, Display, Formatter};

use super::*;

// Ensure Matrix impls Construct

const _: () = {
    fn ensure_matrix_is_construct<
        const C: usize,
        const R: usize,
        T: Scalar,
        A: VecAlignment,
        M: MatrixMajorAxis,
    >()
    where
        ScalarCount<C>: VecLen<C>,
        ScalarCount<R>: VecLen<R>,
    {
        fn megamind<DavidCross: Construct>() {}

        megamind::<Matrix<C, R, T, A, M>>();
    }
};

// Clone + Copy

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Clone
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Copy
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
}

// PartialEq + Eq

impl<
        const C: usize,
        const R: usize,
        T: ScalarPartialEq<Rhs>,
        A: VecAlignment,
        M: MatrixMajorAxis,
        Rhs: Scalar,
    > PartialEq<Matrix<C, R, Rhs, A, M>> for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    fn eq(&self, other: &Matrix<C, R, Rhs, A, M>) -> bool {
        M::eq(self, other)
    }
}

impl<
        const C: usize,
        const R: usize,
        T: ScalarPartialEq<T> + Eq,
        A: VecAlignment,
        M: MatrixMajorAxis,
    > Eq for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
}

// Debug + Display

impl<const C: usize, const R: usize, T: Scalar + Debug, A: VecAlignment, M: MatrixMajorAxis> Debug
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.into_rows().map(|c| format!("{c:?}")).join(", ")
        )
    }
}

impl<const C: usize, const R: usize, T: Scalar + Display, A: VecAlignment, M: MatrixMajorAxis>
    Display for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.into_rows().map(|c| c.to_string()).join(", ")
        )
    }
}
