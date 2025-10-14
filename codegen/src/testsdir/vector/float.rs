use genco::{lang::rust::Tokens, quote, tokens::quoted};

use crate::{iter::Float, testsdir::TokensExtendExt};

pub fn generate(t: Float, result: &mut Tokens) {
    result.extend(quote!(
        $['\n']
        use crate::assert_debug_panic;

        fn float_eq(a: $t, b: $t) -> bool {
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

        fn float_vec_eq<const N: usize, S: Simdness>(a: Vector<N, $t, S>, b: Vector<N, $t, S>) -> bool
        where
            $t: Scalar<N>,
        {
            for i in 0..N {
                if !float_eq(a[i], b[i]) {
                    return false;
                }
            }

            true
        }

        macro_rules! assert_float {
            ($$a:expr, $$b:expr, $$($$message:tt)*) => {{
                let a = $$a;
                let b = $$b;

                if !float_eq(a, b) {
                    panic!("incorrect value: {}\n\nexpected: {b:?}\nactual: {a:?}", format_args!($$($$message)*));
                }
            }}
        }

        macro_rules! assert_vec {
            ($$a:expr, $$b:expr, $$($$message:tt)*) => {{
                let a = $$a;
                let b = $$b;

                if !float_vec_eq(a, b) {
                    panic!("incorrect value: {}\n\nexpected: {b:?}\nactual: {a:?}", format_args!($$($$message)*));
                }
            }}
        }
    ));

    #[allow(non_snake_case)]
    result.extend_for_simdness(|s| {
        quote!(
            $(let Vec2 = &format!("Vec2{}", s.uppercase_postfix()))
            $(let Vec3 = &format!("Vec3{}", s.uppercase_postfix()))
            $(let Vec4 = &format!("Vec4{}", s.uppercase_postfix()))
            $(let vec2 = &format!("vec2{}", s.lowercase_postfix()))
            $(let vec3 = &format!("vec3{}", s.lowercase_postfix()))
            $(let vec4 = &format!("vec4{}", s.lowercase_postfix()))

            #[test]
            fn test_$(s.snakecase())_float_fns() {
                assert_vec!($Vec2::ZERO, $vec2!(0.0), $(quoted(format!("{Vec2}::ZERO"))));
                assert_vec!($Vec3::ZERO, $vec3!(0.0), $(quoted(format!("{Vec3}::ZERO"))));
                assert_vec!($Vec4::ZERO, $vec4!(0.0), $(quoted(format!("{Vec4}::ZERO"))));

                assert_vec!($Vec2::ONE, $vec2!(1.0), $(quoted(format!("{Vec2}::ONE"))));
                assert_vec!($Vec3::ONE, $vec3!(1.0), $(quoted(format!("{Vec3}::ONE"))));
                assert_vec!($Vec4::ONE, $vec4!(1.0), $(quoted(format!("{Vec4}::ONE"))));

                assert_vec!($Vec2::NEG_ONE, $vec2!(-1.0), $(quoted(format!("{Vec2}::NEG_ONE"))));
                assert_vec!($Vec3::NEG_ONE, $vec3!(-1.0), $(quoted(format!("{Vec3}::NEG_ONE"))));
                assert_vec!($Vec4::NEG_ONE, $vec4!(-1.0), $(quoted(format!("{Vec4}::NEG_ONE"))));
                
                assert_vec!($Vec2::X, $vec2!(1.0, 0.0), $(quoted(format!("{Vec2}::X"))));
                assert_vec!($Vec3::Y, $vec3!(0.0, 1.0, 0.0), $(quoted(format!("{Vec3}::Y"))));
                assert_vec!($Vec4::Z, $vec4!(0.0, 0.0, 1.0, 0.0), $(quoted(format!("{Vec4}::Z"))));

                assert_vec!($Vec2::NEG_X, $vec2!(-1.0, 0.0), $(quoted(format!("{Vec2}::NEG_X"))));
                assert_vec!($Vec3::NEG_Y, $vec3!(0.0, -1.0, 0.0), $(quoted(format!("{Vec3}::NEG_Y"))));
                assert_vec!($Vec4::NEG_Z, $vec4!(0.0, 0.0, -1.0, 0.0), $(quoted(format!("{Vec4}::NEG_Z"))));

                #[cfg(feature = "right")]
                {
                    use ggmath::right::*;

                    assert_vec!($Vec2::<$t>::RIGHT, $Vec2::<$t>::X, $(quoted(format!("{Vec2}::RIGHT"))));
                    assert_vec!($Vec3::<$t>::RIGHT, $Vec3::<$t>::X, $(quoted(format!("{Vec3}::RIGHT"))));
                    assert_vec!($Vec4::<$t>::RIGHT, $Vec4::<$t>::X, $(quoted(format!("{Vec4}::RIGHT"))));

                    assert_vec!($Vec2::<$t>::LEFT, $Vec2::<$t>::NEG_X, $(quoted(format!("{Vec2}::LEFT"))));
                    assert_vec!($Vec3::<$t>::LEFT, $Vec3::<$t>::NEG_X, $(quoted(format!("{Vec3}::LEFT"))));
                    assert_vec!($Vec4::<$t>::LEFT, $Vec4::<$t>::NEG_X, $(quoted(format!("{Vec4}::LEFT"))));
                }

                #[cfg(feature = "left")]
                {
                    use ggmath::left::*;

                    assert_vec!($Vec2::<$t>::RIGHT, $Vec2::<$t>::NEG_X, $(quoted(format!("{Vec2}::RIGHT"))));
                    assert_vec!($Vec3::<$t>::RIGHT, $Vec3::<$t>::NEG_X, $(quoted(format!("{Vec3}::RIGHT"))));
                    assert_vec!($Vec4::<$t>::RIGHT, $Vec4::<$t>::NEG_X, $(quoted(format!("{Vec4}::RIGHT"))));

                    assert_vec!($Vec2::<$t>::LEFT, $Vec2::<$t>::X, $(quoted(format!("{Vec2}::LEFT"))));
                    assert_vec!($Vec3::<$t>::LEFT, $Vec3::<$t>::X, $(quoted(format!("{Vec3}::LEFT"))));
                    assert_vec!($Vec4::<$t>::LEFT, $Vec4::<$t>::X, $(quoted(format!("{Vec4}::LEFT"))));
                }

                #[cfg(feature = "backwards")]
                {
                    use ggmath::backwards::*;

                    assert_vec!($Vec3::<$t>::FORWARDS, $Vec3::<$t>::NEG_Z, $(quoted(format!("{Vec3}::FORWARDS"))));
                    assert_vec!($Vec4::<$t>::FORWARDS, $Vec4::<$t>::NEG_Z, $(quoted(format!("{Vec4}::FORWARDS"))));

                    assert_vec!($Vec3::<$t>::BACKWARDS, $Vec3::<$t>::Z, $(quoted(format!("{Vec3}::BACKWARDS"))));
                    assert_vec!($Vec4::<$t>::BACKWARDS, $Vec4::<$t>::Z, $(quoted(format!("{Vec4}::BACKWARDS"))));
                }

                const UNIQUE_FLOAT_VALUES: &[$t] = &[
                    1.0,
                    5.3,
                    -1.0,
                    -23.4,
                    0.0,
                    -0.0,
                    $t::MIN,
                    $t::MAX,
                    $t::INFINITY,
                    $t::NEG_INFINITY,
                    $t::NAN,
                ];

                macro_rules! test_for_combinations {
                    (|$$vec2:ident, $$vec3:ident, $$vec4:ident| { $$($$test:tt)* }) => {
                        for &x in UNIQUE_FLOAT_VALUES {
                            for &y in UNIQUE_FLOAT_VALUES {
                                let vec2 = $vec2!(x, y);
                                let vec3 = $vec3!(x, y, x);
                                let vec4 = $vec4!(x, y, x, y);

                                (|$$vec2: $Vec2<$t>, $$vec3: $Vec3<$t>, $$vec4: $Vec4<$t>| { $$($$test)* })(vec2, vec3, vec4);
                            }
                        }
                    };

                    (|$$first_vec2:ident, $$first_vec3:ident, $$first_vec4:ident|, $$(|$$vec2:ident, $$vec3:ident, $$vec4:ident|),+ { $$($$test:tt)* }) => {
                        test_for_combinations!(
                            $$(|$$vec2, $$vec3, $$vec4|),+ {
                                test_for_combinations!(
                                    |$$first_vec2, $$first_vec3, $$first_vec4| { $$($$test)* }
                                )
                            }
                        )
                    };
                }

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(-vec2, $vec2!(-vec2.x, -vec2.y), "-{vec2}");
                        assert_vec!(-vec3, $vec3!(-vec3.x, -vec3.y, -vec3.z), "-{vec3}");
                        assert_vec!(-vec4, $vec4!(-vec4.x, -vec4.y, -vec4.z, -vec4.w), "-{vec4}");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2 + rhs2, $vec2!(vec2.x + rhs2.x, vec2.y + rhs2.y), "{vec2} + {rhs2}");
                        assert_vec!(vec3 + rhs3, $vec3!(vec3.x + rhs3.x, vec3.y + rhs3.y, vec3.z + rhs3.z), "{vec3} + {rhs3}");
                        assert_vec!(vec4 + rhs4, $vec4!(vec4.x + rhs4.x, vec4.y + rhs4.y, vec4.z + rhs4.z, vec4.w + rhs4.w), "{vec4} + {rhs4}");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2 - rhs2, $vec2!(vec2.x - rhs2.x, vec2.y - rhs2.y), "{vec2} - {rhs2}");
                        assert_vec!(vec3 - rhs3, $vec3!(vec3.x - rhs3.x, vec3.y - rhs3.y, vec3.z - rhs3.z), "{vec3} - {rhs3}");
                        assert_vec!(vec4 - rhs4, $vec4!(vec4.x - rhs4.x, vec4.y - rhs4.y, vec4.z - rhs4.z, vec4.w - rhs4.w), "{vec4} - {rhs4}");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2 * rhs2, $vec2!(vec2.x * rhs2.x, vec2.y * rhs2.y), "{vec2} * {rhs2}");
                        assert_vec!(vec3 * rhs3, $vec3!(vec3.x * rhs3.x, vec3.y * rhs3.y, vec3.z * rhs3.z), "{vec3} * {rhs3}");
                        assert_vec!(vec4 * rhs4, $vec4!(vec4.x * rhs4.x, vec4.y * rhs4.y, vec4.z * rhs4.z, vec4.w * rhs4.w), "{vec4} * {rhs4}");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2 / rhs2, $vec2!(vec2.x / rhs2.x, vec2.y / rhs2.y), "{vec2} / {rhs2}");
                        assert_vec!(vec3 / rhs3, $vec3!(vec3.x / rhs3.x, vec3.y / rhs3.y, vec3.z / rhs3.z), "{vec3} / {rhs3}");
                        assert_vec!(vec4 / rhs4, $vec4!(vec4.x / rhs4.x, vec4.y / rhs4.y, vec4.z / rhs4.z, vec4.w / rhs4.w), "{vec4} / {rhs4}");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2 % rhs2, $vec2!(vec2.x % rhs2.x, vec2.y % rhs2.y), "{vec2} % {rhs2}");
                        assert_vec!(vec3 % rhs3, $vec3!(vec3.x % rhs3.x, vec3.y % rhs3.y, vec3.z % rhs3.z), "{vec3} % {rhs3}");
                        assert_vec!(vec4 % rhs4, $vec4!(vec4.x % rhs4.x, vec4.y % rhs4.y, vec4.z % rhs4.z, vec4.w % rhs4.w), "{vec4} % {rhs4}");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.floor(), $vec2!(vec2.x.floor(), vec2.y.floor()), "{vec2}.floor()");
                        assert_vec!(vec3.floor(), $vec3!(vec3.x.floor(), vec3.y.floor(), vec3.z.floor()), "{vec3}.floor()");
                        assert_vec!(vec4.floor(), $vec4!(vec4.x.floor(), vec4.y.floor(), vec4.z.floor(), vec4.w.floor()), "{vec4}.floor()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.ceil(), $vec2!(vec2.x.ceil(), vec2.y.ceil()), "{vec2}.ceil()");
                        assert_vec!(vec3.ceil(), $vec3!(vec3.x.ceil(), vec3.y.ceil(), vec3.z.ceil()), "{vec3}.ceil()");
                        assert_vec!(vec4.ceil(), $vec4!(vec4.x.ceil(), vec4.y.ceil(), vec4.z.ceil(), vec4.w.ceil()), "{vec4}.ceil()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.round(), $vec2!(vec2.x.round(), vec2.y.round()), "{vec2}.round()");
                        assert_vec!(vec3.round(), $vec3!(vec3.x.round(), vec3.y.round(), vec3.z.round()), "{vec3}.round()");
                        assert_vec!(vec4.round(), $vec4!(vec4.x.round(), vec4.y.round(), vec4.z.round(), vec4.w.round()), "{vec4}.round()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.trunc(), $vec2!(vec2.x.trunc(), vec2.y.trunc()), "{vec2}.trunc()");
                        assert_vec!(vec3.trunc(), $vec3!(vec3.x.trunc(), vec3.y.trunc(), vec3.z.trunc()), "{vec3}.trunc()");
                        assert_vec!(vec4.trunc(), $vec4!(vec4.x.trunc(), vec4.y.trunc(), vec4.z.trunc(), vec4.w.trunc()), "{vec4}.trunc()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.fract(), $vec2!(vec2.x.fract(), vec2.y.fract()), "{vec2}.fract()");
                        assert_vec!(vec3.fract(), $vec3!(vec3.x.fract(), vec3.y.fract(), vec3.z.fract()), "{vec3}.fract()");
                        assert_vec!(vec4.fract(), $vec4!(vec4.x.fract(), vec4.y.fract(), vec4.z.fract(), vec4.w.fract()), "{vec4}.fract()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |a2, a3, a4|, |b2, b3, b4| {
                        assert_vec!(vec2.mul_add(a2, b2), $vec2!(vec2.x.mul_add(a2.x, b2.x), vec2.y.mul_add(a2.y, b2.y)), "{vec2}.mul_add({a2}, {b2})");
                        assert_vec!(vec3.mul_add(a3, b3), $vec3!(vec3.x.mul_add(a3.x, b3.x), vec3.y.mul_add(a3.y, b3.y), vec3.z.mul_add(a3.z, b3.z)), "{vec3}.mul_add({a3}, {b3})");
                        assert_vec!(vec4.mul_add(a4, b4), $vec4!(vec4.x.mul_add(a4.x, b4.x), vec4.y.mul_add(a4.y, b4.y), vec4.z.mul_add(a4.z, b4.z), vec4.w.mul_add(a4.w, b4.w)), "{vec4}.mul_add({a4}, {b4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2.div_euclid(rhs2), $vec2!(vec2.x.div_euclid(rhs2.x), vec2.y.div_euclid(rhs2.y)), "{vec2}.div_euclid({rhs2})");
                        assert_vec!(vec3.div_euclid(rhs3), $vec3!(vec3.x.div_euclid(rhs3.x), vec3.y.div_euclid(rhs3.y), vec3.z.div_euclid(rhs3.z)), "{vec3}.div_euclid({rhs3})");
                        assert_vec!(vec4.div_euclid(rhs4), $vec4!(vec4.x.div_euclid(rhs4.x), vec4.y.div_euclid(rhs4.y), vec4.z.div_euclid(rhs4.z), vec4.w.div_euclid(rhs4.w)), "{vec4}.div_euclid({rhs4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |rhs2, rhs3, rhs4| {
                        assert_vec!(vec2.rem_euclid(rhs2), $vec2!(vec2.x.rem_euclid(rhs2.x), vec2.y.rem_euclid(rhs2.y)), "{vec2}.rem_euclid({rhs2})");
                        assert_vec!(vec3.rem_euclid(rhs3), $vec3!(vec3.x.rem_euclid(rhs3.x), vec3.y.rem_euclid(rhs3.y), vec3.z.rem_euclid(rhs3.z)), "{vec3}.rem_euclid({rhs3})");
                        assert_vec!(vec4.rem_euclid(rhs4), $vec4!(vec4.x.rem_euclid(rhs4.x), vec4.y.rem_euclid(rhs4.y), vec4.z.rem_euclid(rhs4.z), vec4.w.rem_euclid(rhs4.w)), "{vec4}.rem_euclid({rhs4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.sqrt(), $vec2!(vec2.x.sqrt(), vec2.y.sqrt()), "{vec2}.sqrt()");
                        assert_vec!(vec3.sqrt(), $vec3!(vec3.x.sqrt(), vec3.y.sqrt(), vec3.z.sqrt()), "{vec3}.sqrt()");
                        assert_vec!(vec4.sqrt(), $vec4!(vec4.x.sqrt(), vec4.y.sqrt(), vec4.z.sqrt(), vec4.w.sqrt()), "{vec4}.sqrt()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.sin(), $vec2!(vec2.x.sin(), vec2.y.sin()), "{vec2}.sin()");
                        assert_vec!(vec3.sin(), $vec3!(vec3.x.sin(), vec3.y.sin(), vec3.z.sin()), "{vec3}.sin()");
                        assert_vec!(vec4.sin(), $vec4!(vec4.x.sin(), vec4.y.sin(), vec4.z.sin(), vec4.w.sin()), "{vec4}.sin()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.cos(), $vec2!(vec2.x.cos(), vec2.y.cos()), "{vec2}.cos()");
                        assert_vec!(vec3.cos(), $vec3!(vec3.x.cos(), vec3.y.cos(), vec3.z.cos()), "{vec3}.cos()");
                        assert_vec!(vec4.cos(), $vec4!(vec4.x.cos(), vec4.y.cos(), vec4.z.cos(), vec4.w.cos()), "{vec4}.cos()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.tan(), $vec2!(vec2.x.tan(), vec2.y.tan()), "{vec2}.tan()");
                        assert_vec!(vec3.tan(), $vec3!(vec3.x.tan(), vec3.y.tan(), vec3.z.tan()), "{vec3}.tan()");
                        assert_vec!(vec4.tan(), $vec4!(vec4.x.tan(), vec4.y.tan(), vec4.z.tan(), vec4.w.tan()), "{vec4}.tan()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.asin(), $vec2!(vec2.x.asin(), vec2.y.asin()), "{vec2}.asin()");
                        assert_vec!(vec3.asin(), $vec3!(vec3.x.asin(), vec3.y.asin(), vec3.z.asin()), "{vec3}.asin()");
                        assert_vec!(vec4.asin(), $vec4!(vec4.x.asin(), vec4.y.asin(), vec4.z.asin(), vec4.w.asin()), "{vec4}.asin()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.acos(), $vec2!(vec2.x.acos(), vec2.y.acos()), "{vec2}.acos()");
                        assert_vec!(vec3.acos(), $vec3!(vec3.x.acos(), vec3.y.acos(), vec3.z.acos()), "{vec3}.acos()");
                        assert_vec!(vec4.acos(), $vec4!(vec4.x.acos(), vec4.y.acos(), vec4.z.acos(), vec4.w.acos()), "{vec4}.acos()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.atan(), $vec2!(vec2.x.atan(), vec2.y.atan()), "{vec2}.atan()");
                        assert_vec!(vec3.atan(), $vec3!(vec3.x.atan(), vec3.y.atan(), vec3.z.atan()), "{vec3}.atan()");
                        assert_vec!(vec4.atan(), $vec4!(vec4.x.atan(), vec4.y.atan(), vec4.z.atan(), vec4.w.atan()), "{vec4}.atan()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_vec!(vec2.recip(), $vec2!(vec2.x.recip(), vec2.y.recip()), "{vec2}.recip()");
                        assert_vec!(vec3.recip(), $vec3!(vec3.x.recip(), vec3.y.recip(), vec3.z.recip()), "{vec3}.recip()");
                        assert_vec!(vec4.recip(), $vec4!(vec4.x.recip(), vec4.y.recip(), vec4.z.recip(), vec4.w.recip()), "{vec4}.recip()");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |other2, other3, other4| {
                        // don't test max(0.0, -0.0) because its non deterministic
                        if vec2.x == 0.0 && other2.x == 0.0 || vec2.y == 0.0 && other2.y == 0.0 {
                            return;
                        }

                        // don't test nan because its non deterministic
                        if vec2.x.is_nan() || vec2.y.is_nan() || other2.x.is_nan() || other2.y.is_nan() {
                            return;
                        }

                        assert_vec!(vec2.max(other2), $vec2!(vec2.x.max(other2.x), vec2.y.max(other2.y)), "{vec2}.max({other2})");
                        assert_vec!(vec3.max(other3), $vec3!(vec3.x.max(other3.x), vec3.y.max(other3.y), vec3.z.max(other3.z)), "{vec3}.max({other3})");
                        assert_vec!(vec4.max(other4), $vec4!(vec4.x.max(other4.x), vec4.y.max(other4.y), vec4.z.max(other4.z), vec4.w.max(other4.w)), "{vec4}.max({other4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_debug_panic!(vec2.max($vec2!(1.0, $t::NAN)));
                        assert_debug_panic!(vec3.max($vec3!(1.0, $t::NAN, 1.0)));
                        assert_debug_panic!(vec4.max($vec4!(1.0, $t::NAN, 1.0, 1.0)));
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |other2, other3, other4| {
                        // don't test min(0.0, -0.0) because its non deterministic
                        if vec2.x == 0.0 && other2.x == 0.0 || vec2.y == 0.0 && other2.y == 0.0 {
                            return;
                        }

                        // don't test nan because its non deterministic
                        if vec2.x.is_nan() || vec2.y.is_nan() || other2.x.is_nan() || other2.y.is_nan() {
                            return;
                        }

                        assert_vec!(vec2.min(other2), $vec2!(vec2.x.min(other2.x), vec2.y.min(other2.y)), "{vec2}.min({other2})");
                        assert_vec!(vec3.min(other3), $vec3!(vec3.x.min(other3.x), vec3.y.min(other3.y), vec3.z.min(other3.z)), "{vec3}.min({other3})");
                        assert_vec!(vec4.min(other4), $vec4!(vec4.x.min(other4.x), vec4.y.min(other4.y), vec4.z.min(other4.z), vec4.w.min(other4.w)), "{vec4}.min({other4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_debug_panic!(vec2.min($vec2!(1.0, $t::NAN)));
                        assert_debug_panic!(vec3.min($vec3!(1.0, $t::NAN, 1.0)));
                        assert_debug_panic!(vec4.min($vec4!(1.0, $t::NAN, 1.0, 1.0)));
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |other2, other3, other4| {
                        assert_vec!(vec2.midpoint(other2), $vec2!(vec2.x.midpoint(other2.x), vec2.y.midpoint(other2.y)), "{vec2}.midpoint({other2})");
                        assert_vec!(vec3.midpoint(other3), $vec3!(vec3.x.midpoint(other3.x), vec3.y.midpoint(other3.y), vec3.z.midpoint(other3.z)), "{vec3}.midpoint({other3})");
                        assert_vec!(vec4.midpoint(other4), $vec4!(vec4.x.midpoint(other4.x), vec4.y.midpoint(other4.y), vec4.z.midpoint(other4.z), vec4.w.midpoint(other4.w)), "{vec4}.midpoint({other4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4|, |min2, min3, min4|, |max2, max3, max4| {
                        // don't test min > max because its non deterministic
                        if min2.x > max2.x || min2.y > max2.y {
                            return;
                        }

                        // don't test nan min/max because its non deterministic
                        if min2.x.is_nan() || min2.y.is_nan() || max2.x.is_nan() || max2.y.is_nan() {
                            return;
                        }

                        assert_vec!(vec2.clamp(min2, max2), $vec2!(vec2.x.clamp(min2.x, max2.x), vec2.y.clamp(min2.y, max2.y)), "{vec2}.clamp({min2}, {max2})");
                        assert_vec!(vec3.clamp(min3, max3), $vec3!(vec3.x.clamp(min3.x, max3.x), vec3.y.clamp(min3.y, max3.y), vec3.z.clamp(min3.z, max3.z)), "{vec3}.clamp({min3}, {max3})");
                        assert_vec!(vec4.clamp(min4, max4), $vec4!(vec4.x.clamp(min4.x, max4.x), vec4.y.clamp(min4.y, max4.y), vec4.z.clamp(min4.z, max4.z), vec4.w.clamp(min4.w, max4.w)), "{vec4}.clamp({min4}, {max4})");
                    }
                );

                test_for_combinations!(
                    |vec2, vec3, vec4| {
                        assert_debug_panic!(vec2.clamp($vec2!(1.0, 3.0), $vec2!(2.0)));
                        assert_debug_panic!(vec3.clamp($vec3!(1.0, 3.0, 1.0), $vec3!(2.0)));
                        assert_debug_panic!(vec4.clamp($vec4!(1.0, 3.0, 1.0, 1.0), $vec4!(2.0)));

                        assert_debug_panic!(vec2.clamp($vec2!(1.0, $t::NAN), $vec2!(2.0)));
                        assert_debug_panic!(vec3.clamp($vec3!(1.0, $t::NAN, 1.0), $vec3!(2.0)));
                        assert_debug_panic!(vec4.clamp($vec4!(1.0, $t::NAN, 1.0, 1.0), $vec4!(2.0)));

                        assert_debug_panic!(vec2.clamp($vec2!(1.0), $vec2!(2.0, $t::NAN)));
                        assert_debug_panic!(vec3.clamp($vec3!(1.0), $vec3!(2.0, $t::NAN, 2.0)));
                        assert_debug_panic!(vec4.clamp($vec4!(1.0), $vec4!(2.0, $t::NAN, 2.0, 2.0)));
                    }
                );
            }
        )
    });
}
