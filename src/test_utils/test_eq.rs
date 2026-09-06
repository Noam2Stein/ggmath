extern crate std;

use std::{
    fmt::{Debug, Display},
    ops::Neg,
};

use crate::{
    Affine, Alignment, Length, Matrix, PrimitiveInteger, Projective, Quaternion, Rotation2, Scalar,
    SupportedLength, Vector, length::TwoOrThree, utils::specialize_23,
};

/// Checks for equality with specific rules for each type.
///
/// For floats this differs from regular equality by treating NaNs as equal and
/// treating `0.0` and `-0.0` as not equal. For integers and booleans this check
/// is the same as normal equality.
///
/// Optional flags:
///
/// - `abs <= {expr}`: Specifies the maximum absolute
///   difference between two float arguments for them to be considered equal.
///   This may accept multiple types depending on `T`.
///
/// - `0.0 = -0.0`: Treats zeros with different signs as equal.
///
/// - `INFINITY = NAN`: Treats infinity, negative infinity and NaNs as equal.
///
/// - `quat = -quat`: treats opposite quaternions as equal, because they
///   represent the same rotation.
macro_rules! test_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, false, false, false)
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, true, false, false)
    };
    ($actual:expr, $expected:expr, INFINITY = NAN $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, false, true, false)
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, INFINITY = NAN $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, true, true, false)
    };
    ($actual:expr, $expected:expr, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, false, false, true)
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, true, false, true)
    };
    ($actual:expr, $expected:expr, INFINITY = NAN, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, false, true, true)
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, INFINITY = NAN, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_helper(&$actual, &$expected, true, true, true)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, false, false, false)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, true, false, false)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, INFINITY = NAN $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, false, true, false)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0, INFINITY = NAN $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, true, true, false)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, false, false, true)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, true, false, true)
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, INFINITY = NAN, quat = -quat $(,)?) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, false, true, true)
    };
    (
        $actual:expr,
        $expected:expr,
        abs <= $tol:expr,
        0.0 = -0.0,
        INFINITY = NAN,
        quat = -quat
        $(,)?
    ) => {
        crate::test_utils::test_eq_abs_helper(&$actual, &$expected, &$tol, true, true, true)
    };
}
pub(crate) use test_eq;

