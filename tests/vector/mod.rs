use pastey::paste;

test! {
    type T = f32;

    primitive::primitive_tests(1.0, 2.0, 3.0, 4.0);
    float::float_tests();
}
test! {
    type T = f64;

    primitive::primitive_tests(1.0, 2.0, 3.0, 4.0);
    float::float_tests();
}

test! {
    type T = i8;

    primitive::primitive_tests(1, 2, 3, 4);
    int::int_tests();
}
test! {
    type T = i16;

    primitive::primitive_tests(1, 2, 3, 4);
    int::int_tests();
}
test! {
    type T = i32;

    primitive::primitive_tests(1, 2, 3, 4);
    int::int_tests();
}
test! {
    type T = i64;
    primitive::primitive_tests(1, 2, 3, 4);
    int::int_tests();
}
test! {
    type T = i128;

    primitive::primitive_tests(1, 2, 3, 4);
    int::int_tests();
}
test! {
    type T = isize;

    primitive::primitive_tests(1, 2, 3, 4);
    int::int_tests();
}

test! {
    type T = u8;

    primitive::primitive_tests(1, 2, 3, 4);
    uint::uint_tests();
}
test! {
    type T = u16;

    primitive::primitive_tests(1, 2, 3, 4);
    uint::uint_tests();
}
test! {
    type T = u32;

    primitive::primitive_tests(1, 2, 3, 4);
    uint::uint_tests();
}
test! {
    type T = u64;

    primitive::primitive_tests(1, 2, 3, 4);
    uint::uint_tests();
}
test! {
    type T = u128;

    primitive::primitive_tests(1, 2, 3, 4);
    uint::uint_tests();
}
test! {
    type T = usize;

    primitive::primitive_tests(1, 2, 3, 4);
    uint::uint_tests();
}

test! {
    type T = bool;

    primitive::primitive_tests(false, true, false, true);
    bool::bool_tests();
}

macro_rules! test {
    { type T = $T:ident; $($m:ident::$f:ident($($arg:expr),*$(,)?);)* } => {
        paste! {
            #[test]
            fn [<test_ $T>]() {
                $(
                    [<$T _aligned_ $m _tests>]::$f($($arg),*);
                    [<$T _unaligned_ $m _tests>]::$f($($arg),*);
                )*
            }

            $(
                mod [<$T _aligned_ $m _tests>] {
                    type T = $T;
                    type A = ggmath::vector::Aligned;

                    include!(concat!(stringify!($m), ".rs"));
                }

                mod [<$T _unaligned_ $m _tests>] {
                    type T = $T;
                    type A = ggmath::vector::Unaligned;

                    include!(concat!(stringify!($m), ".rs"));
                }
            )*
        }
    };
}

use test;
