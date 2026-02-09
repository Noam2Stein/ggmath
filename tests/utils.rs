use std::{
    fmt::{Arguments, Debug},
    panic::{UnwindSafe, catch_unwind},
};

use colored::Colorize;
use ggmath::{Alignment, Length, Matrix, Quaternion, Scalar, SupportedLength, Vector};

////////////////////////////////////////////////////////////////////////////////
// Float Eq
////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! float_eq {
    ($left:expr, $right:expr $(,)?) => {
        $crate::utils::FloatEq::eq(&$left, &$right, false)
    };
    ($left:expr, $right:expr, 0.0 = -0.0 $(,)?) => {
        $crate::utils::FloatEq::eq(&$left, &$right, true)
    };
    ($left:expr, $right:expr, tol = $tol:expr $(,)?) => {
        $crate::utils::FloatEq::eq_tol(&$left, &$right, $tol, false)
    };
    ($left:expr, $right:expr, tol = $tol:expr, 0.0 = -0.0 $(,)?) => {
        $crate::utils::FloatEq::eq_tol(&$left, &$right, $tol, true)
    };
    ($left:expr, $right:expr, ulps <= $max_ulps:expr $(,)?) => {
        $crate::utils::FloatEq::eq_ulps(&$left, &$right, $max_ulps, false)
    };
    ($left:expr, $right:expr, ulps <= $max_ulps:expr, 0.0 = -0.0 $(,)?) => {
        $crate::utils::FloatEq::eq_ulps(&$left, &$right, $max_ulps, true)
    };
}

#[macro_export]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr $(,)?) => {
        if !$crate::utils::FloatEq::eq(&$left, &$right, false) {
            $crate::utils::assert_float_eq_failed($left, $right, None);
        }
    };
    ($left:expr, $right:expr, 0.0 = -0.0 $(,)?) => {
        if !$crate::utils::FloatEq::eq(&$left, &$right, true) {
            $crate::utils::assert_float_eq_failed($left, $right, None);
        }
    };
    ($left:expr, $right:expr, 0.0 = -0.0, $($arg:tt)+) => {
        if !$crate::utils::FloatEq::eq(&$left, &$right, true) {
            $crate::utils::assert_float_eq_failed($left, $right, Some(format_args!($($arg)*)));
        }
    };
    ($left:expr, $right:expr, tol = $tol:expr $(,)?) => {
        if !$crate::utils::FloatEq::eq_tol(&$left, &$right, $tol, false) {
            $crate::utils::assert_float_eq_failed($left, $right, None);
        }
    };
    ($left:expr, $right:expr, tol = $tol:expr, $($arg:tt)+) => {
        if !$crate::utils::FloatEq::eq_tol(&$left, &$right, $tol, false) {
            $crate::utils::assert_float_eq_failed($left, $right, Some(format_args!($($arg)*)));
        }
    };
    ($left:expr, $right:expr, tol = $tol:expr, 0.0 = -0.0 $(,)?) => {
        if !$crate::utils::FloatEq::eq_tol(&$left, &$right, $tol, true) {
            $crate::utils::assert_float_eq_failed($left, $right, None);
        }
    };
    ($left:expr, $right:expr, tol = $tol:expr, 0.0 = -0.0, $($arg:tt)+) => {
        if !$crate::utils::FloatEq::eq_tol(&$left, &$right, $tol, true) {
            $crate::utils::assert_float_eq_failed($left, $right, Some(format_args!($($arg)*)));
        }
    };
    ($left:expr, $right:expr, ulps <= $max_ulps:expr $(,)?) => {
        if !$crate::utils::FloatEq::eq_ulps(&$left, &$right, $max_ulps, false) {
            $crate::utils::assert_float_eq_failed($left, $right, None);
        }
    };
    ($left:expr, $right:expr, ulps <= $max_ulps:expr, $($arg:tt)+) => {
        if !$crate::utils::FloatEq::eq_ulps(&$left, &$right, $max_ulps, false) {
            $crate::utils::assert_float_eq_failed($left, $right, Some(format_args!($($arg)*)));
        }
    };
    ($left:expr, $right:expr, ulps <= $max_ulps:expr, 0.0 = -0.0 $(,)?) => {
        if !$crate::utils::FloatEq::eq_ulps(&$left, &$right, $max_ulps, true) {
            $crate::utils::assert_float_eq_failed($left, $right, None);
        }
    };
    ($left:expr, $right:expr, ulps <= $max_ulps:expr, 0.0 = -0.0, $($arg:tt)+) => {
        if !$crate::utils::FloatEq::eq_ulps(&$left, &$right, $max_ulps, true) {
            $crate::utils::assert_float_eq_failed($left, $right, Some(format_args!($($arg)*)));
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        if !$crate::utils::FloatEq::eq(&$left, &$right, false) {
            $crate::utils::assert_float_eq_failed($left, $right, Some(format_args!($($arg)*)));
        }
    };
}

