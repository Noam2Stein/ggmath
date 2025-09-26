use genco::{
    lang::rust::{import, Tokens},
    quote, tokens::quoted,
};

use crate::{constants::LENGTHS, primitives::{Primitive, PrimitiveFloat, PrimitiveInt}, vector::primitives::PrimitiveSrcMod};

pub fn push_src(
    primitive: Primitive,
    output: &mut PrimitiveSrcMod,
) {
    output.impl_items.push(quote! {
        $("// The following code is generated for all primitives")

        $(let transmute_copy = &import("core::mem", "transmute_copy"))

        $("/// Variation of `Vector::from_array` that is `const`.")
        $("/// This may be slower than `Vector::from_array`.")
        $("///")
        $("/// When rust stabilizes const traits, this will be deleted.")
        #[inline(always)]
        pub const fn const_from_array(array: [$primitive; N]) -> Self {
            unsafe {
                if S::IS_SIMD {
                    match N {
                        $(
                            for &n in LENGTHS join($['\r']) =>

                            $n => {
                                let array = $transmute_copy::<[$primitive; N], [$primitive; $n]>(&array);
                                let vec = Vector::<$n, _, _>(array);

                                $transmute_copy::<
                                    Vector<$n, $primitive, Simd>,
                                    Vector<N, $primitive, S>,
                                >(&vec)
                            },
                        )
                        _ => panic!("unusual vector type")
                    }
                } else {
                    $transmute_copy::<
                        Vector<N, $primitive, NonSimd>,
                        Vector<N, $primitive, S>,
                    >(&Vector(array))
                }
            }
        }

        $("/// Variation of `Vector::splat` that is `const`.")
        $("/// This may be slower than `Vector::splat`.")
        $("///")
        $("/// When rust stabilizes const traits, this will be deleted.")
        #[inline(always)]
        pub const fn const_splat(value: $primitive) -> Self {
            Self::const_from_array([value; N])
        }
    });

    output.trait_impls.push(quote! {
        impl Scalar for $primitive {
            $(for &n in LENGTHS join($['\r']) => type InnerSimdVec$(n) = [$primitive; $n];)

            $(
                for &n in LENGTHS join($['\n']) =>

                #[inline(always)]
                fn vec$(n)_from_array(array: [$primitive; $n]) -> Vec$(n)<$primitive> {
                    Vector(array)
                }

                #[inline(always)]
                fn vec$(n)_as_array(vec: Vec$(n)<$primitive>) -> [$primitive; $n] {
                    vec.0
                }
            )
        }
    });
}

