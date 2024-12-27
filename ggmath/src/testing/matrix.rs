use std::any::type_name;

use crate::{
    matrix::{Matrix, MatrixMajorAxis},
    scalar::Scalar,
    vector::{ScalarCount, VecAlignment, VecLen},
};

use super::{TestEq, TestFnDesc};

pub use ggmath_proc_macros::mat_test_assert;

impl TestFnDesc {
    pub fn matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>(
        fn_ident: &'static str,
    ) -> Self
    where
        ScalarCount<C>: VecLen,
        ScalarCount<R>: VecLen,
    {
        Self(format!(
            "Matrix::<{C} {R}, {}, {}, {}>::{fn_ident}",
            type_name::<T>().split("::").last().unwrap_or(""),
            type_name::<A>().split("::").last().unwrap_or(""),
            type_name::<M>().split("::").last().unwrap_or(""),
        ))
    }
}

impl<
        const C: usize,
        const R: usize,
        T: Scalar + TestEq<TRhs>,
        A: VecAlignment,
        M: MatrixMajorAxis,
        TRhs: Scalar,
        ARhs: VecAlignment,
        MRhs: MatrixMajorAxis,
    > TestEq<Matrix<C, R, TRhs, ARhs, MRhs>> for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    fn test_eq(&self, other: &Matrix<C, R, TRhs, ARhs, MRhs>) -> bool {
        self.into_rows()
            .iter()
            .zip(other.into_rows().iter())
            .all(|(a, b)| a.test_eq(b))
    }
}
