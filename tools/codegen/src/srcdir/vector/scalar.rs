use genco::quote;
use strum::IntoEnumIterator;

use crate::{module::{SrcFile, TokensExt}, iter::{Axis, BinOp, CmpOp, Length, UnOp}};

pub fn srcmod() -> SrcFile {
    quote! {
        use std::ops::*;

        use crate::{Construct, Vector, VecLen, Usize, Simd};

        $("/// Trait for [`Vector`] element types.")
        $("///")
        $("/// This trait is intended for *scalar* types like `f32`, `i32`, or custom number-like types.")
        $("/// It is not intended for higher-level mathamatical structures like vectors or matrices.")
        $("/// Meaning you cannot do things like `Vec2<Vec2<f32>>`.")
        $("///")
        $("/// Each [`Scalar`] implementation specifies its own inner [`Simd`] vector types,")
        $("/// and how to convert them to and from arrays.")
        $("///")
        $("/// Additionally, each [`Scalar`] implementation can override the implementation of [`Simd`] vector functions to make optimizations.")
        $("///")
        $("/// ## Examples")
        $("///")
        $("/// Non SIMD-accelerated implementation:")
        $("/// ```")
        $("/// use core::ops::*;")
        $("///")
        $("/// use ggmath::*;")
        $("///")
        $("/// #[derive(Clone, Copy)]")
        $("/// struct U256([u128; 2]);")
        $("///")
        $("/// // Because U256 is Add, Vector<_, U256, _> is automatically Add as well.")
        $("/// impl Add for U256 {")
        $("///     type Output = U256;")
        $("///")
        $("///     fn add(self, other: U256) -> U256 {")
        $("///         todo!()")
        $("///     }")
        $("/// }")
        $("///")
        $("///")
        $("/// impl Scalar for U256 {")
        $(for n in Length::iter() => $(format!("///     type InnerSimdVec{n} = [U256; {n}];"))$['\r'])
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_from_array(array: [U256; {n}]) -> Vec{n}<U256> {{"))
            $("///         Vector(array)")
            $("///     }")
        )
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_as_array(vec: Vec{n}<U256>) -> [U256; {n}] {{"))
            $("///         vec.0")
            $("///     }")
        )
        $("/// }")
        $("/// ```")
        $("///")
        $("/// SIMD-accelerated implementation:")
        $("/// ```")
        $("/// use core::ops::*;")
        $("///")
        $("/// use ggmath::*;")
        $("///")
        $("/// #[repr(transparent)]")
        $("/// #[derive(Clone, Copy)]")
        $("/// struct CustomInt(i32);")
        $("///")
        $("/// // Because CustomInt is Add, Vector<_, CustomInt, _> is automatically Add as well.")
        $("/// // This time we will override the implementation of vector addition to make SIMD optimizations.")
        $("/// impl Add for CustomInt {")
        $("///     type Output = CustomInt;")
        $("///")
        $("///     fn add(self, other: CustomInt) -> CustomInt {")
        $("///         CustomInt(self.0 + other.0)")
        $("///     }")
        $("/// }")
        $("///")
        $("/// impl Scalar for CustomInt {")
        $(for n in Length::iter() => $(format!("///     type InnerSimdVec{n} = Vec{n}<i32>;"))$['\r'])
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_from_array(array: [CustomInt; {n}]) -> Vec{n}<CustomInt> {{"))
            $(format!("///         Vector(Vec{n}::<i32>::from_array(array))"))
            $("///     }")
        )
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_as_array(vec: Vec{n}<CustomInt>) -> [CustomInt; {n}] {{"))
            $("///         vec.0.as_array()")
            $("///     }")
        )
        $("///")
        $(
            for n in Length::iter() join($['\r']$("///")$['\r']) =>

            $("///     #[inline(always)]")
            $(format!("///     fn vec{n}_add<T2: Scalar>(vec: Vec{n}<CustomInt>, rhs: Vec{n}<T2>) -> Vec{n}<CustomInt::Output>"))
            $("///     where")
            $("///         CustomInt: Add<T2, Output: Scalar>,")
            $("///     {")
            $("///         specialize! {")
            $(format!("///             (vec: Vec{n}<CustomInt>, rhs: Vec{n}<T2>) -> Vec{n}<CustomInt::Output>:"))
            $("///             ")
            $(format!("///             for (Vec{n}<CustomInt>, Vec{n}<CustomInt>) -> Vec{n}<CustomInt> {{"))
            $("///                 |vec, rhs| Vector(vec.0 + rhs.0)")
            $("///             } else {")
            $("///                 Vector::from_fn(|i| vec.index(i).add(rhs.index(i)))")
            $("///             }")
            $("///         }")
            $("///     }")
        )
        $("/// }")
        $("/// ```")
        pub trait Scalar: Construct {
            $("/// The inner type contained inside [`Simd`] vectors.")
            $("///")
            $("/// To choose a seperate type for each length, use [`Usize<N>::Match`][VecLen::Match].")
            type SimdVectorStorage<const N: usize>: Construct
            where
                Usize<N>: VecLen;

            $("/// Converts an array to a [`Simd`] vector.")
            fn vec_from_array<const N: usize>(array: [Self; N]) -> Vector<N, Self, Simd>
            where
                Usize<N>: VecLen;
            
            $("/// Converts a [`Simd`] vector to an array.")
            fn vec_as_array<const N: usize>(vec: Vector<N, Self, Simd>) -> [Self; N]
            where
                Usize<N>: VecLen;

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
                fn vec_$(op.lowercase_str())<const N: usize>(vec: Vector<N, Self, Simd>) -> Vector<N, <Self as $(op.camelcase_str())>::Output, Simd>
                where
                    Usize<N>: VecLen,
                    Self: $(op.camelcase_str())<Output: Scalar>,
                {
                    vec.map(|v| v.$(op.lowercase_str())())
                }
            )

            $(
                for op in BinOp::iter() join($['\n']) =>

                $(format!("/// Overridable implementation of [`Simd`] [`Vector::{op}`].", op = op.lowercase_str()))
                #[inline(always)]
                fn vec_$(op.lowercase_str())<const N: usize, T2: Scalar>(vec: Vector<N, Self, Simd>, other: Vector<N, T2, Simd>) -> Vector<N, <Self as $(op.camelcase_str())<T2>>::Output, Simd>
                where
                    Usize<N>: VecLen,
                    Self: $(op.camelcase_str())<T2, Output: Scalar>,
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
    }
    .to_srcfile("scalar")
}
