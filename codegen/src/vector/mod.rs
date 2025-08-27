use std::fmt::Display;

use indoc::{formatdoc, indoc};

use super::*;

mod cmp;
mod construct;
mod convert;
mod index;
mod iter;
mod ops;
mod splat;
mod swizzle;

#[derive(Debug)]
struct ScalarTrait {
    overridable_fns: String,
}

pub fn write_mod(mut module: ModDir) {
    let mut scalar_trait = ScalarTrait::new();

    cmp::write_mod(module.submod("cmp"), &mut scalar_trait);
    ops::write_mod(module.submod("ops"), &mut scalar_trait);
    splat::write_mod(module.submod("splat"));
    construct::write_mod(module.submod("construct"));
    convert::write_mod(module.submod("convert"));
    index::write_mod(module.submod("index"));
    iter::write_mod(module.submod("iter"));
    swizzle::write_mod(module.submod("swizzle"));

    let length_list = LENGTHS
        .into_iter()
        .take(LENGTHS.count() - 1)
        .map(|len| format!("`{len}`"))
        .collect::<Vec<_>>()
        .join(", ")
        + format!(" and `{}`", LENGTHS.last().unwrap()).as_str();

    let length_impls = LENGTHS
        .map(|len| {
            formatdoc! {"
            unsafe impl VecLen for Usize<{len}> {{
                type InnerVecA<T: Scalar> = T::InnerVec{len}A;
            }}
        "}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let type_aliases = LENGTHS
        .map(|len| {
            formatdoc! {r#"
                /// Type alias to `Vector<{len}, T, VecAligned>`.
                ///
                /// This type is a vector{len} that is aligned for SIMD instructions and is considered the default vector{len} type.
                /// For an unaligned vector{len} that follows the memory layout of `[T; {len}]`, see [`Vec{len}P`].
                #[cfg(feature = "aliases")]
                pub type Vec{len}<T> = Vector<{len}, T, VecAligned>;

                /// Type alias to `Vector<{len}, T, VecPacked>`.
                ///
                /// This type is a vector{len} that follows the memory layout of `[T; {len}]`.
                /// As a result it has slower operations than [`Vec{len}`] but saves space.
                #[cfg(feature = "aliases")]
                pub type Vec{len}P<T> = Vector<{len}, T, VecPacked>;
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let macro_aliases = LENGTHS
        .map(|len| {
            formatdoc! {r#"
            #[doc = "Type alias to `Vec{len}<" $type ">`"]
            $($vis)* type [<$prefix Vec{len}>] = $crate::Vec{len}<$type>;

            #[doc = "Type alias to `Vec{len}P<" $type ">`"]
            $($vis)* type [<$prefix Vec{len}P>] = $crate::Vec{len}P<$type>;
        "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let impl_hash = impl_hash(&mut scalar_trait);
    let impl_default = impl_default(&mut scalar_trait);
    let impl_fmt = impl_fmt();

    writedoc!(
        module,
        r#"
        //! Module for the vector type.
        
        use std::{{
            fmt::{{Display, Debug}},
            hash::{{Hash, Hasher}},
            ops::*,
        }};

        use super::*;

        mod cmp;
        mod convert;
        mod swizzle;
        mod construct;
        mod ops;
        mod index;
        mod iter;
        mod splat;
        pub use splat::*;
        pub use construct::*;

        /// Marks a [`Usize`] type as a valid length for a vector.
        /// This trait is currently implemented for {length_list}.
        ///
        /// Note that more lengths like `5` could be made valid and it would NOT be a breaking change.
        pub unsafe trait VecLen {{
            /// Specifies the inner memory representation of an aligned vector.
            ///
            /// This currently always returns one of `T`'s inner vector types.
            type InnerVecA<T: Scalar>: Construct;
        }}

        {length_impls}

        {scalar_trait}

        /// Sealed marker trait that marks vectors as either `VecAligned` or `VecPacked`.
        ///
        /// Marking a vector as `VecAligned` via `Vector<_, _, VecAligned>`,
        /// means the vector will have additional memory alignment to `T` if it improves performance,
        /// like with SIMD instructions.
        ///
        /// Marking a vector as `VecPacked` via `Vector<_, _, VecPacked>`,
        /// means the vector will not have additional memory alignment to `T`,
        /// and will be identical in memory to `[T; N]`.
        ///
        /// The `Vec{{N}}` type aliases like `Vec2` are for aligned vectors.
        /// The `Vec{{N}}P` type aliases like `Vec2P` are for packed vectors.
        pub unsafe trait VecAlignment: Sized + 'static + Send + Sync {{
            /// Can be used to branch code based on the generic alignment of a vector.
            const IS_ALIGNED: bool;

            /// The inner memory representation of vectors.
            type InnerVector<const N: usize, T: Scalar>: Construct
            where
                Usize<N>: VecLen;
        }}

        /// Vectors can be marked as either `VecAligned` or `VecPacked`.
        ///
        /// Marking a vector as `VecAligned` via `Vector<_, _, VecAligned>`,
        /// means the vector will have additional memory alignment to `T` if it improves performance,
        /// like with SIMD instructions.
        ///
        /// Marking a vector as `VecPacked` via `Vector<_, _, VecPacked>`,
        /// means the vector will not have additional memory alignment to `T`,
        /// and will be identical in memory to `[T; N]`.
        pub struct VecAligned;

        /// Vectors can be marked as either `VecAligned` or `VecPacked`.
        ///
        /// Marking a vector as `VecAligned` via `Vector<_, _, VecAligned>`,
        /// means the vector will have additional memory alignment to `T` if it improves performance,
        /// like with SIMD instructions.
        ///
        /// Marking a vector as `VecPacked` via `Vector<_, _, VecPacked>`,
        /// means the vector will not have additional memory alignment to `T`,
        /// and will be identical in memory to `[T; N]`.
        pub struct VecPacked;

        unsafe impl VecAlignment for VecAligned {{
            const IS_ALIGNED: bool = true;

            type InnerVector<const N: usize, T: Scalar>
                = <Usize<N> as VecLen>::InnerVecA<T>
            where
                Usize<N>: VecLen;
        }}

        unsafe impl VecAlignment for VecPacked {{
            const IS_ALIGNED: bool = false;

            type InnerVector<const N: usize, T: Scalar>
                = [T; N]
            where
                Usize<N>: VecLen;
        }}

        /// The vector type.
        ///
        /// This type has alot of generics, and most of the time type aliases should be used instead of this type.
        /// See [`Vec2`], [`Vec3`] and [`Vec4`].
        ///
        /// This type is generic over three parameters:
        /// - `N`: The length of the vector. Can be 2, 3 or 4.
        /// - `T`: The scalar type of the vector. Must implement [`Scalar`].
        /// - `A`: The alignment of the vector. Specifies if the vector is aligned for SIMD, or unaligned (saves space).
        #[repr(transparent)]
        pub struct Vector<const N: usize, T: Scalar, A: VecAlignment>
        where
            Usize<N>: VecLen,
        {{
            pub inner: A::InnerVector<N, T>,
        }}

        {type_aliases}

        /// Expands to a declaration of type specific vector aliases.
        ///
        /// Syntax:
        /// `<vis> <prefix> => <type>`
        ///
        /// Example:
        /// ```rust
        /// use ggmath::*;
        ///
        /// // Declare a `Scalar` type.
        /// type BigInt = i128;
        ///
        /// vector_aliases!(pub Big => BigInt);
        /// ```
        ///
        /// Expands to:
        /// ```rust
        /// use ggmath::*;
        ///
        /// // Declare a `Scalar` type.
        /// type BigInt = i128;
        ///
        /// pub type BigVec2 = Vec2<BigInt>;
        /// pub type BigVec3 = Vec3<BigInt>;
        /// pub type BigVec4 = Vec4<BigInt>;
        ///
        /// pub type BigVec2P = Vec2P<BigInt>;
        /// pub type BigVec3P = Vec3P<BigInt>;
        /// pub type BigVec4P = Vec4P<BigInt>;
        /// ```
        #[cfg(feature = "vector")]
        #[macro_export]
        macro_rules! vector_aliases {{
            (pub($($vis:tt)*) $prefix:ident => $type:ident) => {{
                $crate::vector_aliases! {{ @(pub($($vis)*)) $prefix => $type }}
            }};
            (pub $prefix:ident => $type:ident) => {{
                $crate::vector_aliases! {{ @(pub) $prefix => $type }}
            }};
            ($prefix:ident => $type:ident) => {{
                $crate::vector_aliases! {{ @() $prefix => $type }}
            }};

            (@($($vis:tt)*) $prefix:ident => $type:ident) => {{
                $crate::_hidden_::paste! {{
                    {macro_aliases}
                }}
            }};
        }}

        impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn clone(&self) -> Self {{
                *self
            }}
        }}
        impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{}}

        {impl_hash}

        {impl_default}

        {impl_fmt}

        #[cfg(test)]
        mod tests {{
            use super::*;

            // verify size
            const _: () = assert!(size_of::<Vector<2, f32, VecAligned>>() >= 8);
            const _: () = assert!(size_of::<Vector<3, f32, VecAligned>>() >= 12);
            const _: () = assert!(size_of::<Vector<4, f32, VecAligned>>() >= 16);
            const _: () = assert!(size_of::<Vector<2, f32, VecPacked>>() == 8);
            const _: () = assert!(size_of::<Vector<3, f32, VecPacked>>() == 12);
            const _: () = assert!(size_of::<Vector<4, f32, VecPacked>>() == 16);

            const _: () = assert!(size_of::<Vector<2, bool, VecAligned>>() >= 2);
            const _: () = assert!(size_of::<Vector<3, bool, VecAligned>>() >= 3);
            const _: () = assert!(size_of::<Vector<4, bool, VecAligned>>() >= 4);
            const _: () = assert!(size_of::<Vector<2, bool, VecPacked>>() == 2);
            const _: () = assert!(size_of::<Vector<3, bool, VecPacked>>() == 3);
            const _: () = assert!(size_of::<Vector<4, bool, VecPacked>>() == 4);

            // verify alignment
            const _: () = assert!(align_of::<Vector<2, f32, VecAligned>>() >= align_of::<f32>());
            const _: () = assert!(align_of::<Vector<3, f32, VecAligned>>() >= align_of::<f32>());
            const _: () = assert!(align_of::<Vector<4, f32, VecAligned>>() >= align_of::<f32>());
            const _: () = assert!(align_of::<Vector<2, f32, VecPacked>>() == align_of::<f32>());
            const _: () = assert!(align_of::<Vector<3, f32, VecPacked>>() == align_of::<f32>());
            const _: () = assert!(align_of::<Vector<4, f32, VecPacked>>() == align_of::<f32>());

            const _: () = assert!(align_of::<Vector<2, bool, VecAligned>>() >= align_of::<bool>());
            const _: () = assert!(align_of::<Vector<3, bool, VecAligned>>() >= align_of::<bool>());
            const _: () = assert!(align_of::<Vector<4, bool, VecAligned>>() >= align_of::<bool>());
            const _: () = assert!(align_of::<Vector<2, bool, VecPacked>>() == align_of::<bool>());
            const _: () = assert!(align_of::<Vector<3, bool, VecPacked>>() == align_of::<bool>());
            const _: () = assert!(align_of::<Vector<4, bool, VecPacked>>() == align_of::<bool>());

            // verify Construct
            fn _verify_vector_construct<const N: usize, T: Scalar, A: VecAlignment>()
            where
                Usize<N>: VecLen,
            {{
                fn _verify_construct<T: Construct>() {{}}

                _verify_construct::<Vector<N, T, A>>();
            }}
        }}
        "#,
        length_list = length_list.trim(),
        length_impls = length_impls.trim(),
        scalar_trait = scalar_trait.to_string().trim(),
        type_aliases = type_aliases.trim(),
        macro_aliases = macro_aliases.trim().replace("\n", "\n\t\t\t"),
    )
    .unwrap();
}

impl ScalarTrait {
    fn new() -> Self {
        Self {
            overridable_fns: String::new(),
        }
    }

    fn push_overridable_fn(&mut self, name: &str, content: impl std::fmt::Display) {
        writedoc!(
            self.overridable_fns,
            "
            /// Overridable implementation of [`Vector::{name}`].
            {content}
            "
        )
        .unwrap();
    }
}

impl Display for ScalarTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner_vecs = LENGTHS
            .map(|len| {
                formatdoc! {"
                    /// Specifies the inner memory representation of an aligned vector{len}.
                    /// This type's reference must be a valid `&[T; {len}]`.
                    ///
                    /// This type is useful to add extra memory alignment to vectors for use in SIMD instructions.
                    /// If `Self` does not benifit from SIMD or additional memory alignment, use `[Self; {len}]` to minimize size.
                    type InnerVec{len}A: Construct;
                "}
            })
            .collect::<Vec<_>>()
            .join("\n");

        let inner_vec_garbage = LENGTHS
            .map(|len| {
                formatdoc! {"
                /// Specifies a valid value of the `InnerVec{len}A` type.
                ///
                /// This is used when initializing vectors to make sure padding is initialized.
                const INNER_VEC{len}A_GARBAGE: Self::InnerVec{len}A;
            "}
            })
            .collect::<Vec<_>>()
            .join("\n");

        writedoc!(
            f,
            "
            /// Allows types to be put inside math-types like `Vector` and `Matrix`.
            /// For example: `f32`, `u8` and `bool` are scalars.
            ///
            /// All scalar types are `Copy` and some more basic traits like `Send` and `Sync`.
            pub trait Scalar: Construct {{
                {inner_vecs}

                {inner_vec_garbage}

                {overridable_fns}
            }}
            ",
            inner_vecs = inner_vecs.replace("\n", "\n\t").trim(),
            inner_vec_garbage = inner_vec_garbage.replace("\n", "\n\t").trim(),
            overridable_fns = self.overridable_fns.replace("\n", "\n\t").trim(),
        )
    }
}

fn impl_hash(scalar_trait: &mut ScalarTrait) -> String {
    scalar_trait.push_overridable_fn(
        "hash",
        formatdoc! {r#"
        #[inline(always)]
        fn vec_hash<const N: usize, A: VecAlignment, H: std::hash::Hasher>(
            vec: &Vector<N, Self, A>,
            state: &mut H,
        )
        where
            Usize<N>: VecLen,
            Self: std::hash::Hash,
        {{
            vec.as_array_ref().hash(state);
        }}
        "#},
    );

    formatdoc! {r#"
        impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {{
                T::vec_hash(self, state);
            }}
        }}
    "#}
}

fn impl_default(scalar_trait: &mut ScalarTrait) -> String {
    scalar_trait.push_overridable_fn(
        "default",
        indoc! {"
            #[inline(always)]
            fn vec_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
                Self: Default,
            {
                Vector::splat(Self::default())
            }
        "},
    );

    formatdoc! {r#"
        impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn default() -> Self {{
                T::vec_default()
            }}
        }}
    "#}
}

fn impl_fmt() -> String {
    formatdoc! {r#"
        impl<const N: usize, T: Scalar + Display, A: VecAlignment> Display for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(f, "(")?;

                for item in &self.as_array_ref()[..N - 1] {{
                    write!(f, "{{item}}, ")?;
                }}
                write!(f, "{{}}", self.as_array_ref()[N - 1])?;

                write!(f, ")")?;

                Ok(())
            }}
        }}

        impl<const N: usize, T: Scalar + Debug, A: VecAlignment> Debug for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(f, "(")?;

                for item in &self.as_array_ref()[..N - 1] {{
                    write!(f, "{{item:?}}, ")?;
                }}
                write!(f, "{{:?}}", self.as_array_ref()[N - 1])?;

                write!(f, ")")?;

                Ok(())
            }}
        }}
    "#}
}
