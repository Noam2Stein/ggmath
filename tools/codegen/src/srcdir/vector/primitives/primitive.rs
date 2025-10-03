use genco::quote;

use crate::{iter::Primitive, srcdir::vector::primitives::PrimitiveSrcMod};

pub fn push_src(primitive: Primitive, output: &mut PrimitiveSrcMod) {
    output.impl_items.push(quote! {
        $("// The following code is generated for all primitives")

        $("/// Variation of `Vector::from_array` that is `const`.")
        $("/// This may be slower than `Vector::from_array`.")
        $("///")
        $("/// When rust stabilizes const traits, this will be deleted.")
        #[inline(always)]
        pub const fn const_from_array(array: [$primitive; N]) -> Self {
            Self::pick_by_simdness(Vector(array), Vector(array))
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
