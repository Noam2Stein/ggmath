use std::{
    fmt::{self, Debug, Formatter},
    panic::{catch_unwind, set_hook, take_hook, UnwindSafe},
};

pub enum ValueOrPanic<T> {
    Value(T),
    Panic,
}

impl<T: UnwindSafe> ValueOrPanic<T> {
    pub fn from_fn(f: impl FnOnce() -> T + UnwindSafe) -> Self {
        set_hook(Box::new(|_| {}));
        let catch = catch_unwind(f);
        let _ = take_hook();

        match catch {
            Ok(ok) => Self::Value(ok),
            Err(_) => Self::Panic,
        }
    }
}

impl<T: Debug> Debug for ValueOrPanic<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(value) => write!(f, "{value:?}"),
            Self::Panic => write!(f, "panic"),
        }
    }
}

impl<T: UnwindSafe + PartialEq<Rhs>, Rhs: UnwindSafe> PartialEq<ValueOrPanic<Rhs>>
    for ValueOrPanic<T>
{
    fn eq(&self, other: &ValueOrPanic<Rhs>) -> bool {
        match self {
            Self::Value(value) => match other {
                ValueOrPanic::Value(other) => value == other,
                ValueOrPanic::Panic => false,
            },
            Self::Panic => match other {
                ValueOrPanic::Value(_) => false,
                ValueOrPanic::Panic => true,
            },
        }
    }
}
