use std::{
    fmt::Debug,
    panic::{catch_unwind, set_hook, take_hook, UnwindSafe},
};

use crate::{TestFnDesc, TestResult, TestingError};

#[doc(hidden)]
pub fn _test_assert_helper<T: Debug + PartialEq<E>, E: Debug>(
    failed_fn: impl FnOnce() -> TestFnDesc,
    value: T,
    expected_value: E,
    inputs: impl FnOnce() -> Box<[String]>,
) -> Result<(), TestingError> {
    if value == expected_value {
        Ok(())
    } else {
        Err(TestingError::new(
            &failed_fn(),
            &format!(
                "{}\nexpected `{expected_value:?}`\nfound `{value:?}`\n",
                inputs().join("")
            ),
        ))
    }
}

pub fn test_assert<V: Debug + UnwindSafe + PartialEq<E>, E: Debug + UnwindSafe>(
    fn_desc: impl FnOnce() -> TestFnDesc,
    f: impl FnOnce() -> V + UnwindSafe,
    f_expected: impl FnOnce() -> E + UnwindSafe,
    input_descs: impl FnOnce() -> String,
) -> TestResult {
    set_hook(Box::new(|_| {}));
    let value = catch_unwind(f).ok();
    let expected = catch_unwind(f_expected).ok();
    take_hook();

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
