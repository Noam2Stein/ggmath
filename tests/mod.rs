#![allow(deprecated)]

use ggmath::{Alignment, Length, Scalar, SupportedLength, Vector};

mod vector;

////////////////////////////////////////////////////////////////////////////////
// Float Equality
////////////////////////////////////////////////////////////////////////////////

trait FloatEq {
    type Float: Copy;

    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool;
    fn approx_eq(self, other: Self, epsilon: Self::Float, ignore_zero_sign: bool) -> bool;
}

impl FloatEq for f32 {
    type Float = Self;

    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self == 0.0 && other == 0.0 && ignore_zero_sign {
            true
        } else if self.is_sign_positive() != other.is_sign_positive() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            self == other
        }
    }

    fn approx_eq(self, other: Self, epsilon: Self, ignore_zero_sign: bool) -> bool {
        let epsilon = epsilon.max(Self::EPSILON * 2.0);

        if self.is_nan() && other.is_nan() {
            true
        } else if self == 0.0 && other == 0.0 && ignore_zero_sign {
            true
        } else if self.is_sign_positive() != other.is_sign_positive() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            (self - other).abs() <= epsilon
        }
    }
}

impl FloatEq for f64 {
    type Float = Self;

    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self == 0.0 && other == 0.0 && ignore_zero_sign {
            true
        } else if self.is_sign_positive() != other.is_sign_positive() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            self == other
        }
    }

    fn approx_eq(self, other: Self, epsilon: Self, ignore_zero_sign: bool) -> bool {
        let epsilon = epsilon.max(Self::EPSILON * 2.0);

        if self.is_nan() && other.is_nan() {
            true
        } else if self == 0.0 && other == 0.0 && ignore_zero_sign {
            true
        } else if self.is_sign_positive() != other.is_sign_positive() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            (self - other).abs() <= epsilon
        }
    }
}

impl<const N: usize, T: Scalar + FloatEq, A: Alignment> FloatEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    type Float = T::Float;

    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.float_eq(b, ignore_zero_sign))
    }

    fn approx_eq(self, other: Self, epsilon: T::Float, ignore_zero_sign: bool) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.approx_eq(b, epsilon, ignore_zero_sign))
    }
}

impl<T: FloatEq> FloatEq for Option<T> {
    type Float = T::Float;

    fn float_eq(self, other: Self, ignore_zero_sign: bool) -> bool {
        match (self, other) {
            (Some(value), Some(other)) => value.float_eq(other, ignore_zero_sign),
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
        }
    }

    fn approx_eq(self, other: Self, epsilon: T::Float, ignore_zero_sign: bool) -> bool {
        match (self, other) {
            (Some(value), Some(other)) => value.approx_eq(other, epsilon, ignore_zero_sign),
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Helper Macros
////////////////////////////////////////////////////////////////////////////////

macro_rules! vec2 {
    ($($arg:expr),*$(,)?) => {
        Vector::<2, T, A>::from(($($arg,)*))
    };
}

use vec2;

macro_rules! vec3 {
    ($($arg:expr),*$(,)?) => {
        Vector::<3, T, A>::from(($($arg,)*))
    };
}

use vec3;

macro_rules! vec4 {
    ($($arg:expr),*$(,)?) => {
        Vector::<4, T, A>::from(($($arg,)*))
    };
}

use vec4;

macro_rules! assert_eq {
    ($a:expr, $b:expr$(,)?) => {
        $crate::assert_eq!(@$a, $b, "");
    };
    ($a:expr, $b:expr, $ctx:ident$(,)?) => {
        $crate::assert_eq!(@$a, $b, format_args!("\n\n{}", $ctx));
    };

    (@$a:expr, $b:expr, $ctx:expr) => {{
        let [a, b] = [$a, $b];
        if !(a == b) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}{}",
                stringify!($a),
                stringify!($b),
                b,
                a,
                $ctx
            );
        }
    }};
}

use assert_eq;

