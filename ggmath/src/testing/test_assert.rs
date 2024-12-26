use std::{
    fmt::Debug,
    panic::{catch_unwind, set_hook, take_hook, UnwindSafe},
};

use crate::testing::{TestError, TestFnDesc, TestResult};

pub use ggmath_proc_macros::{mat_test_assert, rect_test_assert, test_assert, vec_test_assert};

/// FOR USE BY ```test_assert!``` ONLY!
///
/// Used by ```test_assert!``` to represent how the value and the expected value are checked for equality.
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum _TestAssertEqMode {
    #[default]
    PartialEq,
    Bytes,
}
impl _TestAssertEqMode {
    fn test_eq<T: Sized + PartialEq<E>, E: Sized>(value: &Option<T>, expected: &Option<E>) -> bool {

    }
}

/// FOR USE BY ```test_assert!``` ONLY!
///
/// Called by ```test_assert!``` to handle the complex logic that will have blown up the binary if it was copied to every macro invocation.
/// Compares the results of the 2 functions with panics handled and formats an error if not equal.
#[doc(hidden)]
pub fn _test_assert_helper<V: Debug + UnwindSafe + PartialEq<E>, E: Debug + UnwindSafe>(
    fn_desc: impl FnOnce() -> TestFnDesc,
    eq_mode: _TestAssertEqMode,
    f: impl FnOnce() -> V + UnwindSafe,
    f_expected: impl FnOnce() -> E + UnwindSafe,
    input_descs: impl FnOnce() -> String,
) -> TestResult {
    set_hook(Box::new(|_| {}));
    let value = catch_unwind(f).ok();
    let expected = catch_unwind(f_expected).ok();
    let _ = take_hook();

    if value.as.as_ref().map_or(expected.is_none(), |value| {
        expected
            .as_ref()
            .map_or(false, |expected| value == expected)
    }) {
        Ok(())
    } else {
        Err(TestError::new(
            &fn_desc(),
            &format!(
                "{}\nexpected `{expected:?}`\nfound `{value:?}`\n",
                input_descs()
            ),
        ))
    }
}
