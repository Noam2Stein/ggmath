use genco::quote;
use strum::IntoEnumIterator;

use crate::{
    iter::{Length, Primitive},
    srcdir::vector::primitives::PrimitiveSrcMod,
};

pub fn push_src(primitive: Primitive, output: &mut PrimitiveSrcMod) {
    output.impl_items.push(quote! {
        $("// The following code is generated for all primitives")

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
                            for n in Length::iter() join($['\r']) =>

                            $n => {
                                let array = transmute_copy::<[$primitive; N], [$primitive; $n]>(&array);
                                let vec = Vector::<$n, _, _>(array);

                                transmute_copy::<
                                    Vector<$n, $primitive, Simd>,
                                    Vector<N, $primitive, S>,
                                >(&vec)
                            },
                        )
                        _ => panic!("unusual vector type")
                    }
                } else {
                    transmute_copy::<
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
            type SimdVectorStorage<const N: usize> = [$primitive; N]
            where
                Usize<N>: VecLen;

            #[inline(always)]
            fn vec_from_array<const N: usize>(array: [$primitive; N]) -> Vector<N, $primitive, Simd>
            where
                Usize<N>: VecLen,
            {
                Vector(array)
            }

            #[inline(always)]
            fn vec_as_array<const N: usize>(vec: Vector<N, $primitive, Simd>) -> [$primitive; N]
            where
                Usize<N>: VecLen,
            {
                vec.0
            }
        }
    });
}
