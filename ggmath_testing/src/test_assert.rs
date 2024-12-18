use std::fmt::Debug;

use crate::{FailedFn, TestingError};

#[doc(hidden)]
pub fn _test_assert_helper<T: Debug + PartialEq<E>, E: Debug>(
    failed_fn: impl FnOnce() -> FailedFn,
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

#[macro_export]
macro_rules! test_assert {
    ($failed_fn:expr, $value:expr, $expected_value:expr $(, $input:ident)* $(,)?) => {
        $crate::_test_assert_helper(
            || $failed_fn,
            $crate::ValueOrPanic::from_fn(|| $value),
            $crate::ValueOrPanic::from_fn(|| $expected_value),
            || Box::new([$(format!(" * {} = {:?} \n", stringify!($input), $input)), *]),
        )?;
    };
}
#[macro_export]
macro_rules! vector_test_assert {
    ($failed_fn:ident: $value:expr, $expected_value:expr $(, $input:ident)* $(,)?) => {
        $crate::_test_assert_helper(
            || $crate::FailedFn::vector::<N, T, A>(stringify!($failed_fn)),
            $crate::ValueOrPanic::from_fn(|| $value),
            $crate::ValueOrPanic::from_fn(|| $expected_value),
            || Box::new([$(format!(" * {} = {:?} \n", stringify!($input), $input)), *]),
        )?;
    };
}
#[macro_export]
macro_rules! matrix_test_assert {
    ($failed_fn:ident: $value:expr, $expected_value:expr $(, $input:ident)* $(,)?) => {
        $crate::_test_assert_helper(
            || $crate::FailedFn::matrix::<C, R, T, A, M>(stringify!($failed_fn)),
            $crate::ValueOrPanic::from_fn(|| $value),
            $crate::ValueOrPanic::from_fn(|| $expected_value),
            || Box::new([$(format!(" * {} = {:?} \n", stringify!($input), $input)), *]),
        )?;
    };
}
#[macro_export]
macro_rules! rectangle_test_assert {
    ($failed_fn:ident: $value:expr, $expected_value:expr $(, $input:ident)* $(,)?) => {
        $crate::_test_assert_helper(
            || $crate::FailedFn::rectangle::<N, T, A, R>(stringify!($failed_fn)),
            $crate::ValueOrPanic::from_fn(|| $value),
            $crate::ValueOrPanic::from_fn(|| $expected_value),
            || Box::new([$(format!(" * {} = {:?} \n", stringify!($input), $input)), *]),
        )?;
    };
}