macro_rules! assert_float_eq {
    ($a:expr, $b:expr$(,)?) => {
        $crate::assert_float_eq!(@$a, $b, false, "");
    };
    ($a:expr, $b:expr, 0.0 = -0.0$(,)?) => {
        $crate::assert_float_eq!(@$a, $b, true, "");
    };
    ($a:expr, $b:expr, $ctx:ident$(,)?) => {
        $crate::assert_float_eq!(@$a, $b, false, format_args!("\n\n{}", $ctx));
    };
    ($a:expr, $b:expr, 0.0 = -0.0, $ctx:ident$(,)?) => {
        $crate::assert_float_eq!(@$a, $b, true, format_args!("\n\n{}", $ctx));
    };

    (@$a:expr, $b:expr, $ignore_zero_sign:literal, $ctx:expr) => {
        if !$crate::FloatEq::float_eq($a, $b, $ignore_zero_sign) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}{}",
                stringify!($a),
                stringify!($b),
                $b,
                $a,
                $ctx
            );
        }
    };
}

use assert_float_eq;

#[expect(unused)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $epsilon:expr$(,)?) => {
        $crate::assert_approx_eq!(@$a, $b, $epsilon, false, "");
    };
    ($a:expr, $b:expr, $epsilon:expr, 0.0 = -0.0$(,)?) => {
        $crate::assert_approx_eq!(@$a, $b, $epsilon, true, "");
    };
    ($a:expr, $b:expr, $epsilon:expr, $ctx:ident$(,)?) => {
        $crate::assert_approx_eq!(@$a, $b, $epsilon, false, format_args!("\n\n{}", $ctx));
    };
    ($a:expr, $b:expr, $epsilon:expr, 0.0 = -0.0, $ctx:ident$(,)?) => {
        $crate::assert_approx_eq!(@$a, $b, $epsilon, true, format_args!("\n\n{}", $ctx));
    };

    (@$a:expr, $b:expr, $epsilon:expr, $ignore_zero_sign:literal, $ctx:expr) => {
        if !$crate::FloatEq::approx_eq($a, $b, $epsilon, $ignore_zero_sign) {
            panic!(
                "assertion failed: {} == {}\n\nexpected: {:?}\nfound: {:?}{}",
                stringify!($a),
                stringify!($b),
                $b,
                $a,
                $ctx
            );
        }
    };
}

use assert_approx_eq;

macro_rules! assert_panic {
    ($e:expr $(,)?) => {
        $crate::assert_panic!(@$e, "");
    };
    ($e:expr, $ctx:ident$(,)?) => {
        $crate::assert_panic!(@$e, format_args!("\n\n{}", $ctx));
    };

    (@$e:expr, $ctx:expr) => {
        match ::std::panic::catch_unwind(|| -> () {
            let _ = $e;
        }) {
            Ok(_) => panic!("assert_panic! argument did not panic{}", $ctx),
            Err(_) => println!("argument paniced as expected ... {}", colored::Colorize::green("ok")),
        }
    }
}

use assert_panic;

macro_rules! assert_assertion_panic {
    ($e:expr$(, $ctx:ident)?$(,)?) => {
        if cfg!(feature = "assertions")
            || cfg!(feature = "debug_assertions") && cfg!(debug_assertions)
        {
            $crate::assert_panic!($e$(, $ctx)?);
        } else {
            let _ = $e;
        }
    };
}

use assert_assertion_panic;

macro_rules! assert_eq_or_panic {
    ($a:expr, $b:expr$(,)?) => {
        $crate::assert_eq_or_panic!(@$a, $b, "");
    };
    ($a:expr, $b:expr, $ctx:ident$(,)?) => {
        $crate::assert_eq_or_panic!(@$a, $b, format_args!("\n\n{}", $ctx));
    };

    (@$a:expr, $b:expr, $ctx:expr) => {
        match (
            ::std::panic::catch_unwind(|| $a),
            ::std::panic::catch_unwind(|| $b),
        ) {
            (Ok(a), Ok(b)) => {
                if !(a == b) {
                    panic!(
                        "assertion failed: {} != {}\n\nexpected: {:?}\nfound: {:?}{}",
                        stringify!($a),
                        stringify!($b),
                        b,
                        a,
                        $ctx
                    );
                }
            }
            (Ok(_), Err(_)) => {
                panic!(
                    "assert_eq_or_panic! argument did not panic as expected{}",
                    $ctx
                );
            }
            (Err(_), Ok(_)) => {
                panic!("assert_eq_or_panic! argument paniced unexpectedly{}", $ctx);
            }
            (Err(_), Err(_)) => {
                println!(
                    "arguments paniced as expected ... {}",
                    colored::Colorize::green("ok")
                );
            }
        }
    };
}

use assert_eq_or_panic;
