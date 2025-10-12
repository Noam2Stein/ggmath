#![allow(arithmetic_overflow, unconditional_panic)]

mod vector;

#[macro_export]
macro_rules! assert_panic {
    ($e:expr) => {
        match std::panic::catch_unwind(|| { let _ = $e; }) {
            Ok(_) => panic!("function did not panic as expected"),
            Err(_) => {}
        }
    };
    ($e:expr, $($arg:tt)*) => {
        match std::panic::catch_unwind(|| { let _ = $e; }) {
            Ok(_) => panic!("function did not panic as expected: {}", format_args!($($arg)*)),
            Err(_) => {}
        }
    };
}

#[macro_export]
macro_rules! assert_debug_panic {
    ($e:expr) => {{
        #[cfg(debug_assertions)]
        assert_panic!($e);

        #[cfg(not(debug_assertions))]
        $e;
    }};
    ($e:expr, $($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        assert_panic!($e, $($arg)*);

        #[cfg(not(debug_assertions))]
        $e;
    }};
}
