use std::{
    fmt::Debug,
    panic::{catch_unwind, set_hook, take_hook, UnwindSafe},
};

use crate::testing::{TestFnDesc, TestResult, TestingError};

pub use ggmath_proc_macros::{mat_test_assert, rect_test_assert, test_assert, vec_test_assert};

/// FOR USE BY ```test_assert!``` ONLY!
///
/// Called by ```test_assert!``` to handle the complex logic that will have blown up the binary if it was copied to every macro invocation.
/// Compares the results of the 2 functions with panics handled and formats an error if not equal.
#[doc(hidden)]
pub fn __test_assert_helper<V: Debug + UnwindSafe + PartialEq<E>, E: Debug + UnwindSafe>(
    fn_desc: impl FnOnce() -> TestFnDesc,
    f: impl FnOnce() -> V + UnwindSafe,
    f_expected: impl FnOnce() -> E + UnwindSafe,
    input_descs: impl FnOnce() -> String,
) -> TestResult {
    set_hook(Box::new(|_| {}));
    let value = catch_unwind(f).ok();
    let expected = catch_unwind(f_expected).ok();
    let _ = take_hook();

    if value.as_ref().map_or(expected.is_none(), |value| {
        expected
            .as_ref()
            .map_or(false, |expected| value == expected)
    }) {
        Ok(())
    } else {
        Err(TestingError::new(
            &fn_desc(),
            &format!(
                "{}\nexpected `{expected:?}`\nfound `{value:?}`\n",
                input_descs()
            ),
        ))
    }
}
