use genco::{lang::rust::Tokens, quote, tokens::quoted};

use crate::iter::{Length, Primitive, Simdness, Vector, VectorInfo};

pub fn push(t: Primitive, output: &mut Tokens) {
    output.extend(quote! {$(
        for vec in Vector::test_iter(t) join($['\n']) =>

        $(let VectorInfo { n, s, alias: ref vec_alias, r#macro: ref vec_macro, name_snakecase: ref vec_name, .. } = vec.info())

        $(let values = match t {
            Primitive::Float(primitive) => (0..n.as_usize()).map(|i| format!("{i}.0{primitive}")).collect::<Vec<String>>(),
            Primitive::Int(primitive) => (0..n.as_usize()).map(|i| format!("{i}{primitive}")).collect::<Vec<String>>(),
            Primitive::Bool => ["false".to_string(), "true".to_string()].into_iter().cycle().take(n.as_usize()).collect::<Vec<String>>(),
        })
        $(let values_str = &values.join(", "))
        
        $(let values2 = match t {
            Primitive::Float(primitive) => (n.as_usize()..n.as_usize() * 2).map(|i| format!("{i}.0{primitive}")).collect::<Vec<String>>(),
            Primitive::Int(primitive) => (n.as_usize()..n.as_usize() * 2).map(|i| format!("{i}{primitive}")).collect::<Vec<String>>(),
            Primitive::Bool => ["true".to_string(), "false".to_string()].into_iter().cycle().take(n.as_usize()).collect::<Vec<String>>(),
        })
        $(let values2_str = &values2.join(", "))

        $(
            if vec.s == Simdness::NonSimd =>

            const _: () = assert!(size_of::<$(vec.alias())<$t>>() == size_of::<[$t; $(n)]>());
        )

        #[test]
        fn test_$(vec_name)_constructor() {
            $(match n {
                Length::Two => {
                    assert_eq!($vec_macro!($values_str), $vec_alias::from_array([$values_str]));
                    assert_eq!($vec_macro!($vec_macro!($values_str)), $vec_alias::from_array([$values_str]));
                }
                Length::Three => {
                    assert_eq!($vec_macro!($values_str), $vec_alias::from_array([$values_str]));
                    assert_eq!($vec_macro!($(&values[0]), vec2$(s.postfix_lowercase())!($(&values[1]), $(&values[2]))), $vec_alias::from_array([$values_str]));
                    assert_eq!($vec_macro!($vec_macro!($values_str)), $vec_alias::from_array([$values_str]));
                }
                Length::Four => {
                    assert_eq!($vec_macro!($values_str), $vec_alias::from_array([$values_str]));
                    assert_eq!($vec_macro!($(&values[0]), vec2$(s.postfix_lowercase())!($(&values[1]), $(&values[2])), $(&values[3])), $vec_alias::from_array([$values_str]));
                    assert_eq!($vec_macro!($vec_macro!($values_str)), $vec_alias::from_array([$values_str]));
                }
            })
        }

        #[test]
        fn test_$(vec_name)_as_simd() {
            assert_eq!($vec_macro!($values_str).as_simd(), vec$(n)!($values_str));
        }

        #[test]
        fn test_$(vec_name)_as_non_simd() {
            assert_eq!($vec_macro!($values_str).as_non_simd(), vec$(n)s!($values_str));
        }

        #[test]
        fn test_$(vec_name)_from_array_as_array() {
            assert_eq!($vec_alias::from_array([$values_str]).as_array(), [$values_str]);
        }

        #[test]
        fn test_$(vec_name)_splat() {
            assert_eq!($vec_alias::splat($(&values[0])), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => $(&values[0]))));
        }

        #[test]
        fn test_$(vec_name)_index() {
            $(
                for i in 0..n.as_usize() join($['\r']) =>

                assert_eq!($vec_macro!($values_str).index($i), $(&values[i]));
            )
        }

        #[test]
        #[should_panic]
        fn test_$(vec_name)_index_panic() {
            $vec_macro!($values_str).index($n);
        }

        #[test]
        fn test_$(vec_name)_get() {
            $(
                for i in 0..n.as_usize() join($['\r']) =>

                assert_eq!($vec_macro!($values_str).get($i), Some($(&values[i])));
            )

            assert_eq!($vec_macro!($values_str).get($n), None);
        }

        #[test]
        fn test_$(vec_name)_get_unchecked() {
            unsafe {
                $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    assert_eq!($vec_macro!($values_str).get_unchecked($i), $(&values[i]));
                )
            }
        }

        #[test]
        fn test_$(vec_name)_set() {
            $(
                for i in 0..n.as_usize() join($['\r']) =>

                $(let value = &match t {
                    Primitive::Int(primitive) => format!("50{primitive}"),
                    Primitive::Float(primitive) => format!("50.0{primitive}"),
                    Primitive::Bool => (i % 2 == 0).to_string(),
                })
                {
                    let mut vec = $vec_macro!($values_str);
                    vec.set($(i), $value);

                    assert_eq!(vec, $vec_macro!($(for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { $value } else { $(&values[i2]) }))));
                }
            )
        }

        #[test]
        #[should_panic]
        fn test_$(vec_name)_set_panic() {
            let mut vec = $vec_macro!($values_str);
            vec.set($n, $(&values[0]));
        }

        #[test]
        fn test_$(vec_name)_try_set() {
            $(
                for i in 0..n.as_usize() join($['\r']) =>

                $(let value = &match t {
                    Primitive::Int(primitive) => format!("50{primitive}"),
                    Primitive::Float(primitive) => format!("50.0{primitive}"),
                    Primitive::Bool => (i % 2 == 0).to_string(),
                })
                {
                    let mut vec = $vec_macro!($values_str);
                    vec.try_set($i, $value).unwrap();

                    assert_eq!(vec, $vec_macro!($(for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { $value } else { $(&values[i2]) }))));
                }
            )

            assert_eq!($vec_macro!($values_str).try_set($n, $(&values[0])), Err(IndexOutOfBoundsError));
        }

        #[test]
        fn test_$(vec_name)_set_unchecked() {
            unsafe {
                $(
                    for i in 0..n.as_usize() join($['\r']) =>

                    $(let value = &match t {
                        Primitive::Int(primitive) => format!("50{primitive}"),
                        Primitive::Float(primitive) => format!("50.0{primitive}"),
                        Primitive::Bool => (i % 2 == 0).to_string(),
                    })
                    {
                        let mut vec = $vec_macro!($values_str);
                        vec.set_unchecked($i, $value);

                        assert_eq!(vec, $vec_macro!($(for i2 in 0..n.as_usize() join(, ) => $(if i2 == i { $value } else { $(&values[i2]) }))));
                    }
                )
            }
        }

        #[test]
        fn test_$(vec_name)_swizzle() {
            $(match n {
                Length::Two => {
                    assert_eq!($vec_macro!($values_str).y(), $(&values[1]));
                    assert_eq!($vec_macro!($values_str).yx(), vec2$(s.postfix_lowercase())!($(&values[1]), $(&values[0])));
                    assert_eq!($vec_macro!($values_str).yxy(), vec3$(s.postfix_lowercase())!($(&values[1]), $(&values[0]), $(&values[1])));
                    assert_eq!($vec_macro!($values_str).yxyy(), vec4$(s.postfix_lowercase())!($(&values[1]), $(&values[0]), $(&values[1]), $(&values[1])));
                }
                Length::Three => {
                    assert_eq!($vec_macro!($values_str).z(), $(&values[2]));
                    assert_eq!($vec_macro!($values_str).zx(), vec2$(s.postfix_lowercase())!($(&values[2]), $(&values[0])));
                    assert_eq!($vec_macro!($values_str).zxy(), vec3$(s.postfix_lowercase())!($(&values[2]), $(&values[0]), $(&values[1])));
                    assert_eq!($vec_macro!($values_str).zxyz(), vec4$(s.postfix_lowercase())!($(&values[2]), $(&values[0]), $(&values[1]), $(&values[2])));
                }
                Length::Four => {
                    assert_eq!($vec_macro!($values_str).z(), $(&values[2]));
                    assert_eq!($vec_macro!($values_str).zw(), vec2$(s.postfix_lowercase())!($(&values[2]), $(&values[3])));
                    assert_eq!($vec_macro!($values_str).zwy(), vec3$(s.postfix_lowercase())!($(&values[2]), $(&values[3]), $(&values[1])));
                    assert_eq!($vec_macro!($values_str).zwyz(), vec4$(s.postfix_lowercase())!($(&values[2]), $(&values[3]), $(&values[1]), $(&values[2])));
                }
            })
        }

        #[test]
        fn test_$(vec_name)_with_swizzle() {
            $(match n {
                Length::Two => {
                    assert_eq!($vec_macro!($values_str).with_y($(&values[0])), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_macro!($values_str).with_yx(vec2$(s.postfix_lowercase())!($(&values[0]), $(&values[1]))), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                }
                Length::Three => {
                    assert_eq!($vec_macro!($values_str).with_y($(&values[0])), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_macro!($values_str).with_yx(vec2$(s.postfix_lowercase())!($(&values[0]), $(&values[1]))), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_macro!($values_str).with_yxz(vec3$(s.postfix_lowercase())!($(&values[0]), $(&values[2]), $(&values[1]))), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[2]),
                            2 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                }
                Length::Four => {
                    assert_eq!($vec_macro!($values_str).with_y($(&values[0])), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_macro!($values_str).with_yx(vec2$(s.postfix_lowercase())!($(&values[0]), $(&values[1]))), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_macro!($values_str).with_yxz(vec3$(s.postfix_lowercase())!($(&values[0]), $(&values[2]), $(&values[1]))), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[2]),
                            2 => $(&values[1]),
                            i => $(&values[i]),
                        })
                    )));
                    assert_eq!($vec_macro!($values_str).with_yxzw(vec4$(s.postfix_lowercase())!($(&values[0]), $(&values[2]), $(&values[1]), $(&values[0]))), $vec_macro!($(
                        for i in 0..n.as_usize() join(, ) => $(match i {
                            1 => $(&values[0]),
                            0 => $(&values[2]),
                            2 => $(&values[1]),
                            3 => $(&values[0]),
                            i => $(&values[i]),
                        })
                    )));
                }
            })
        }

        $(
            if n == Length::Four && s != Simdness::Simd =>

            #[test]
            fn test_$(vec_name)_swizzle_set() {
                let mut vec = $vec_macro!($values_str);
                vec.set_yxz(vec3$(s.postfix_lowercase())!($(&values[0]), $(&values[2]), $(&values[1])));
    
                assert_eq!(vec, $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(
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
            if s == Simdness::NonSimd =>

            #[test]
            fn test_$(vec_name)_swizzle_ref() {
                $(match n {
                    Length::Two => {
                        assert_eq!($vec_macro!($values_str).y_ref(), &$(&values[1]));
                        assert_eq!($vec_macro!($values_str).xy_ref(), &vec2s!($values_str));
                    }
                    Length::Three => {
                        assert_eq!($vec_macro!($values_str).y_ref(), &$(&values[1]));
                        assert_eq!($vec_macro!($values_str).yz_ref(), &vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_macro!($values_str).xyz_ref(), &vec3s!($values_str));
                    }
                    Length::Four => {
                        assert_eq!($vec_macro!($values_str).y_ref(), &$(&values[1]));
                        assert_eq!($vec_macro!($values_str).yz_ref(), &vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_macro!($values_str).xyz_ref(), &vec3s!($(for i in 0..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_macro!($values_str).xyzw_ref(), &vec4s!($values_str));
                    }
                })
            }

            #[test]
            fn test_$(vec_name)_swizzle_mut() {
                $(match n {
                    Length::Two => {
                        assert_eq!($vec_macro!($values_str).y_mut(), &mut $(&values[1]));
                        assert_eq!($vec_macro!($values_str).xy_mut(), &mut vec2s!($values_str));

                        assert_eq!($vec_macro!($values_str).x_y_mut(), (&mut $(&values[0]), &mut $(&values[1])));
                    }
                    Length::Three => {
                        assert_eq!($vec_macro!($values_str).y_mut(), &mut $(&values[1]));
                        assert_eq!($vec_macro!($values_str).yz_mut(), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_macro!($values_str).xyz_mut(), &mut vec3s!($values_str));

                        assert_eq!($vec_macro!($values_str).x_yz_mut(), (&mut $(&values[0]), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i])))));
                    }
                    Length::Four => {
                        assert_eq!($vec_macro!($values_str).y_mut(), &mut $(&values[1]));
                        assert_eq!($vec_macro!($values_str).yz_mut(), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_macro!($values_str).xyz_mut(), &mut vec3s!($(for i in 0..3 join(, ) => $(&values[i]))));
                        assert_eq!($vec_macro!($values_str).xyzw_mut(), &mut vec4s!($values_str));

                        assert_eq!($vec_macro!($values_str).x_yz_mut(), (&mut $(&values[0]), &mut vec2s!($(for i in 1..3 join(, ) => $(&values[i])))));
                    }
                })
            }
        )

        #[test]
        fn test_$(vec_name)_fold() {
            $(match t {
                Primitive::Int(_) => {
                    assert_eq!($vec_macro!($values_str).fold(13, |acc, x| acc + x), 13 + $(for value in &values join( + ) => $value));
                }
                Primitive::Float(_) => {
                    assert_eq!($vec_macro!($values_str).fold(13.0, |acc, x| acc + x), 13.0 + $(for value in &values join( + ) => $value));
                }
                Primitive::Bool => {
                    assert_eq!($vec_macro!($values_str).fold(false, |acc, x| acc | x), true);
                }
            })
        }

        #[test]
        fn test_$(vec_name)_reduce() {
            $(match t {
                Primitive::Int(_) => {
                    assert_eq!($vec_macro!($values_str).reduce(|acc, x| acc + x), $(for value in &values join( + ) => $value));
                }
                Primitive::Float(_) => {
                    assert_eq!($vec_macro!($values_str).reduce(|acc, x| acc + x), $(for value in &values join( + ) => $value));
                }
                Primitive::Bool => {
                    assert_eq!($vec_macro!($values_str).reduce(|acc, x| acc | x), true);
                }
            })
        }

        #[test]
        fn test_$(vec_name)_eq_mask() {
            assert_eq!($vec_macro!($values_str).eq_mask($vec_macro!($values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => true)));
            assert_eq!(
                $vec_macro!($values_str).eq_mask($vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                )),
                $vec_macro!($(
                    for i in 0..n.as_usize() join(, ) => $(if i == 1 { false } else { true })
                )),
            );
            assert_eq!($vec_macro!($values_str).eq_mask($vec_macro!($values2_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => false)));

            $(
                if let Primitive::Float(primitive) = t {
                    $(let nan_values_str = &(0..n.as_usize()).map(|_| format!("{primitive}::NAN")).collect::<Vec<_>>().join(", "));
                    assert_eq!($vec_macro!($nan_values_str).eq_mask($vec_macro!($nan_values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => false)));
                }
            )
        }

        #[test]
        fn test_$(vec_name)_ne_mask() {
            assert_eq!($vec_macro!($values_str).ne_mask($vec_macro!($values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => false)));
            assert_eq!(
                $vec_macro!($values_str).ne_mask($vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                )),
                $vec_macro!($(
                    for i in 0..n.as_usize() join(, ) => $(if i == 1 { true } else { false })
                )),
            );
            assert_eq!($vec_macro!($values_str).ne_mask($vec_macro!($values2_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => true)));

            $(
                if let Primitive::Float(primitive) = t {
                    $(let nan_values_str = &(0..n.as_usize()).map(|_| format!("{primitive}::NAN")).collect::<Vec<_>>().join(", "));
                    assert_eq!($vec_macro!($nan_values_str).ne_mask($vec_macro!($nan_values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => true)));
                }
            )
        }

        #[test]
        fn test_$(vec_name)_lt_mask() {
            assert_eq!($vec_macro!($values_str).lt_mask($vec_macro!($values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => false)));
            assert_eq!(
                $vec_macro!($values_str).lt_mask($vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_macro!($(
                    for i in 0..n.as_usize() join(, ) => $(if i == 0 { true } else { false })
                )),
            );

            assert_eq!($vec_macro!($values_str).lt_mask($vec_macro!($values2_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => true,
                Primitive::Bool => $((i.is_multiple_of(2)).to_string()),
            }))));
            assert_eq!($vec_macro!($values2_str).lt_mask($vec_macro!($values_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => false,
                Primitive::Bool => $((!i.is_multiple_of(2)).to_string()),
            }))));
        }

        #[test]
        fn test_$(vec_name)_gt_mask() {
            assert_eq!($vec_macro!($values_str).gt_mask($vec_macro!($values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => false)));
            assert_eq!(
                $vec_macro!($values_str).gt_mask($vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_macro!($(
                    for i in 0..n.as_usize() join(, ) => $(if i == 1 { true } else { false })
                )),
            );
            
            assert_eq!($vec_macro!($values_str).gt_mask($vec_macro!($values2_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => false,
                Primitive::Bool => $((!i.is_multiple_of(2)).to_string()),
            }))));
            assert_eq!($vec_macro!($values2_str).gt_mask($vec_macro!($values_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => true,
                Primitive::Bool => $((i.is_multiple_of(2)).to_string()),
            }))));
        }

        #[test]
        fn test_$(vec_name)_le_mask() {
            assert_eq!($vec_macro!($values_str).le_mask($vec_macro!($values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => true)));
            assert_eq!(
                $vec_macro!($values_str).le_mask($vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_macro!($(
                    for i in 0..n.as_usize() join(, ) => $(if i == 1 { false } else { true })
                )),
            );

            assert_eq!($vec_macro!($values_str).le_mask($vec_macro!($values2_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => true,
                Primitive::Bool => $((i.is_multiple_of(2)).to_string()),
            }))));
            assert_eq!($vec_macro!($values2_str).le_mask($vec_macro!($values_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => false,
                Primitive::Bool => $((!i.is_multiple_of(2)).to_string()),
            }))));
        }

        #[test]
        fn test_$(vec_name)_ge_mask() {
            assert_eq!($vec_macro!($values_str).ge_mask($vec_macro!($values_str)), $vec_macro!($(for _ in 0..n.as_usize() join(, ) => true)));
            assert_eq!(
                $vec_macro!($values_str).ge_mask($vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(match i {
                        0 => $(&values[1]),
                        1 => $(&values[0]),
                        i => $(&values[i]),
                    }))
                )),
                $vec_macro!($(
                    for i in 0..n.as_usize() join(, ) => $(if i == 0 { false } else { true })
                )),
            );

            assert_eq!($vec_macro!($values_str).ge_mask($vec_macro!($values2_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => false,
                Primitive::Bool => $((!i.is_multiple_of(2)).to_string()),
            }))));
            assert_eq!($vec_macro!($values2_str).ge_mask($vec_macro!($values_str)), $vec_macro!($(for i in 0..n.as_usize() join(, ) => $(match t {
                Primitive::Float(_) | Primitive::Int(_) => true,
                Primitive::Bool => $((i.is_multiple_of(2)).to_string()),
            }))));
        }

        #[test]
        fn test_$(vec_name)_eq() {
            assert_eq!($vec_macro!($values_str) == $vec_macro!($values_str), true);
            assert_eq!(
                $vec_macro!($values_str) == $vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                ),
                false
            );
            assert_eq!($vec_macro!($values_str) == $vec_macro!($values2_str), false);
        }
        
        #[test]
        fn test_$(vec_name)_ne() {
            assert_eq!($vec_macro!($values_str) != $vec_macro!($values_str), false);
            assert_eq!(
                $vec_macro!($values_str) != $vec_macro!(
                    $(for i in 0..n.as_usize() join(, ) => $(if i == 1 { $(&values[0]) } else { $(&values[i]) }))
                ),
                true
            );
            assert_eq!($vec_macro!($values_str) != $vec_macro!($values2_str), true);
        }

        #[test]
        fn test_$(vec_name)_debug() {
            $(let expected_value_strs = match t {
                Primitive::Float(_) => (0..n.as_usize()).map(|i| format!("{i}.0")).collect::<Vec<_>>(),
                Primitive::Int(_) => (0..n.as_usize()).map(|i| format!("{i}")).collect::<Vec<_>>(),
                Primitive::Bool => (0..n.as_usize()).map(|i| (i % 2 != 0).to_string()).collect::<Vec<_>>(),
            });
            $(let expected_str = quoted(format!("({})", expected_value_strs.join(", "))));
            assert_eq!(format!("{:?}", $vec_macro!($values_str)), $expected_str);
        }

        #[test]
        fn test_$(vec_name)_display() {
            $(let expected_value_strs = match t {
                Primitive::Float(_) => (0..n.as_usize()).map(|i| format!("{i}")).collect::<Vec<_>>(),
                Primitive::Int(_) => (0..n.as_usize()).map(|i| format!("{i}")).collect::<Vec<_>>(),
                Primitive::Bool => (0..n.as_usize()).map(|i| (i % 2 != 0).to_string()).collect::<Vec<_>>(),
            });
            $(let expected_str = quoted(format!("({})", expected_value_strs.join(", "))));
            assert_eq!(format!("{}", $vec_macro!($values_str)), $expected_str);
        }

        #[test]
        fn test_$(vec_name)_const_from_array() {
            assert_eq!($vec_alias::<$t>::const_from_array([$values_str]), $vec_alias::from_array([$values_str]));
        }
    )});
}
