use std::collections::HashMap;

use genco::{
    lang::rust::{Tokens, import},
    quote,
};

use crate::constants::{FLOAT_PRIMITIVES, INT_PRIMITIVES, LENGTHS};

pub fn push_src(
    primitive: &str,
    use_crate_items: &mut Vec<Tokens>,
    functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    trait_impls: &mut Vec<Tokens>,
) {
    use_crate_items
        .push(quote! { Vector, VecAlignment, VecAligned, VecPacked, Usize, VecLen, Scalar });
    use_crate_items.push(quote! { $(for &n in LENGTHS join(, ) => Vec$(n)) });

    functions.push(quote! {
        $("// The following code is for all primitives")

        $(let transmute_copy = &import("core::mem", "transmute_copy"))

        $("/// Variation of `Vector::from_array` that is `const`.")
        $("/// This may be slower than `Vector::from_array`.")
        $("///")
        $("/// When rust stabilizes const traits, this will be deleted.")
        #[inline(always)]
        pub const fn const_from_array(array: [$primitive; N]) -> Self {
            unsafe {
                if A::IS_ALIGNED {
                    match N {
                        $(
                            for &n in LENGTHS join($['\r']) =>

                            $n => {
                                let array = $transmute_copy::<[$primitive; N], [$primitive; $n]>(&array);
                                let vec = Vector::<$n, _, _>(array);

                                $transmute_copy::<
                                    Vector<$n, $primitive, VecAligned>,
                                    Vector<N, $primitive, A>,
                                >(&vec)
                            },
                        )
                        _ => panic!("unusual vector type")
                    }
                } else {
                    $transmute_copy::<
                        Vector<N, $primitive, VecPacked>,
                        Vector<N, $primitive, A>,
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

    trait_impls.push(quote! {
        impl Scalar for $primitive {
            $(for &n in LENGTHS join($['\r']) => type InnerAlignedVec$(n) = [$primitive; $n];)

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

pub fn push_test(primitive: &str, use_stmts: &mut Vec<Tokens>, functions: &mut Vec<Tokens>) {
    use_stmts.push(quote! {
        use core::mem::size_of;

        use ggmath::*;
    });

    functions.push(quote! {
        $(
            for &n in LENGTHS join($['\r']) =>

            $(let n_values = match primitive {
                "bool" => ["false".to_string(), "true".to_string()].into_iter().cycle().take(n).collect::<Vec<String>>(),

                "f32" | "f64" => (0..n).map(|i| format!("{i}.0{primitive}")).collect::<Vec<String>>(),

                "u8"
                    | "u16"
                    | "u32"
                    | "u64"
                    | "u128"
                    | "usize"
                    | "i8"
                    | "i16"
                    | "i32"
                    | "i64"
                    | "i128"
                    | "isize"
                    => (0..n).map(|i| format!("{i}{primitive}")).collect::<Vec<String>>(),

                _ => unreachable!(),
            })

            $(let n_values_str = &n_values.join(", "))

            const _: () = assert!(size_of::<Vec$(n)P<$primitive>>() == size_of::<[$primitive; $n]>());

            $(
                for a in ["VecAligned", "VecPacked"] join($['\r']) =>

                $(let a_postfix_lowercase = match a {
                    "VecAligned" => "",
                    "VecPacked" => "p",
                    _ => unreachable!(),
                })
                $(let a_postfix_camelcase = match a {
                    "VecAligned" => "",
                    "VecPacked" => "P",
                    _ => unreachable!(),
                })

                $(let vec_lowercase = &format!("vec{n}{a_postfix_lowercase}"))
                $(let vec_camelcase = &format!("Vec{n}{a_postfix_camelcase}"))

                #[test]
                fn test_$(vec_lowercase)_align() {
                    assert_eq!($vec_lowercase!($n_values_str).align(), vec$(n)!($n_values_str));
                }

                #[test]
                fn test_$(vec_lowercase)_pack() {
                    assert_eq!($vec_lowercase!($n_values_str).pack(), vec$(n)p!($n_values_str));
                }

                #[test]
                fn test_$(vec_lowercase)_from_array_as_array() {
                    assert_eq!($vec_camelcase::from_array([$n_values_str]).as_array(), [$n_values_str]);
                }

                #[test]
                fn test_$(vec_lowercase)_splat() {
                    assert_eq!($vec_camelcase::splat($(&n_values[0])), $vec_lowercase!($(for _ in 0..n join(, ) => $(&n_values[0]))));
                }

                #[test]
                fn test_$(vec_lowercase)_index() {
                    $(
                        for i in 0..n join($['\r']) =>

                        assert_eq!($vec_lowercase!($n_values_str).index($i), $(&n_values[i]));
                    )
                }

                #[test]
                #[should_panic]
                fn test_$(vec_lowercase)_index_panic() {
                    $vec_lowercase!($n_values_str).index($n);
                }

                #[test]
                fn test_$(vec_lowercase)_get() {
                    $(
                        for i in 0..n join($['\r']) =>

                        assert_eq!($vec_lowercase!($n_values_str).get($i), Some($(&n_values[i])));
                    )

                    assert_eq!($vec_lowercase!($n_values_str).get($n), None);
                }

                #[test]
                fn test_$(vec_lowercase)_get_unchecked() {
                    unsafe {
                        $(
                            for i in 0..n join($['\r']) =>
    
                            assert_eq!($vec_lowercase!($n_values_str).get_unchecked($i), $(&n_values[i]));
                        )
                    }
                }

                #[test]
                fn test_$(vec_lowercase)_set() {
                    $(
                        for i in 0..n join($['\r']) =>

                        $(let value = &match primitive {
                            _ if INT_PRIMITIVES.contains(&primitive) => format!("50{primitive}"),
                            _ if FLOAT_PRIMITIVES.contains(&primitive) => format!("50.0{primitive}"),
                            "bool" => (i % 2 == 0).to_string(),
                            _ => unreachable!(),
                        })
                        {
                            let mut vec = $vec_lowercase!($n_values_str);
                            vec.set($i, $value);

                            assert_eq!(vec, $vec_lowercase!($(for i2 in 0..n join(, ) => $(if i2 == i { $value } else { $(&n_values[i2]) }))));
                        }
                    )
                }

                #[test]
                #[should_panic]
                fn test_$(vec_lowercase)_set_panic() {
                    let mut vec = $vec_lowercase!($n_values_str);
                    vec.set($n, $(&n_values[0]));
                }

                #[test]
                fn test_$(vec_lowercase)_try_set() {
                    $(
                        for i in 0..n join($['\r']) =>

                        $(let value = &match primitive {
                            _ if INT_PRIMITIVES.contains(&primitive) => format!("50{primitive}"),
                            _ if FLOAT_PRIMITIVES.contains(&primitive) => format!("50.0{primitive}"),
                            "bool" => (i % 2 == 0).to_string(),
                            _ => unreachable!(),
                        })
                        {
                            let mut vec = $vec_lowercase!($n_values_str);
                            vec.try_set($i, $value).unwrap();

                            assert_eq!(vec, $vec_lowercase!($(for i2 in 0..n join(, ) => $(if i2 == i { $value } else { $(&n_values[i2]) }))));
                        }
                    )

                    assert_eq!($vec_lowercase!($n_values_str).try_set($n, $(&n_values[0])), Err(IndexOutOfBoundsError));
                }

                #[test]
                fn test_$(vec_lowercase)_set_unchecked() {
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
                                let mut vec = $vec_lowercase!($n_values_str);
                                vec.set_unchecked($i, $value);

                                assert_eq!(vec, $vec_lowercase!($(for i2 in 0..n join(, ) => $(if i2 == i { $value } else { $(&n_values[i2]) }))));
                            }
                        )
                    }
                }

                #[test]
                fn test_$(vec_lowercase)_swizzle() {
                    $(match n {
                        2 => {
                            assert_eq!($vec_lowercase!($n_values_str).yx(), vec2$(a_postfix_lowercase)!($(&n_values[1]), $(&n_values[0])));
                            assert_eq!($vec_lowercase!($n_values_str).yxy(), vec3$(a_postfix_lowercase)!($(&n_values[1]), $(&n_values[0]), $(&n_values[1])));
                            assert_eq!($vec_lowercase!($n_values_str).yxyy(), vec4$(a_postfix_lowercase)!($(&n_values[1]), $(&n_values[0]), $(&n_values[1]), $(&n_values[1])));
                        }
                        3 => {
                            assert_eq!($vec_lowercase!($n_values_str).zx(), vec2$(a_postfix_lowercase)!($(&n_values[2]), $(&n_values[0])));
                            assert_eq!($vec_lowercase!($n_values_str).zxy(), vec3$(a_postfix_lowercase)!($(&n_values[2]), $(&n_values[0]), $(&n_values[1])));
                            assert_eq!($vec_lowercase!($n_values_str).zxyz(), vec4$(a_postfix_lowercase)!($(&n_values[2]), $(&n_values[0]), $(&n_values[1]), $(&n_values[2])));
                        }
                        4 => {
                            assert_eq!($vec_lowercase!($n_values_str).zw(), vec2$(a_postfix_lowercase)!($(&n_values[2]), $(&n_values[3])));
                            assert_eq!($vec_lowercase!($n_values_str).zwy(), vec3$(a_postfix_lowercase)!($(&n_values[2]), $(&n_values[3]), $(&n_values[1])));
                            assert_eq!($vec_lowercase!($n_values_str).zwyz(), vec4$(a_postfix_lowercase)!($(&n_values[2]), $(&n_values[3]), $(&n_values[1]), $(&n_values[2])));
                        }
                        _ => unreachable!("unhandled vector length"),
                    })
                }

                #[test]
                fn test_$(vec_lowercase)_fold() {
                    $(match primitive {
                        _ if INT_PRIMITIVES.contains(&primitive) => {
                            assert_eq!($vec_lowercase!($n_values_str).fold(13, |acc, x| acc + x), 13 + $(for value in &n_values join( + ) => $value));
                        }
                        _ if FLOAT_PRIMITIVES.contains(&primitive) => {
                            assert_eq!($vec_lowercase!($n_values_str).fold(13.0, |acc, x| acc + x), 13.0 + $(for value in &n_values join( + ) => $value));
                        }
                        "bool" => {
                            assert_eq!($vec_lowercase!($n_values_str).fold(false, |acc, x| acc | x), true);
                        }
                        _ => unreachable!("unhandled primitive"),
                    })
                }

                #[test]
                fn test_$(vec_lowercase)_reduce() {
                    $(match primitive {
                        _ if INT_PRIMITIVES.contains(&primitive) => {
                            assert_eq!($vec_lowercase!($n_values_str).reduce(|acc, x| acc + x), $(for value in &n_values join( + ) => $value));
                        }
                        _ if FLOAT_PRIMITIVES.contains(&primitive) => {
                            assert_eq!($vec_lowercase!($n_values_str).reduce(|acc, x| acc + x), $(for value in &n_values join( + ) => $value));
                        }
                        "bool" => {
                            assert_eq!($vec_lowercase!($n_values_str).reduce(|acc, x| acc | x), true);
                        }
                        _ => unreachable!("unhandled primitive"),
                    })
                }
            )
        )
    });
}
