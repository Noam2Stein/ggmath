use genco::quote;
use strum::IntoEnumIterator;

use crate::{module::{SrcFile, TokensExt}, iter::{Axis, BinOp, CmpOp, Length, UnOp}};

pub fn srcmod() -> SrcFile {
    quote! {
        use std::ops::*;

        use crate::{Construct, Vector, $(for n in Length::iter() join(, ) => Vec$(n))};

        $("/// Trait for types that can be put inside [`Vector`].")
        $("/// This is only implemented for actual scalar types (e.g., `f32`),")
        $("/// not vectors, matrices, etc.")
        $("///")
        $("/// When implementing this trait you need to specify the inner types of [`Simd`] vectors.")
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
        $(for n in Length::iter() => $(format!("///     type InnerSimdVec{n} = [MyInt; {n}];"))$['\r'])
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_from_array(array: [MyInt; {n}]) -> Vec{n}<MyInt> {{"))
            $("///         Vector(array)")
            $("///     }")
        )
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_as_array(vec: Vec{n}<MyInt>) -> [MyInt; {n}] {{"))
            $("///         vec.0")
            $("///     }")
        )
        $("/// }")
        $("/// ```")
        pub trait Scalar: Construct {
            $(
                for n in Length::iter() join($['\n']) =>

                $(format!("/// The inner type contained inside `Vector<{n}, Self, Simd>`."))
                type InnerSimdVec$(n): Construct;
            )

            $(
                for n in Length::iter() join($['\n']) =>

                $(format!("/// Constructs a simd vec{n} from an array."))
                fn vec$(n)_from_array(array: [Self; $n]) -> Vec$(n)<Self>;

                $(format!("/// Converts a simd vec{n} to an array."))
                fn vec$(n)_as_array(vec: Vec$(n)<Self>) -> [Self; $n];
            )

            $(
                for n in Length::iter() join($['\n']) =>

                $(format!("/// Overridable implementation of `Vector::splat` for simd vec{n}s."))
                #[inline(always)]
                fn vec$(n)_splat(value: Self) -> Vec$(n)<Self> {
                    Vec$(n)::from_array([value; $n])
                }

                $(format!("/// Overridable implementation of `Vector::get_unchecked` for simd vec{n}s."))
                #[inline(always)]
                unsafe fn vec$(n)_get_unchecked(vec: Vec$(n)<Self>, index: usize) -> Self {
                    unsafe { *vec.as_array().get_unchecked(index) }
                }

                $(format!("/// Overridable implementation of `Vector::set_unchecked` for simd vec{n}s."))
                #[inline(always)]
                unsafe fn vec$(n)_with_unchecked(vec: Vec$(n)<Self>, index: usize, value: Self) -> Vec$(n)<Self> {
                    let mut array = vec.as_array();
                    unsafe { *array.get_unchecked_mut(index) = value; }

                    Vec$(n)::from_array(array)
                }

                $(
                    for n2 in Length::iter() join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::shuffle_{n2}` for simd vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_shuffle_$(n2)<$(for i in 0..n2.as_usize() join(, ) => const $(Axis(i).uppercase_str())_SRC: usize)>(vec: Vec$(n)<Self>) -> Vec$(n2)<Self> {
                        Vec$(n2)::from_array([$(for i in 0..n2.as_usize() join(, ) => vec.index($(Axis(i).uppercase_str())_SRC))])
                    }
                )

                $(
                    for n2 in Length::iter().filter(|&n2| n2 <= n) join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::with_{n2}` for simd vec{n}s."))
                    #[inline(always)]
                    fn vec$(n)_with_shuffle_$(n2)<$(for i in 0..n2.as_usize() join(, ) => const $(Axis(i).uppercase_str())_DST: usize)>(vec: Vec$(n)<Self>, value: Vec$(n2)<Self>) -> Vec$(n)<Self> {
                        let mut output = vec;
                        $(
                            for i in 0..n2.as_usize() =>

                            output.set($(Axis(i).uppercase_str())_DST, value.index($i));
                            $['\r']
                        )

                        output
                    }
                )

                $(format!("/// Overridable implementation of `Vector::eq` for simd vec{n}s."))
                #[inline(always)]
                fn vec$(n)_eq<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> bool
                where
                    Self: PartialEq<T2>,
                {
                    (0..$n).all(|i| vec.index(i) == other.index(i))
                }
        
                $(format!("/// Overridable implementation of `Vector::ne` for simd vec{n}s."))
                #[inline(always)]
                fn vec$(n)_ne<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> bool
                where
                    Self: PartialEq<T2>,
                {
                    (0..$n).any(|i| vec.index(i) != other.index(i))
                }

                $(
                    for op in UnOp::iter() join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::{}` for simd vec{n}s.", op.lowercase_str()))
                    #[inline(always)]
                    fn vec$(n)_$(op.lowercase_str())(vec: Vec$(n)<Self>) -> Vec$(n)<<Self as $(op.camelcase_str())>::Output>
                    where
                        Self: $(op.camelcase_str())<Output: Scalar>,
                    {
                        vec.map(|v| v.$(op.lowercase_str())())
                    }
                )

                $(
                    for op in BinOp::iter() join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::{}` for simd vec{n}s.", op.lowercase_str()))
                    #[inline(always)]
                    fn vec$(n)_$(op.lowercase_str())<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> Vec$(n)<<Self as $(op.camelcase_str())<T2>>::Output>
                    where
                        Self: $(op.camelcase_str())<T2, Output: Scalar>,
                    {
                        Vector::from_fn(|i| vec.index(i).$(op.lowercase_str())(other.index(i)))
                    }
                )

                $(
                    for op in CmpOp::iter() join($['\n']) =>

                    $(format!("/// Overridable implementation of `Vector::{}_mask` for simd vec{n}s.", op.lowercase_str()))
                    #[inline(always)]
                    fn vec$(n)_$(op.lowercase_str())_mask<T2: Scalar>(vec: Vec$(n)<Self>, other: Vec$(n)<T2>) -> Vec$(n)<bool>
                    where
                        Self: $(op.trait_str())<T2>,
                    {
                        Vector::from_fn(|i| vec.index(i) $(op.punct_str()) other.index(i))
                    }
                )

                $(format!("/// Overridable implementation of `Vector::sum` for simd vec{n}s."))
                #[inline(always)]
                fn vec$(n)_sum(vec: Vec$(n)<Self>) -> Self
                where
                    Self: Add<Output = Self>,
                {
                    vec.reduce(|a, b| a + b)
                }

                $(format!("/// Overridable implementation of `Vector::product` for simd vec{n}s."))
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
    .to_srcfile("scalar")
}