/// Asserts equality with specific rules for each type.
///
/// For floats this differs from regular equality by treating NaNs as equal and
/// treating `0.0` and `-0.0` as not equal. For integers and booleans this check
/// is the same as normal equality.
///
/// Optional flags:
///
/// - `abs <= {expr}`: Specifies the maximum absolute
///   difference between two float arguments for them to be considered equal.
///   This may accept multiple types depending on `T`.
///
/// - `0.0 = -0.0`: Treats zeros with different signs as equal.
///
/// - `INFINITY = NAN`: Treats infinity, negative infinity and NaNs as equal.
///
/// - `quat = -quat`: treats opposite quaternions as equal, because they
///   represent the same rotation.
macro_rules! assert_test_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, false, false, false, "")
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, true, false, false, "")
    };
    ($actual:expr, $expected:expr, INFINITY = NAN $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, false, true, false, "")
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, INFINITY = NAN $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, true, true, false, "")
    };
    ($actual:expr, $expected:expr, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, false, false, true, "")
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, true, false, true, "")
    };
    ($actual:expr, $expected:expr, INFINITY = NAN, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, false, true, true, "")
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, INFINITY = NAN, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_helper(&$actual, &$expected, true, true, true, "")
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            false,
            false,
            false,
            "",
        )
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0 $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(&$actual, &$expected, &$tol, true, false, false, "")
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, INFINITY = NAN $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(&$actual, &$expected, &$tol, false, true, false, "")
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0, INFINITY = NAN $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(&$actual, &$expected, &$tol, true, true, false, "")
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(&$actual, &$expected, &$tol, false, false, true, "")
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(&$actual, &$expected, &$tol, true, false, true, "")
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, INFINITY = NAN, quat = -quat $(,)?) => {
        crate::test_utils::assert_test_eq_abs_helper(&$actual, &$expected, &$tol, false, true, true, "")
    };
    (
        $actual:expr,
        $expected:expr,
        abs <= $tol:expr,
        0.0 = -0.0,
        INFINITY = NAN,
        quat = -quat
        $(,)?
    ) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            true,
            true,
            true,
            "",
        )
    };
    (
        $actual:expr,
        $expected:expr,
        abs <= $tol:expr,
        0.0 = -0.0,
        INFINITY = NAN,
        quat = -quat,
        $($message:tt)+
    ) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            true,
            true,
            true,
            format_args!($($message)+),
        )
    };
    (
        $actual:expr,
        $expected:expr,
        abs <= $tol:expr,
        INFINITY = NAN,
        quat = -quat,
        $($message:tt)+
    ) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            false,
            true,
            true,
            format_args!($($message)+),
        )
    };
    (
        $actual:expr,
        $expected:expr,
        abs <= $tol:expr,
        0.0 = -0.0,
        quat = -quat,
        $($message:tt)+
    ) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            true,
            false,
            true,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, quat = -quat, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            false,
            false,
            true,
            format_args!($($message)+),
        )
    };
    (
        $actual:expr,
        $expected:expr,
        abs <= $tol:expr,
        0.0 = -0.0,
        INFINITY = NAN,
        $($message:tt)+
    ) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            true,
            true,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, INFINITY = NAN, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            false,
            true,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, 0.0 = -0.0, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            true,
            false,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, abs <= $tol:expr, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_abs_helper(
            &$actual,
            &$expected,
            &$tol,
            false,
            false,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, INFINITY = NAN, quat = -quat, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            true,
            true,
            true,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, INFINITY = NAN, quat = -quat, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            false,
            true,
            true,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, quat = -quat, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            true,
            false,
            true,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, quat = -quat, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            false,
            false,
            true,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, INFINITY = NAN, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            true,
            true,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, INFINITY = NAN, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            false,
            true,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, 0.0 = -0.0, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            true,
            false,
            false,
            format_args!($($message)+),
        )
    };
    ($actual:expr, $expected:expr, $($message:tt)+) => {
        crate::test_utils::assert_test_eq_helper(
            &$actual,
            &$expected,
            false,
            false,
            false,
            format_args!($($message)+),
        )
    };
}
pub(crate) use assert_test_eq;

#[doc(hidden)]
#[expect(private_bounds)]
#[expect(dead_code)]
pub fn test_eq_helper<T>(
    actual: &T,
    expected: &T,
    zero_eq_neg_zero: bool,
    infinity_eq_nan: bool,
    quat_eq_neg_quat: bool,
) -> bool
where
    T: TestEq,
{
    T::eq(
        actual,
        expected,
        zero_eq_neg_zero,
        infinity_eq_nan,
        quat_eq_neg_quat,
    )
}

#[doc(hidden)]
#[expect(private_bounds)]
pub fn test_eq_abs_helper<T, Tol>(
    actual: &T,
    expected: &T,
    tol: &Tol,
    zero_eq_neg_zero: bool,
    infinity_eq_nan: bool,
    quat_eq_neg_quat: bool,
) -> bool
where
    T: TestEqAbs<Tol>,
{
    T::eq(
        actual,
        expected,
        tol,
        zero_eq_neg_zero,
        infinity_eq_nan,
        quat_eq_neg_quat,
    )
}

#[doc(hidden)]
#[track_caller]
#[expect(private_bounds)]
pub fn assert_test_eq_helper<T>(
    actual: &T,
    expected: &T,
    zero_eq_neg_zero: bool,
    infinity_eq_nan: bool,
    quat_eq_neg_quat: bool,
    message: impl Display,
) where
    T: Debug + TestEq,
{
    if !actual.eq(
        expected,
        zero_eq_neg_zero,
        infinity_eq_nan,
        quat_eq_neg_quat,
    ) {
        panic!(
            concat!(
                "assertion `actual == expected` failed\n",
                "  actual: {:?}\n",
                "expected: {:?}\n",
                "{}"
            ),
            actual, expected, message
        )
    }
}

