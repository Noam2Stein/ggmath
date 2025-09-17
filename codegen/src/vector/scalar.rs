use genco::quote;

use crate::{
    constants::{
        BINARY_OPS, COMPARISON_OP_TOKENS, COMPARISON_OP_TRAITS, COMPARISON_OPS, COMPONENTS,
        LENGTHS, UNARY_OPS,
    },
    module::{ModFile, TokensExt},
};

pub fn mod_() -> ModFile {
    quote! {
        use std::ops::*;

        use crate::{Construct, Vector, $(for &n in LENGTHS join(, ) => Vec$(n))};

        $("/// Trait for types that can be put inside [`Vector`].")
        $("/// This is only implemented for actual scalar types (e.g., `f32`),")
        $("/// not vectors, matrices, etc.")
        $("///")
        $("/// When implementing this trait you need to specify the inner types of [`VecAligned`] vectors.")
        $("/// You can also override the implementation of vector functions to make optimizations.")
        $("///")
        $("/// For an example of an optimized `Scalar` implementation,")
        $("/// look at the `f32` implementation.")
        $("///")
        $("/// ## Example")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// #[derive(Clone, Copy)]")
        $("/// struct MyInt(i32);")
        $("///")
        $("/// impl Scalar for MyInt {")
        $("///     // If we wanted to SIMD-accelerate this scalar type,")
        $("///     // we would use another SIMD type like from `std::arch`, `ggmath`, `glam`, etc.")
        $(
            for &n in LENGTHS =>

            $(format!("///     type InnerAlignedVec{n} = [MyInt; {n}];"))$['\r']
        )
        $("///")
        $(
            for &n in LENGTHS join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_from_array(array: [MyInt; {n}]) -> Vec{n}<MyInt> {{"))
            $("///         Vector(array)")
            $("///     }")
        )
        $("///")
        $(
            for &n in LENGTHS join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_as_array(vec: Vec{n}<MyInt>) -> [MyInt; {n}] {{"))
            $("///         vec.0")
            $("///     }")
        )
        $("/// }")
        $("/// ```")
        pub trait Scalar: Construct {
            $(
                for &n in LENGTHS join($['\n']) =>

                $(format!("/// The inner type contained inside `Vector<{n}, Self, VecAligned>`."))
                type InnerAlignedVec$(n): Construct;
            )

            $(
                for &n in LENGTHS join($['\n']) =>

                $(format!("/// Constructs an aligned vec{n} from an array."))
                fn vec$(n)_from_array(array: [Self; $n]) -> Vec$(n)<Self>;

                $(format!("/// Converts an aligned vec{n} to an array."))
                fn vec$(n)_as_array(vec: Vec$(n)<Self>) -> [Self; $n];
            )

            $(
                for &n in LENGTHS join($['\n']) =>

                $(format!("/// Overridable implementation of `Vector::splat` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_splat(value: Self) -> Vec$(n)<Self> {
                    Vec$(n)::from_array([value; $n])
                }

                $(format!("/// Overridable implementation of `Vector::get_unchecked` for aligned vec{n}s."))
                #[inline(always)]
                unsafe fn vec$(n)_get_unchecked(vec: Vec$(n)<Self>, index: usize) -> Self {
                    unsafe { *vec.as_array().get_unchecked(index) }
                }

                $(format!("/// Overridable implementation of `Vector::set_unchecked` for aligned vec{n}s."))
                #[inline(always)]
                unsafe fn vec$(n)_with_unchecked(vec: Vec$(n)<Self>, index: usize, value: Self) -> Vec$(n)<Self> {
                    let mut array = vec.as_array();
                    unsafe { *array.get_unchecked_mut(index) = value; }

                    Vec$(n)::from_array(array)
                }

                $(
                    for &n2 in LENGTHS join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::shuffle_{n2}` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_shuffle_$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i].to_uppercase())_SRC: usize)>(vec: Vec$(n)<Self>) -> Vec$(n2)<Self> {
                        Vec$(n2)::from_array([$(for i in 0..n2 join(, ) => vec.index($(COMPONENTS[i].to_uppercase())_SRC))])
                    }
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::with_{n2}` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_with_shuffle_$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i].to_uppercase())_DST: usize)>(vec: Vec$(n)<Self>, value: Vec$(n2)<Self>) -> Vec$(n)<Self> {
                        let mut output = vec;
                        $(
                            for i in 0..n2 =>

                            output.set($(COMPONENTS[i].to_uppercase())_DST, value.index($i));
                            $['\r']
                        )

                        output
                    }
                )

                $(format!("/// Overridable implementation of `Vector::eq` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_eq<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> bool
                where
                    Self: PartialEq<T2>,
                {
                    (0..$n).all(|i| vec.index(i) == other.index(i))
                }
        
                $(format!("/// Overridable implementation of `Vector::ne` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_ne<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> bool
                where
                    Self: PartialEq<T2>,
                {
                    (0..$n).any(|i| vec.index(i) != other.index(i))
                }

                $(
                    for &op_camelcase in UNARY_OPS join($['\n']) =>

                    $(let op_snakecase = &op_camelcase.to_lowercase())

                    $(format!("/// Overridable implementation of `Vector::{op_snakecase}` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_$(op_snakecase)(vec: Vec$(n)<Self>) -> Vec$(n)<<Self as $op_camelcase>::Output>
                    where
                        Self: $op_camelcase<Output: Scalar>,
                    {
                        vec.map(|v| v.$op_snakecase())
                    }
                )

                $(
                    for &op_camelcase in BINARY_OPS join($['\n']) =>

                    $(let op_snakecase = &op_camelcase.to_lowercase())

                    $(format!("/// Overridable implementation of `Vector::{op_snakecase}` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_$(op_snakecase)<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> Vec$(n)<<Self as $op_camelcase<T2>>::Output>
                    where
                        Self: $op_camelcase<T2, Output: Scalar>,
                    {
                        Vector::from_fn(|i| vec.index(i).$op_snakecase(other.index(i)))
                    }
                )

                $(
                    for cmp_op_idx in 0..COMPARISON_OPS.len() join($['\n']) =>

                    $(let cmp_lower = COMPARISON_OPS[cmp_op_idx])
                    $(let cmp_tt = COMPARISON_OP_TOKENS[cmp_op_idx])
                    $(let cmp_trait = COMPARISON_OP_TRAITS[cmp_op_idx])

                    $(format!("/// Overridable implementation of `Vector::{cmp_lower}_mask` for aligned vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_$(cmp_lower)_mask<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> Vec$(n)<bool>
                    where
                        Self: $cmp_trait<T2>,
                    {
                        Vector::from_fn(|i| vec.index(i) $cmp_tt other.index(i))
                    }
                )

                $(format!("/// Overridable implementation of `Vector::sum` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_sum(vec: Vec$(n)<Self>) -> Self
                where
                    Self: Add<Output = Self>,
                {
                    vec.reduce(|a, b| a + b)
                }

                $(format!("/// Overridable implementation of `Vector::product` for aligned vec{n}s."))
                #[inline(always)]
                fn vec$(n)_product(vec: Vec$(n)<Self>) -> Self
                where
                    Self: Mul<Output = Self>,
                {
                    vec.reduce(|a, b| a * b)
                }
            )
        }
    }
    .to_mod_file("scalar")
}
