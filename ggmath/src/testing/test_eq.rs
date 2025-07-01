use crate::{
    scalar::Scalar,
    vector::{MaybeVecLen, VecAlignment, VecLen, Vector},
};

pub trait TestEq<Rhs = Self> {
    fn test_eq(&self, other: &Rhs) -> bool;
}

impl TestEq for () {
    fn test_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<const N: usize, T: TestEq<Rhs>, Rhs> TestEq<[Rhs; N]> for [T; N] {
    fn test_eq(&self, other: &[Rhs; N]) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(value, other)| value.test_eq(other))
    }
}

impl<T: TestEq<Rhs>, Rhs> TestEq<Option<Rhs>> for Option<T> {
    fn test_eq(&self, other: &Option<Rhs>) -> bool {
        match self {
            Some(value) => match other {
                Some(other) => value.test_eq(other),
                None => false,
            },
            None => other.is_none(),
        }
    }
}

impl<T: TestEq<TRhs>, E: TestEq<ERhs>, TRhs, ERhs> TestEq<Result<TRhs, ERhs>> for Result<T, E> {
    fn test_eq(&self, other: &Result<TRhs, ERhs>) -> bool {
        match self {
            Ok(value) => match other {
                Ok(other) => value.test_eq(other),
                Err(_) => false,
            },
            Err(value) => match other {
                Ok(_) => false,
                Err(other) => value.test_eq(other),
            },
        }
    }
}

impl<
        const N: usize,
        T: Scalar + TestEq<TRhs>,
        A: VecAlignment,
        TRhs: Scalar,
        ARhs: VecAlignment,
    > TestEq<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    fn test_eq(&self, other: &Vector<N, TRhs, ARhs>) -> bool {
        self.as_array().test_eq(other.as_array())
    }
}

macro_rules! impl_test_eq_for_eqs {
    ($($type:ty), *) => {$(
        impl TestEq for $type {
            fn test_eq(&self, other: &Self) -> bool {
                self == other
            }
        }
    )*};
}
impl_test_eq_for_eqs!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, bool);

impl TestEq for f32 {
    fn test_eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_infinite()
            && other.is_infinite()
            && self.signum().test_eq(&other.signum())
        {
            true
        } else {
            (self - other).abs() < 0.0001
        }
    }
}
impl TestEq for f64 {
    fn test_eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_infinite()
            && other.is_infinite()
            && self.signum().test_eq(&other.signum())
        {
            true
        } else {
            (self - other).abs() < 0.000001
        }
    }
}
