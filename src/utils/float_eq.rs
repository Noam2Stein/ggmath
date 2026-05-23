extern crate std;

use std::fmt::Debug;

use crate::{Affine, Alignment, Length, Matrix, Quaternion, Scalar, SupportedLength, Vector};

macro_rules! float_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::utils::FloatEq::eq(&$left, &$right, false)
    };
    ($left:expr, $right:expr, 0.0 = -0.0 $(,)?) => {
        crate::utils::FloatEq::eq(&$left, &$right, true)
    };
    ($left:expr, $right:expr, abs <= $tol:expr $(,)?) => {
        crate::utils::FloatEq::eq_abs(&$left, &$right, &$tol, false)
    };
    ($left:expr, $right:expr, abs <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::utils::FloatEq::eq_abs(&$left, &$right, &$tol, true)
    };
    ($left:expr, $right:expr, r2nd <= $tol:expr $(,)?) => {
        crate::utils::FloatEq::eq_r2nd(&$left, &$right, &$tol, false)
    };
    ($left:expr, $right:expr, r2nd <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::utils::FloatEq::eq_r2nd(&$left, &$right, &$tol, true)
    };
}

pub(crate) use float_eq;

macro_rules! assert_float_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::utils::FloatEq::assert_eq(&$left, &$right, false)
    };
    ($left:expr, $right:expr, 0.0 = -0.0 $(,)?) => {
        crate::utils::FloatEq::assert_eq(&$left, &$right, true)
    };
    ($left:expr, $right:expr, abs <= $tol:expr $(,)?) => {
        crate::utils::FloatEq::assert_eq_abs(&$left, &$right, &$tol, false)
    };
    ($left:expr, $right:expr, abs <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::utils::FloatEq::assert_eq_abs(&$left, &$right, &$tol, true)
    };
    ($left:expr, $right:expr, r2nd <= $tol:expr $(,)?) => {
        crate::utils::FloatEq::assert_eq_r2nd(&$left, &$right, &$tol, false)
    };
    ($left:expr, $right:expr, r2nd <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::utils::FloatEq::assert_eq_r2nd(&$left, &$right, &$tol, true)
    };
}

pub(crate) use assert_float_eq;

#[doc(hidden)]
pub trait FloatEq: Sized {
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool;

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool;

    fn abs_mul(&self, rhs: &Self) -> Self;

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

    #[track_caller]
    fn assert_eq_r2nd(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool)
    where
        Self: Debug,
    {
        let tol = other.abs_mul(tol);
        if !self.eq_abs(other, &tol, zero_eq_neg_zero) {
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
                } else if tol.is_nan() {
                    self == other
                } else {
                    (self - other).abs() <= *tol
                }
            }

            fn abs_mul(&self, other: &Self) -> Self {
                self.abs() * other
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

    fn abs_mul(&self, rhs: &Self) -> Self {
        (self.0.abs_mul(&rhs.0), self.1.abs_mul(&rhs.1))
    }
}

impl<Ta, Tb, Tc> FloatEq for (Ta, Tb, Tc)
where
    Ta: FloatEq,
    Tb: FloatEq,
    Tc: FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        self.0.eq(&other.0, zero_eq_neg_zero)
            && self.1.eq(&other.1, zero_eq_neg_zero)
            && self.2.eq(&other.2, zero_eq_neg_zero)
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        self.0.eq_abs(&other.0, &tol.0, zero_eq_neg_zero)
            && self.1.eq_abs(&other.1, &tol.1, zero_eq_neg_zero)
            && self.2.eq_abs(&other.2, &tol.2, zero_eq_neg_zero)
    }

    fn abs_mul(&self, rhs: &Self) -> Self {
        (
            self.0.abs_mul(&rhs.0),
            self.1.abs_mul(&rhs.1),
            self.2.abs_mul(&rhs.2),
        )
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

    fn abs_mul(&self, rhs: &Self) -> Self {
        Self::from_fn(|i| self[i].abs_mul(&rhs[i]))
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Matrix<N, T, A>
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

    fn abs_mul(&self, rhs: &Self) -> Self {
        Self::from_row_fn(|i| self[i].abs_mul(&rhs[i]))
    }
}

impl<T, A: Alignment> FloatEq for Quaternion<T, A>
where
    T: Scalar + FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq(
            self.as_vector_ref(),
            other.as_vector_ref(),
            zero_eq_neg_zero,
        )
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq_abs(
            self.as_vector_ref(),
            other.as_vector_ref(),
            tol.as_vector_ref(),
            zero_eq_neg_zero,
        )
    }

    fn abs_mul(&self, rhs: &Self) -> Self {
        Self::from_vector(self.as_vector_ref().abs_mul(rhs.as_vector_ref()))
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + FloatEq,
{
    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        self.submatrix.eq(&other.submatrix, zero_eq_neg_zero)
            && self.translation.eq(&other.translation, zero_eq_neg_zero)
    }

    fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
        self.submatrix
            .eq_abs(&other.submatrix, &tol.submatrix, zero_eq_neg_zero)
            && self
                .translation
                .eq_abs(&other.translation, &tol.translation, zero_eq_neg_zero)
    }

    fn abs_mul(&self, rhs: &Self) -> Self {
        Self::from_submatrix_translation(
            self.submatrix.abs_mul(&rhs.submatrix),
            self.translation.abs_mul(&rhs.translation),
        )
    }
}

#[cfg(feature = "wide")]
mod wide {
    use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

    use crate::utils::FloatEq;

    macro_rules! impl_wide_float {
        ($Wide:ident, $T:ident, $LANES:literal) => {
            impl FloatEq for $Wide {
                fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
                    (0..$LANES).all(|i| {
                        FloatEq::eq(&self.as_array()[i], &other.as_array()[i], zero_eq_neg_zero)
                    })
                }

                fn eq_abs(&self, other: &Self, tol: &Self, zero_eq_neg_zero: bool) -> bool {
                    (0..$LANES).all(|i| {
                        FloatEq::eq_abs(
                            &self.as_array()[i],
                            &other.as_array()[i],
                            &tol.as_array()[i],
                            zero_eq_neg_zero,
                        )
                    })
                }

                fn abs_mul(&self, rhs: &Self) -> Self {
                    $Wide::new(self.as_array().map($T::abs)) * rhs
                }
            }
        };
    }
    impl_wide_float!(f32x4, f32, 4);
    impl_wide_float!(f32x8, f32, 8);
    impl_wide_float!(f32x16, f32, 16);
    impl_wide_float!(f64x2, f64, 2);
    impl_wide_float!(f64x4, f64, 4);
    impl_wide_float!(f64x8, f64, 8);
}
