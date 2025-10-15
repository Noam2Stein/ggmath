use ggmath::{NonSimd, Simd, SupportedVecLen, VecLen, Vector, vec2g, vec3g, vec4g};

use crate::assert_debug_panic;

macro_rules! test_float_api {
    ($dollar:tt $T:ty, $S:ident) => {{
        type Vec2T = Vector<2, $T, $S>;
        type Vec3T = Vector<3, $T, $S>;
        type Vec4T = Vector<4, $T, $S>;

        macro_rules! vec2t {
            ($dollar($tt:tt)*) => {{
                let v: ggmath::Vector<_, _, $S> = vec2g!($dollar($tt)*);
                v
            }};
        }
        macro_rules! vec3t {
            ($dollar($tt:tt)*) => {{
                let v: ggmath::Vector<_, _, $S> = vec3g!($dollar($tt)*);
                v
            }};
        }
        macro_rules! vec4t {
            ($dollar($tt:tt)*) => {{
                let v: ggmath::Vector<_, _, $S> = vec4g!($dollar($tt)*);
                v
            }};
        }

        fn float_eq(a: $T, b: $T) -> bool {
            if a.is_nan() && b.is_nan() {
                true
            } else if a.signum() != b.signum() {
                false
            } else if a.is_infinite() && b.is_infinite() {
                true
            } else {
                a == b
            }
        }

        fn approx_float_eq(a: $T, b: $T) -> bool {
            if a.is_nan() && b.is_nan() {
                true
            } else if a.signum() != b.signum() {
                false
            } else if a.is_infinite() && b.is_infinite() {
                true
            } else {
                (a - b).abs() < 0.0001
            }
        }

        fn float_vec_eq<const N: usize>(a: Vector<N, $T, $S>, b: Vector<N, $T, $S>) -> bool
        where
            VecLen<N>: SupportedVecLen,
        {
            a.iter().zip(b.iter()).all(|(a, b)| float_eq(a, b))
        }

        #[expect(unused)]
        fn approx_float_vec_eq<const N: usize>(a: Vector<N, $T, $S>, b: Vector<N, $T, $S>) -> bool
        where
            VecLen<N>: SupportedVecLen,
        {
            a.iter().zip(b.iter()).all(|(a, b)| approx_float_eq(a, b))
        }

        #[expect(unused_macros)]
        macro_rules! assert_float {
            ($a:expr, $b:expr, $tested:expr) => {{
                let a = $a;
                let b = $b;

                if !float_eq(a, b) {
                    panic!(
                        "incorrect {} value: {}\n\nexpected: {b:?}\nactual: {a:?}",
                        stringify!($T),
                        format_args!($tested),
                    );
                }
            }};
        }

        macro_rules! assert_approx_float {
            ($a:expr, $b:expr, $tested:expr) => {{
                let a = $a;
                let b = $b;

                if !approx_float_eq(a, b) {
                    panic!(
                        "incorrect {} value: {}\n\nexpected: {b:?}\nactual: {a:?}",
                        stringify!($T),
                        format_args!($tested),
                    );
                }
            }};
        }

        macro_rules! assert_float_vec {
            ($a:expr, $b:expr, $tested:expr) => {{
                let a = $a;
                let b = $b;

                if !float_vec_eq(a, b) {
                    panic!(
                        "incorrect {} value: {}\n\nexpected: {b:?}\nactual: {a:?}",
                        stringify!($T),
                        format_args!($tested),
                    );
                }
            }};
        }

        #[expect(unused_macros)]
        macro_rules! assert_approx_float_vec {
            ($a:expr, $b:expr, $tested:expr) => {{
                let a = $a;
                let b = $b;

                if !approx_float_vec_eq(a, b) {
                    panic!(
                        "incorrect {} value: {}\n\nexpected: {b:?}\nactual: {a:?}",
                        stringify!($T),
                        format_args!($tested),
                    );
                }
            }};
        }

        fn reset_zero_sign(x: $T) -> $T {
            if x == -0.0 {
                0.0
            } else {
                x
            }
        }

        fn vreset_zero_sign<const N: usize>(vec: Vector<N, $T, $S>) -> Vector<N, $T, $S>
        where
            VecLen<N>: SupportedVecLen,
        {
            vec.map(reset_zero_sign)
        }

        const UNIQUE_FLOATS: &[$T] = &[
            0.0,
            1.0,
            5.3,
            -0.0,
            -1.0,
            -23.4,
            <$T>::MIN,
            <$T>::MAX,
            <$T>::INFINITY,
            <$T>::NEG_INFINITY,
            <$T>::NAN,
        ];

        macro_rules! test_for_combinations {
            (|$vec2:ident, $vec3:ident, $vec4:ident| { $dollar($test:tt)* }) => {
                for &x in UNIQUE_FLOATS {
                    for &y in UNIQUE_FLOATS {
                        let vec2 = vec2t!(x, y);
                        let vec3 = vec3t!(x, y, x);
                        let vec4 = vec4t!(x, y, x, y);

                        (|$vec2: Vec2T, $vec3: Vec3T, $vec4: Vec4T| { $dollar($test)* })(vec2, vec3, vec4);
                    }
                }
            };

            (|$first_vec2:ident, $first_vec3:ident, $first_vec4:ident|, $dollar(|$vec2:ident, $vec3:ident, $vec4:ident|),+ { $dollar($test:tt)* }) => {
                test_for_combinations!(
                    $dollar(|$vec2, $vec3, $vec4|),+ {
                        test_for_combinations!(
                            |$first_vec2, $first_vec3, $first_vec4| { $dollar($test)* }
                        )
                    }
                )
            };
        }

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(-vec2, vec2t!(-vec2.x, -vec2.y), "-{vec2}");
            assert_float_vec!(-vec3, vec3t!(-vec3.x, -vec3.y, -vec3.z), "-{vec3}");
            assert_float_vec!(-vec4, vec4t!(-vec4.x, -vec4.y, -vec4.z, -vec4.w), "-{vec4}");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
            assert_float_vec!(vec2 + rhs2, vec2t!(vec2.x + rhs2.x, vec2.y + rhs2.y), "{vec2} + {rhs2}");
            assert_float_vec!(vec3 + rhs3, vec3t!(vec3.x + rhs3.x, vec3.y + rhs3.y, vec3.z + rhs3.z), "{vec3} + {rhs3}");
            assert_float_vec!(vec4 + rhs4, vec4t!(vec4.x + rhs4.x, vec4.y + rhs4.y, vec4.z + rhs4.z, vec4.w + rhs4.w), "{vec4} + {rhs4}");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
            assert_float_vec!(vec2 - rhs2, vec2t!(vec2.x - rhs2.x, vec2.y - rhs2.y), "{vec2} - {rhs2}");
            assert_float_vec!(vec3 - rhs3, vec3t!(vec3.x - rhs3.x, vec3.y - rhs3.y, vec3.z - rhs3.z), "{vec3} - {rhs3}");
            assert_float_vec!(vec4 - rhs4, vec4t!(vec4.x - rhs4.x, vec4.y - rhs4.y, vec4.z - rhs4.z, vec4.w - rhs4.w), "{vec4} - {rhs4}");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
            assert_float_vec!(vec2 * rhs2, vec2t!(vec2.x * rhs2.x, vec2.y * rhs2.y), "{vec2} * {rhs2}");
            assert_float_vec!(vec3 * rhs3, vec3t!(vec3.x * rhs3.x, vec3.y * rhs3.y, vec3.z * rhs3.z), "{vec3} * {rhs3}");
            assert_float_vec!(vec4 * rhs4, vec4t!(vec4.x * rhs4.x, vec4.y * rhs4.y, vec4.z * rhs4.z, vec4.w * rhs4.w), "{vec4} * {rhs4}");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
            assert_float_vec!(vec2 / rhs2, vec2t!(vec2.x / rhs2.x, vec2.y / rhs2.y), "{vec2} / {rhs2}");
            assert_float_vec!(vec3 / rhs3, vec3t!(vec3.x / rhs3.x, vec3.y / rhs3.y, vec3.z / rhs3.z), "{vec3} / {rhs3}");
            assert_float_vec!(vec4 / rhs4, vec4t!(vec4.x / rhs4.x, vec4.y / rhs4.y, vec4.z / rhs4.z, vec4.w / rhs4.w), "{vec4} / {rhs4}");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
            // `Vector::rem` is allowed to be imprecise for large lhs values
            if vec2.iter().any(|x| x == <$T>::MAX || x == <$T>::MIN) {
                return;
            }

            // `Vector::rem` is allowed to be incorrect for infinite rhs values
            if rhs2.iter().any(|x| x.is_infinite()) {
                return;
            }

            // `Vector::rem` is allowed to return -0.0 instead of 0.0, and 0.0 instead of -0.0

            assert_float_vec!(vreset_zero_sign(vec2 % rhs2), vreset_zero_sign(vec2t!(vec2.x % rhs2.x, vec2.y % rhs2.y)), "{vec2} % {rhs2}");
            assert_float_vec!(vreset_zero_sign(vec3 % rhs3), vreset_zero_sign(vec3t!(vec3.x % rhs3.x, vec3.y % rhs3.y, vec3.z % rhs3.z)), "{vec3} % {rhs3}");
            assert_float_vec!(vreset_zero_sign(vec4 % rhs4), vreset_zero_sign(vec4t!(vec4.x % rhs4.x, vec4.y % rhs4.y, vec4.z % rhs4.z, vec4.w % rhs4.w)), "{vec4} % {rhs4}");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.floor(), vec2t!(vec2.x.floor(), vec2.y.floor()), "{vec2}.floor()");
            assert_float_vec!(vec3.floor(), vec3t!(vec3.x.floor(), vec3.y.floor(), vec3.z.floor()), "{vec3}.floor()");
            assert_float_vec!(vec4.floor(), vec4t!(vec4.x.floor(), vec4.y.floor(), vec4.z.floor(), vec4.w.floor()), "{vec4}.floor()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.ceil(), vec2t!(vec2.x.ceil(), vec2.y.ceil()), "{vec2}.ceil()");
            assert_float_vec!(vec3.ceil(), vec3t!(vec3.x.ceil(), vec3.y.ceil(), vec3.z.ceil()), "{vec3}.ceil()");
            assert_float_vec!(vec4.ceil(), vec4t!(vec4.x.ceil(), vec4.y.ceil(), vec4.z.ceil(), vec4.w.ceil()), "{vec4}.ceil()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.round(), vec2t!(vec2.x.round(), vec2.y.round()), "{vec2}.round()");
            assert_float_vec!(vec3.round(), vec3t!(vec3.x.round(), vec3.y.round(), vec3.z.round()), "{vec3}.round()");
            assert_float_vec!(vec4.round(), vec4t!(vec4.x.round(), vec4.y.round(), vec4.z.round(), vec4.w.round()), "{vec4}.round()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.trunc(), vec2t!(vec2.x.trunc(), vec2.y.trunc()), "{vec2}.trunc()");
            assert_float_vec!(vec3.trunc(), vec3t!(vec3.x.trunc(), vec3.y.trunc(), vec3.z.trunc()), "{vec3}.trunc()");
            assert_float_vec!(vec4.trunc(), vec4t!(vec4.x.trunc(), vec4.y.trunc(), vec4.z.trunc(), vec4.w.trunc()), "{vec4}.trunc()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.fract(), vec2t!(vec2.x.fract(), vec2.y.fract()), "{vec2}.fract()");
            assert_float_vec!(vec3.fract(), vec3t!(vec3.x.fract(), vec3.y.fract(), vec3.z.fract()), "{vec3}.fract()");
            assert_float_vec!(vec4.fract(), vec4t!(vec4.x.fract(), vec4.y.fract(), vec4.z.fract(), vec4.w.fract()), "{vec4}.fract()");
        });

        test_for_combinations!(
            |vec2, vec3, vec4|, |a2, a3, a4|, |b2, b3, b4| {
                assert_float_vec!(vec2.mul_add(a2, b2), vec2t!(vec2.x.mul_add(a2.x, b2.x), vec2.y.mul_add(a2.y, b2.y)), "{vec2}.mul_add({a2}, {b2})");
                assert_float_vec!(vec3.mul_add(a3, b3), vec3t!(vec3.x.mul_add(a3.x, b3.x), vec3.y.mul_add(a3.y, b3.y), vec3.z.mul_add(a3.z, b3.z)), "{vec3}.mul_add({a3}, {b3})");
                assert_float_vec!(vec4.mul_add(a4, b4), vec4t!(vec4.x.mul_add(a4.x, b4.x), vec4.y.mul_add(a4.y, b4.y), vec4.z.mul_add(a4.z, b4.z), vec4.w.mul_add(a4.w, b4.w)), "{vec4}.mul_add({a4}, {b4})");
            }
        );

        test_for_combinations!(
            |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                assert_float_vec!(vec2.div_euclid(rhs2), vec2t!(vec2.x.div_euclid(rhs2.x), vec2.y.div_euclid(rhs2.y)), "{vec2}.div_euclid({rhs2})");
                assert_float_vec!(vec3.div_euclid(rhs3), vec3t!(vec3.x.div_euclid(rhs3.x), vec3.y.div_euclid(rhs3.y), vec3.z.div_euclid(rhs3.z)), "{vec3}.div_euclid({rhs3})");
                assert_float_vec!(vec4.div_euclid(rhs4), vec4t!(vec4.x.div_euclid(rhs4.x), vec4.y.div_euclid(rhs4.y), vec4.z.div_euclid(rhs4.z), vec4.w.div_euclid(rhs4.w)), "{vec4}.div_euclid({rhs4})");
            }
        );

        test_for_combinations!(
            |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                assert_float_vec!(vec2.rem_euclid(rhs2), vec2t!(vec2.x.rem_euclid(rhs2.x), vec2.y.rem_euclid(rhs2.y)), "{vec2}.rem_euclid({rhs2})");
                assert_float_vec!(vec3.rem_euclid(rhs3), vec3t!(vec3.x.rem_euclid(rhs3.x), vec3.y.rem_euclid(rhs3.y), vec3.z.rem_euclid(rhs3.z)), "{vec3}.rem_euclid({rhs3})");
                assert_float_vec!(vec4.rem_euclid(rhs4), vec4t!(vec4.x.rem_euclid(rhs4.x), vec4.y.rem_euclid(rhs4.y), vec4.z.rem_euclid(rhs4.z), vec4.w.rem_euclid(rhs4.w)), "{vec4}.rem_euclid({rhs4})");
            }
        );

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.sqrt(), vec2t!(vec2.x.sqrt(), vec2.y.sqrt()), "{vec2}.sqrt()");
            assert_float_vec!(vec3.sqrt(), vec3t!(vec3.x.sqrt(), vec3.y.sqrt(), vec3.z.sqrt()), "{vec3}.sqrt()");
            assert_float_vec!(vec4.sqrt(), vec4t!(vec4.x.sqrt(), vec4.y.sqrt(), vec4.z.sqrt(), vec4.w.sqrt()), "{vec4}.sqrt()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.sin(), vec2t!(vec2.x.sin(), vec2.y.sin()), "{vec2}.sin()");
            assert_float_vec!(vec3.sin(), vec3t!(vec3.x.sin(), vec3.y.sin(), vec3.z.sin()), "{vec3}.sin()");
            assert_float_vec!(vec4.sin(), vec4t!(vec4.x.sin(), vec4.y.sin(), vec4.z.sin(), vec4.w.sin()), "{vec4}.sin()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.cos(), vec2t!(vec2.x.cos(), vec2.y.cos()), "{vec2}.cos()");
            assert_float_vec!(vec3.cos(), vec3t!(vec3.x.cos(), vec3.y.cos(), vec3.z.cos()), "{vec3}.cos()");
            assert_float_vec!(vec4.cos(), vec4t!(vec4.x.cos(), vec4.y.cos(), vec4.z.cos(), vec4.w.cos()), "{vec4}.cos()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.tan(), vec2t!(vec2.x.tan(), vec2.y.tan()), "{vec2}.tan()");
            assert_float_vec!(vec3.tan(), vec3t!(vec3.x.tan(), vec3.y.tan(), vec3.z.tan()), "{vec3}.tan()");
            assert_float_vec!(vec4.tan(), vec4t!(vec4.x.tan(), vec4.y.tan(), vec4.z.tan(), vec4.w.tan()), "{vec4}.tan()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.asin(), vec2t!(vec2.x.asin(), vec2.y.asin()), "{vec2}.asin()");
            assert_float_vec!(vec3.asin(), vec3t!(vec3.x.asin(), vec3.y.asin(), vec3.z.asin()), "{vec3}.asin()");
            assert_float_vec!(vec4.asin(), vec4t!(vec4.x.asin(), vec4.y.asin(), vec4.z.asin(), vec4.w.asin()), "{vec4}.asin()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.acos(), vec2t!(vec2.x.acos(), vec2.y.acos()), "{vec2}.acos()");
            assert_float_vec!(vec3.acos(), vec3t!(vec3.x.acos(), vec3.y.acos(), vec3.z.acos()), "{vec3}.acos()");
            assert_float_vec!(vec4.acos(), vec4t!(vec4.x.acos(), vec4.y.acos(), vec4.z.acos(), vec4.w.acos()), "{vec4}.acos()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.atan(), vec2t!(vec2.x.atan(), vec2.y.atan()), "{vec2}.atan()");
            assert_float_vec!(vec3.atan(), vec3t!(vec3.x.atan(), vec3.y.atan(), vec3.z.atan()), "{vec3}.atan()");
            assert_float_vec!(vec4.atan(), vec4t!(vec4.x.atan(), vec4.y.atan(), vec4.z.atan(), vec4.w.atan()), "{vec4}.atan()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.recip(), vec2t!(vec2.x.recip(), vec2.y.recip()), "{vec2}.recip()");
            assert_float_vec!(vec3.recip(), vec3t!(vec3.x.recip(), vec3.y.recip(), vec3.z.recip()), "{vec3}.recip()");
            assert_float_vec!(vec4.recip(), vec4t!(vec4.x.recip(), vec4.y.recip(), vec4.z.recip(), vec4.w.recip()), "{vec4}.recip()");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |other2, other3, other4| {
            // `Vector::max` is allowed to be incorrect for NaN values
            if vec2.iter().any(|x| x.is_nan()) || other2.iter().any(|x| x.is_nan()) {
                return;
            }

            assert_float_vec!(vreset_zero_sign(vec2.max(other2)), vreset_zero_sign(vec2t!(vec2.x.max(other2.x), vec2.y.max(other2.y))), "{vec2}.max({other2})");
            assert_float_vec!(vreset_zero_sign(vec3.max(other3)), vreset_zero_sign(vec3t!(vec3.x.max(other3.x), vec3.y.max(other3.y), vec3.z.max(other3.z))), "{vec3}.max({other3})");
            assert_float_vec!(vreset_zero_sign(vec4.max(other4)), vreset_zero_sign(vec4t!(vec4.x.max(other4.x), vec4.y.max(other4.y), vec4.z.max(other4.z), vec4.w.max(other4.w))), "{vec4}.max({other4})");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_debug_panic!(vec2.max(vec2t!(1.0, <$T>::NAN)), "{vec2}.max((1.0, NaN))");
            assert_debug_panic!(vec3.max(vec3t!(1.0, <$T>::NAN, 1.0)), "{vec3}.max((1.0, NaN, 1.0))");
            assert_debug_panic!(vec4.max(vec4t!(1.0, <$T>::NAN, 1.0, 1.0)), "{vec4}.max((1.0, NaN, 1.0, 1.0))");

            assert_debug_panic!(vec2t!(1.0, <$T>::NAN).max(vec2), "(1.0, NaN).max({vec2})");
            assert_debug_panic!(vec3t!(1.0, <$T>::NAN, 1.0).max(vec3), "(1.0, NaN, 1.0).max({vec3})");
            assert_debug_panic!(vec4t!(1.0, <$T>::NAN, 1.0, 1.0).max(vec4), "(1.0, NaN, 1.0, 1.0).max({vec4})");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |other2, other3, other4| {
            // `Vector::min` is allowed to be incorrect for NaN values
            if vec2.iter().any(|x| x.is_nan()) || other2.iter().any(|x| x.is_nan()) {
                return;
            }

            assert_float_vec!(vreset_zero_sign(vec2.min(other2)), vreset_zero_sign(vec2t!(vec2.x.min(other2.x), vec2.y.min(other2.y))), "{vec2}.min({other2})");
            assert_float_vec!(vreset_zero_sign(vec3.min(other3)), vreset_zero_sign(vec3t!(vec3.x.min(other3.x), vec3.y.min(other3.y), vec3.z.min(other3.z))), "{vec3}.min({other3})");
            assert_float_vec!(vreset_zero_sign(vec4.min(other4)), vreset_zero_sign(vec4t!(vec4.x.min(other4.x), vec4.y.min(other4.y), vec4.z.min(other4.z), vec4.w.min(other4.w))), "{vec4}.min({other4})");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_debug_panic!(vec2.min(vec2t!(1.0, <$T>::NAN)), "{vec2}.min((1.0, NaN))");
            assert_debug_panic!(vec3.min(vec3t!(1.0, <$T>::NAN, 1.0)), "{vec3}.min((1.0, NaN, 1.0))");
            assert_debug_panic!(vec4.min(vec4t!(1.0, <$T>::NAN, 1.0, 1.0)), "{vec4}.min((1.0, NaN, 1.0, 1.0))");

            assert_debug_panic!(vec2t!(1.0, <$T>::NAN).min(vec2), "(1.0, NaN).min({vec2})");
            assert_debug_panic!(vec3t!(1.0, <$T>::NAN, 1.0).min(vec3), "(1.0, NaN, 1.0).min({vec3})");
            assert_debug_panic!(vec4t!(1.0, <$T>::NAN, 1.0, 1.0).min(vec4), "(1.0, NaN, 1.0, 1.0).min({vec4})");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |other2, other3, other4| {
            // `Vector::midpoint` is allowed to be imprecise for large values
            if vec2.iter().any(|x| x == <$T>::MAX || x == <$T>::MIN) || other2.iter().any(|x| x == <$T>::MAX || x == <$T>::MIN) {
                return;
            }

            assert_float_vec!(vec2.midpoint(other2), vec2t!(vec2.x.midpoint(other2.x), vec2.y.midpoint(other2.y)), "{vec2}.midpoint({other2})");
            assert_float_vec!(vec3.midpoint(other3), vec3t!(vec3.x.midpoint(other3.x), vec3.y.midpoint(other3.y), vec3.z.midpoint(other3.z)), "{vec3}.midpoint({other3})");
            assert_float_vec!(vec4.midpoint(other4), vec4t!(vec4.x.midpoint(other4.x), vec4.y.midpoint(other4.y), vec4.z.midpoint(other4.z), vec4.w.midpoint(other4.w)), "{vec4}.midpoint({other4})");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |min2, min3, min4|, |max2, max3, max4| {
            // `Vector::clamp` is allowed to be incorrect for min values greater than max values
            if min2.iter().zip(max2).any(|(min, max)| min > max) {
                return;
            }

            // `Vector::clamp` is allowed to be incorrect for NaN values
            if vec2.iter().any(|x| x.is_nan()) || min2.iter().any(|x| x.is_nan()) || max2.iter().any(|x| x.is_nan()) {
                return;
            }

            assert_float_vec!(vreset_zero_sign(vec2.clamp(min2, max2)), vreset_zero_sign(vec2t!(vec2.x.clamp(min2.x, max2.x), vec2.y.clamp(min2.y, max2.y))), "{vec2}.clamp({min2}, {max2})");
            assert_float_vec!(vreset_zero_sign(vec3.clamp(min3, max3)), vreset_zero_sign(vec3t!(vec3.x.clamp(min3.x, max3.x), vec3.y.clamp(min3.y, max3.y), vec3.z.clamp(min3.z, max3.z))), "{vec3}.clamp({min3}, {max3})");
            assert_float_vec!(vreset_zero_sign(vec4.clamp(min4, max4)), vreset_zero_sign(vec4t!(vec4.x.clamp(min4.x, max4.x), vec4.y.clamp(min4.y, max4.y), vec4.z.clamp(min4.z, max4.z), vec4.w.clamp(min4.w, max4.w))), "{vec4}.clamp({min4}, {max4})");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_debug_panic!(vec2.clamp(vec2t!(1.0, 3.0), vec2t!(2.0)), "{vec2}.clamp((1.0, 3.0), (2.0, 2.0))");
            assert_debug_panic!(vec3.clamp(vec3t!(1.0, 3.0, 1.0), vec3t!(2.0)), "{vec3}.clamp((1.0, 3.0, 1.0), (2.0, 2.0, 2.0))");
            assert_debug_panic!(vec4.clamp(vec4t!(1.0, 3.0, 1.0, 1.0), vec4t!(2.0)), "{vec4}.clamp((1.0, 3.0, 1.0, 1.0), (2.0, 2.0, 2.0, 2.0))");

            assert_debug_panic!(vec2.clamp(vec2t!(1.0, <$T>::NAN), vec2t!(2.0)), "{vec2}.clamp((1.0, NaN), (2.0, 2.0))");
            assert_debug_panic!(vec3.clamp(vec3t!(1.0, <$T>::NAN, 1.0), vec3t!(2.0)), "{vec3}.clamp((1.0, NaN, 1.0), (2.0, 2.0, 2.0))");
            assert_debug_panic!(vec4.clamp(vec4t!(1.0, <$T>::NAN, 1.0, 1.0), vec4t!(2.0)), "{vec4}.clamp((1.0, NaN, 1.0, 1.0), (2.0, 2.0, 2.0, 2.0))");

            assert_debug_panic!(vec2.clamp(vec2t!(1.0), vec2t!(2.0, <$T>::NAN)), "{vec2}.clamp((1.0, 1.0), (2.0, NaN))");
            assert_debug_panic!(vec3.clamp(vec3t!(1.0), vec3t!(2.0, <$T>::NAN, 2.0)), "{vec3}.clamp((1.0, 1.0, 1.0), (2.0, NaN, 2.0))");
            assert_debug_panic!(vec4.clamp(vec4t!(1.0), vec4t!(2.0, <$T>::NAN, 2.0, 2.0)), "{vec4}.clamp((1.0, 1.0, 1.0, 1.0), (2.0, NaN, 2.0, 2.0))");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.abs(), vec2t!(vec2.x.abs(), vec2.y.abs()), "{vec2}.abs()");
            assert_float_vec!(vec3.abs(), vec3t!(vec3.x.abs(), vec3.y.abs(), vec3.z.abs()), "{vec3}.abs()");
            assert_float_vec!(vec4.abs(), vec4t!(vec4.x.abs(), vec4.y.abs(), vec4.z.abs(), vec4.w.abs()), "{vec4}.abs()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_float_vec!(vec2.signum(), vec2t!(vec2.x.signum(), vec2.y.signum()), "{vec2}.signum()");
            assert_float_vec!(vec3.signum(), vec3t!(vec3.x.signum(), vec3.y.signum(), vec3.z.signum()), "{vec3}.signum()");
            assert_float_vec!(vec4.signum(), vec4t!(vec4.x.signum(), vec4.y.signum(), vec4.z.signum(), vec4.w.signum()), "{vec4}.signum()");
        });

        test_for_combinations!(|vec2, vec3, vec4|, |sign2, sign3, sign4| {
            assert_float_vec!(vec2.copysign(sign2), vec2t!(vec2.x.copysign(sign2.x), vec2.y.copysign(sign2.y)), "{vec2}.copysign({sign2})");
            assert_float_vec!(vec3.copysign(sign3), vec3t!(vec3.x.copysign(sign3.x), vec3.y.copysign(sign3.y), vec3.z.copysign(sign3.z)), "{vec3}.copysign({sign3})");
            assert_float_vec!(vec4.copysign(sign4), vec4t!(vec4.x.copysign(sign4.x), vec4.y.copysign(sign4.y), vec4.z.copysign(sign4.z), vec4.w.copysign(sign4.w)), "{vec4}.copysign({sign4})");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_approx_float!(vec2.sum(), vec2.x + vec2.y, "{vec2}.sum()");
            assert_approx_float!(vec3.sum(), vec3.x + vec3.y + vec3.z, "{vec3}.sum()");
            assert_approx_float!(vec4.sum(), vec4.x + vec4.y + vec4.z + vec4.w, "{vec4}.sum()");
        });

        test_for_combinations!(|vec2, vec3, vec4| {
            assert_approx_float!(vec2.product(), vec2.x * vec2.y, "{vec2}.product()");
            assert_approx_float!(vec3.product(), vec3.x * vec3.y * vec3.z, "{vec3}.product()");
            assert_approx_float!(vec4.product(), vec4.x * vec4.y * vec4.z * vec4.w, "{vec4}.product()");
        });
    }};
}

#[test]
fn test_f32_simd_float_api() {
    test_float_api!($f32, Simd);
}
#[test]
fn test_f32_nonsimd_float_api() {
    test_float_api!($f32, NonSimd);
}

#[test]
fn test_f64_simd_float_api() {
    test_float_api!($f64, Simd);
}
#[test]
fn test_f64_nonsimd_float_api() {
    test_float_api!($f64, NonSimd);
}