#[doc(hidden)]
#[track_caller]
#[expect(private_bounds)]
pub fn assert_test_eq_abs_helper<T, Tol>(
    actual: &T,
    expected: &T,
    tol: &Tol,
    zero_eq_neg_zero: bool,
    infinity_eq_nan: bool,
    quat_eq_neg_quat: bool,
    message: impl Display,
) where
    T: Debug + TestEqAbs<Tol>,
    Tol: Debug,
{
    if !actual.eq(
        expected,
        tol,
        zero_eq_neg_zero,
        infinity_eq_nan,
        quat_eq_neg_quat,
    ) {
        panic!(
            concat!(
                "assertion `actual == expected` failed\n",
                "  actual: {:?}\n",
                "expected: {:?}\n",
                "     tol: {:?}\n",
                "{}"
            ),
            actual, expected, tol, message
        )
    }
}

trait TestEq: Sized {
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool;
}

trait TestEqAbs<Tol = Self> {
    fn eq(
        &self,
        expected: &Self,
        tol: &Tol,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool;
}

macro_rules! float_impl {
    ($T:ident) => {
        impl TestEq for $T {
            fn eq(
                &self,
                expected: &Self,
                zero_eq_neg_zero: bool,
                infinity_eq_nan: bool,
                _quat_eq_neg_quat: bool,
            ) -> bool {
                if infinity_eq_nan && !self.is_finite() && !expected.is_finite() {
                    true
                } else if self.is_nan() || expected.is_nan() {
                    self.is_nan() && expected.is_nan()
                } else if *self == 0.0 && *expected == 0.0 {
                    zero_eq_neg_zero || self.signum() == expected.signum()
                } else {
                    self == expected
                }
            }
        }

        impl TestEqAbs for $T {
            fn eq(
                &self,
                expected: &Self,
                tol: &Self,
                zero_eq_neg_zero: bool,
                infinity_eq_nan: bool,
                _quat_eq_neg_quat: bool,
            ) -> bool {
                if infinity_eq_nan && !self.is_finite() && !expected.is_finite() {
                    true
                } else if self.is_nan() || expected.is_nan() {
                    self.is_nan() && expected.is_nan()
                } else if self.is_infinite() || expected.is_infinite() {
                    self.is_infinite()
                        && expected.is_infinite()
                        && self.is_sign_positive() == expected.is_sign_positive()
                } else if *self == 0.0 && *expected == 0.0 {
                    zero_eq_neg_zero || self.signum() == expected.signum()
                } else if tol.is_nan() {
                    self == expected
                } else {
                    (self - expected).abs() <= *tol
                }
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);

impl<T> TestEq for T
where
    T: PrimitiveInteger,
{
    fn eq(
        &self,
        expected: &Self,
        _zero_eq_neg_zero: bool,
        _infinity_eq_nan: bool,
        _quat_eq_neg_quat: bool,
    ) -> bool {
        self == expected
    }
}

impl TestEq for bool {
    fn eq(
        &self,
        expected: &Self,
        _zero_eq_neg_zero: bool,
        _infinity_eq_nan: bool,
        _quat_eq_neg_quat: bool,
    ) -> bool {
        self == expected
    }
}

impl TestEq for () {
    fn eq(
        &self,
        (): &Self,
        _zero_eq_neg_zero: bool,
        _infinity_eq_nan: bool,
        _quat_eq_neg_quat: bool,
    ) -> bool {
        true
    }
}

impl<T0, T1> TestEq for (T0, T1)
where
    T0: TestEq,
    T1: TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.0.eq(
            &expected.0,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.1.eq(
            &expected.1,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T0, Tol0, T1, Tol1> TestEqAbs<(Tol0, Tol1)> for (T0, T1)
where
    T0: TestEqAbs<Tol0>,
    T1: TestEqAbs<Tol1>,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &(Tol0, Tol1),
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.0.eq(
            &expected.0,
            &tol.0,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.1.eq(
            &expected.1,
            &tol.1,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T0, T1, T2> TestEq for (T0, T1, T2)
where
    T0: TestEq,
    T1: TestEq,
    T2: TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.0.eq(
            &expected.0,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.1.eq(
            &expected.1,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.2.eq(
            &expected.2,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T0, Tol0, T1, Tol1, T2, Tol2> TestEqAbs<(Tol0, Tol1, Tol2)> for (T0, T1, T2)
where
    T0: TestEqAbs<Tol0>,
    T1: TestEqAbs<Tol1>,
    T2: TestEqAbs<Tol2>,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &(Tol0, Tol1, Tol2),
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.0.eq(
            &expected.0,
            &tol.0,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.1.eq(
            &expected.1,
            &tol.1,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.2.eq(
            &expected.2,
            &tol.2,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T> TestEq for Option<T>
where
    T: TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        match (self, expected) {
            (Some(actual), Some(expected)) => actual.eq(
                expected,
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            ),
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
        }
    }
}

impl<const N: usize, T, A: Alignment> TestEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        (0..N).all(|i| {
            self[i].eq(
                &expected[i],
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        })
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        (0..N).all(|i| {
            self[i].eq(
                &expected[i],
                &tol[i],
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        })
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs<T> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &T,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        (0..N).all(|i| {
            self[i].eq(
                &expected[i],
                tol,
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        })
    }
}

impl<const N: usize, T, A: Alignment> TestEq for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        (0..N).all(|i| {
            self[i].eq(
                &expected[i],
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        })
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        (0..N).all(|i| {
            self[i].eq(
                &expected[i],
                &tol[i],
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        })
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs<T> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &T,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        (0..N).all(|i| {
            self[i].eq(
                &expected[i],
                tol,
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        })
    }
}

impl<const N: usize, T, A: Alignment> TestEq for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.matrix.eq(
            &expected.matrix,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.translation.eq(
            &expected.translation,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.matrix.eq(
            &expected.matrix,
            &tol.matrix,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.translation.eq(
            &expected.translation,
            &tol.translation,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs<T> for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &T,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        self.matrix.eq(
            &expected.matrix,
            tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ) && self.translation.eq(
            &expected.translation,
            tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<const N: usize, T, A: Alignment> TestEq for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        specialize_23!(Projective::<N, T, A>::test_eq_backend(
            self,
            expected,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ))
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        specialize_23!(Projective::<N, T, A>::test_eq_abs_backend(
            self,
            expected,
            tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ))
    }
}

impl<const N: usize, T, A: Alignment> TestEqAbs<T> for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &T,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        specialize_23!(Projective::<N, T, A>::test_eq_abs_scalar_backend(
            self,
            expected,
            *tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        ))
    }
}

macro_rules! projective_backend {
    ($N:literal) => {
        impl<T, A: Alignment> Projective<$N, T, A>
        where
            T: Scalar,
        {
            fn test_eq_backend(
                &self,
                expected: &Self,
                zero_eq_neg_zero: bool,
                infinity_eq_nan: bool,
                quat_eq_neg_quat: bool,
            ) -> bool
            where
                T: TestEq,
            {
                self.0.eq(
                    &expected.0,
                    zero_eq_neg_zero,
                    infinity_eq_nan,
                    quat_eq_neg_quat,
                )
            }

            fn test_eq_abs_backend(
                &self,
                expected: &Self,
                tol: &Self,
                zero_eq_neg_zero: bool,
                infinity_eq_nan: bool,
                quat_eq_neg_quat: bool,
            ) -> bool
            where
                T: TestEqAbs,
            {
                self.0.eq(
                    &expected.0,
                    &tol.0,
                    zero_eq_neg_zero,
                    infinity_eq_nan,
                    quat_eq_neg_quat,
                )
            }

            fn test_eq_abs_scalar_backend(
                &self,
                expected: &Self,
                tol: T,
                zero_eq_neg_zero: bool,
                infinity_eq_nan: bool,
                quat_eq_neg_quat: bool,
            ) -> bool
            where
                T: TestEqAbs,
            {
                self.0.eq(
                    &expected.0,
                    &tol,
                    zero_eq_neg_zero,
                    infinity_eq_nan,
                    quat_eq_neg_quat,
                )
            }
        }
    };
}
projective_backend!(2);
projective_backend!(3);

impl<T, A: Alignment> TestEq for Rotation2<T, A>
where
    T: Scalar + TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        TestEq::eq(
            self.as_vector(),
            expected.as_vector(),
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T, A: Alignment> TestEqAbs for Rotation2<T, A>
where
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        TestEqAbs::eq(
            self.as_vector(),
            expected.as_vector(),
            tol.as_vector(),
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T, A: Alignment> TestEqAbs<T> for Rotation2<T, A>
where
    T: Scalar + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &T,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        TestEqAbs::<T>::eq(
            self.as_vector(),
            expected.as_vector(),
            tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        )
    }
}

impl<T, A: Alignment> TestEq for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T> + TestEq,
{
    fn eq(
        &self,
        expected: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        let eq = TestEq::eq(
            self.as_vector(),
            expected.as_vector(),
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        );

        if quat_eq_neg_quat {
            eq || TestEq::eq(
                self.as_vector(),
                (-*expected).as_vector(),
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        } else {
            eq
        }
    }
}

impl<T, A: Alignment> TestEqAbs for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T> + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Self,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        let eq = Vector::<4, T, A>::eq(
            self.as_vector(),
            expected.as_vector(),
            tol.as_vector(),
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        );

        if quat_eq_neg_quat {
            eq || Vector::<4, T, A>::eq(
                self.as_vector(),
                (-*expected).as_vector(),
                tol.as_vector(),
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        } else {
            eq
        }
    }
}

impl<T, A: Alignment> TestEqAbs<T> for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T> + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &T,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        let eq = Vector::<4, T, A>::eq(
            self.as_vector(),
            expected.as_vector(),
            tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        );

        if quat_eq_neg_quat {
            eq || Vector::<4, T, A>::eq(
                self.as_vector(),
                (-*expected).as_vector(),
                tol,
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        } else {
            eq
        }
    }
}

impl<T, A: Alignment> TestEqAbs<Vector<4, T, A>> for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T> + TestEqAbs,
{
    fn eq(
        &self,
        expected: &Self,
        tol: &Vector<4, T, A>,
        zero_eq_neg_zero: bool,
        infinity_eq_nan: bool,
        quat_eq_neg_quat: bool,
    ) -> bool {
        let eq = Vector::<4, T, A>::eq(
            self.as_vector(),
            expected.as_vector(),
            tol,
            zero_eq_neg_zero,
            infinity_eq_nan,
            quat_eq_neg_quat,
        );

        if quat_eq_neg_quat {
            eq || Vector::<4, T, A>::eq(
                self.as_vector(),
                (-*expected).as_vector(),
                tol,
                zero_eq_neg_zero,
                infinity_eq_nan,
                quat_eq_neg_quat,
            )
        } else {
            eq
        }
    }
}

#[cfg(feature = "wide")]
mod wide {
    use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i32x4};

    use crate::test_utils::test_eq::{TestEq, TestEqAbs};

    macro_rules! wide_float_impl {
        ($Wide:ident, $T:ident, $LANES:literal) => {
            impl TestEq for $Wide {
                fn eq(
                    &self,
                    expected: &Self,
                    zero_eq_neg_zero: bool,
                    infinity_eq_nan: bool,
                    quat_eq_neg_quat: bool,
                ) -> bool {
                    (0..$LANES).all(|i| {
                        TestEq::eq(
                            &self.as_array()[i],
                            &expected.as_array()[i],
                            zero_eq_neg_zero,
                            infinity_eq_nan,
                            quat_eq_neg_quat,
                        )
                    })
                }
            }

            impl TestEqAbs for $Wide {
                fn eq(
                    &self,
                    expected: &Self,
                    tol: &Self,
                    zero_eq_neg_zero: bool,
                    infinity_eq_nan: bool,
                    quat_eq_neg_quat: bool,
                ) -> bool {
                    (0..$LANES).all(|i| {
                        TestEqAbs::eq(
                            &self.as_array()[i],
                            &expected.as_array()[i],
                            &tol.as_array()[i],
                            zero_eq_neg_zero,
                            infinity_eq_nan,
                            quat_eq_neg_quat,
                        )
                    })
                }
            }
        };
    }
    wide_float_impl!(f32x4, f32, 4);
    wide_float_impl!(f32x8, f32, 8);
    wide_float_impl!(f32x16, f32, 16);
    wide_float_impl!(f64x2, f64, 2);
    wide_float_impl!(f64x4, f64, 4);
    wide_float_impl!(f64x8, f64, 8);

    macro_rules! wide_integer_impl {
        ($T:ident) => {
            impl TestEq for $T {
                fn eq(
                    &self,
                    expected: &Self,
                    _zero_eq_neg_zero: bool,
                    _infinity_eq_nan: bool,
                    _quat_eq_neg_quat: bool,
                ) -> bool {
                    self == expected
                }
            }
        };
    }
    wide_integer_impl!(i32x4);
}
