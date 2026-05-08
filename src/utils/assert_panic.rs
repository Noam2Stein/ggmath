extern crate std;

use std::{
    fmt::Debug,
    panic::{UnwindSafe, catch_unwind},
    println,
};

use colored::Colorize;

macro_rules! assert_panic {
    ($expr:expr $(,)?) => {
        crate::utils::assert_panic_helper(|| {
            let _ = $expr;
        })
    };
}

pub(crate) use assert_panic;

/// When debug assertions are enabled, asserts that the given expression panics.
/// When debug assertions are disabled, asserts that the given expression does
/// not panic.
#[cfg(debug_assertions)]
macro_rules! assert_debug_panic {
    ($expr:expr $(,)?) => {
        crate::utils::assert_panic_helper(|| {
            let _ = $expr;
        })
    };
}
/// When debug assertions are enabled, asserts that the given expression panics.
/// When debug assertions are disabled, asserts that the given expression does
/// not panic.
#[cfg(not(debug_assertions))]
macro_rules! assert_debug_panic {
    ($expr:expr $(,)?) => {{
        let _ = $expr;
    }};
}

pub(crate) use assert_debug_panic;

macro_rules! assert_panic_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::utils::assert_panic_eq_helper(|| $left, || $right)
    };
}

pub(crate) use assert_panic_eq;

macro_rules! assert_panic_float_eq {
    ($left:expr, $right:expr $(, $($arg:tt)*)?) => {
        crate::utils::assert_panic_float_eq_helper(
            || $left,
            || $right,
            |left, right| crate::utils::assert_float_eq!(left, right $(, $($arg)*)?),
        )
    };
}

pub(crate) use assert_panic_float_eq;

/// Asserts that either `left == right` or `right` panics.
///
/// If `left` panics this fails even if `right` panicked too.
#[cfg(feature = "wide")]
macro_rules! assert_float_eq_or_panic {
    ($left:expr, $right:expr $(, $($arg:tt)*)?) => {{
        extern crate std;

        let _left = $left;
        if let Ok(right) = std::panic::catch_unwind(|| $right) {
            crate::utils::assert_float_eq!(_left, right $(, $($arg)*)?);
        }
    }};
}

#[cfg(feature = "wide")]
pub(crate) use assert_float_eq_or_panic;

#[doc(hidden)]
#[track_caller]
pub fn assert_panic_helper(f: impl FnOnce() + UnwindSafe) {
    match catch_unwind(f) {
        Ok(_) => panic!("assertion `panic` failed"),
        Err(_) => println_panic_expected(),
    }
}

#[doc(hidden)]
#[track_caller]
pub fn assert_panic_eq_helper<T>(
    left: impl FnOnce() -> T + UnwindSafe,
    right: impl FnOnce() -> T + UnwindSafe,
) where
    T: Debug + PartialEq,
{
    match (catch_unwind(left), catch_unwind(right)) {
        (Ok(left), Ok(right)) => {
            if left != right {
                panic!(
                    concat!(
                        "assertion `left == right` failed\n",
                        "  left: {:?}\n",
                        " right: {:?}",
                    ),
                    left, right,
                );
            }
        }
        (Ok(left), Err(_)) => {
            println_panic_expected();
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: {:?}\n",
                    " right: panic",
                ),
                left
            );
        }
        (Err(_), Ok(right)) => {
            println_panic_expected();
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: panic\n",
                    " right: {:?}",
                ),
                right
            );
        }
        (Err(_), Err(_)) => println_panic_expected(),
    }
}

#[doc(hidden)]
#[track_caller]
pub fn assert_panic_float_eq_helper<T>(
    left: impl FnOnce() -> T + UnwindSafe,
    right: impl FnOnce() -> T + UnwindSafe,
    assert_float_eq: impl FnOnce(T, T) + UnwindSafe,
) where
    T: Debug,
{
    match (catch_unwind(left), catch_unwind(right)) {
        (Ok(left), Ok(right)) => {
            assert_float_eq(left, right);
        }
        (Ok(left), Err(_)) => {
            println_panic_expected();
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: {:?}\n",
                    " right: panic",
                ),
                left
            );
        }
        (Err(_), Ok(right)) => {
            println_panic_expected();
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: panic\n",
                    " right: {:?}",
                ),
                right
            );
        }
        (Err(_), Err(_)) => println_panic_expected(),
    }
}

fn println_panic_expected() {
    println!("{}: panic is expected", "ok".green().bold());
}

mod tests {
    #[test]
    #[expect(clippy::diverging_sub_expression)]
    fn test_assert_panic() {
        assert_panic!(panic!());
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_panic() {
        assert_panic!(());
    }

    #[test]
    #[expect(clippy::diverging_sub_expression)]
    fn test_assert_assertions_panic() {
        #[cfg(debug_assertions)]
        assert_debug_panic!(panic!());
        #[cfg(debug_assertions)]
        assert_panic!(assert_debug_panic!(()));

        #[cfg(not(debug_assertions))]
        assert_debug_panic!(());
        #[cfg(not(debug_assertions))]
        assert_panic!(assert_debug_panic!(panic!()));
    }

    #[test]
    fn test_assert_panic_eq() {
        assert_panic_eq!(1, 1);
        assert_panic_eq!(panic!(), panic!());
        assert_panic!(assert_panic_eq!(1, 2));
        assert_panic!(assert_panic_eq!(panic!(), 1));
        assert_panic!(assert_panic_eq!(1, panic!()));
    }

    #[test]
    #[expect(unreachable_code)]
    fn test_assert_panic_float_eq() {
        assert_panic_float_eq!(1.0, 1.0);
        assert_panic_float_eq!(
            {
                panic!();
                0.0
            },
            panic!()
        );
        assert_panic!(assert_panic_float_eq!(1.0, 2.0));
        assert_panic!(assert_panic_float_eq!(panic!(), 1.0));
        assert_panic!(assert_panic_float_eq!(1.0, panic!()));
    }

    #[cfg(feature = "wide")]
    #[test]
    #[expect(unreachable_code)]
    #[expect(clippy::diverging_sub_expression)]
    fn test_assert_float_eq_or_panic() {
        assert_float_eq_or_panic!(1.0, 1.0);
        assert_float_eq_or_panic!(1.0, panic!());
        assert_panic!(assert_float_eq_or_panic!(1.0, 2.0));
        assert_panic!(assert_float_eq_or_panic!(panic!(), 1.0));
    }
}
