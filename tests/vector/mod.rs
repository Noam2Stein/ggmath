test_primitive! {
    fn test_f32() {
        type T = f32;

        <"primitive.rs">::primitive_tests(1.0, 2.0, 3.0, 4.0);
        <"float.rs">::float_tests();
    }
}

test_primitive! {
    fn test_f64() {
        type T = f64;

        <"primitive.rs">::primitive_tests(1.0, 2.0, 3.0, 4.0);
        <"float.rs">::float_tests();
    }
}

test_primitive! {
    fn test_i8() {
        type T = i8;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_i16() {
        type T = i16;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_i32() {
        type T = i32;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_i64() {
        type T = i64;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_i128() {
        type T = i128;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_isize() {
        type T = isize;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_u8() {
        type T = u8;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_u16() {
        type T = u16;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_u32() {
        type T = u32;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_u64() {
        type T = u64;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_u128() {
        type T = u128;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_usize() {
        type T = usize;

        <"primitive.rs">::primitive_tests(1, 2, 3, 4);
    }
}

test_primitive! {
    fn test_bool() {
        type T = bool;

        <"primitive.rs">::primitive_tests(false, true, false, true);
    }
}

macro_rules! test_primitive {
    (fn $test_primitive:ident() { type T = $T:ty; $(<$path:literal>::$test:ident($($e:expr),* $(,)?);)* }) => {
        #[test]
        fn $test_primitive() {
            $(
                mod $test {
                    type T = $T;

                    include!($path);
                }

                $test::$test($($e),*);
            )*
        }
    };
}

use test_primitive;
