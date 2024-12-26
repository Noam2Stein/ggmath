use std::fmt::{self, Display, Formatter};

use crate::testing::TestFnDesc;

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
