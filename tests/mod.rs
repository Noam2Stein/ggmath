mod vector;

macro_rules! assert_panic {
    ($e:expr, $($arg:tt)*) => {{
        let _ = std::panic::set_hook(Box::new(|_| {}));
        let result = std::panic::catch_unwind(|| { let _ = $e; });
        let _ = std::panic::take_hook();

        match result {
            Ok(_) => panic!("function did not panic as expected: {}", format_args!($($arg)*)),
            Err(_) => {}
        }
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

use assert_debug_panic;
use assert_panic;
