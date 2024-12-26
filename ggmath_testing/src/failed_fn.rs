use std::{
    any::type_name,
    fmt::{self, Display, Formatter},
};

use ggmath::{
    matrix::MatrixMajorAxis,
    scalar::Scalar,
    vector::{ScalarCount, VecAlignment, VecLen},
};

pub struct TestFnDesc(pub String);

impl TestFnDesc {
    pub fn vector<const N: usize, T: Scalar, A: VecAlignment>(fn_ident: &'static str) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        Self(format!(
            "Vector::<{N}, {}, {}>::{fn_ident}",
            type_name::<T>().split("::").last().unwrap_or(""),
            type_name::<A>().split("::").last().unwrap_or(""),
        ))
    }

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

    pub fn rectangle<const N: usize, T: Scalar, A: VecAlignment, R>(fn_ident: &'static str) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        Self(format!(
            "Rectangle::<{N}, {}, {}, {}>::{fn_ident}",
            type_name::<T>().split("::").last().unwrap_or(""),
            type_name::<A>().split("::").last().unwrap_or(""),
            type_name::<R>().split("::").last().unwrap_or(""),
        ))
    }
}

impl Display for TestFnDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
