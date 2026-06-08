extern crate std;

use std::{
    fmt::Debug,
    panic::{UnwindSafe, catch_unwind},
    println,
};

use colored::Colorize;

/// Asserts that the given expression panics.
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

/// Asserts that either both expressions panic or that their result is equal
/// with [`test_eq`] rules.
///
/// [`test_eq`]: crate::utils::test_eq
macro_rules! assert_panic_test_eq {
    ($actual:expr, $expected:expr $(, $($arg:tt)*)?) => {
        crate::utils::assert_panic_test_eq_helper(
            || $actual,
            || $expected,
            |left, right| crate::utils::assert_test_eq!(left, right $(, $($arg)*)?),
        )
    };
}
pub(crate) use assert_panic_test_eq;

/// Asserts that either `actual == expected` or `expected` panics.
///
/// Equality follows [`test_eq`] rules.
///
/// If `actual` panics this fails even if `expected` panicked too.
///
/// [`test_eq`]: crate::utils::test_eq
#[cfg(feature = "wide")]
macro_rules! assert_test_eq_or_panic {
    ($actual:expr, $expected:expr $(, $($arg:tt)*)?) => {{
        extern crate std;

        #[allow(unused_variables)]
        let actual = $actual;
        if let Ok(expected) = std::panic::catch_unwind(|| $expected) {
            crate::utils::assert_test_eq!(actual, expected $(, $($arg)*)?);
        } else {
            crate::utils::println_panic_expected();
        }
    }};
}
#[cfg(feature = "wide")]
pub(crate) use assert_test_eq_or_panic;

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
pub fn assert_panic_test_eq_helper<T>(
    actual: impl FnOnce() -> T + UnwindSafe,
    expected: impl FnOnce() -> T + UnwindSafe,
    assert_test_eq: impl FnOnce(T, T) + UnwindSafe,
) where
    T: Debug,
{
    match (catch_unwind(actual), catch_unwind(expected)) {
        (Ok(actual), Ok(expected)) => {
            assert_test_eq(actual, expected);
        }
        (Ok(actual), Err(_)) => {
            println_panic_expected();
            panic!(
                concat!(
                    "assertion `actual == expected` failed\n",
                    "  actual: {:?}\n",
                    "expected: panic",
                ),
                actual
            );
        }
        (Err(_), Ok(expected)) => {
            panic!(
                concat!(
                    "assertion `actual == expected` failed\n",
                    "  actual: panic\n",
                    "expected: {:?}",
                ),
                expected
            );
        }
        (Err(_), Err(_)) => println_panic_expected(),
    }
}

#[doc(hidden)]
pub fn println_panic_expected() {
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
    fn test_assert_debug_panic() {
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
    #[expect(unreachable_code)]
    fn test_assert_panic_test_eq() {
        assert_panic_test_eq!(1.0, 1.0);
        assert_panic_test_eq!(
            {
                panic!();
                0.0
            },
            panic!()
        );
        assert_panic!(assert_panic_test_eq!(1.0, 2.0));
        assert_panic!(assert_panic_test_eq!(panic!(), 1.0));
        assert_panic!(assert_panic_test_eq!(1.0, panic!()));
    }

    #[cfg(feature = "wide")]
    #[test]
    #[expect(unreachable_code)]
    #[expect(clippy::diverging_sub_expression)]
    fn test_assert_test_eq_or_panic() {
        assert_test_eq_or_panic!(1.0, 1.0);
        assert_test_eq_or_panic!(1.0, panic!());
        assert_panic!(assert_test_eq_or_panic!(1.0, 2.0));
        assert_panic!(assert_test_eq_or_panic!(panic!(), 1.0));
        assert_panic!(assert_test_eq_or_panic!(
            {
                panic!();
                1.0
            },
            panic!()
        ));
    }
}
