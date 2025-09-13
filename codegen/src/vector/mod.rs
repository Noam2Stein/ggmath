use indoc::formatdoc;

use crate::{
    constants::{LENGTH_NAMES, LENGTHS},
    r#gen::ModDir,
    join_and,
};

mod dir;
mod impl_api;
mod impl_std;
mod primitives;
mod swizzle;

pub fn module() -> ModDir {
    let vector = vector();
    let vector_aliases = vector_aliases();
    let (vec_len, vec_len_impl) = vec_len();
    let (vec_alignment, vec_alignment_impl) = vec_alignment();

    let mut scalar_fns = Vec::new();

    let impl_api = impl_api::impl_api(&mut scalar_fns);
    let impl_std = impl_std::impl_std(&mut scalar_fns);
    let swizzle = swizzle::module(&mut scalar_fns);
    let dir = dir::module();

    let scalar = scalar(scalar_fns);

    ModDir::new(
        "vector",
        formatdoc! {r#"
            //! Vector related types and traits

            use core::{{
                fmt::{{Debug, Display}},
                ops::*,
                slice::SliceIndex,
            }};

            use crate::{{Construct, Usize}};

            mod dir;
            mod primitives;
            mod swizzle;
            pub use dir::*;

            {vector}

            {vector_aliases}

            {vec_len}

            {scalar}

            {vec_alignment}

            {vec_len_impl}

            {vec_alignment_impl}

            {impl_api}

            {impl_std}
        "#},
        vec![swizzle, dir],
        vec![],
        vec![],
    )
}

fn vector() -> String {
    let len_list = join_and(LENGTHS.iter().map(|len| len.to_string()));

    formatdoc! {r#"
        /// A generic vector type.
        ///
        /// This type is generic over 3 parameters:
        /// - `N`: The length of the vector, which currently supports {len_list}.
        /// - `T`: The type of the vector, which must implement the [`Scalar`] trait.
        /// - `A`: The "alignment" of the vector, which enables or disables SIMD memory alignment.
        ///
        /// This type has very very useful type-aliases:
        /// - `Vec{{N}}<T>` like `Vec2<f32>` is for SIMD aligned vectors
        /// - `Vec{{N}}P<T>` like `Vec2P<f32>` is for non-SIMD aligned vectors
        ///
        /// # Length
        ///
        /// Currently only the lengths {len_list} are supported in order to specialize their inner vector type.
        /// In the future if rust gains more type-system features, more lengths will be supported.
        ///
        /// Beware that code should never rely on the fact that {len_list} are the only supported lengths.
        /// Code that branches based on vector length should either properly handle all usize values or use [`VecLenEnum`].
        ///
        /// # Alignment
        ///
        /// The `A` generic parameter controls whether or not the vector is SIMD aligned,
        /// and can be set to either `VecAligned` or `VecPacked`.
        ///
        /// SIMD can improve performance of vector operations,
        /// but it can also increase the size of the vector in memory.
        ///
        /// `Vector<N, T, VecAligned>` vectors are SIMD aligned if it increases performance,
        /// while `Vector<N, T, VecPacked>` vectors are not SIMD aligned and are always stored as `[T; N]`.
        ///
        /// This means that `VecAligned` are for performance and `VecPacked` are for memory efficiency.
        ///
        /// Beware that while `VecPacked` guarentees an exact memory layout of `[T; N]`, `VecAligned` does not guarantee a specific alignment rule/pattern.
        /// For example, `Vector<3, f32, VecAligned`/`Vec3<f32>` isn't guaranteed to be aligned to a 128-bit boundary.
        /// It is up to the implementation of [`Scalar`] to determine `VecAligned` alignment for whatever is most performant.
        ///
        /// # Examples
        /// ```
        /// use ggmath::aliases::*;
        ///
        /// // This is a non memory critical scenario so we should use `VecAligned`.
        /// struct PlayerState {{
        ///     // Vector<3, f32, VecAligned>
        ///     position: Vec3<f32>,
        ///     // ...
        /// }}
        ///
        /// // This is a memory critical scenario so we should use `VecPacked`.
        /// struct GpuVertex {{
        ///     // Vector<3, f32, VecPacked>
        ///     position: Vec3P<f32>,
        ///     // Vector<3, f32, VecPacked>
        ///     normal: Vec3P<f32>,
        ///     // Vector<2, f32, VecPacked>
        ///     uv: Vec2P<f32>,
        /// }}
        /// ```
        #[repr(transparent)]
        pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>(pub A::InnerVector<N, T>)
        where
            Usize<N>: VecLen;
    "#}
}

fn vector_aliases() -> String {
    let aliases = LENGTHS
        .iter()
        .map(|len| {
            formatdoc! {r#"
            /// Type alias for [`Vector<{len}, T, VecAligned>`][Vector].
            pub type Vec{len}<T> = Vector<{len}, T, VecAligned>;

            /// Type alias for [`Vector<{len}, T, VecPacked>`][Vector].
            pub type Vec{len}P<T> = Vector<{len}, T, VecPacked>;
        "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let macro_aliases = LENGTHS
        .iter()
        .map(|len| {
            formatdoc! {r#"
            #[doc = "Type alias to [`Vector<{len}, " $t ", VecAligned>`][Vector]."]
            $($vis)* type [<$prefix Vec{len}>] = $crate::Vec{len}<$t>;

            #[doc = "Type alias to [`Vector<{len}, " $t ", VecPacked>`][Vector]."]
            $($vis)* type [<$prefix Vec{len}P>] = $crate::Vec{len}P<$t>;
        "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t\t\t");

    let example_aliases = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                pub type FVec{n} = ggmath::Vec{n}<f32>;
                pub type FVec{n}P = ggmath::Vec{n}P<f32>;
            "#}
        })
        .collect::<Vec<_>>()
        .join("")
        .replace("\n", "\n/// ");

    formatdoc! {r#"
        {aliases}

        /// Macro that generates vector type aliases for a specific scalar type.
        ///
        /// Syntax: `vector_aliases!(<visibility> <prefix> => <scalar>)`
        ///
        /// # Examples
        ///
        /// ```
        /// use ggmath::*;
        /// 
        /// vector_aliases!(pub F => f32);
        /// ```
        /// Generates:
        /// ```
        /// {example_aliases}
        /// ```
        #[macro_export]
        macro_rules! vector_aliases {{
            (pub($($vis:tt)*) $prefix:ident => $t:ty) => {{
                $crate::vector_aliases!(@(pub $($vis)*) $prefix => $t);
            }};
            (pub $prefix:ident => $t:ty) => {{
                $crate::vector_aliases!(@(pub) $prefix => $t);
            }};
            ($prefix:ident => $t:ty) => {{
                $crate::vector_aliases!(@() $prefix => $t);
            }};

            (@($($vis:tt)*) $prefix:ident => $t:ty) => {{
                $crate::_hidden_::paste::paste! {{
                    {macro_aliases}
                }}
            }};
        }}
    "#}
}

fn vec_len() -> (String, String) {
    let veclen_enum_variants = LENGTHS
        .iter()
        .zip(LENGTH_NAMES)
        .map(|(len, name)| {
            formatdoc! {r#"
            /// `{len}`
            {name},
        "#}
        })
        .collect::<String>()
        .replace("\n", "\n\t");

    let impls = LENGTHS
        .iter()
        .zip(LENGTH_NAMES)
        .map(|(&n, name)| {
            formatdoc! {r#"
            impl VecLen for Usize<{n}> {{
                type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec{n};

                const ENUM: VecLenEnum = VecLenEnum::{name};
            }}
        "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    (
        formatdoc! {r#"
            /// A trait that marks a `Usize<N>` type as a valid vector length.
            /// See [`Vector`] for more information.
            pub trait VecLen {{
                /// The inner type contained inside `Vector<N, T, VecAligned>`.
                ///
                /// This redirects to `T::InnerAlignedVec{{N}}`,
                /// for example `T::InnerAlignedVec2` for `Usize<2>`.
                type InnerAlignedVector<T: Scalar>: Construct;

                /// The length value as an enum.
                const ENUM: VecLenEnum;
            }}

            /// An enum with all currently supported vector lengths.
            ///
            /// The enum value of a generic `const N: usize` value can be accessed with [`<Usize<N> as VecLen>::ENUM`][`VecLen::ENUM`].
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum VecLenEnum {{
                {veclen_enum_variants}
            }}
        "#},
        formatdoc! {r#"
            {impls}
        "#},
    )
}

fn scalar(scalar_fns: Vec<String>) -> String {
    let vector_types = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                /// The inner type contained inside `Vector<{n}, Self, VecAligned>` vectors.
                type InnerAlignedVec{n} = [Self; {n}];
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\n", "\n\t");

    let bigint_vector_types = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                type InnerAlignedVec{n} = [Self; {n}];
            "#}
        })
        .collect::<Vec<_>>()
        .join("")
        .replace("\n", "\n/// \t");

    let smallint_vector_types = LENGTHS
        .iter()
        .map(|&n| {
            formatdoc! {r#"
                type InnerAlignedVec{n} = Vector<{n}, Self, VecAligned>;
            "#}
        })
        .collect::<Vec<_>>()
        .join("")
        .replace("\n", "\n/// \t");

    let scalar_fns = scalar_fns.join("\n").replace("\n", "\n\t");

    formatdoc! {r#"
        /// A trait that marks a type as a valid scalar type that can be used in a vector.
        /// This trait is implemented for most primitive types, like `f32`, `f64`, `bool`, `usize`, etc.
        ///
        /// # Implementing `Scalar`
        ///
        /// When implementing `Scalar` you need to fill:
        ///
        /// 1.
        /// `InnerAlignedVec2`, `InnerAlignedVec3`, etc
        ///
        /// These are the inner types stored inside `VecAligned` vectors,
        /// for example `Vector<3, f32, VecAligned>` is stored as `f32::InnerAlignedVec3`.
        ///
        /// The reference of these types MUST be transmutable to `&[T; N]`,
        /// if its not then using that vector is undefined behavior.
        /// This means that you cannot do things like expand `Vec3<bool>` into a 128-bit SIMD register with 32-bit lanes.
        ///
        /// 2.
        /// `GARBAGE`, `INNER_ALIGNED_VEC2_GARBAGE`, `INNER_ALIGNED_VEC3_GARBAGE`, etc
        ///
        /// These need to be any valid value of `Self`, `Self::InnerAlignedVec2`, `Self::InnerAlignedVec3`, etc.
        /// This is used to properly initialize aligned vectors.
        ///
        /// # Examples
        ///
        /// ```
        /// #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
        /// struct BigInt {{
        ///     // private fields
        /// }}
        ///
        /// // impl Add, Sub... for BigInt
        ///
        /// // lets say BigInt cannot benefit from SIMD operations, or we just don't want to optimize it yet.
        /// // When not wanting SIMD we can fill `InnerAlignedVec{{N}}` with `[Self; N]`.
        /// impl Scalar for BigInt {{
        ///     {bigint_vector_types}
        /// }}
        ///
        /// struct SmallInt(i32);
        ///
        /// // impl Add, Sub... for SmallInt
        ///
        /// // lets say SmallInt can benefit from SIMD operations.
        /// impl Scalar for SmallInt {{
        ///     // use i32 vector types for aligned vectors.
        ///     {smallint_vector_types}
        /// }}
        /// ```
        pub trait Scalar: Construct {{
            {vector_types}

            {scalar_fns}
        }}
    "#}
}

fn vec_alignment() -> (String, String) {
    (
        formatdoc! {r#"
            /// See [`Vector`] for information.
            pub trait VecAlignment: 'static {{
                /// The inner type contained inside [`Vector`].
                ///
                /// For `VecAligned` vectors this is `T::InnerAlignedVec{{N}}`,
                /// for example `T::InnerAlignedVec2` for `Vec2`.
                ///
                /// For `VecPacked` vectors this is `[T; N]`,
                /// for example `[T; 2]` for `Vec2`.
                type InnerVector<const N: usize, T: Scalar>: Construct
                where
                    Usize<N>: VecLen;

                /// Whether or not the vector is SIMD aligned.
                const IS_ALIGNED: bool;
            }}

            /// See [`Vector`] for information.
            pub struct VecAligned;

            /// See [`Vector`] for information.
            pub struct VecPacked;
        "#},
        formatdoc! {r#"
            impl VecAlignment for VecAligned {{
                type InnerVector<const N: usize, T: Scalar>
                    = <Usize<N> as VecLen>::InnerAlignedVector<T>
                where
                    Usize<N>: VecLen;

                const IS_ALIGNED: bool = true;
            }}

            impl VecAlignment for VecPacked {{
                type InnerVector<const N: usize, T: Scalar>
                    = [T; N]
                where
                    Usize<N>: VecLen;
                    
                const IS_ALIGNED: bool = false;
            }}
        "#},
    )
}
