use std::{
    fmt::Display,
    panic::{UnwindSafe, catch_unwind, resume_unwind},
};

use crate::EulerRot;

macro_rules! for_parameters {
    (|T: PrimitiveNumber| $expr:expr) => {{
        fn f<T>(t: &'static str)
        where
            T: crate::Scalar
                + crate::constants::Zero
                + crate::constants::One
                + num_primitive::PrimitiveNumber
                + crate::utils::Values,
        {
            crate::utils::call_in_context(|| $expr, format_args!("T: {t}"));
        }

        f::<f32>("f32");
        f::<f64>("f64");
        f::<i8>("i8");
        f::<i16>("i16");
        f::<i32>("i32");
        f::<i64>("i64");
        f::<i128>("i128");
        f::<isize>("isize");
        f::<u8>("u8");
        f::<u16>("u16");
        f::<u32>("u32");
        f::<u64>("u64");
        f::<u128>("u128");
        f::<usize>("usize");
    }};
    (|T: PrimitiveFloat| $expr:expr) => {{
        {
            type T = f32;
            crate::utils::call_in_context(|| $expr, "T: f32");
        }
        {
            type T = f64;
            crate::utils::call_in_context(|| $expr, "T: f64");
        }
    }};
    (|T: PrimitiveSigned| $expr:expr) => {{
        {
            type T = i8;
            crate::utils::call_in_context(|| $expr, "T: i8");
        }
        {
            type T = i16;
            crate::utils::call_in_context(|| $expr, "T: i16");
        }
        {
            type T = i32;
            crate::utils::call_in_context(|| $expr, "T: i32");
        }
        {
            type T = i64;
            crate::utils::call_in_context(|| $expr, "T: i64");
        }
        {
            type T = i128;
            crate::utils::call_in_context(|| $expr, "T: i128");
        }
        {
            type T = isize;
            crate::utils::call_in_context(|| $expr, "T: isize");
        }
    }};
    (|T: PrimitiveUnsigned| $expr:expr) => {{
        {
            type T = u8;
            crate::utils::call_in_context(|| $expr, "T: u8");
        }
        {
            type T = u16;
            crate::utils::call_in_context(|| $expr, "T: u16");
        }
        {
            type T = u32;
            crate::utils::call_in_context(|| $expr, "T: u32");
        }
        {
            type T = u64;
            crate::utils::call_in_context(|| $expr, "T: u64");
        }
        {
            type T = u128;
            crate::utils::call_in_context(|| $expr, "T: u128");
        }
        {
            type T = usize;
            crate::utils::call_in_context(|| $expr, "T: usize");
        }
    }};
    (|T: PrimitiveInteger| $expr:expr) => {{
        for_parameters!(|T: PrimitiveSigned| $expr);
        for_parameters!(|T: PrimitiveUnsigned| $expr);
    }};
    (|A| $expr:expr) => {{
        fn fa<A: crate::Alignment>(a: &'static str) {
            crate::utils::call_in_context(|| $expr, format_args!("A: {a}"));
        }

        fa::<crate::Aligned>("Aligned");
        fa::<crate::Unaligned>("Unaligned");
    }};
    (|T: PrimitiveNumber, A| $expr:expr) => {{
        fn ft<T, A: crate::Alignment>(t: &'static str)
        where
            T: crate::Scalar
                + crate::constants::Zero
                + crate::constants::One
                + num_primitive::PrimitiveNumber
                + crate::utils::Values,
        {
            crate::utils::call_in_context(|| $expr, format_args!("T: {t}"));
        }

        for_parameters!(|A| {
            ft::<f32, A>("f32");
            ft::<f64, A>("f64");
            ft::<i8, A>("i8");
            ft::<i16, A>("i16");
            ft::<i32, A>("i32");
            ft::<i64, A>("i64");
            ft::<i128, A>("i128");
            ft::<isize, A>("isize");
            ft::<u8, A>("u8");
            ft::<u16, A>("u16");
            ft::<u32, A>("u32");
            ft::<u64, A>("u64");
            ft::<u128, A>("u128");
            ft::<usize, A>("usize");
        });
    }};
    (|T: $PrimitiveTrait:ident, A| $expr:expr) => {{
        for_parameters!(|A| for_parameters!(|T: $PrimitiveTrait| $expr));
    }};
    (|T: $PrimitiveTrait:ident, A, $($x:ident),*| $expr:expr) => {{
        for_parameters!(|T: $PrimitiveTrait, A| for_parameters!(|$($x),*| $expr));
    }};
    (|T: $PrimitiveTrait:ident, $($x:ident),*| $expr:expr) => {{
        for_parameters!(|T: $PrimitiveTrait| for_parameters!(|$($x),*| $expr));
    }};
    (|A, $($x:ident),*| $expr:expr) => {{
        for_parameters!(|A| for_parameters!(|$($x),*| $expr));
    }};
    (|$x:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            crate::utils::call_in_context(
                || $expr,
                format_args!("{}: {:?}", stringify!($x), $x),
            );
        }
    };
    // Manually implementing all cases leads to better compile times than macro
    // recursion.
    (|$x:ident, $y:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                crate::utils::call_in_context(
                    || $expr,
                    format_args!("{}: {:?}\n{}: {:?}", stringify!($x), $x, stringify!($y), $y),
                );
            }
        }
    };
    (|$x:ident, $y:ident, $z:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                for &$z in crate::utils::Values::VALUES {
                    crate::utils::call_in_context(
                        || $expr,
                        format_args!(
                            "{}: {:?}\n{}: {:?}\n{}: {:?}",
                            stringify!($x),
                            $x,
                            stringify!($y),
                            $y,
                            stringify!($z),
                            $z
                        ),
                    );
                }
            }
        }
    };
    (|$x:ident, $y:ident, $z:ident, $w:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                for &$z in crate::utils::Values::VALUES {
                    for &$w in crate::utils::Values::VALUES {
                        crate::utils::call_in_context(
                            || $expr,
                            format_args!(
                                "{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}",
                                stringify!($x),
                                $x,
                                stringify!($y),
                                $y,
                                stringify!($z),
                                $z,
                                stringify!($w),
                                $w
                            ),
                        );
                    }
                }
            }
        }
    };
    (|$x:ident, $y:ident, $z:ident, $w:ident, $a:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                for &$z in crate::utils::Values::VALUES {
                    for &$w in crate::utils::Values::VALUES {
                        for &$a in crate::utils::Values::VALUES {
                            crate::utils::call_in_context(
                                || $expr,
                                format_args!(
                                    "{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}",
                                    stringify!($x),
                                    $x,
                                    stringify!($y),
                                    $y,
                                    stringify!($z),
                                    $z,
                                    stringify!($w),
                                    $w,
                                    stringify!($a),
                                    $a
                                ),
                            );
                        }
                    }
                }
            }
        }
    };
    (|$x:ident, $y:ident, $z:ident, $w:ident, $a:ident, $b:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                for &$z in crate::utils::Values::VALUES {
                    for &$w in crate::utils::Values::VALUES {
                        for &$a in crate::utils::Values::VALUES {
                            for &$b in crate::utils::Values::VALUES {
                                crate::utils::call_in_context(
                                    || $expr,
                                    format_args!(
                                        "{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}",
                                        stringify!($x),
                                        $x,
                                        stringify!($y),
                                        $y,
                                        stringify!($z),
                                        $z,
                                        stringify!($w),
                                        $w,
                                        stringify!($a),
                                        $a,
                                        stringify!($b),
                                        $b
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    };
    (|$x:ident, $y:ident, $z:ident, $w:ident, $a:ident, $b:ident, $c:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                for &$z in crate::utils::Values::VALUES {
                    for &$w in crate::utils::Values::VALUES {
                        for &$a in crate::utils::Values::VALUES {
                            for &$b in crate::utils::Values::VALUES {
                                for &$c in crate::utils::Values::VALUES {
                                    crate::utils::call_in_context(
                                        || $expr,
                                        format_args!(
                                            "{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}",
                                            stringify!($x),
                                            $x,
                                            stringify!($y),
                                            $y,
                                            stringify!($z),
                                            $z,
                                            stringify!($w),
                                            $w,
                                            stringify!($a),
                                            $a,
                                            stringify!($b),
                                            $b,
                                            stringify!($c),
                                            $c
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    (|$x:ident, $y:ident, $z:ident, $w:ident, $a:ident, $b:ident, $c:ident, $d:ident| $expr:expr) => {
        for &$x in crate::utils::Values::VALUES {
            for &$y in crate::utils::Values::VALUES {
                for &$z in crate::utils::Values::VALUES {
                    for &$w in crate::utils::Values::VALUES {
                        for &$a in crate::utils::Values::VALUES {
                            for &$b in crate::utils::Values::VALUES {
                                for &$c in crate::utils::Values::VALUES {
                                    for &$d in crate::utils::Values::VALUES {
                                        crate::utils::call_in_context(
                                            || $expr,
                                            format_args!(
                                                "{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}\n{}: {:?}",
                                                stringify!($x),
                                                $x,
                                                stringify!($y),
                                                $y,
                                                stringify!($z),
                                                $z,
                                                stringify!($w),
                                                $w,
                                                stringify!($a),
                                                $a,
                                                stringify!($b),
                                                $b,
                                                stringify!($c),
                                                $c,
                                                stringify!($d),
                                                $d
                                            ),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}

pub(crate) use for_parameters;

#[doc(hidden)]
pub trait Values: Sized + 'static {
    const VALUES: &[Self];
}

macro_rules! impl_float {
    ($T:ident) => {
        impl Values for $T {
            const VALUES: &[Self] = &[
                0.0,
                -0.0,
                1.0,
                -1.0,
                8.6,
                -8.6,
                20.3,
                -20.3,
                1005.2,
                -1005.2,
                500000.1,
                -500000.1,
                100000000000.5,
                -100000000000.5,
                $T::MAX,
                $T::MIN,
                $T::INFINITY,
                $T::NEG_INFINITY,
                $T::NAN,
            ];
        }
    };
}
impl_float!(f32);
impl_float!(f64);

macro_rules! impl_signed {
    ($T:ident) => {
        impl Values for $T {
            const VALUES: &[Self] = &[
                0,
                1,
                -1,
                2,
                -2,
                10,
                -10,
                100,
                -100,
                $T::MAX / 2,
                $T::MIN / 2,
                $T::MAX - 1,
                $T::MIN + 1,
                $T::MAX,
                $T::MIN,
            ];
        }
    };
}
impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);

macro_rules! impl_unsigned {
    ($T:ident) => {
        impl Values for $T {
            const VALUES: &[Self] = &[0, 1, 2, 10, 100, 200, $T::MAX / 2, $T::MAX - 1, $T::MAX];
        }
    };
}
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
impl_unsigned!(usize);

impl Values for bool {
    const VALUES: &[Self] = &[false, true];
}

impl Values for EulerRot {
    const VALUES: &[Self] = &[
        Self::Xyz,
        Self::Xzy,
        Self::Yxz,
        Self::Yzx,
        Self::Zxy,
        Self::Zyx,
        Self::Xyx,
        Self::Xzx,
        Self::Yxy,
        Self::Yzy,
        Self::Zxz,
        Self::Zyz,
        Self::XyzEx,
        Self::XzyEx,
        Self::YxzEx,
        Self::YzxEx,
        Self::ZxyEx,
        Self::ZyxEx,
        Self::XyxEx,
        Self::XzxEx,
        Self::YxyEx,
        Self::YzyEx,
        Self::ZxzEx,
        Self::ZyzEx,
    ];
}

#[doc(hidden)]
pub fn call_in_context(f: impl FnOnce() + UnwindSafe, context: impl Display) {
    match catch_unwind(f) {
        Ok(_) => {}
        Err(payload) => {
            println!("{context}");
            resume_unwind(payload);
        }
    }
}
