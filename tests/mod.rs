#![allow(arithmetic_overflow)]

use ggmath::*;

mod vector;

macro_rules! test {
    {
        $api:ident for T, S in [$($T:ty),* $(,)*], [Simd, NonSimd $(,)*]:

        $($test:tt)*
    } => {
        crate::test! {
            $dollar
            $api for T, S in [$($T),*], [Simd, NonSimd] {
                $($test)*
            }
        }
    };

    {
        $dollar:tt dollar
        $api:ident for T, S in [$($T:ty),* $(,)*], [Simd, NonSimd $(,)*] $test:expr
    } => {
        paste::paste! {$(
            #[test]
            fn [<test_ $T _simd_ $api>]() {
                #[allow(unused)]
                type T = $T;
                #[allow(unused)]
                type S = ggmath::Simd;
                #[allow(unused)]
                type Vec2T = ggmath::Vector<2, $T, ggmath::Simd>;
                #[allow(unused)]
                type Vec3T = ggmath::Vector<3, $T, ggmath::Simd>;
                #[allow(unused)]
                type Vec4T = ggmath::Vector<4, $T, ggmath::Simd>;

                #[allow(unused)]
                macro_rules! vec2t {
                    ($dollar($tt:tt)*) => {{
                        let v: Vec2T = ggmath::vec2!($dollar($tt)*);
                        v
                    }};
                }
                #[allow(unused)]
                macro_rules! vec3t {
                    ($dollar($tt:tt)*) => {{
                        let v: Vec3T = ggmath::vec3!($dollar($tt)*);
                        v
                    }};
                }
                #[allow(unused)]
                macro_rules! vec4t {
                    ($dollar($tt:tt)*) => {{
                        let v: Vec4T = ggmath::vec4!($dollar($tt)*);
                        v
                    }};
                }

                $test
            }

            #[test]
            fn [<test_ $T _nonsimd_ $api>]() {
                #[allow(unused)]
                type T = $T;
                #[allow(unused)]
                type S = ggmath::NonSimd;
                #[allow(unused)]
                type Vec2T = ggmath::Vector<2, $T, ggmath::NonSimd>;
                #[allow(unused)]
                type Vec3T = ggmath::Vector<3, $T, ggmath::NonSimd>;
                #[allow(unused)]
                type Vec4T = ggmath::Vector<4, $T, ggmath::NonSimd>;

                #[allow(unused)]
                macro_rules! vec2t {
                    ($dollar($tt:tt)*) => {{
                        let v: Vec2T = ggmath::vec2s!($dollar($tt)*);
                        v
                    }};
                }
                #[allow(unused)]
                macro_rules! vec3t {
                    ($dollar($tt:tt)*) => {{
                        let v: Vec3T = ggmath::vec3s!($dollar($tt)*);
                        v
                    }};
                }
                #[allow(unused)]
                macro_rules! vec4t {
                    ($dollar($tt:tt)*) => {{
                        let v: Vec4T = ggmath::vec4s!($dollar($tt)*);
                        v
                    }};
                }

                $test
            }
        )*}
    };
}

macro_rules! assert_val {
    ($val:expr, $expected:expr, $val_str:literal) => {{
        let val = $val;
        let expected = $expected;

        #[allow(unused_parens)]
        if !(val == expected) {
            panic!(
                "wrong value for: {}\n\nexpected: {expected:?}\nactual: {val:?}",
                format_args!($val_str),
            );
        }
    }};
}

macro_rules! assert_val_logic {
    ($val:expr, $expected:expr, $val_str:literal) => {{
        let val = $val;
        let expected = $expected;

        #[allow(unused_parens)]
        if !crate::LogicEq::logic_eq(&val, &expected) {
            panic!(
                "wrong value for: {}\n\nexpected: {expected:?}\nactual: {val:?}",
                format_args!($val_str),
            );
        }
    }};
}

