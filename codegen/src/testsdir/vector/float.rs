use genco::{lang::rust::Tokens, quote, tokens::quoted};

use crate::{iter::Float, testsdir::TokensExtendExt};

pub fn generate(t: Float, result: &mut Tokens) {
    result.extend(quote!(
        $['\n']
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
            $t: ElementOfVector<N, S>,
        {
            for i in 0..N {
                if !float_eq(a[i], b[i]) {
                    return false;
                }
            }

            true
        }

        macro_rules! assert_float_eq {
            ($$a:expr, $$b:expr, $$($$message:tt)*) => {{
                let a = $$a;
                let b = $$b;

                if !float_eq(a, b) {
                    panic!("assertion failed: {}\n\nexpected: {b:?}\nactual: {a:?}", format_args!($$($$message)*));
                }
            }}
        }

        macro_rules! assert_float_vec_eq {
            ($$a:expr, $$b:expr, $$($$message:tt)*) => {{
                let a = $$a;
                let b = $$b;

                if !float_vec_eq(a, b) {
                    panic!("assertion failed: {}\n\nexpected: {b:?}\nactual: {a:?}", format_args!($$($$message)*));
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
                const UNIQUE_FLOAT_VALUES: &[$t] = &[
                    1.0,
                    -1.0,
                    0.0,
                    -0.0,
                    $t::MIN,
                    $t::MAX,
                    $t::INFINITY,
                    $t::NEG_INFINITY,
                    $t::NAN,
                ];

                macro_rules! test_unary_op {
                    ($$op:tt) => {
                        for x in UNIQUE_FLOAT_VALUES.iter().copied() {
                            let op = stringify!($$op);
                            let op_x = $$op x;
                            let op_one = $$op 1.0;

                            assert_float_vec_eq!(
                                $$op $vec2!(1.0, x),
                                $vec2!(op_one, op_x),
                                $(quoted(format!("{{op}}{vec2}!(1.0, {{x:?}}) == {vec2}!({{op_one:?}}, {{op_x:?}})")))
                            );
                            assert_float_vec_eq!(
                                $$op $vec3!(1.0, x, 1.0),
                                $vec3!(op_one, op_x, op_one),
                                $(quoted(format!("{{op}}{vec3}!(1.0, {{x:?}}, 1.0) == {vec3}!({{op_one:?}}, {{op_x:?}}, {{op_one:?}})")))
                            );
                            assert_float_vec_eq!(
                                $$op $vec4!(1.0, x, 1.0, 1.0),
                                $vec4!(op_one, op_x, op_one, op_one),
                                $(quoted(format!("{{op}}{vec4}!(1.0, {{x:?}}, 1.0, 1.0) == {vec4}!({{op_one:?}}, {{op_x:?}}, {{op_one:?}}, {{op_one:?}})")))
                            );
                        }
                    }
                }
                
                macro_rules! test_binary_op {
                    ($$op:tt) => {
                        for (x, y) in UNIQUE_FLOAT_VALUES.iter().copied().flat_map(|x| UNIQUE_FLOAT_VALUES.iter().copied().map(move |y| (x, y))) {
                            let op = stringify!($$op);
                            let x_op_y = x $$op y;
                            let one_op_one = 1.0 $$op 1.0;

                            assert_float_vec_eq!(
                                $vec2!(1.0, x) $$op $vec2!(1.0, y),
                                $vec2!(one_op_one, x_op_y),
                                $(quoted(format!("{vec2}!(1.0, {{x:?}}) {{op}} {vec2}!(1.0, {{y:?}}) == {vec2}!({{one_op_one:?}}, {{x_op_y:?}})")))
                            );
                            assert_float_vec_eq!(
                                $vec3!(1.0, x, 1.0) $$op $vec3!(1.0, y, 1.0),
                                $vec3!(one_op_one, x_op_y, one_op_one),
                                $(quoted(format!("{vec3}!(1.0, {{x:?}}, 1.0) {{op}} {vec3}!(1.0, {{y:?}}, 1.0) == {vec3}!({{one_op_one:?}}, {{x_op_y:?}}, {{one_op_one:?}})")))
                            );
                            assert_float_vec_eq!(
                                $vec4!(1.0, x, 1.0, 1.0) $$op $vec4!(1.0, y, 1.0, 1.0),
                                $vec4!(one_op_one, x_op_y, one_op_one, one_op_one),
                                $(quoted(format!("{vec4}!(1.0, {{x:?}}, 1.0, 1.0) {{op}} {vec4}!(1.0, {{y:?}}, 1.0, 1.0) == {vec4}!({{one_op_one:?}}, {{x_op_y:?}}, {{one_op_one:?}}, {{one_op_one:?}})")))
                            );
                        }
                    }
                }

                test_unary_op!(-);
                test_binary_op!(+);
                test_binary_op!(-);
                test_binary_op!(*);
                test_binary_op!(/);
                test_binary_op!(%);

                assert_float_vec_eq!($Vec2::ZERO, $vec2!(0.0, 0.0), $(quoted(format!("{Vec2}::ZERO == {vec2}!(0.0, 0.0)"))));
                assert_float_vec_eq!($Vec3::ZERO, $vec3!(0.0, 0.0, 0.0), $(quoted(format!("{Vec3}::ZERO == {vec3}!(0.0, 0.0, 0.0)"))));
                assert_float_vec_eq!($Vec4::ZERO, $vec4!(0.0, 0.0, 0.0, 0.0), $(quoted(format!("{Vec4}::ZERO == {vec4}!(0.0, 0.0, 0.0, 0.0)"))));

                assert_float_vec_eq!($Vec2::ONE, $vec2!(1.0, 1.0), $(quoted(format!("{Vec2}::ONE == {vec2}!(1.0, 1.0)"))));
                assert_float_vec_eq!($Vec3::ONE, $vec3!(1.0, 1.0, 1.0), $(quoted(format!("{Vec3}::ONE == {vec3}!(1.0, 1.0, 1.0)"))));
                assert_float_vec_eq!($Vec4::ONE, $vec4!(1.0, 1.0, 1.0, 1.0), $(quoted(format!("{Vec4}::ONE == {vec4}!(1.0, 1.0, 1.0, 1.0)"))));

                assert_float_vec_eq!($Vec2::NEG_ONE, $vec2!(-1.0, -1.0), $(quoted(format!("{Vec2}::NEG_ONE == {vec2}!(-1.0, -1.0)"))));
                assert_float_vec_eq!($Vec3::NEG_ONE, $vec3!(-1.0, -1.0, -1.0), $(quoted(format!("{Vec3}::NEG_ONE == {vec3}!(-1.0, -1.0, -1.0)"))));
                assert_float_vec_eq!($Vec4::NEG_ONE, $vec4!(-1.0, -1.0, -1.0, -1.0), $(quoted(format!("{Vec4}::NEG_ONE == {vec4}!(-1.0, -1.0, -1.0, -1.0)"))));
                
                assert_float_vec_eq!($Vec2::X, $vec2!(1.0, 0.0), $(quoted(format!("{Vec2}::X == {vec2}!(1.0, 0.0)"))));
                assert_float_vec_eq!($Vec3::Y, $vec3!(0.0, 1.0, 0.0), $(quoted(format!("{Vec3}::Y == {vec3}!(0.0, 1.0, 0.0)"))));
                assert_float_vec_eq!($Vec4::Z, $vec4!(0.0, 0.0, 1.0, 0.0), $(quoted(format!("{Vec4}::Z == {vec4}!(0.0, 0.0, 1.0, 0.0)"))));

                assert_float_vec_eq!($Vec2::NEG_X, $vec2!(-1.0, 0.0), $(quoted(format!("{Vec2}::NEG_X == {vec2}!(-1.0, 0.0)"))));
                assert_float_vec_eq!($Vec3::NEG_Y, $vec3!(0.0, -1.0, 0.0), $(quoted(format!("{Vec3}::NEG_Y == {vec3}!(0.0, -1.0, 0.0)"))));
                assert_float_vec_eq!($Vec4::NEG_Z, $vec4!(0.0, 0.0, -1.0, 0.0), $(quoted(format!("{Vec4}::NEG_Z == {vec4}!(0.0, 0.0, -1.0, 0.0)"))));

                #[cfg(feature = "right")]
                {
                    use ggmath::right::*;

                    assert_float_vec_eq!($Vec2::<$t>::RIGHT, $Vec2::<$t>::X, $(quoted(format!("{Vec2}::RIGHT == {Vec2}::X"))));
                    assert_float_vec_eq!($Vec3::<$t>::RIGHT, $Vec3::<$t>::X, $(quoted(format!("{Vec3}::RIGHT == {Vec3}::X"))));
                    assert_float_vec_eq!($Vec4::<$t>::RIGHT, $Vec4::<$t>::X, $(quoted(format!("{Vec4}::RIGHT == {Vec4}::X"))));

                    assert_float_vec_eq!($Vec2::<$t>::LEFT, $Vec2::<$t>::NEG_X, $(quoted(format!("{Vec2}::LEFT == {Vec2}::NEG_X"))));
                    assert_float_vec_eq!($Vec3::<$t>::LEFT, $Vec3::<$t>::NEG_X, $(quoted(format!("{Vec3}::LEFT == {Vec3}::NEG_X"))));
                    assert_float_vec_eq!($Vec4::<$t>::LEFT, $Vec4::<$t>::NEG_X, $(quoted(format!("{Vec4}::LEFT == {Vec4}::NEG_X"))));
                }

                #[cfg(feature = "left")]
                {
                    use ggmath::left::*;

                    assert_float_vec_eq!($Vec2::<$t>::RIGHT, $Vec2::<$t>::NEG_X, $(quoted(format!("{Vec2}::RIGHT == {Vec2}::NEG_X"))));
                    assert_float_vec_eq!($Vec3::<$t>::RIGHT, $Vec3::<$t>::NEG_X, $(quoted(format!("{Vec3}::RIGHT == {Vec3}::NEG_X"))));
                    assert_float_vec_eq!($Vec4::<$t>::RIGHT, $Vec4::<$t>::NEG_X, $(quoted(format!("{Vec4}::RIGHT == {Vec4}::NEG_X"))));

                    assert_float_vec_eq!($Vec2::<$t>::LEFT, $Vec2::<$t>::X, $(quoted(format!("{Vec2}::LEFT == {Vec2}::X"))));
                    assert_float_vec_eq!($Vec3::<$t>::LEFT, $Vec3::<$t>::X, $(quoted(format!("{Vec3}::LEFT == {Vec3}::X"))));
                    assert_float_vec_eq!($Vec4::<$t>::LEFT, $Vec4::<$t>::X, $(quoted(format!("{Vec4}::LEFT == {Vec4}::X"))));
                }

                #[cfg(feature = "backwards")]
                {
                    use ggmath::backwards::*;

                    assert_float_vec_eq!($Vec3::<$t>::FORWARDS, $Vec3::<$t>::NEG_Z, $(quoted(format!("{Vec3}::FORWARDS == {Vec3}::NEG_Z"))));
                    assert_float_vec_eq!($Vec4::<$t>::FORWARDS, $Vec4::<$t>::NEG_Z, $(quoted(format!("{Vec4}::FORWARDS == {Vec4}::NEG_Z"))));

                    assert_float_vec_eq!($Vec3::<$t>::BACKWARDS, $Vec3::<$t>::Z, $(quoted(format!("{Vec3}::BACKWARDS == {Vec3}::Z"))));
                    assert_float_vec_eq!($Vec4::<$t>::BACKWARDS, $Vec4::<$t>::Z, $(quoted(format!("{Vec4}::BACKWARDS == {Vec4}::Z"))));
                }
            }
        )
    });
}
