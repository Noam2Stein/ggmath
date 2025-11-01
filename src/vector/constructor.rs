use crate::{Scalar, Simdness, SupportedVecLen, VecLen, Vector};

////////////////////////////////////////////////////////////////////////////////
// Constructor Macros
////////////////////////////////////////////////////////////////////////////////

declare_constructor_macro! {
    /// Creates a [`Vec2`](crate::Vec2) from two elements.
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly two
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec2, vec2};
    ///
    /// let v: Vec2<f32> = vec2!(1.0, 2.0);
    /// let v: Vec2<f32> = vec2!(vec2!(1.0, 2.0));
    /// let v: Vec2<f32> = vec2!(1.0);
    /// ```
    vec2 => Vector<2, _, Simd>
}

declare_constructor_macro! {
    /// Creates a [`Vec3`](crate::Vec3) from three elements.
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly three
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec3, vec2, vec3};
    ///
    /// let v: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    /// let v: Vec3<f32> = vec3!(vec2!(1.0, 2.0), 3.0);
    /// let v: Vec3<f32> = vec3!(1.0);
    /// ```
    vec3 => Vector<3, _, Simd>
}

declare_constructor_macro! {
    /// Creates a [`Vec4`](crate::Vec4) from four elements.
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly four
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec4, vec2, vec4};
    ///
    /// let v: Vec4<f32> = vec4!(1.0, 2.0, 3.0, 4.0);
    /// let v: Vec4<f32> = vec4!(1.0, vec2!(2.0, 3.0), 4.0);
    /// let v: Vec4<f32> = vec4!(1.0);
    /// ```
    vec4 => Vector<4, _, Simd>
}

declare_constructor_macro! {
    /// Creates a [`Vec2S`](crate::Vec2S) from two elements.
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly two
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec2S, vec2s};
    ///
    /// let v: Vec2S<f32> = vec2s!(1.0, 2.0);
    /// let v: Vec2S<f32> = vec2s!(vec2s!(1.0, 2.0));
    /// let v: Vec2S<f32> = vec2s!(1.0);
    /// ```
    vec2s => Vector<2, _, NonSimd>
}

declare_constructor_macro! {
    /// Creates a [`Vec3S`](crate::Vec3S) from three elements.
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly three
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec3S, vec2s, vec3s};
    ///
    /// let v: Vec3S<f32> = vec3s!(1.0, 2.0, 3.0);
    /// let v: Vec3S<f32> = vec3s!(vec2s!(1.0, 2.0), 3.0);
    /// let v: Vec3S<f32> = vec3s!(1.0);
    /// ```
    vec3s => Vector<3, _, NonSimd>
}

declare_constructor_macro! {
    /// Creates a [`Vec4S`](crate::Vec4S) from four elements.
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly four
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec4S, vec2s, vec4s};
    ///
    /// let v: Vec4S<f32> = vec4s!(1.0, 2.0, 3.0, 4.0);
    /// let v: Vec4S<f32> = vec4s!(1.0, vec2s!(2.0, 3.0), 4.0);
    /// let v: Vec4S<f32> = vec4s!(1.0);
    /// ```
    vec4s => Vector<4, _, NonSimd>
}

declare_constructor_macro! {
    /// Creates a `vector2` from two elements, where type inference determines if it
    /// is [`Simd`] or [`NonSimd`].
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly two
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec2, Vec2S, vec2g};
    ///
    /// let v: Vec2<f32> = vec2g!(1.0, 2.0);
    /// let v: Vec2<f32> = vec2g!(vec2g!(1.0, 2.0));
    /// let v: Vec2<f32> = vec2g!(1.0);
    ///
    /// let v: Vec2S<f32> = vec2g!(1.0, 2.0);
    /// let v: Vec2S<f32> = vec2g!(vec2g!(1.0, 2.0));
    /// let v: Vec2S<f32> = vec2g!(1.0);
    /// ```
    vec2g => Vector<2, _, _>
}

declare_constructor_macro! {
    /// Creates a `vector3` from three elements, where type inference determines if it
    /// is [`Simd`] or [`NonSimd`].
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly three
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec3, Vec3S, vec2g, vec3g};
    ///
    /// let v: Vec3<f32> = vec3g!(1.0, 2.0, 3.0);
    /// let v: Vec3<f32> = vec3g!(vec2g!(1.0, 2.0), 3.0);
    /// let v: Vec3<f32> = vec3g!(1.0);
    ///
    /// let v: Vec3S<f32> = vec3g!(1.0, 2.0, 3.0);
    /// let v: Vec3S<f32> = vec3g!(vec2g!(1.0, 2.0), 3.0);
    /// let v: Vec3S<f32> = vec3g!(1.0);
    /// ```
    vec3g => Vector<3, _, _>
}

