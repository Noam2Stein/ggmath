use genco::{
    lang::rust::{Tokens, import},
    quote,
};

use crate::constants::LENGTHS;

pub fn push_fns(
    primitive: &str,
    functions: &mut Vec<Tokens>,
    _std_functions: &mut Vec<Tokens>,
    trait_impls: &mut Vec<Tokens>,
    use_crate_items: &mut Vec<Tokens>,
) {
    use_crate_items
        .push(quote! { Vector, VecAlignment, VecAligned, VecPacked, Usize, VecLen, Scalar });
    use_crate_items.push(quote! { $(for &n in LENGTHS join(, ) => Vec$(n)) });

    functions.push(quote! {
        $("// The following code is for all primitives")

        $(let transmute_copy = &import("core::mem", "transmute_copy"))

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