#[doc(hidden)]
#[track_caller]
pub fn assert_float_eq_failed<T: Debug + FloatEq>(
    left_val: T,
    right_val: T,
    args: Option<Arguments>,
) {
    if let Some(args) = args {
        panic!(
            concat!(
                "assertion `left == right` failed: {}\n",
                "  left: {:?}\n",
                " right: {:?}",
            ),
            args, left_val, right_val
        )
    } else {
        panic!(
            concat!(
                "assertion `left == right` failed\n",
                "  left: {:?}\n",
                " right: {:?}",
            ),
            left_val, right_val
        )
    }
}

#[doc(hidden)]
pub trait FloatEq {
    type Tol;

    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool;

    fn eq_tol(&self, other: &Self, tol: Self::Tol, zero_eq_neg_zero: bool) -> bool;

    fn eq_ulps(&self, other: &Self, max_ulps: u8, zero_eq_neg_zero: bool) -> bool;
}

impl FloatEq for f32 {
    type Tol = Self;

    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_nan() || other.is_nan() {
            false
        } else if self.is_sign_positive() != other.is_sign_positive() {
            *self == 0.0 && *other == 0.0 && zero_eq_neg_zero
        } else {
            self == other
        }
    }

    fn eq_tol(&self, other: &Self, tol: Self::Tol, zero_eq_neg_zero: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_nan() || other.is_nan() {
            false
        } else if self.is_sign_positive() != other.is_sign_positive() {
            *self == 0.0 && *other == 0.0 && zero_eq_neg_zero
        } else {
            (self - other).abs() <= tol
        }
    }

    fn eq_ulps(&self, other: &Self, max_ulps: u8, zero_eq_neg_zero: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_nan() || other.is_nan() {
            false
        } else if self.is_sign_positive() != other.is_sign_positive() {
            *self == 0.0 && *other == 0.0 && zero_eq_neg_zero
        } else {
            let self_bits = self.to_bits();
            let other_bits = other.to_bits();
            let max_bits = self_bits.max(other_bits);
            let min_bits = self_bits.min(other_bits);

            (max_bits - min_bits) <= max_ulps as u32
        }
    }
}