declare_constructor_macro! {
    /// Creates a `vector4` from four elements, where type inference determines if it
    /// is [`Simd`] or [`NonSimd`].
    ///
    /// Also accepts vectors as arguments, as long as they contain exactly four
    /// elements in total. If only a single element is provided, it will be
    /// repeated across all elements.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec4, Vec4S, vec2g, vec4g};
    ///
    /// let v: Vec4<f32> = vec4g!(1.0, 2.0, 3.0, 4.0);
    /// let v: Vec4<f32> = vec4g!(1.0, vec2g!(2.0, 3.0), 4.0);
    /// let v: Vec4<f32> = vec4g!(1.0);
    ///
    /// let v: Vec4S<f32> = vec4g!(1.0, 2.0, 3.0, 4.0);
    /// let v: Vec4S<f32> = vec4g!(1.0, vec2g!(2.0, 3.0), 4.0);
    /// let v: Vec4S<f32> = vec4g!(1.0);
    /// ```
    vec4g => Vector<4, _, _>
}

macro_rules! declare_constructor_macro {
    { $(#[$meta:meta])* $name:ident => Vector<$N:expr, _, $S:ident> } => {
        declare_constructor_macro! { @core $i $(#[$meta])* $name => Vector<$N, _, $crate::$S> }
    };

    { $(#[$meta:meta])* $name:ident => Vector<$N:expr, _, _> } => {
        declare_constructor_macro! { @core $i $(#[$meta])* $name => Vector<$N, _, _> }
    };

    { @core $dollar:tt i $(#[$meta:meta])* $name:ident => Vector<$N:expr, _, $S:ty> } => {
        $(#[$meta])*
        #[macro_export]
        macro_rules! $name {
            ($dollar($dollar arg:expr),* $dollar (,)?) => {
                $crate::Vector::<$N, _, $S>::from(($dollar ($dollar arg,)*))
            }
        }

        pub use $name;
    };
}

use declare_constructor_macro;

////////////////////////////////////////////////////////////////////////////////
// Generic Length Constructor
////////////////////////////////////////////////////////////////////////////////

impl<const N: usize, T: Scalar, S: Simdness> From<(Vector<N, T, S>,)> for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(value: (Vector<N, T, S>,)) -> Self {
        value.0
    }
}

impl<const N: usize, T: Scalar, S: Simdness> From<(T,)> for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn from(value: (T,)) -> Self {
        Vector::splat(value.0)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector2 Constructor
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar, S: Simdness> From<(T, T)> for Vector<2, T, S> {
    #[inline(always)]
    fn from(value: (T, T)) -> Self {
        Vector::from_array([value.0, value.1])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3 Constructor
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar, S: Simdness> From<(T, T, T)> for Vector<3, T, S> {
    #[inline(always)]
    fn from(value: (T, T, T)) -> Self {
        Vector::from_array([value.0, value.1, value.2])
    }
}

impl<T: Scalar, S: Simdness> From<(T, Vector<2, T, S>)> for Vector<3, T, S> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, S>)) -> Self {
        Vector::from_array([value.0, value.1[0], value.1[1]])
    }
}

impl<T: Scalar, S: Simdness> From<(Vector<2, T, S>, T)> for Vector<3, T, S> {
    #[inline(always)]
    fn from(value: (Vector<2, T, S>, T)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.1])
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector4 Constructor
////////////////////////////////////////////////////////////////////////////////

impl<T: Scalar, S: Simdness> From<(T, T, T, T)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (T, T, T, T)) -> Self {
        Vector::from_array([value.0, value.1, value.2, value.3])
    }
}

impl<T: Scalar, S: Simdness> From<(T, T, Vector<2, T, S>)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (T, T, Vector<2, T, S>)) -> Self {
        Vector::from_array([value.0, value.1, value.2[0], value.2[1]])
    }
}

impl<T: Scalar, S: Simdness> From<(T, Vector<2, T, S>, T)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (T, Vector<2, T, S>, T)) -> Self {
        Vector::from_array([value.0, value.1[0], value.1[1], value.2])
    }
}

impl<T: Scalar, S: Simdness> From<(T, Vector<3, T, S>)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (T, Vector<3, T, S>)) -> Self {
        Vector::from_array([value.0, value.1[0], value.1[1], value.1[2]])
    }
}

impl<T: Scalar, S: Simdness> From<(Vector<2, T, S>, T, T)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (Vector<2, T, S>, T, T)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.1, value.2])
    }
}

impl<T: Scalar, S: Simdness> From<(Vector<2, T, S>, Vector<2, T, S>)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (Vector<2, T, S>, Vector<2, T, S>)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.1[0], value.1[1]])
    }
}

impl<T: Scalar, S: Simdness> From<(Vector<3, T, S>, T)> for Vector<4, T, S> {
    #[inline(always)]
    fn from(value: (Vector<3, T, S>, T)) -> Self {
        Vector::from_array([value.0[0], value.0[1], value.0[2], value.1])
    }
}
