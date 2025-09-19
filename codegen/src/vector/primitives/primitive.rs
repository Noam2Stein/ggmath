use std::collections::HashMap;

use genco::{
    lang::rust::{Tokens, import},
    quote,
};

use crate::constants::LENGTHS;

pub fn push_fns(
    primitive: &str,
    use_crate_items: &mut Vec<Tokens>,
    functions: &mut Vec<Tokens>,
    _len_functions: &mut HashMap<usize, Vec<Tokens>>,
    _std_functions: &mut Vec<Tokens>,
    _std_len_functions: &mut HashMap<usize, Vec<Tokens>>,
    trait_impls: &mut Vec<Tokens>,
    test_functions: &mut Vec<Tokens>,
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

    test_functions.push(quote! {
        fn assert_typed_eq<T: PartialEq>(a: T, b: T) {
            assert_eq!(a, b);
        }

        $(
            for &n in LENGTHS join($['\r']) =>

            $(let n_values = match primitive {
                "bool" => ["false".to_string(), "true".to_string()].into_iter().cycle().take(n).collect::<Vec<String>>(),

                "f32" | "f64" => (0..n).map(|i| format!("{i}.0")).collect::<Vec<String>>(),

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
                    => (0..n).map(|i| i.to_string()).collect::<Vec<String>>(),

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

                #[test]
                fn test_vec$(n)$(a_postfix_lowercase)_align() {
                    assert_typed_eq(vec$(n)$(a_postfix_lowercase)!($n_values_str).align(), vec$(n)!($n_values_str));
                }

                #[test]
                fn test_vec$(n)$(a_postfix_lowercase)_pack() {
                    assert_typed_eq(vec$(n)$(a_postfix_lowercase)!($n_values_str).pack(), vec$(n)p!($n_values_str));
                }
            )
        )
    });
}
