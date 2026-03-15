use std::fmt::Debug;

use crate::{Affine, Alignment, Length, Matrix, Quaternion, Scalar, SupportedLength, Vector};

macro_rules! float_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::test_utils::FloatEq::eq(&$left, &$right, false)
    };
    ($left:expr, $right:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::FloatEq::eq(&$left, &$right, true)
    };
    ($left:expr, $right:expr, abs <= $tol:expr $(,)?) => {
        crate::test_utils::FloatEq::eq_abs(&$left, &$right, &$tol, false)
    };
    ($left:expr, $right:expr, abs <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::FloatEq::eq_abs(&$left, &$right, &$tol, true)
    };
}

pub(crate) use float_eq;

macro_rules! assert_float_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::test_utils::FloatEq::assert_eq(&$left, &$right, false)
    };
    ($left:expr, $right:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::FloatEq::assert_eq(&$left, &$right, true)
    };
    ($left:expr, $right:expr, abs <= $tol:expr $(,)?) => {
        crate::test_utils::FloatEq::assert_eq_abs(&$left, &$right, &$tol, false)
    };
    ($left:expr, $right:expr, abs <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::FloatEq::assert_eq_abs(&$left, &$right, &$tol, true)
    };
}

pub(crate) use assert_float_eq;

#[doc(hidden)]
pub trait FloatEq {
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool;

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool;

    #[track_caller]
    fn assert_eq(&self, other: &Self, zero_eq_neg_zero: bool)
    where
        Self: Debug,
    {
        if !self.eq(other, zero_eq_neg_zero) {
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: {:?}\n",
                    " right: {:?}",
                ),
                self, other
            )
        }
    }

    #[track_caller]
    fn assert_eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool)
    where
        Self: Debug,
    {
        if !self.eq_abs(other, tol, zero_eq_neg_zero) {
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: {:?}\n",
                    " right: {:?}\n",
                    "   tol: {:?}\n"
                ),
                self, other, tol
            )
        }
    }
}

macro_rules! impl_float {
    ($T:ident) => {
        impl FloatEq for $T {
            fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
                if self.is_nan() || other.is_nan() {
                    self.is_nan() && other.is_nan()
                } else if *self == 0.0 && *other == 0.0 {
                    zero_eq_neg_zero || self.signum() == other.signum()
                } else {
                    self == other
                }
            }

            fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
                if self.is_nan() || other.is_nan() {
                    self.is_nan() && other.is_nan()
                } else if self.is_infinite() || other.is_infinite() {
                    self.is_infinite()
                        && other.is_infinite()
                        && self.is_sign_positive() == other.is_sign_positive()
                } else if *self == 0.0 && *other == 0.0 {
                    zero_eq_neg_zero || self.signum() == other.signum()
                } else {
                    (self - other).abs() <= *tol
                }
            }
        }
    };
}
impl_float!(f32);
impl_float!(f64);

impl<Ta, Tb> FloatEq for (Ta, Tb)
where
    Ta: FloatEq,
    Tb: FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        self.0.eq(&other.0, zero_eq_neg_zero) && self.1.eq(&other.1, zero_eq_neg_zero)
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        self.0.eq_abs(&other.0, &tol.0, zero_eq_neg_zero)
            && self.1.eq_abs(&other.1, &tol.1, zero_eq_neg_zero)
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self[i].eq(&other[i], zero_eq_neg_zero))
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self[i].eq_abs(&other[i], &tol[i], zero_eq_neg_zero))
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self.column(i).eq(&other.column(i), zero_eq_neg_zero))
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| {
            self.column(i)
                .eq_abs(&other.column(i), &tol.column(i), zero_eq_neg_zero)
        })
    }
}

impl<T, A: Alignment> FloatEq for Quaternion<T, A>
where
    T: Scalar + FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq(self.as_vec_ref(), other.as_vec_ref(), zero_eq_neg_zero)
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq_abs(
            self.as_vec_ref(),
            other.as_vec_ref(),
            tol.as_vec_ref(),
            zero_eq_neg_zero,
        )
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        self.matrix.eq(&other.matrix, zero_eq_neg_zero)
            && self.translation.eq(&other.translation, zero_eq_neg_zero)
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        self.matrix
            .eq_abs(&other.matrix, &tol.matrix, zero_eq_neg_zero)
            && self
                .translation
                .eq_abs(&other.translation, &tol.translation, zero_eq_neg_zero)
    }
}
