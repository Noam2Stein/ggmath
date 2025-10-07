use genco::quote;
use strum::IntoEnumIterator;

use crate::{module::{SrcFile, TokensExt}, iter::{Axis, BinOp, CmpOp, Length, UnOp}};

pub fn srcmod() -> SrcFile {
    quote! {
        use std::ops::*;

        use crate::{Construct, Vector, VecLen, Usize, Simd};

        $("/// Trait for [`Vector`] element types.")
        $("///")
        $("/// To enable SIMD optimizations in a [`Scalar`] implementation,")
        $("/// you need to specify [`Simd`] vector representation,")
        $("/// and override the implementation of [`Simd`] vector functions to use SIMD operations.")
        $("///")
        $("/// To implement [`Scalar`] without SIMD optimizations, use the [`default_scalar_boilerplate!`] macro.")
        $("///")
        $("/// ## Example")
        $("///")
        $("/// ```")
        $("/// use ggmath::*;")
        $("///")
        $("/// #[derive(Clone, Copy)]")
        $("/// struct U256([u128; 2]);")
        $("///")
        $("/// impl Scalar for U256 {")
        $("///     default_scalar_boilerplate! {}")
        $("/// }")
        $("/// ```")
        $("///")
        $("/// ## SIMD Example")
        $("///")
        $("/// ```")
        $("/// use core::ops::Add;")
        $("///")
        $("/// use ggmath::*;")
        $("///")
        $("/// #[repr(transparent)]")
        $("/// #[derive(Clone, Copy)]")
        $("/// struct Int(i32);")
        $("///")
        $("/// // Implement `Add` for `Int` which automatically makes `Vector<N, Int, S>` `Add`.")
        $("/// impl Add for Int {")
        $("///     type Output = Int;")
        $("///")
        $("///     fn add(self, other: Int) -> Int {")
        $("///         Int(self.0 + other.0)")
        $("///     }")
        $("/// }")
        $("///")
        $("/// impl Scalar for Int {")
        $("///     // Because `Int` wraps `i32`, we can store `Int` SIMD vectors as high-level `i32` SIMD vectors,")
        $("///     // instead of low-level SIMD types like `__m128i`.")
        $("///     type SimdVectorStorage<const N: usize> = Vector<N, i32, Simd> where Usize<N>: VecLen;")
        $("///")
        $("///     #[inline(always)]")
        $("///     fn vec_from_array<const N: usize>(array: [Int; N]) -> Vector<N, Int, Simd> {")
        $("///         let array = core::mem::transmute_copy::<[Int; N], [i32; N]>(&array);")
        $("///         Vector(Vector::from_array(array))")
        $("///     }")
        $("///")
        $("///     #[inline(always)]")
        $("///     fn vec_as_array<const N: usize>(vec: Vector<N, Int, Simd>) -> [Int; N] {")
        $("///         let array = vec.0.as_array();")
        $("///         core::mem::transmute_copy::<[i32; N], [Int; N]>(&array)")
        $("///     }")
        $("///")
        $("///     // Override vector addition to use SIMD operations.")
        $("///     #[inline(always)]")
        $("///     fn vec_add<const N: usize>(vec: Vector<N, Int, Simd>, rhs: Vector<N, Int, Simd>) -> Vector<N, Int, Simd> {")
        $("///         Vector(vec.0 + rhs.0)")
        $("///     }")
        $("/// }")
        $("/// ```")
        pub unsafe trait Scalar: Construct {
            $("/// The inner type contained inside [`Simd`] vectors.")
            $("/// Its reference must be transmutable to `&[Self; N]`.")
            $("///")
            $("/// To choose a seperate type for each length, use [`Usize<N>::Match`][VecLen::Match].")
            type SimdVectorStorage<const N: usize>: Construct
            where
                Usize<N>: VecLen;

            const ANY_SIMD_VECTOR_VALUE: Vector<N, Self, Simd>;

            $("/// Overridable implementation of [`Simd`] [`Vector::from_array`].")
            #[inline(always)]
            fn vec_from_array<const N: usize>(array: [Self; N]) -> Vector<N, Self, Simd>
            where
                Usize<N>: VecLen
            {
                Vector::const_from_array(array)
            }

            $("/// Overridable implementation of [`Simd`] [`Vector::splat`].")
            #[inline(always)]
            fn vec_splat<const N: usize>(value: Self) -> Vector<N, Self, Simd>
            where
                Usize<N>: VecLen
            {
                Vector::from_array([value; N])
            }

            $("/// Overridable implementation of [`Simd`] [`Vector::get_unchecked`].")
            #[inline(always)]
            unsafe fn vec_get_unchecked<const N: usize>(vec: Vector<N, Self, Simd>, index: usize) -> Self
            where
                Usize<N>: VecLen,
            {
                unsafe { *vec.as_array().get_unchecked(index) }
            }

            $("/// Overridable implementation of [`Simd`] [`Vector::with_unchecked`].")
            #[inline(always)]
            unsafe fn vec_with_unchecked<const N: usize>(vec: Vector<N, Self, Simd>, index: usize, value: Self) -> Vector<N, Self, Simd>
            where
                Usize<N>: VecLen,
            {
                let mut array = vec.as_array();
                unsafe { *array.get_unchecked_mut(index) = value; }

                Vector::from_array(array)
            }

            $(
                for n2 in Length::iter() join($['\n']) =>

                $(let src_params = &(0..n2.as_usize()).map(|i| format!("const {}_SRC: usize", Axis(i).uppercase_str())).collect::<Vec<_>>().join(", "))

                $(format!("/// Overridable implementation of [`Simd`] [`Vector::shuffle_{n2}`]."))
                #[inline(always)]
                fn vec_shuffle_$n2<const N: usize, $src_params>(vec: Vector<N, Self, Simd>) -> Vector<$n2, Self, Simd>
                where
                    Usize<N>: VecLen,
                {
                    Vector::<$n2, Self, Simd>::from_array([$(
                        for i in 0..n2.as_usize() join(, ) =>
                        
                        vec.index($(Axis(i).uppercase_str())_SRC)
                    )])
                }
            )

            $(
                for n2 in Length::iter() join($['\n']) =>

                $(let dst_params = &(0..n2.as_usize()).map(|i| format!("const {}_DST: usize", Axis(i).uppercase_str())).collect::<Vec<_>>().join(", "))

                $(format!("/// Overridable implementation of [`Simd`] [`Vector::with_shuffle_{n2}`]."))
                #[inline(always)]
                fn vec_with_shuffle_$n2<const N: usize, $dst_params>(vec: Vector<N, Self, Simd>, value: Vector<$n2, Self, Simd>) -> Vector<N, Self, Simd>
                where
                    Usize<N>: VecLen,
                {
                    let mut output = vec;
                    $(
                        for i in 0..n2.as_usize() join($['\r']) =>

                        output.set($(Axis(i).uppercase_str())_DST, value.index($i));
                    )

                    output
                }
            )

            $("/// Overridable implementation of [`Simd`] [`Vector::eq`].")
            #[inline(always)]
            fn vec_eq<const N: usize, T2: Scalar>(vec: Vector<N, Self, Simd>, other: Vector<N, T2, Simd>) -> bool
            where
                Usize<N>: VecLen,
                Self: PartialEq<T2>,
            {
                (0..N).all(|i| vec.index(i) == other.index(i))
            }
    
            $("/// Overridable implementation of [`Simd`] [`Vector::ne`].")
            #[inline(always)]
            fn vec_ne<const N: usize, T2: Scalar>(vec: Vector<N, Self, Simd>, other: Vector<N, T2, Simd>) -> bool
            where
                Usize<N>: VecLen,
                Self: PartialEq<T2>,
            {
                (0..N).any(|i| vec.index(i) != other.index(i))
            }

            $(
                for op in UnOp::iter() join($['\n']) =>

                $(format!("/// Overridable implementation of [`Simd`] [`Vector::{op}`].", op = op.lowercase_str()))
                #[inline(always)]
                fn vec_$(op.lowercase_str())<const N: usize>(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
                where
                    Usize<N>: VecLen,
                    Self: $(op.camelcase_str())<Output = Self>,
                {
                    vec.map(|v| v.$(op.lowercase_str())())
                }
            )

            $(
                for op in BinOp::iter() join($['\n']) =>

                $(format!("/// Overridable implementation of [`Simd`] [`Vector::{op}`].", op = op.lowercase_str()))
                #[inline(always)]
                fn vec_$(op.lowercase_str())<const N: usize>(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
                where
                    Usize<N>: VecLen,
                    Self: $(op.camelcase_str())<Output = Self>,
                {
                    Vector::from_fn(|i| vec.index(i).$(op.lowercase_str())(other.index(i)))
                }
            )

            $(
                for op in CmpOp::iter() join($['\n']) =>

                $(format!("/// Overridable implementation of [`Simd`] [`Vector::{op}_mask`].", op = op.lowercase_str()))
                #[inline(always)]
                fn vec_$(op.lowercase_str())_mask<const N: usize, T2: Scalar>(vec: Vector<N, Self, Simd>, other: Vector<N, T2, Simd>) -> Vector<N, bool, Simd>
                where
                    Usize<N>: VecLen,
                    Self: $(op.trait_str())<T2>,
                {
                    Vector::from_fn(|i| vec.index(i) $(op.punct_str()) other.index(i))
                }
            )

            $(format!("/// Overridable implementation of [`Simd`] [`Vector::sum`]."))
            #[inline(always)]
            fn vec_sum<const N: usize>(vec: Vector<N, Self, Simd>) -> Self
            where
                Usize<N>: VecLen,
                Self: Add<Output = Self>,
            {
                vec.reduce(|a, b| a + b)
            }

            $(format!("/// Overridable implementation of [`Simd`] [`Vector::product`]."))
            #[inline(always)]
            fn vec_product<const N: usize>(vec: Vector<N, Self, Simd>) -> Self
            where
                Usize<N>: VecLen,
                Self: Mul<Output = Self>,
            {
                vec.reduce(|a, b| a * b)
            }
        }

        $("/// Macro for implementing [`Scalar`] without SIMD optimizations.")
        $("///")
        $("/// ## Example")
        $("///")
        $("/// ```")
        $("/// use ggmath::{Scalar, vec2};")
        $("///")
        $("/// #[derive(Debug, Clone, Copy)]")
        $("/// struct MyScalar(f32);")
        $("///")
        $("/// impl_scalar! { impl Scalar for MyScalar }")
        $("///")
        $("/// fn main() {")
        $(r#"///     println!("{:?}", vec2!(MyScalar(1.0), MyScalar(2.0)));"#)
        $("/// }")
        $("///")
        $("/// ```")
        #[macro_export]
        macro_rules! impl_scalar {
            { impl$$(<$$($$generics:tt)*>)? Scalar for $$T $$(where $$($$where_clause:tt)*)? } => {
                unsafe impl$$(<$$($$generics)*>)? Scalar for $$T $$(where $$($$where_clause)*)? {
                    type SimdVectorStorage<const N: usize> = [Self; N]
                    where
                        $crate::Usize<N>: $crate::VecLen;
                }
            }
        }
    }
    .to_srcfile("scalar")
}
