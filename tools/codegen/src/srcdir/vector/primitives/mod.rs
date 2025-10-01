use std::collections::HashMap;

use genco::{lang::rust::Tokens, quote};
use strum::IntoEnumIterator;

use crate::{
    iter::{Length, Primitive, PrimitiveInt},
    module::{SrcDir, SrcFile, TokensExt},
};

mod bool_;
mod float;
mod int;
mod option;
mod primitive;
mod sint;
mod uint;

pub fn srcmod() -> SrcDir {
    quote! {
        $(for primitive in Primitive::iter() join($['\r']) => mod $primitive;)
        mod option;
    }
    .to_srcdir("primitives")
    .with_submod_files(Primitive::iter().map(primitive_srcmod))
    .with_submod_file(option::srcmod())
}

#[derive(Default)]
struct PrimitiveSrcMod {
    impl_items: Vec<Tokens>,
    std_impl_items: Vec<Tokens>,
    len_impl_items: HashMap<Length, Vec<Tokens>>,
    std_len_impl_items: HashMap<Length, Vec<Tokens>>,
    trait_impls: Vec<Tokens>,
}

fn primitive_srcmod(primitive: Primitive) -> SrcFile {
    let mut output = PrimitiveSrcMod::default();

    primitive::push_src(primitive, &mut output);

    match primitive {
        Primitive::Float(primitive) => {
            float::push_src(primitive, &mut output);
        }
        Primitive::Int(primitive) => {
            int::push_src(primitive, &mut output);

            match primitive {
                PrimitiveInt::Sint(primitive) => {
                    sint::push_src(primitive, &mut output);
                }
                PrimitiveInt::Uint(primitive) => {
                    uint::push_src(primitive, &mut output);
                }
            }
        }
        Primitive::Bool => {
            bool_::push_src(&mut output);
        }
    }

    quote! {
        use core::mem::transmute_copy;

        use crate::{
            Vector, Simdness, Simd, NonSimd, Usize, VecLen, Scalar,
            $(for n in Length::iter() join(, ) => Vec$(n)),
            $(if primitive.is_float() => ScalarZero, ScalarOne, ScalarNegOne,)
            $(if primitive.is_sint() => ScalarZero, ScalarOne, ScalarNegOne,)
            $(if primitive.is_uint() => ScalarZero, ScalarOne,)
        };

        $(
            if !output.impl_items.is_empty() =>

            impl<const N: usize, S: Simdness> Vector<N, $primitive, S>
            where
                Usize<N>: VecLen,
            {
                $(for impl_item in output.impl_items join($['\n']) => $impl_item)
            }
        )

        $(
            for (n, impl_items) in Length::iter().filter_map(|n| output.len_impl_items.get(&n).map(|impl_items| (n, impl_items))) join($['\n']) =>

            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for impl_item in impl_items join($['\n']) => $impl_item)
            }
        )

        $(
            if !output.std_impl_items.is_empty() =>

            #[cfg(feature = "std")]
            impl<const N: usize, S: Simdness> Vector<N, $primitive, S>
            where
                Usize<N>: VecLen,
            {
                $(for impl_item in output.std_impl_items join($['\n']) => $impl_item)
            }
        )

        $(
            for (n, impl_items) in Length::iter().filter_map(|n| output.std_len_impl_items.get(&n).map(|impl_items| (n, impl_items))) join($['\n']) =>

            #[cfg(feature = "std")]
            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for impl_item in impl_items join($['\n']) => $impl_item)
            }
        )

        $(for trait_impl in output.trait_impls join($['\n']) => $trait_impl)
    }
    .to_srcfile(primitive.to_string())
}
