#![allow(arithmetic_overflow)]

mod vector;

macro_rules! assert_panic {
    ($e:expr, $($arg:tt)*) => {{
        match std::panic::catch_unwind(|| { let _ = $e; }) {
            Ok(_) => panic!("function did not panic as expected: {}", format_args!($($arg)*)),
            Err(_) => {},
        };
    }};
}

macro_rules! assert_debug_panic {
    ($e:expr, $($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        crate::assert_panic!($e, $($arg)*);

        #[cfg(not(debug_assertions))]
        $e;
    }};
}

macro_rules! assert_eq_debug_panic {
    ($a:expr, $b:expr, $($arg:tt)*) => {
        assert_debug_panic!(assert_eq!($a, $b, $($arg)*), $($arg)*)
    };
}

use assert_debug_panic;
use assert_eq_debug_panic;
use assert_panic;
