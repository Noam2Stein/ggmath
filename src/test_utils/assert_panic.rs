use std::{
    fmt::Debug,
    panic::{UnwindSafe, catch_unwind},
};

use colored::Colorize;

macro_rules! assert_panic {
    ($expr:expr $(,)?) => {
        crate::test_utils::assert_panic_helper(|| {
            let _ = $expr;
        })
    };
}

pub(crate) use assert_panic;

macro_rules! assert_panic_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::test_utils::assert_panic_eq_helper(|| $left, || $right)
    };
}

pub(crate) use assert_panic_eq;

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
    T: Debug,
{
    match (catch_unwind(left), catch_unwind(right)) {
        (Ok(_), Ok(_)) => {}
        (Ok(left), Err(_)) => {
            println_panic_expected();
            panic!(
                concat!(
                    "assertion `left == right` failed\n",
                    "  left: {:?}\n",
                    " right: panic",
                ),
                left
            )
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
            )
        }
        (Err(_), Err(_)) => println_panic_expected(),
    }
}

fn println_panic_expected() {
    println!("{}: panic is expected", "ok".green().bold());
}
