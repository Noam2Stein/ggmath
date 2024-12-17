use std::{
    any::type_name,
    fmt::{self, Display, Formatter},
};

use ggmath::{scalar::Scalar, vector::VecAlignment};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScalarTestingError {
    pub vector_fn: &'static str,
    pub vector_length: u8,
    pub scalar_type: &'static str,
    pub vector_alignment: &'static str,
    pub err: String,
}
impl ScalarTestingError {
    pub fn new<const N: usize, T: Scalar, A: VecAlignment>(
        vector_fn: &'static str,
        err: impl Into<String>,
    ) -> Self {
        Self {
            vector_fn,
            vector_length: N as u8,
            scalar_type: type_name::<T>().split("::").last().unwrap_or(""),
            vector_alignment: type_name::<A>().split("::").last().unwrap_or(""),
            err: err.into(),
        }
    }
}
impl Display for ScalarTestingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\x1b[1;31mFailed\x1b[0m \x1b[4mVector::<{}, {}, {}>::{}\x1b[0m\n\n{}",
            self.vector_length, self.scalar_type, self.vector_alignment, self.vector_fn, self.err
        )
    }
}

pub trait ScalarTestingResultFmt {
    #[must_use]
    fn scalar_test_fmt(&self) -> String;
}
impl ScalarTestingResultFmt for Result<(), ScalarTestingError> {
    fn scalar_test_fmt(&self) -> String {
        match self {
            Ok(()) => format!("\n\x1b[1;32mTests passed! GG\x1b[0m"),
            Err(error) => format!("\n{error}"),
        }
    }
}