macro_rules! assert_approx_val {
    ($val:expr, $expected:expr, $val_str:literal) => {{
        let val = $val;
        let expected = $expected;

        #[allow(unused_parens)]
        if !crate::ApproxEq::approx_eq(&val, &expected) {
            panic!(
                "wrong approximate value for: {}\n\nexpected: {expected:?}\nactual: {val:?}",
                format_args!($val_str),
            );
        }
    }};
}

macro_rules! assert_panic {
    ($val:expr, $val_str:literal) => {{
        match std::panic::catch_unwind(|| {
            let _ = $val;
        }) {
            Ok(_) => panic!(
                "function did not panic as expected: {}",
                format_args!($val_str)
            ),
            Err(_) => {}
        };
    }};
}

macro_rules! assert_debug_panic {
    ($e:expr, $val_str:literal) => {{
        #[cfg(debug_assertions)]
        crate::assert_panic!($e, $val_str);

        #[cfg(not(debug_assertions))]
        $e;
    }};
}

macro_rules! assert_debug_panic_val {
    ($val:expr, $expected:expr, $val_str:literal) => {
        assert_debug_panic!(crate::assert_val!($val, $expected, $val_str), $val_str)
    };
}

#[expect(unused)]
macro_rules! assert_debug_panic_val_logic {
    ($val:expr, $expected:expr, $val_str:literal) => {
        assert_debug_panic!(
            crate::assert_val_logic!($val, $expected, $val_str),
            $val_str
        )
    };
}

#[expect(unused)]
macro_rules! assert_debug_panic_approx_val {
    ($val:expr, $expected:expr, $val_str:literal) => {
        assert_debug_panic!(
            crate::assert_approx_val!($val, $expected, $val_str),
            $val_str
        )
    };
}

use assert_approx_val;
use assert_debug_panic;
#[expect(unused)]
use assert_debug_panic_approx_val;
use assert_debug_panic_val;
#[expect(unused)]
use assert_debug_panic_val_logic;
use assert_panic;
use assert_val;
use assert_val_logic;
use test;

trait TestScalar: Scalar {
    /// `TEST_VALUES[1]` must be greater than `TEST_VALUES[0]`, and `TEST_VALUES[3]` must be greater than `TEST_VALUES[2]`.
    const TEST_VALUES: [Self; 4];
}

trait LogicEq {
    fn logic_eq(&self, other: &Self) -> bool;
}

trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
}

impl<const N: usize, T: Scalar + LogicEq, S: Simdness> LogicEq for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    fn logic_eq(&self, other: &Self) -> bool {
        (0..N).all(|i| self[i].logic_eq(&other[i]))
    }
}

impl<const N: usize, T: Scalar + ApproxEq, S: Simdness> ApproxEq for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    fn approx_eq(&self, other: &Self) -> bool {
        (0..N).all(|i| self[i].approx_eq(&other[i]))
    }
}

impl TestScalar for f32 {
    const TEST_VALUES: [Self; 4] = [0.0, 1.0, 2.0, 3.0];
}
impl TestScalar for f64 {
    const TEST_VALUES: [Self; 4] = [0.0, 1.0, 2.0, 3.0];
}
impl TestScalar for i8 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for i16 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for i32 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for i64 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for i128 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for isize {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for u8 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for u16 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for u32 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for u64 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for u128 {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for usize {
    const TEST_VALUES: [Self; 4] = [0, 1, 2, 3];
}
impl TestScalar for bool {
    const TEST_VALUES: [Self; 4] = [false, true, false, true];
}

impl LogicEq for f32 {
    fn logic_eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.signum() != other.signum() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            self == other
        }
    }
}
impl LogicEq for f64 {
    fn logic_eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.signum() != other.signum() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            self == other
        }
    }
}

impl ApproxEq for f32 {
    fn approx_eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.signum() != other.signum() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            (self - other).abs() < 0.002
        }
    }
}
impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        if self.is_nan() && other.is_nan() {
            true
        } else if self.signum() != other.signum() {
            false
        } else if self.is_infinite() && other.is_infinite() {
            true
        } else {
            (self - other).abs() < 0.002
        }
    }
}
