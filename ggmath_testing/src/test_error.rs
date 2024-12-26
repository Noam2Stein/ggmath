use std::{
    any::type_name,
    fmt::{self, Display, Formatter},
};

use ggmath::{scalar::Scalar, vector::VecAlignment};

use crate::TestFnDesc;

pub type TestResult = Result<(), TestingError>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestingError(pub String);
impl TestingError {
    pub fn new(failed_fn: &TestFnDesc, error: &impl Display) -> Self {
        Self(format!(
            "{}Failed{} {}{failed_fn}{}\n\n{error}",
            "\x1b[1;31m", "\x1b[0m", "\x1b[4m", "\x1b[0m"
        ))
    }

    #[deprecated = "use new FailedFn system"]
    pub fn vector<const N: usize, T: Scalar, A: VecAlignment>(
        fn_ident: &'static str,
        error: &impl Display,
    ) -> Self {
        Self(format!(
            "{}Failed{} {}Vector::<{N}, {}, {}>::{fn_ident}{}\n\n{error}",
            "\x1b[1;31m",
            "\x1b[0m",
            "\x1b[4m",
            type_name::<T>().split("::").last().unwrap_or(""),
            type_name::<A>().split("::").last().unwrap_or(""),
            "\x1b[0m"
        ))
    }
}
impl Display for TestingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait FormatTestingResult {
    #[must_use]
    fn fmt_test_result(&self) -> String;
}
impl FormatTestingResult for Result<(), TestingError> {
    fn fmt_test_result(&self) -> String {
        match self {
            Ok(()) => format!("\n{}Tests passed! GG{}", "\x1b[1;32m", "\x1b[0m"),
            Err(error) => format!("\n{error}"),
        }
    }
}
