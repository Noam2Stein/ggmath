use indoc::formatdoc;

use super::*;

mod construct;
mod convert;

pub fn write_mod(mut module: ModDir) {
    construct::write_mod(module.submod("construct"));
    convert::write_mod(module.submod("convert"));

    let type_aliases = LENGTHS
        .map(|len| {
            let aabb_n = match len {
                2 => "Rect".to_string(),
                _ => format!("Aabb{len}"),
            };

            formatdoc! {r#"
                /// Type alias to [`Aabb<{len}, T, VecAligned, AabbCornered>`].
                pub type {aabb_n}<T> = Aabb<{len}, T, VecAligned, AabbCornered>;

                /// Type alias to [`Aabb<{len}, T, VecAligned, AabbCentered>`].
                pub type {aabb_n}C<T> = Aabb<{len}, T, VecAligned, AabbCentered>;

                /// Type alias to [`Aabb<{len}, T, VecAligned, AabbMinMaxed>`].
                pub type {aabb_n}M<T> = Aabb<{len}, T, VecAligned, AabbMinMaxed>;

                /// Type alias to [`Aabb<{len}, T, VecPacked, AabbCornered>`].
                pub type {aabb_n}P<T> = Aabb<{len}, T, VecPacked, AabbCornered>;

                /// Type alias to [`Aabb<{len}, T, VecPacked, AabbCentered>`].
                pub type {aabb_n}CP<T> = Aabb<{len}, T, VecPacked, AabbCentered>;

                /// Type alias to [`Aabb<{len}, T, VecPacked, AabbMinMaxed>`].
                pub type {aabb_n}MP<T> = Aabb<{len}, T, VecPacked, AabbMinMaxed>;
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    writedoc!(
        module,
        r#"
        //! Module for the aabb type.

        use std::{{
            fmt::{{Debug, Display}},
            hash::{{Hash, Hasher}},
            ops::*,
        }};

        use super::*;

        mod convert;
        mod intersect;
        mod construct;
        mod properties;

        /// Trait required to put a type inside a `Aabb`.
        ///
        /// This trait contains simple arithmetic operations that are used by the `Aabb` type,
        /// like doubling or halving a vector.
        pub trait AabbScalar:
            Scalar + Add<Output = Self> + Sub<Output = Self> + PartialEq + PartialOrd
        {{
            /// Return `vec * 2`.
            /// Used by aabb functions.
            fn aabb_vec_double<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen;

            /// Return `vec / 2`.
            /// Used by aabb functions.
            ///
            /// For ints this should FLOOR the output.
            fn aabb_vec_half<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen;

            /// Maps the vectors to the absolute difference of their components.
            /// Used by aabb functions.
            fn aabb_vec_abs_diff<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                rhs: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen;

            /// Maps the vectors to the minimum of their components.
            /// Used by aabb functions.
            fn aabb_vec_min<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen;

            /// Maps the vectors to the maximum of their components.
            /// Used by aabb functions.
            fn aabb_vec_max<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen;
        }}

        /// Marker trait that marks the inner representation of an [`Aabb`].
        pub unsafe trait AabbRepr: Sized + 'static {{
            const ANCHOR_A: AabbAnchorEnum;
            const ANCHOR_B: AabbAnchorEnum;
        }}

        /// Marks an [`Aabb`] as being represented by its minimum corner.
        pub struct AabbMin;

        /// Marks an [`Aabb`] as being represented by its maximum corner.
        pub struct AabbMax;

        /// Marks an [`Aabb`] as being represented by its center.
        pub struct AabbCenter;

        /// Marks an [`Aabb`] as being represented by its size.
        pub struct AabbSize;

        /// Marks an [`Aabb`] as being represented by its extents.
        pub struct AabbExtents;

        /// Enum that marks the inner representation of an [`Aabb`].
        pub enum AabbAnchorEnum {{
            Min,
            Max,
            Center,
            Size,
            Extents,
        }}

        unsafe impl AabbRepr for (AabbMin, AabbMax) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Min;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Max;
        }}
        unsafe impl AabbRepr for (AabbMin, AabbCenter) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Min;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Center;
        }}
        unsafe impl AabbRepr for (AabbMin, AabbSize) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Min;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Size;
        }}
        unsafe impl AabbRepr for (AabbMin, AabbExtents) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Min;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Extents;
        }}

        unsafe impl AabbRepr for (AabbMax, AabbMin) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Max;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Min;
        }}
        unsafe impl AabbRepr for (AabbMax, AabbCenter) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Max;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Center;
        }}
        unsafe impl AabbRepr for (AabbMax, AabbSize) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Max;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Size;
        }}
        unsafe impl AabbRepr for (AabbMax, AabbExtents) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Max;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Extents;
        }}

        unsafe impl AabbRepr for (AabbCenter, AabbMin) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Center;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Min;
        }}
        unsafe impl AabbRepr for (AabbCenter, AabbMax) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Center;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Max;
        }}
        unsafe impl AabbRepr for (AabbCenter, AabbSize) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Center;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Size;
        }}
        unsafe impl AabbRepr for (AabbCenter, AabbExtents) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Center;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Extents;
        }}

        unsafe impl AabbRepr for (AabbSize, AabbMin) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Size;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Min;
        }}
        unsafe impl AabbRepr for (AabbSize, AabbMax) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Size;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Max;
        }}
        unsafe impl AabbRepr for (AabbSize, AabbCenter) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Size;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Center;
        }}
        
        unsafe impl AabbRepr for (AabbExtents, AabbMin) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Extents;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Min;
        }}
        unsafe impl AabbRepr for (AabbExtents, AabbMax) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Extents;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Max;
        }}
        unsafe impl AabbRepr for (AabbExtents, AabbCenter) {{
            const ANCHOR_A: AabbAnchorEnum = AabbAnchorEnum::Extents;
            const ANCHOR_B: AabbAnchorEnum = AabbAnchorEnum::Center;
        }}

        /// Axis-aligned bounding box.
        #[repr(transparent)]
        pub struct Aabb<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr>
        where
            Usize<N>: VecLen,
        {{
            anchor0: Vector<N, T, A>,
            anchor1: Vector<N, T, A>,
        }}

        {type_aliases}

        impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Clone for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{
            fn clone(&self) -> Self {{
                *self
            }}
        }}
        impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Copy for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{}}

        impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> PartialEq for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{
            fn eq(&self, other: &Self) -> bool {{
                self.anchor0 == other.anchor0 && self.anchor1 == other.anchor1
            }}
        }}
        impl<const N: usize, T: AabbScalar + Eq, A: VecAlignment, R: AabbRepr> Eq for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{}}

        impl<const N: usize, T: AabbScalar + Hash, A: VecAlignment, R: AabbRepr> Hash for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{
            fn hash<H: Hasher>(&self, state: &mut H) {{
                self.anchor0.hash(state);
                self.anchor1.hash(state);
            }}
        }}

        impl<const N: usize, T: AabbScalar + Debug, A: VecAlignment, R: AabbRepr> Debug for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(
                    f,
                    "Aabb {{{{ {{}}: {{:?}}, {{}}: {{:?}} }}}}",
                    match R::ANCHOR_A {{
                        AabbAnchorEnum::Min => "min",
                        AabbAnchorEnum::Max => "max",
                        AabbAnchorEnum::Center => "center",
                        AabbAnchorEnum::Size => "size",
                        AabbAnchorEnum::Extents => "extents",
                    }},
                    self.anchor0,
                    match R::ANCHOR_B {{
                        AabbAnchorEnum::Min => "min",
                        AabbAnchorEnum::Max => "max",
                        AabbAnchorEnum::Center => "center",
                        AabbAnchorEnum::Size => "size",
                        AabbAnchorEnum::Extents => "extents",
                    }},
                    self.anchor1,
                )
            }}
        }}

        impl<const N: usize, T: AabbScalar + Display, A: VecAlignment, R: AabbRepr> Display for Aabb<N, T, A, R>
        where
            Usize<N>: VecLen,
        {{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                write!(
                    f,
                    "Aabb {{{{ {{}}: {{}}, {{}}: {{}} }}}}",
                    match R::ANCHOR_A {{
                        AabbAnchorEnum::Min => "min",
                        AabbAnchorEnum::Max => "max",
                        AabbAnchorEnum::Center => "center",
                        AabbAnchorEnum::Size => "size",
                        AabbAnchorEnum::Extents => "extents",
                    }},
                    self.anchor0,
                    match R::ANCHOR_B {{
                        AabbAnchorEnum::Min => "min",
                        AabbAnchorEnum::Max => "max",
                        AabbAnchorEnum::Center => "center",
                        AabbAnchorEnum::Size => "size",
                        AabbAnchorEnum::Extents => "extents",
                    }},
                    self.anchor1,
                )
            }}
        }}
        "#,
    )
    .unwrap();
}
