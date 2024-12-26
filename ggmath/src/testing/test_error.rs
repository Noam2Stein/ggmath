use std::fmt::{self, Display, Formatter};

use crate::testing::TestFnDesc;

pub type TestResult = Result<(), TestError>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestError(pub String);
impl TestError {
    pub fn new(failed_fn: &TestFnDesc, error: &impl Display) -> Self {
        Self(format!(
            "{}Failed{} {}{failed_fn}{}\n\n{error}",
            "\x1b[1;31m", "\x1b[0m", "\x1b[4m", "\x1b[0m"
        ))
    }
}
impl Display for TestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait FormatTestResult {
    #[must_use]
    fn fmt_test_result(&self) -> String;
}
impl FormatTestResult for Result<(), TestError> {
    fn fmt_test_result(&self) -> String {
        match self {
            Ok(()) => format!("\n{}Tests passed! GG{}", "\x1b[1;32m", "\x1b[0m"),
            Err(error) => format!("\n{error}"),
        }
    }
}