pub fn push_tests(n: usize, primitive: Primitive, is_simd: bool, tests: &mut Vec<Tokens>) {
    let s_postfix_lowercase = if is_simd { "" } else { "s" };
    let s_postfix_camelcase = if is_simd { "" } else { "S" };

    let vec_snakecase = &format!("vec{n}{s_postfix_lowercase}");
    let vec_camelcase = &format!("Vec{n}{s_postfix_camelcase}");

    let values = match primitive {
        Primitive::Float(primitive) => (0..n).map(|i| format!("{i}.0{primitive}")).collect::<Vec<String>>(),
        Primitive::Int(primitive) => (0..n).map(|i| format!("{i}{primitive}")).collect::<Vec<String>>(),
        Primitive::Bool => ["false".to_string(), "true".to_string()].into_iter().cycle().take(n).collect::<Vec<String>>(),
    };
    let values_str = &values.join(", ");

    let values2 = match primitive {
        _ if FLOAT_PRIMITIVES.contains(&primitive) => (n..n * 2).map(|i| format!("{i}.0{primitive}")).collect::<Vec<String>>(),
        _ if INT_PRIMITIVES.contains(&primitive) => (n..n * 2).map(|i| format!("{i}{primitive}")).collect::<Vec<String>>(),
        "bool" => ["true".to_string(), "false".to_string()].into_iter().cycle().take(n).collect::<Vec<String>>(),
        _ => (0..n).map(|_| "compile_error!(\"unhandled primitive\")".to_string()).collect::<Vec<String>>(),
    };
    let values2_str = &values2.join(", ");

    tests.push(quote! {
        $("// The following code is generated for all primitives")

        $(
            if !is_simd =>

            const _: () = assert!(size_of::<$vec_camelcase<$primitive>>() == size_of::<[$primitive; $n]>());
        )

        #[test]
        fn test_$(vec_snakecase)_constructor() {
            $(match n {
                2 => {
                    assert_eq!($vec_snakecase!($values_str), $vec_camelcase::from_array([$values_str]));
                    assert_eq!($vec_snakecase!($vec_snakecase!($values_str)), $vec_camelcase::from_array([$values_str]));
                }
                3 => {
                    assert_eq!($vec_snakecase!($values_str), $vec_camelcase::from_array([$values_str]));
                    assert_eq!($vec_snakecase!($(&values[0]), vec2$(s_postfix_lowercase)!($(&values[1]), $(&values[2]))), $vec_camelcase::from_array([$values_str]));
                    assert_eq!($vec_snakecase!($vec_snakecase!($values_str)), $vec_camelcase::from_array([$values_str]));
                }
                4 => {
                    assert_eq!($vec_snakecase!($values_str), $vec_camelcase::from_array([$values_str]));
                    assert_eq!($vec_snakecase!($(&values[0]), vec2$(s_postfix_lowercase)!($(&values[1]), $(&values[2])), $(&values[3])), $vec_camelcase::from_array([$values_str]));
                    assert_eq!($vec_snakecase!($vec_snakecase!($values_str)), $vec_camelcase::from_array([$values_str]));
                }
                _ => compile_error!("unhandled vector length"),
            })
        }

        #[test]
        fn test_$(vec_snakecase)_as_simd() {
            assert_eq!($vec_snakecase!($values_str).as_simd(), vec$(n)!($values_str));
        }

        #[test]
        fn test_$(vec_snakecase)_as_non_simd() {
            assert_eq!($vec_snakecase!($values_str).as_non_simd(), vec$(n)s!($values_str));
        }

        #[test]
        fn test_$(vec_snakecase)_from_array_as_array() {
            assert_eq!($vec_camelcase::from_array([$values_str]).as_array(), [$values_str]);
        }

        #[test]
        fn test_$(vec_snakecase)_splat() {
            assert_eq!($vec_camelcase::splat($(&values[0])), $vec_snakecase!($(for _ in 0..n join(, ) => $(&values[0]))));
        }

        #[test]
        fn test_$(vec_snakecase)_index() {
            $(
                for i in 0..n join($['\r']) =>

                assert_eq!($vec_snakecase!($values_str).index($i), $(&values[i]));
            )
        }

        #[test]
        #[should_panic]
        fn test_$(vec_snakecase)_index_panic() {
            $vec_snakecase!($values_str).index($n);
        }

        #[test]
        fn test_$(vec_snakecase)_get() {
            $(
                for i in 0..n join($['\r']) =>

                assert_eq!($vec_snakecase!($values_str).get($i), Some($(&values[i])));
            )

            assert_eq!($vec_snakecase!($values_str).get($n), None);
        }

        #[test]
        fn test_$(vec_snakecase)_get_unchecked() {
            unsafe {
                $(
                    for i in 0..n join($['\r']) =>

                    assert_eq!($vec_snakecase!($values_str).get_unchecked($i), $(&values[i]));
                )
            }
        }

        #[test]
        fn test_$(vec_snakecase)_set() {
            $(
                for i in 0..n join($['\r']) =>

                $(let value = &match primitive {
                    _ if INT_PRIMITIVES.contains(&primitive) => format!("50{primitive}"),
                    _ if FLOAT_PRIMITIVES.contains(&primitive) => format!("50.0{primitive}"),
                    "bool" => (i % 2 == 0).to_string(),
                    _ => unreachable!(),
                })
                {
                    let mut vec = $vec_snakecase!($values_str);
                    vec.set($i, $value);

                    assert_eq!(vec, $vec_snakecase!($(for i2 in 0..n join(, ) => $(if i2 == i { $value } else { $(&values[i2]) }))));
                }
            )
        }

        #[test]
        #[should_panic]
        fn test_$(vec_snakecase)_set_panic() {
            let mut vec = $vec_snakecase!($values_str);
            vec.set($n, $(&values[0]));
        }

        #[test]
        fn test_$(vec_snakecase)_try_set() {
            $(
                for i in 0..n join($['\r']) =>

                $(let value = &match primitive {
                    _ if INT_PRIMITIVES.contains(&primitive) => format!("50{primitive}"),
                    _ if FLOAT_PRIMITIVES.contains(&primitive) => format!("50.0{primitive}"),
                    "bool" => (i % 2 == 0).to_string(),
                    _ => unreachable!(),
                })
                {
                    let mut vec = $vec_snakecase!($values_str);
                    vec.try_set($i, $value).unwrap();

                    assert_eq!(vec, $vec_snakecase!($(for i2 in 0..n join(, ) => $(if i2 == i { $value } else { $(&values[i2]) }))));
                }
            )

            assert_eq!($vec_snakecase!($values_str).try_set($n, $(&values[0])), Err(IndexOutOfBoundsError));
        }

        #[test]
        fn test_$(vec_snakecase)_set_unchecked() {
            unsafe {
                $(
                    for i in 0..n join($['\r']) =>

                    $(let value = &match primitive {
                        _ if INT_PRIMITIVES.contains(&primitive) => format!("50{primitive}"),
                        _ if FLOAT_PRIMITIVES.contains(&primitive) => format!("50.0{primitive}"),
                        "bool" => (i % 2 == 0).to_string(),
                        _ => unreachable!(),
                    })
                    {
                        let mut vec = $vec_snakecase!($values_str);
                        vec.set_unchecked($i, $value);

                        assert_eq!(vec, $vec_snakecase!($(for i2 in 0..n join(, ) => $(if i2 == i { $value } else { $(&values[i2]) }))));
                    }
                )
            }
        }

        #[test]
        fn test_$(vec_snakecase)_swizzle() {
            $(match n {
                2 => {
                    assert_eq!($vec_snakecase!($values_str).y(), $(&values[1]));
                    assert_eq!($vec_snakecase!($values_str).yx(), vec2$(s_postfix_lowercase)!($(&values[1]), $(&values[0])));
                    assert_eq!($vec_snakecase!($values_str).yxy(), vec3$(s_postfix_lowercase)!($(&values[1]), $(&values[0]), $(&values[1])));
                    assert_eq!($vec_snakecase!($values_str).yxyy(), vec4$(s_postfix_lowercase)!($(&values[1]), $(&values[0]), $(&values[1]), $(&values[1])));
                }
                3 => {
                    assert_eq!($vec_snakecase!($values_str).z(), $(&values[2]));
                    assert_eq!($vec_snakecase!($values_str).zx(), vec2$(s_postfix_lowercase)!($(&values[2]), $(&values[0])));
                    assert_eq!($vec_snakecase!($values_str).zxy(), vec3$(s_postfix_lowercase)!($(&values[2]), $(&values[0]), $(&values[1])));
                    assert_eq!($vec_snakecase!($values_str).zxyz(), vec4$(s_postfix_lowercase)!($(&values[2]), $(&values[0]), $(&values[1]), $(&values[2])));
                }
                4 => {
                    assert_eq!($vec_snakecase!($values_str).z(), $(&values[2]));
                    assert_eq!($vec_snakecase!($values_str).zw(), vec2$(s_postfix_lowercase)!($(&values[2]), $(&values[3])));
                    assert_eq!($vec_snakecase!($values_str).zwy(), vec3$(s_postfix_lowercase)!($(&values[2]), $(&values[3]), $(&values[1])));
                    assert_eq!($vec_snakecase!($values_str).zwyz(), vec4$(s_postfix_lowercase)!($(&values[2]), $(&values[3]), $(&values[1]), $(&values[2])));
                }
                _ => compile_error!("unhandled vector length"),
            })
        }

        #[test]
        fn test_$(vec_snakecase)_with_swizzle() {
            $(match n {
                2 => {
                    assert_eq!($vec_snakecase!($values_str).with_y($(&values[0])), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_snakecase!($values_str).with_yx(vec2$(s_postfix_lowercase)!($(&values[0]), $(&values[1]))), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                }
                3 => {
                    assert_eq!($vec_snakecase!($values_str).with_y($(&values[0])), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_snakecase!($values_str).with_yx(vec2$(s_postfix_lowercase)!($(&values[0]), $(&values[1]))), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_snakecase!($values_str).with_yxz(vec3$(s_postfix_lowercase)!($(&values[0]), $(&values[2]), $(&values[1]))), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[2]),
                            2 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                }
                4 => {
                    assert_eq!($vec_snakecase!($values_str).with_y($(&values[0])), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_snakecase!($values_str).with_yx(vec2$(s_postfix_lowercase)!($(&values[0]), $(&values[1]))), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_snakecase!($values_str).with_yxz(vec3$(s_postfix_lowercase)!($(&values[0]), $(&values[2]), $(&values[1]))), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[2]),
                            2 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_snakecase!($values_str).with_yxzw(vec4$(s_postfix_lowercase)!($(&values[0]), $(&values[2]), $(&values[1]), $(&values[0]))), $vec_snakecase!($(
                        for i in 0..n join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[2]),
                            2 => $(&values[1]),
                            3 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                }
                _ => compile_error!("unhandled vector length"),
            })
        }

        $(
            if n == 4 && !is_simd =>

            #[test]
            fn test_$(vec_snakecase)_swizzle_set() {
                let mut vec = $vec_snakecase!($values_str);
                vec.set_yxz(vec3$(s_postfix_lowercase)!($(&values[0]), $(&values[2]), $(&values[1])));
    
                assert_eq!(vec, $vec_snakecase!($(for i in 0..n join(, ) => $(
                    match i {
                        1 => $(&values[0]),
                        0 => $(&values[2]),
                        2 => $(&values[1]),
                        i => $(&values[i]),
                    }
                ))));
            }   
        )

        $(
            if !is_simd =>

            #[test]
            fn test_$(vec_snakecase)_swizzle_ref() {
                $(match n {
                    2 => {
                        assert_eq!($vec_snakecase!($values_str).y_ref(), &$(&values[1]));
                        assert_eq!($vec_snakecase!($values_str).xy_ref(), &vec2s!($values_str));
                    }
                    3 => {
                        assert_eq!($vec_snakecase!($values_str).y_ref(), &$(&values[1]));
                        assert_eq!($vec_snakecase!($values_str).yz_ref(), &vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_snakecase!($values_str).xyz_ref(), &vec3s!($values_str));
                    }
                    4 => {
                        assert_eq!($vec_snakecase!($values_str).y_ref(), &$(&values[1]));
                        assert_eq!($vec_snakecase!($values_str).yz_ref(), &vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_snakecase!($values_str).xyz_ref(), &vec3s!($(for i in 0..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_snakecase!($values_str).xyzw_ref(), &vec4s!($values_str));
                    }
                    _ => compile_error!("unhandled vector length"),
                })
            }

            #[test]
            fn test_$(vec_snakecase)_swizzle_mut() {
                $(match n {
                    2 => {
                        assert_eq!($vec_snakecase!($values_str).y_mut(), &mut $(&values[1]));
                        assert_eq!($vec_snakecase!($values_str).xy_mut(), &mut vec2s!($values_str));

                        assert_eq!($vec_snakecase!($values_str).x_y_mut(), (&mut $(&values[0]), &mut $(&values[1])));
                    }
                    3 => {
                        assert_eq!($vec_snakecase!($values_str).y_mut(), &mut $(&values[1]));
                        assert_eq!($vec_snakecase!($values_str).yz_mut(), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_snakecase!($values_str).xyz_mut(), &mut vec3s!($values_str));

                        assert_eq!($vec_snakecase!($values_str).x_yz_mut(), (&mut $(&values[0]), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i])))));
                    }
                    4 => {
                        assert_eq!($vec_snakecase!($values_str).y_mut(), &mut $(&values[1]));
                        assert_eq!($vec_snakecase!($values_str).yz_mut(), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_snakecase!($values_str).xyz_mut(), &mut vec3s!($(for i in 0..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_snakecase!($values_str).xyzw_mut(), &mut vec4s!($values_str));

                        assert_eq!($vec_snakecase!($values_str).x_yz_mut(), (&mut $(&values[0]), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i])))));
                    }
                    _ => compile_error!("unhandled vector length"),
                })
            }
        )

        #[test]
        fn test_$(vec_snakecase)_fold() {
            $(match primitive {
                _ if INT_PRIMITIVES.contains(&primitive) => {
                    assert_eq!($vec_snakecase!($values_str).fold(13, |acc, x| acc + x), 13 + $(for value in &values join( + ) => $value));
                }
                _ if FLOAT_PRIMITIVES.contains(&primitive) => {
                    assert_eq!($vec_snakecase!($values_str).fold(13.0, |acc, x| acc + x), 13.0 + $(for value in &values join( + ) => $value));
                }
                "bool" => {
                    assert_eq!($vec_snakecase!($values_str).fold(false, |acc, x| acc | x), true);
                }
                _ => unreachable!("unhandled primitive"),
            })
        }

        #[test]
        fn test_$(vec_snakecase)_reduce() {
            $(match primitive {
                _ if INT_PRIMITIVES.contains(&primitive) => {
                    assert_eq!($vec_snakecase!($values_str).reduce(|acc, x| acc + x), $(for value in &values join( + ) => $value));
                }
                _ if FLOAT_PRIMITIVES.contains(&primitive) => {
                    assert_eq!($vec_snakecase!($values_str).reduce(|acc, x| acc + x), $(for value in &values join( + ) => $value));
                }
                "bool" => {
                    assert_eq!($vec_snakecase!($values_str).reduce(|acc, x| acc | x), true);
                }
                _ => unreachable!("unhandled primitive"),
            })
        }

        #[test]
        fn test_$(vec_snakecase)_eq_mask() {
            assert_eq!($vec_snakecase!($values_str).eq_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
            assert_eq!(
                $vec_snakecase!($values_str).eq_mask($vec_snakecase!(
                    $(for i in 0..n join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                )),
                $vec_snakecase!($(
                    for i in 0..n join(, ) => $(if i == 1 { false } else { true })
                )),
            );
            assert_eq!($vec_snakecase!($values_str).eq_mask($vec_snakecase!($values2_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));

            $(
                if FLOAT_PRIMITIVES.contains(&primitive) {
                    $(let nan_values_str = &(0..n).map(|_| format!("{primitive}::NAN")).collect::<Vec<_>>().join(", "));
                    assert_eq!($vec_snakecase!($nan_values_str).eq_mask($vec_snakecase!($nan_values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
                }
            )
        }

        #[test]
        fn test_$(vec_snakecase)_ne_mask() {
            assert_eq!($vec_snakecase!($values_str).ne_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
            assert_eq!(
                $vec_snakecase!($values_str).ne_mask($vec_snakecase!(
                    $(for i in 0..n join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                )),
                $vec_snakecase!($(
                    for i in 0..n join(, ) => $(if i == 1 { true } else { false })
                )),
            );
            assert_eq!($vec_snakecase!($values_str).ne_mask($vec_snakecase!($values2_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));

            $(
                if FLOAT_PRIMITIVES.contains(&primitive) {
                    $(let nan_values_str = &(0..n).map(|_| format!("{primitive}::NAN")).collect::<Vec<_>>().join(", "));
                    assert_eq!($vec_snakecase!($nan_values_str).ne_mask($vec_snakecase!($nan_values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
                }
            )
        }

        #[test]
        fn test_$(vec_snakecase)_lt_mask() {
            assert_eq!($vec_snakecase!($values_str).lt_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
            assert_eq!(
                $vec_snakecase!($values_str).lt_mask($vec_snakecase!(
                    $(for i in 0..n join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_snakecase!($(
                    for i in 0..n join(, ) => $(if i == 0 { true } else { false })
                )),
            );
            $(
                if NUM_PRIMITIVES.contains(&primitive) {
                    assert_eq!($vec_snakecase!($values_str).lt_mask($vec_snakecase!($values2_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
                    assert_eq!($vec_snakecase!($values2_str).lt_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
                }
            )
        }

        #[test]
        fn test_$(vec_snakecase)_gt_mask() {
            assert_eq!($vec_snakecase!($values_str).gt_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
            assert_eq!(
                $vec_snakecase!($values_str).gt_mask($vec_snakecase!(
                    $(for i in 0..n join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_snakecase!($(
                    for i in 0..n join(, ) => $(if i == 1 { true } else { false })
                )),
            );
            $(
                if NUM_PRIMITIVES.contains(&primitive) {
                    assert_eq!($vec_snakecase!($values_str).gt_mask($vec_snakecase!($values2_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
                    assert_eq!($vec_snakecase!($values2_str).gt_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
                }
            )
        }

        #[test]
        fn test_$(vec_snakecase)_le_mask() {
            assert_eq!($vec_snakecase!($values_str).le_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
            assert_eq!(
                $vec_snakecase!($values_str).le_mask($vec_snakecase!(
                    $(for i in 0..n join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_snakecase!($(
                    for i in 0..n join(, ) => $(if i == 1 { false } else { true })
                )),
            );
            $(
                if NUM_PRIMITIVES.contains(&primitive) {
                    assert_eq!($vec_snakecase!($values_str).le_mask($vec_snakecase!($values2_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
                    assert_eq!($vec_snakecase!($values2_str).le_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
                }
            )
        }

        #[test]
        fn test_$(vec_snakecase)_ge_mask() {
            assert_eq!($vec_snakecase!($values_str).ge_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
            assert_eq!(
                $vec_snakecase!($values_str).ge_mask($vec_snakecase!(
                    $(for i in 0..n join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_snakecase!($(
                    for i in 0..n join(, ) => $(if i == 0 { false } else { true })
                )),
            );
            $(
                if NUM_PRIMITIVES.contains(&primitive) {
                    assert_eq!($vec_snakecase!($values_str).ge_mask($vec_snakecase!($values2_str)), $vec_snakecase!($(for _ in 0..n join(, ) => false)));
                    assert_eq!($vec_snakecase!($values2_str).ge_mask($vec_snakecase!($values_str)), $vec_snakecase!($(for _ in 0..n join(, ) => true)));
                }
            )
        }

        #[test]
        fn test_$(vec_snakecase)_eq() {
            assert_eq!($vec_snakecase!($values_str) == $vec_snakecase!($values_str), true);
            assert_eq!(
                $vec_snakecase!($values_str) == $vec_snakecase!(
                    $(for i in 0..n join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                ),
                false
            );
            assert_eq!($vec_snakecase!($values_str) == $vec_snakecase!($values2_str), false);
        }
        
        #[test]
        fn test_$(vec_snakecase)_ne() {
            assert_eq!($vec_snakecase!($values_str) != $vec_snakecase!($values_str), false);
            assert_eq!(
                $vec_snakecase!($values_str) != $vec_snakecase!(
                    $(for i in 0..n join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                ),
                true
            );
            assert_eq!($vec_snakecase!($values_str) != $vec_snakecase!($values2_str), true);
        }

        #[test]
        fn test_$(vec_snakecase)_debug() {
            $(let expected_value_strs = match primitive {
                _ if FLOAT_PRIMITIVES.contains(&primitive) => (0..n).map(|i| format!("{i}.0")).collect::<Vec<_>>(),
                _ if INT_PRIMITIVES.contains(&primitive) => (0..n).map(|i| format!("{i}")).collect::<Vec<_>>(),
                "bool" => (0..n).map(|i| (i % 2 != 0).to_string()).collect::<Vec<_>>(),
                _ => unreachable!("unhandled primitive"),
            });
            $(let expected_str = quoted(format!("({})", expected_value_strs.join(", "))));
            assert_eq!(format!("{:?}", $vec_snakecase!($values_str)), $expected_str);
        }

        #[test]
        fn test_$(vec_snakecase)_display() {
            $(let expected_value_strs = match primitive {
                _ if FLOAT_PRIMITIVES.contains(&primitive) => (0..n).map(|i| format!("{i}")).collect::<Vec<_>>(),
                _ if INT_PRIMITIVES.contains(&primitive) => (0..n).map(|i| format!("{i}")).collect::<Vec<_>>(),
                "bool" => (0..n).map(|i| (i % 2 != 0).to_string()).collect::<Vec<_>>(),
                _ => unreachable!("unhandled primitive"),
            });
            $(let expected_str = quoted(format!("({})", expected_value_strs.join(", "))));
            assert_eq!(format!("{}", $vec_snakecase!($values_str)), $expected_str);
        }

        #[test]
        fn test_$(vec_snakecase)_const_from_array() {
            assert_eq!($vec_camelcase::<$primitive>::const_from_array([$values_str]), $vec_camelcase::from_array([$values_str]));
        }
    });
}