impl FloatEq for f64 {
    type Tol = Self;

    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_nan() || other.is_nan() {
            false
        } else if self.is_sign_positive() != other.is_sign_positive() {
            *self == 0.0 && *other == 0.0 && zero_eq_neg_zero
        } else {
            self == other
        }
    }

    fn eq_tol(&self, other: &Self, tol: Self::Tol, zero_eq_neg_zero: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_nan() || other.is_nan() {
            false
        } else if self.is_sign_positive() != other.is_sign_positive() {
            *self == 0.0 && *other == 0.0 && zero_eq_neg_zero
        } else {
            (self - other).abs() <= tol
        }
    }

    fn eq_ulps(&self, other: &Self, max_ulps: u8, zero_eq_neg_zero: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.is_nan() || other.is_nan() {
            false
        } else if self.is_sign_positive() != other.is_sign_positive() {
            *self == 0.0 && *other == 0.0 && zero_eq_neg_zero
        } else {
            let self_bits = self.to_bits();
            let other_bits = other.to_bits();
            let max_bits = self_bits.max(other_bits);
            let min_bits = self_bits.min(other_bits);

            (max_bits - min_bits) <= max_ulps as u64
        }
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + FloatEq<Tol = T>,
{
    type Tol = T;

    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self[i].eq(&other[i], zero_eq_neg_zero))
    }

    fn eq_tol(&self, other: &Self, tol: Self::Tol, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self[i].eq_tol(&other[i], tol, zero_eq_neg_zero))
    }

    fn eq_ulps(&self, other: &Self, max_ulps: u8, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self[i].eq_ulps(&other[i], max_ulps, zero_eq_neg_zero))
    }
}

impl<const N: usize, T, A: Alignment> FloatEq for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + FloatEq<Tol = T>,
{
    type Tol = T;

    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self.col(i).eq(&other.col(i), zero_eq_neg_zero))
    }

    fn eq_tol(&self, other: &Self, tol: Self::Tol, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| self.col(i).eq_tol(&other.col(i), tol, zero_eq_neg_zero))
    }

    fn eq_ulps(&self, other: &Self, max_ulps: u8, zero_eq_neg_zero: bool) -> bool {
        (0..N).all(|i| {
            self.col(i)
                .eq_ulps(&other.col(i), max_ulps, zero_eq_neg_zero)
        })
    }
}

impl<T, A: Alignment> FloatEq for Quaternion<T, A>
where
    T: Scalar + FloatEq<Tol = T>,
{
    type Tol = T;

    fn eq(&self, other: &Self, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq(self.as_vec_ref(), other.as_vec_ref(), zero_eq_neg_zero)
    }

    fn eq_tol(&self, other: &Self, tol: Self::Tol, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq_tol(self.as_vec_ref(), other.as_vec_ref(), tol, zero_eq_neg_zero)
    }

    fn eq_ulps(&self, other: &Self, max_ulps: u8, zero_eq_neg_zero: bool) -> bool {
        FloatEq::eq_ulps(
            self.as_vec_ref(),
            other.as_vec_ref(),
            max_ulps,
            zero_eq_neg_zero,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Assert Panic
////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! assert_panic {
    ($op:expr $(,)?) => {{
        $crate::utils::assert_panic_helper(|| { let _ = $op; }, None);
    }};
    ($op:expr, $($arg:tt)+) => {{
        $crate::utils::assert_panic_helper(|| { let _ = $op; }, Some(format_args!($($arg)+)));
    }};
}

#[doc(hidden)]
#[track_caller]
pub fn assert_panic_helper(f: impl FnOnce() + UnwindSafe, args: Option<Arguments>) {
    if catch_unwind(f).is_ok() {
        if let Some(args) = args {
            panic!("assertion `panic` failed: {args}");
        } else {
            panic!("assertion `panic` failed");
        }
    } else {
        println!("{}: panic is expected", "ok".green().bold());
    }
}

////////////////////////////////////////////////////////////////////////////////
// Padding
////////////////////////////////////////////////////////////////////////////////

pub fn vec3_with_padding<T, A: Alignment>(vec: Vector<3, T, A>, padding: T) -> Vector<3, T, A>
where
    T: Scalar,
{
    match size_of::<Vector<3, T, A>>() / size_of::<T>() {
        3 => vec,
        4 => {
            let mut result = vec;
            // SAFETY: There is guaranteed to be a fourth padding element
            // because of the vector's size.
            unsafe {
                *result.as_array_mut().as_mut_ptr().add(3) = padding;
            }

            result
        }
        _ => unreachable!(),
    }
}
