//! Vector related items.

use core::{
    fmt::{Debug, Display},
    mem::{MaybeUninit, transmute, transmute_copy},
    ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use crate::{Construct, sealed::Sealed};

mod constructor;
mod deref;
mod dir;
mod ops;
mod primitive_api;
mod primitive_impls;
pub use constructor::*;
pub use dir::*;

#[cfg(feature = "swizzle")]
mod swizzle;

/// A generic vector type.
///
/// This type is parameterized over:
/// - `N`: The number of elements in the vector.
/// - `T`: The element type.
/// - `S`: Whether the vector uses SIMD acceleration.
///
/// ## Type Aliases
///
/// This type provides the following convenient aliases:
/// - `Vec{N}<T>` — shorthand for [`Vector<N, T, Simd>`], e.g. [`Vec2<f32>`] =
///   [`Vector<2, f32, Simd>`].
/// - `Vec{N}S<T>` — shorthand for [`Vector<N, T, NonSimd>`], e.g. [`Vec2S<f32>`] =
///   [`Vector<2, f32, NonSimd>`].
///
/// ## SIMD Acceleration
///
/// The `S` parameter determines whether the vector uses SIMD.  
/// It can be either [`Simd`] or [`NonSimd`].
///
/// You might wonder: if SIMD makes operations faster, why disable it?  
///
/// While SIMD provides significant performance benefits, SIMD-backed vectors also
/// require higher memory alignment. Using [`NonSimd`] can be beneficial when
/// minimizing memory usage is more important than maximizing performance.
///
/// The implementation of [`Simd`] vectors is controlled by the [`SimdBehaviour`] trait,
/// which is implemented for `T`. Each implementation can choose how to store and
/// operate on the vector, and may use SIMD instructions when doing so improves
/// performance.
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, S: Simdness>(<Self as VectorReprExt>::Repr)
where
    VecLen<N>: SupportedVecLen;

impl<const N: usize, T: Scalar, S: Simdness> Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    specialized_vector_api! {
        VectorApi for <N, T, S>:

        /// Creates a vector from an array.
        pub fn from_array(array: [T; N]) -> Self;

        /// Creates a vector with all elements set to the same value.
        pub fn splat(value: T) -> Self;

        /// Creates a vector2 from `(self[X_SRC], self[Y_SRC])`.
        ///
        /// ## Examples
        ///
        /// ```
        /// use ggmath::{Vec4, vec2, vec4};
        ///
        /// let v: Vec4<f32> = vec4!(1.0, 2.0, 3.0, 4.0);
        ///
        /// assert_eq!(v.swizzle2::<0, 1>(), vec2!(1.0, 2.0));
        /// assert_eq!(v.swizzle2::<2, 3>(), vec2!(3.0, 4.0));
        /// assert_eq!(v.swizzle2::<3, 0>(), vec2!(4.0, 1.0));
        ///
        /// // Elements can be repeated
        /// assert_eq!(v.swizzle2::<1, 1>(), vec2!(2.0, 2.0));
        /// ```
        pub fn swizzle2<const X_SRC: usize, const Y_SRC: usize>(self) -> Vector<2, T, S>;

        /// Creates a vector3 from `(self[X_SRC], self[Y_SRC], self[Z_SRC])`.
        ///
        /// ## Examples
        ///
        /// ```
        /// use ggmath::{Vec4, vec3, vec4};
        ///
        /// let v: Vec4<f32> = vec4!(1.0, 2.0, 3.0, 4.0);
        ///
        /// assert_eq!(v.swizzle3::<0, 1, 2>(), vec3!(1.0, 2.0, 3.0));
        /// assert_eq!(v.swizzle3::<2, 1, 0>(), vec3!(3.0, 2.0, 1.0));
        /// assert_eq!(v.swizzle3::<3, 0, 1>(), vec3!(4.0, 1.0, 2.0));
        ///
        /// // Elements can be repeated
        /// assert_eq!(v.swizzle3::<1, 1, 1>(), vec3!(2.0, 2.0, 2.0));
        /// ```
        pub fn swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(self) -> Vector<3, T, S>;

        /// Creates a vector4 from `(self[X_SRC], self[Y_SRC], self[Z_SRC], self[W_SRC])`.
        ///
        /// ## Examples
        ///
        /// ```
        /// use ggmath::{Vec4, vec4};
        ///
        /// let v: Vec4<f32> = vec4!(1.0, 2.0, 3.0, 4.0);
        ///
        /// assert_eq!(v.swizzle4::<0, 1, 2, 3>(), vec4!(1.0, 2.0, 3.0, 4.0));
        /// assert_eq!(v.swizzle4::<3, 2, 1, 0>(), vec4!(4.0, 3.0, 2.0, 1.0));
        /// assert_eq!(v.swizzle4::<3, 0, 1, 2>(), vec4!(4.0, 1.0, 2.0, 3.0));
        ///
        /// // Elements can be repeated
        /// assert_eq!(v.swizzle4::<1, 1, 1, 1>(), vec4!(2.0, 2.0, 2.0, 2.0));
        /// ```
        pub fn swizzle4<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize, const W_SRC: usize>(self) -> Vector<4, T, S>;
    }

    /// Creates a vector where each element is produced by calling `f` with that
    /// element's index.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec4, vec4};
    ///
    /// let v: Vec4<f32> = Vec4::from_fn(|i| i as f32);
    /// assert_eq!(v, vec4!(0.0, 1.0, 2.0, 3.0));
    /// ```
    #[inline(always)]
    pub fn from_fn(f: impl FnMut(usize) -> T) -> Self {
        Vector::from_array(core::array::from_fn(f))
    }

    /// Creates a vector from an array, *in a const context*.
    ///
    /// [`Vector::from_array`] performs the same operation but cannot be used in const
    /// contexts. This function is intended for initializing constants and should not
    /// be used at runtime, as it is slower than [`Vector::from_array`].
    #[inline(always)]
    pub const fn const_from_array(array: [T; N]) -> Self {
        const {
            assert!(size_of::<Vector<N, T, S>>() % size_of::<T>() == 0);
            assert!(size_of::<Vector<N, T, S>>() / size_of::<T>() >= N);
        }

        // SAFETY: Vector<N, MaybeUninit<T>, S> is transparent to [MaybeUninit<T>; N] which allows all bit patterns
        let mut result =
            unsafe { MaybeUninit::<Vector<N, MaybeUninit<T>, S>>::uninit().assume_init() };

        let mut i = 0;
        while i < N {
            result.as_mut_array()[i] = MaybeUninit::new(array[i]);
            i += 1;
        }

        let mut i = N;
        while i < size_of::<Vector<N, T, S>>() / size_of::<T>() {
            unsafe {
                *result.as_mut_array().as_mut_ptr().add(i) = MaybeUninit::new(array[N - 1]);
            }
            i += 1;
        }

        // SAFETY:
        // Vector<N, MaybeUninit<T>, S> is guaranteed to begin with N MaybeUninit<T>'s, so it also begins with N T's,
        // and all elements are initialized.
        unsafe { transmute_copy::<Vector<N, MaybeUninit<T>, S>, Vector<N, T, S>>(&result) }
    }

    /// Returns the number of elements in the vector, which is a statically known
    /// constant.
    #[inline(always)]
    pub const fn len(self) -> usize {
        N
    }

    /// Returns true for [`Simd`] vectors, false for [`NonSimd`] vectors.
    #[inline(always)]
    pub const fn is_simd(self) -> bool {
        S::IS_SIMD
    }

    /// Converts `self` to a vector of a different [`Simdness`].
    #[inline(always)]
    pub fn as_simdness<S2: Simdness>(self) -> Vector<N, T, S2> {
        if S::IS_SIMD == S2::IS_SIMD {
            // SAFETY: S and S2 are the same type, so we are transmuting a type to itself
            unsafe { transmute_copy::<Vector<N, T, S>, Vector<N, T, S2>>(&self) }
        } else {
            Vector::from_array(self.as_array())
        }
    }

    /// Converts `self` to a [`Simd`] vector.
    #[inline(always)]
    pub fn as_simd(self) -> Vector<N, T, Simd> {
        self.as_simdness::<Simd>()
    }

    /// Converts `self` to a [`NonSimd`] vector.
    #[inline(always)]
    pub fn as_nonsimd(self) -> Vector<N, T, NonSimd> {
        self.as_simdness::<NonSimd>()
    }

    /// Converts `self` to an owned array.
    #[inline(always)]
    pub const fn as_array(self) -> [T; N] {
        *self.as_array_ref()
    }

    /// Converts a vector reference to an array reference.
    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        // SAFETY: Vector<N, T, S> is guaranteed to begin with N T's
        unsafe { transmute::<&Vector<N, T, S>, &[T; N]>(self) }
    }

    /// Converts a mutable vector reference to an mutable array reference.
    #[inline(always)]
    pub const fn as_mut_array(&mut self) -> &mut [T; N] {
        // SAFETY: Vector<N, T, S> is guaranteed to begin with N T's
        unsafe { transmute::<&mut Vector<N, T, S>, &mut [T; N]>(self) }
    }

    /// Returns the element at the given index, or `None` if the index is out of
    /// bounds.
    #[inline(always)]
    pub const fn get(self, index: usize) -> Option<T> {
        if index < N {
            // SAFETY: index is guaranteed to be in bounds
            Some(unsafe { *self.as_array_ref().as_ptr().add(index) })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index, or `None`
    /// if the index is out of bounds.
    #[inline(always)]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            // SAFETY: index is guaranteed to be in bounds
            Some(unsafe { &mut *self.as_mut_array().as_mut_ptr().add(index) })
        } else {
            None
        }
    }

    /// Returns a new vector with the elements of `self` in reverse order.
    #[inline(always)]
    pub fn reverse(self) -> Self {
        (const {
            match N {
                // SAFETY: N is guaranteed to be 2, so we are transmuting a type to itself
                2 => unsafe {
                    let func: fn(_) -> _ = Vector::<2, T, S>::swizzle2::<1, 0>;

                    transmute_copy::<
                        fn(Vector<2, T, S>) -> Vector<2, T, S>,
                        fn(Vector<N, T, S>) -> Vector<N, T, S>,
                    >(&func)
                },
                // SAFETY: N is guaranteed to be 3, so we are transmuting a type to itself
                3 => unsafe {
                    let func: fn(_) -> _ = Vector::<3, T, S>::swizzle3::<2, 1, 0>;

                    transmute_copy::<
                        fn(Vector<3, T, S>) -> Vector<3, T, S>,
                        fn(Vector<N, T, S>) -> Vector<N, T, S>,
                    >(&func)
                },
                // SAFETY: N is guaranteed to be 4, so we are transmuting a type to itself
                4 => unsafe {
                    let func: fn(_) -> _ = Vector::<4, T, S>::swizzle4::<3, 2, 1, 0>;

                    transmute_copy::<
                        fn(Vector<4, T, S>) -> Vector<4, T, S>,
                        fn(Vector<N, T, S>) -> Vector<N, T, S>,
                    >(&func)
                },
                ..2 | 5.. => panic!("N must be 2, 3, or 4"),
            }
        })(self)
    }

    /// Returns an iterator over the elements of the vector.
    #[inline(always)]
    pub fn iter(self) -> <[T; N] as IntoIterator>::IntoIter {
        self.as_array().into_iter()
    }

    /// Returns an iterator over mutable references to the elements of the vector.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> <&mut [T; N] as IntoIterator>::IntoIter {
        self.as_mut_array().iter_mut()
    }

    /// Returns a new vector with function `f` applied to each element.
    ///
    /// ## Examples
    ///
    /// ```
    /// use ggmath::{Vec2, vec2};
    ///
    /// let v: Vec2<f32> = vec2!(1.0, 2.0);
    /// assert_eq!(v.map(|x| x * 2.0), vec2!(2.0, 4.0));
    /// ```
    #[inline(always)]
    pub fn map<U: Scalar>(self, f: impl Fn(T) -> U) -> Vector<N, U, S> {
        Vector::from_array(self.as_array().map(f))
    }

    specialized_vector_api! {
        VectorApi for <N, T, S>:

        fn eq(self, other: Self) -> bool where T: PartialEq;
        fn ne(self, other: Self) -> bool where T: PartialEq;
    }
}

impl<T: Scalar, S: Simdness> Vector<2, T, S> {
    /// Returns a vector with the same elements as `self`, but with the x component replaced with `value`.
    #[inline(always)]
    pub fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.x = value;

        result
    }

    /// Returns a vector with the same elements as `self`, but with the y component replaced with `value`.
    #[inline(always)]
    pub fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.y = value;

        result
    }
}

impl<T: Scalar, S: Simdness> Vector<3, T, S> {
    /// Returns a vector with the same elements as `self`, but with the x component replaced with `value`.
    #[inline(always)]
    pub fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.x = value;

        result
    }

    /// Returns a vector with the same elements as `self`, but with the y component replaced with `value`.
    #[inline(always)]
    pub fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.y = value;

        result
    }

    /// Returns a vector with the same elements as `self`, but with the z component replaced with `value`.
    #[inline(always)]
    pub fn with_z(self, value: T) -> Self {
        let mut result = self;
        result.z = value;

        result
    }
}

impl<T: Scalar, S: Simdness> Vector<4, T, S> {
    /// Returns a vector with the same elements as `self`, but with the x component replaced with `value`.
    #[inline(always)]
    pub fn with_x(self, value: T) -> Self {
        let mut result = self;
        result.x = value;

        result
    }

    /// Returns a vector with the same elements as `self`, but with the y component replaced with `value`.
    #[inline(always)]
    pub fn with_y(self, value: T) -> Self {
        let mut result = self;
        result.y = value;

        result
    }

    /// Returns a vector with the same elements as `self`, but with the z component replaced with `value`.
    #[inline(always)]
    pub fn with_z(self, value: T) -> Self {
        let mut result = self;
        result.z = value;

        result
    }

    /// Returns a vector with the same elements as `self`, but with the w component replaced with `value`.
    #[inline(always)]
    pub fn with_w(self, value: T) -> Self {
        let mut result = self;
        result.w = value;

        result
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, Simd>
where
    VecLen<N>: SupportedVecLen,
{
    /// Creates a [`Simd`] vector from its internal representation, which is
    /// [`<T as SimdBehaviour<N>>::VectorRepr`][SimdBehaviour::VectorRepr].
    ///
    /// This function is useful when implementing SIMD optimizations for custom
    /// scalar types.
    #[inline(always)]
    pub const fn from_repr(repr: <T as SimdBehaviour<N>>::VectorRepr) -> Self
    where
        T: SimdBehaviour<N>,
    {
        const {
            assert!(
                size_of::<Vector<N, T, Simd>>() == size_of::<<T as SimdBehaviour<N>>::VectorRepr>()
            );
            assert!(
                align_of::<Vector<N, T, Simd>>()
                    == align_of::<<T as SimdBehaviour<N>>::VectorRepr>()
            );
        }

        // SAFETY: Vector<N, T, Simd> is transparent to <T as SimdBehaviour<N>>::VectorRepr
        unsafe { transmute_copy::<<T as SimdBehaviour<N>>::VectorRepr, Vector<N, T, Simd>>(&repr) }
    }

    /// Returns the internal representation of a [`Simd`] vector, which is
    /// [`<T as SimdBehaviour<N>>::VectorRepr`][SimdBehaviour::VectorRepr].
    ///
    /// This function is useful when implementing SIMD optimizations for custom
    /// scalar types.
    #[inline(always)]
    pub const fn repr(self) -> <T as SimdBehaviour<N>>::VectorRepr
    where
        T: SimdBehaviour<N>,
    {
        const {
            assert!(
                size_of::<Vector<N, T, Simd>>() == size_of::<<T as SimdBehaviour<N>>::VectorRepr>()
            );
            assert!(
                align_of::<Vector<N, T, Simd>>()
                    == align_of::<<T as SimdBehaviour<N>>::VectorRepr>()
            );
        }

        // SAFETY: Vector<N, T, Simd> is transparent to <T as SimdBehaviour<N>>::VectorRepr
        unsafe { transmute_copy::<Vector<N, T, Simd>, <T as SimdBehaviour<N>>::VectorRepr>(&self) }
    }
}

impl<const N: usize, T: Scalar> Vector<N, T, NonSimd>
where
    VecLen<N>: SupportedVecLen,
{
    /// Converts an array reference to a [`NonSimd`] vector reference.
    #[inline(always)]
    pub const fn from_array_ref(array: &[T; N]) -> &Self {
        // SAFETY: Vector<N, T, NonSimd> is transparent to [T; N]
        unsafe { transmute::<&[T; N], &Vector<N, T, NonSimd>>(array) }
    }

    /// Converts a mutable array reference to a mutable [`NonSimd`] vector reference.
    #[inline(always)]
    pub const fn from_mut_array(array: &mut [T; N]) -> &mut Self {
        // SAFETY: Vector<N, T, NonSimd> is transparent to [T; N]
        unsafe { transmute::<&mut [T; N], &mut Vector<N, T, NonSimd>>(array) }
    }
}

impl<const N: usize, T: Scalar, S: Simdness> Clone for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar, S: Simdness> Copy for Vector<N, T, S> where
    VecLen<N>: SupportedVecLen
{
}

impl<const N: usize, T: Scalar, S: Simdness> Index<usize> for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array_ref().index(index)
    }
}

impl<const N: usize, T: Scalar, S: Simdness> IndexMut<usize> for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_mut_array().index_mut(index)
    }
}

impl<const N: usize, T: Scalar, S: Simdness> IntoIterator for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T: Scalar, S: Simdness> IntoIterator for &'a Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Item = &'a T;
    type IntoIter = <&'a [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_array_ref().iter()
    }
}

impl<'a, const N: usize, T: Scalar, S: Simdness> IntoIterator for &'a mut Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T: Scalar + PartialEq, S: Simdness> PartialEq for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        (*self).eq(*other)
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        (*self).ne(*other)
    }
}

impl<const N: usize, T: Scalar + Debug, S: Simdness> Debug for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            write!(f, "{:?}", self[i])?;

            if i < N - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}

impl<const N: usize, T: Scalar + Display, S: Simdness> Display for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(")?;

        for i in 0..N {
            write!(f, "{}", self[i])?;

            if i < N - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}

////////////////////////////////////////////////////////////////////////////////
// Type Aliases
////////////////////////////////////////////////////////////////////////////////

/// A 2D vector of `T` elements.
///
/// This is a type alias for [`Vector<2, T, Simd>`]. For appropriate `T`s,
/// this type is SIMD-aligned and uses performant SIMD instructions.
pub type Vec2<T> = Vector<2, T, Simd>;

/// A 3D vector of `T` elements.
///
/// This is a type alias for [`Vector<3, T, Simd>`]. For appropriate `T`s,
/// this type is SIMD-aligned and uses performant SIMD instructions.
pub type Vec3<T> = Vector<3, T, Simd>;

/// A 4D vector of `T` elements.
///
/// This is a type alias for [`Vector<4, T, Simd>`]. For appropriate `T`s,
/// this type is SIMD-aligned and uses performant SIMD instructions.
pub type Vec4<T> = Vector<4, T, Simd>;

/// A 2D vector of `T` elements.
///
/// Unlike [`Vec2`], this type is never SIMD-aligned ("s" stands for
/// "scalar"). This is a type alias for [`Vector<2, T, NonSimd>`].
pub type Vec2S<T> = Vector<2, T, NonSimd>;

/// A 3D vector of `T` elements.
///
/// Unlike [`Vec3`], this type is never SIMD-aligned ("s" stands for
/// "scalar").
/// This is a type alias for [`Vector<3, T, NonSimd>`].
pub type Vec3S<T> = Vector<3, T, NonSimd>;

/// A 4D vector of `T` elements.
///
/// Unlike [`Vec4`], this type is never SIMD-aligned ("s" stands for
/// "scalar").
/// This is a type alias for [`Vector<4, T, NonSimd>`].
pub type Vec4S<T> = Vector<4, T, NonSimd>;

/// Declares vector type aliases for a custom scalar type.
///
/// ## Example
///
/// ```
/// use ggmath::declare_vector_aliases;
///
/// struct CustomScalar(f32);
///
/// declare_vector_aliases!(type C => CustomScalar);
///
/// // Generated aliases:
/// // type CVec2 = Vector<2, CustomScalar, Simd>;
/// // type CVec3 = Vector<3, CustomScalar, Simd>;
/// // type CVec4 = Vector<4, CustomScalar, Simd>;
/// // type CVec2S = Vector<2, CustomScalar, NonSimd>;
/// // type CVec3S = Vector<3, CustomScalar, NonSimd>;
/// // type CVec4S = Vector<4, CustomScalar, NonSimd>;
/// ```
#[macro_export]
macro_rules! declare_vector_aliases {
    ($vis:vis type $prefix:ident => $T:ty) => {
        $crate::hidden::paste! {
            #[doc = "A 2D vector of `" $T "` elements."]
            #[doc = ""]
            #[doc = "This is a type alias for `Vector<2, " $T ", Simd>`."]
            #[doc = "This type may be SIMD-aligned to use performant SIMD instructions."]
            $vis type [<$prefix Vec2>] = $crate::Vector<2, $T, $crate::Simd>;

            #[doc = "A 3D vector of `" $T "` elements."]
            #[doc = ""]
            #[doc = "This is a type alias for `Vector<3, " $T ", Simd>`."]
            #[doc = "This type may be SIMD-aligned to use performant SIMD instructions."]
            $vis type [<$prefix Vec3>] = $crate::Vector<3, $T, $crate::Simd>;

            #[doc = "A 4D vector of `" $T "` elements."]
            #[doc = ""]
            #[doc = "This is a type alias for `Vector<4, " $T ", Simd>`."]
            #[doc = "This type may be SIMD-aligned to use performant SIMD instructions."]
            $vis type [<$prefix Vec4>] = $crate::Vector<4, $T, $crate::Simd>;

            #[doc = "A 2D vector of `" $T "` elements."]
            #[doc = ""]
            #[doc = "Unlike [`" $prefix "Vec2`], this type is never SIMD-aligned (\"s\" stands for \"scalar\")."]
            #[doc = "This is a type alias for `Vector<2, " $T ", NonSimd>`."]
            $vis type [<$prefix Vec2S>] = $crate::Vector<2, $T, $crate::NonSimd>;

            #[doc = "A 3D vector of `" $T "` elements."]
            #[doc = ""]
            #[doc = "Unlike [`" $prefix "Vec3`], this type is never SIMD-aligned (\"s\" stands for \"scalar\")."]
            #[doc = "This is a type alias for `Vector<3, " $T ", NonSimd>`."]
            $vis type [<$prefix Vec3S>] = $crate::Vector<3, $T, $crate::NonSimd>;

            #[doc = "A 4D vector of `" $T "` elements."]
            #[doc = ""]
            #[doc = "Unlike [`" $prefix "Vec4`], this type is never SIMD-aligned (\"s\" stands for \"scalar\")."]
            #[doc = "This is a type alias for `Vector<4, " $T ", NonSimd>`."]
            $vis type [<$prefix Vec4S>] = $crate::Vector<4, $T, $crate::NonSimd>;
        }
    };
}

pub use declare_vector_aliases;

////////////////////////////////////////////////////////////////////////////////
// Scalar
////////////////////////////////////////////////////////////////////////////////

/// A trait for types that can be used as elements in a [`Vector`].
///
/// To implement this trait, you must also implement the [`SimdBehaviour<N>`] trait
/// for `N = 2, 3, 4`. The [`SimdBehaviour`] trait controls the implementation
/// details of [`Vector<N, Self, Simd>`].
///
/// For an example of a SIMD-optimized implementation, see the
/// `fixed_point_scalar` example.
///
/// ## Example
///
/// ```
/// use ggmath::*;
///
/// #[derive(Clone, Copy)]
/// struct CustomScalar(f32);
///
/// impl Scalar for CustomScalar {}
///
/// impl SimdBehaviour<2> for CustomScalar {
///     type VectorRepr = [CustomScalar; 2];
/// }
///
/// impl SimdBehaviour<3> for CustomScalar {
///     type VectorRepr = [CustomScalar; 3];
/// }
///
/// impl SimdBehaviour<4> for CustomScalar {
///     type VectorRepr = [CustomScalar; 4];
/// }
///
/// // Now `CustomScalar` can be used as a vector element type.
/// ```
pub trait Scalar: SimdBehaviour<2> + SimdBehaviour<3> + SimdBehaviour<4> {}

/// Controls the implementation details of [`Vector<N, Self, Simd>`]. For
/// example, `f32` implementing `SimdBehaviour<4>` controls
/// `Vector<4, f32, Simd>`.
///
/// The goal of implementing this trait is to enable SIMD optimizations,
/// but it's acceptable not to use SIMD if it doesn't provide a performance
/// benefit, or if it's only a placeholder implementation.
pub trait SimdBehaviour<const N: usize>: Construct
where
    VecLen<N>: SupportedVecLen,
{
    /// The internal representation of the vector.
    ///
    /// There are two ways to define the internal representation:
    ///
    /// 1. Array:  
    ///    If the internal representation is `[Self; N]`, there is no SIMD alignment.  
    ///    In this case, the behavior of [`Simd`] and [`NonSimd`] is the same, with no SIMD optimizations.
    ///
    /// 2. Wrapped Vector Type:  
    ///    If `Self` is a wrapper around an existing scalar type,  
    ///    the internal representation can be a vector of that scalar type.  
    ///    This allows you to leverage the SIMD-optimized API of the base type.  
    ///    To enable this, `Self` must implement [`ScalarWrapper<TInner>`].
    ///
    /// ## Example
    ///
    /// ```rust
    /// use ggmath::*;
    ///
    /// #[repr(transparent)]
    /// #[derive(Copy, Clone)]
    /// struct CustomScalar(f32);
    ///
    /// impl Scalar for CustomScalar {}
    ///
    /// // SAFETY: CustomScalar is a direct wrapper around f32, so it's safe to
    /// // implement ScalarWrapper<f32> for it.
    /// unsafe impl ScalarWrapper<f32> for CustomScalar {}
    ///
    /// impl<const N: usize> SimdBehaviour<N> for CustomScalar
    /// where
    ///     VecLen<N>: SupportedVecLen,
    /// {
    ///     type VectorRepr = Vector<N, f32, Simd>;
    ///
    ///     // Now you can use f32's SIMD-optimized vector API to optimize your vector
    ///     // implementation.
    /// }
    /// ```
    #[expect(private_bounds)]
    type VectorRepr: SoundVectorRepr<N, Self>;

    /// Overridable implementation of [`Vector::from_array`].
    #[inline(always)]
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, Simd>
    where
        Self: Scalar,
    {
        Vector::const_from_array(array)
    }

    /// Overridable implementation of [`Vector::splat`].
    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<N, Self, Simd>
    where
        Self: Scalar,
    {
        Vector::from_array([value; N])
    }

    /// Overridable implementation of [`Vector::swizzle2`].
    #[inline(always)]
    unsafe fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<N, Self, Simd>,
    ) -> Vector<2, Self, Simd>
    where
        Self: Scalar,
    {
        Vector::<2, _, _>::from_array([vec[X_SRC], vec[Y_SRC]])
    }

    /// Overridable implementation of [`Vector::swizzle3`].
    #[inline(always)]
    unsafe fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<N, Self, Simd>,
    ) -> Vector<3, Self, Simd>
    where
        Self: Scalar,
    {
        Vector::<3, _, _>::from_array([vec[X_SRC], vec[Y_SRC], vec[Z_SRC]])
    }

    /// Overridable implementation of [`Vector::swizzle4`].
    #[inline(always)]
    unsafe fn vec_swizzle4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<N, Self, Simd>,
    ) -> Vector<4, Self, Simd>
    where
        Self: Scalar,
    {
        Vector::<4, _, _>::from_array([vec[X_SRC], vec[Y_SRC], vec[Z_SRC], vec[W_SRC]])
    }

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        vec.iter().zip(other).all(|(a, b)| a == b)
    }

    /// Overridable implementation of [`Vector::ne`].
    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        vec.iter().zip(other).any(|(a, b)| a != b)
    }

    /// Overridable implementation of [`Vector::neg`].
    ///
    /// Implementations may deviate from `Self as Neg` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Neg`.
    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Neg<Output = Self>,
    {
        vec.map(Self::neg)
    }

    /// Overridable implementation of [`Vector::not`].
    ///
    /// Implementations may deviate from `Self as Not` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Not`.
    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Not<Output = Self>,
    {
        vec.map(Self::not)
    }

    /// Overridable implementation of [`Vector::add`].
    ///
    /// Implementations may deviate from `Self as Add` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Add`.
    #[inline(always)]
    fn vec_add(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Add<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] + rhs[i])
    }

    /// Overridable implementation of [`Vector::sub`].
    ///
    /// Implementations may deviate from `Self as Sub` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Sub`.
    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Sub<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] - rhs[i])
    }

    /// Overridable implementation of [`Vector::mul`].
    ///
    /// Implementations may deviate from `Self as Mul` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Mul`.
    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Mul<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] * rhs[i])
    }

    /// Overridable implementation of [`Vector::div`].
    ///
    /// Implementations may deviate from `Self as Div` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Div`.
    #[inline(always)]
    fn vec_div(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Div<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] / rhs[i])
    }

    /// Overridable implementation of [`Vector::rem`].
    ///
    /// Implementations may deviate from `Self as Rem` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Rem`.
    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Rem<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] % rhs[i])
    }

    /// Overridable implementation of [`Vector::shl`].
    ///
    /// Implementations may deviate from `Self as Shl` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Shl`.
    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Shl<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] << rhs[i])
    }

    /// Overridable implementation of [`Vector::shr`].
    ///
    /// Implementations may deviate from `Self as Shr` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as Shr`.
    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + Shr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] >> rhs[i])
    }

    /// Overridable implementation of [`Vector::bitand`].
    ///
    /// Implementations may deviate from `Self as BitAnd` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as BitAnd`.
    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] & rhs[i])
    }

    /// Overridable implementation of [`Vector::bitor`].
    ///
    /// Implementations may deviate from `Self as BitOr` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as BitOr`.
    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] | rhs[i])
    }

    /// Overridable implementation of [`Vector::bitxor`].
    ///
    /// Implementations may deviate from `Self as BitXor` in edge cases, but should
    /// otherwise closely follow the behavior of `Self as BitXor`.
    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Scalar + BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] ^ rhs[i])
    }
}

/// Unsafe trait for scalar types that are direct wrappers around another scalar type.
///
/// This trait is used by [`SimdBehaviour::VectorRepr`].
///
/// ## Example
///
/// ```
/// use ggmath::*;
///
/// #[repr(transparent)]
/// #[derive(Copy, Clone)]
/// struct CustomScalar(f32);
///
/// // SAFETY: CustomScalar is a direct wrapper around f32, so it's safe to
/// // implement ScalarWrapper<f32> for it.
/// unsafe impl ScalarWrapper<f32> for CustomScalar {}
/// ```
pub unsafe trait ScalarWrapper<T: Scalar> {}

////////////////////////////////////////////////////////////////////////////////
// Length
////////////////////////////////////////////////////////////////////////////////

/// A type that represents the length of a vector, and allows for this pattern:
///
/// ```ignore
/// impl<const N: usize, T: Scalar, S: Simdness> Vector<N, T, S>
/// where
///     VecLen<N>: SupportedVecLen,
/// {
/// }
/// ```
pub struct VecLen<const N: usize>;

/// A trait that marks supported [`Vector`] lengths.
pub trait SupportedVecLen: Sealed {
    #[doc(hidden)]
    type Pick<T2: Construct, T3: Construct, T4: Construct>: Construct;
}

impl SupportedVecLen for VecLen<2> {
    type Pick<T2: Construct, T3: Construct, T4: Construct> = T2;
}
impl SupportedVecLen for VecLen<3> {
    type Pick<T2: Construct, T3: Construct, T4: Construct> = T3;
}
impl SupportedVecLen for VecLen<4> {
    type Pick<T2: Construct, T3: Construct, T4: Construct> = T4;
}

impl Sealed for VecLen<2> {}
impl Sealed for VecLen<3> {}
impl Sealed for VecLen<4> {}

////////////////////////////////////////////////////////////////////////////////
// Simdness
////////////////////////////////////////////////////////////////////////////////

/// A type that marks a [`Vector`] as SIMD-backed.
pub struct Simd;

/// A type that marks a [`Vector`] as not SIMD-backed.
pub struct NonSimd;

/// A trait that is implemented for [`Simd`] and [`NonSimd`].
pub trait Simdness: Sealed + 'static {
    #[doc(hidden)]
    type Pick<TSimd: Construct, TNonSimd: Construct>: Construct;

    /// Is true for [`Simd`], and false for [`NonSimd`].
    const IS_SIMD: bool;
}

impl Simdness for Simd {
    type Pick<TSimd: Construct, TNonSimd: Construct> = TSimd;

    const IS_SIMD: bool = true;
}
impl Simdness for NonSimd {
    type Pick<TSimd: Construct, TNonSimd: Construct> = TNonSimd;

    const IS_SIMD: bool = false;
}

impl Sealed for Simd {}
impl Sealed for NonSimd {}

////////////////////////////////////////////////////////////////////////////////
// Vector Representation
////////////////////////////////////////////////////////////////////////////////

trait VectorReprExt {
    type Repr: Construct;
}

impl<const N: usize, T: Scalar, S: Simdness> VectorReprExt for Vector<N, T, S>
where
    VecLen<N>: SupportedVecLen,
{
    type Repr = <S as Simdness>::Pick<
        <VecLen<N> as SupportedVecLen>::Pick<
            <T as SimdBehaviour<2>>::VectorRepr,
            <T as SimdBehaviour<3>>::VectorRepr,
            <T as SimdBehaviour<4>>::VectorRepr,
        >,
        [T; N],
    >;
}

unsafe trait SoundVectorRepr<const N: usize, T: Construct>: Construct {}

// SAFETY: [T; N] begins with N T's
unsafe impl<const N: usize, T: Scalar> SoundVectorRepr<N, T> for [T; N] {}

// SAFETY: Vector<N, InsideT, InsideS> begins with N InsideT's, so it also begins with N T's
unsafe impl<const N: usize, T: Scalar, InsideT: Scalar, InsideS: Simdness> SoundVectorRepr<N, T>
    for Vector<N, InsideT, InsideS>
where
    T: ScalarWrapper<InsideT>,
    VecLen<N>: SupportedVecLen,
{
}

////////////////////////////////////////////////////////////////////////////////
// Specialization
////////////////////////////////////////////////////////////////////////////////

trait VectorApi<const N: usize, S: Simdness>: Scalar
where
    VecLen<N>: SupportedVecLen,
{
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, S>;

    fn vec_splat(value: Self) -> Vector<N, Self, S>;

    fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<N, Self, S>,
    ) -> Vector<2, Self, S>;

    fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<N, Self, S>,
    ) -> Vector<3, Self, S>;

    fn vec_swizzle4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<N, Self, S>,
    ) -> Vector<4, Self, S>;

    fn vec_eq(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> bool
    where
        Self: PartialEq;

    fn vec_ne(vec: Vector<N, Self, S>, other: Vector<N, Self, S>) -> bool
    where
        Self: PartialEq;

    fn vec_neg(vec: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Neg<Output = Self>;

    fn vec_not(vec: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Not<Output = Self>;

    fn vec_add(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Add<Output = Self>;

    fn vec_sub(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Sub<Output = Self>;

    fn vec_mul(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Mul<Output = Self>;

    fn vec_div(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Div<Output = Self>;

    fn vec_rem(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Rem<Output = Self>;

    fn vec_shl(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Shl<Output = Self>;

    fn vec_shr(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: Shr<Output = Self>;

    fn vec_bitand(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: BitAnd<Output = Self>;

    fn vec_bitor(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: BitOr<Output = Self>;

    fn vec_bitxor(vec: Vector<N, Self, S>, rhs: Vector<N, Self, S>) -> Vector<N, Self, S>
    where
        Self: BitXor<Output = Self>;
}

impl<const N: usize, T: Scalar + SimdBehaviour<N>> VectorApi<N, Simd> for T
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, Simd> {
        <T as SimdBehaviour<N>>::vec_from_array(array)
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<N, Self, Simd> {
        <T as SimdBehaviour<N>>::vec_splat(value)
    }

    #[inline(always)]
    fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<N, Self, Simd>,
    ) -> Vector<2, Self, Simd> {
        const {
            assert!(X_SRC < N);
            assert!(Y_SRC < N);
        }

        // SAFETY: it is guaranteed that X_SRC and Y_SRC are in bounds
        unsafe { <T as SimdBehaviour<N>>::vec_swizzle2::<X_SRC, Y_SRC>(vec) }
    }

    #[inline(always)]
    fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<N, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        const {
            assert!(X_SRC < N);
            assert!(Y_SRC < N);
            assert!(Z_SRC < N);
        }

        // SAFETY: it is guaranteed that X_SRC, Y_SRC, and Z_SRC are in bounds
        unsafe { <T as SimdBehaviour<N>>::vec_swizzle3::<X_SRC, Y_SRC, Z_SRC>(vec) }
    }

    #[inline(always)]
    fn vec_swizzle4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<N, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        const {
            assert!(X_SRC < N);
            assert!(Y_SRC < N);
            assert!(Z_SRC < N);
            assert!(W_SRC < N);
        }

        // SAFETY: it is guaranteed that X_SRC, Y_SRC, Z_SRC, and W_SRC are in bounds
        unsafe { <T as SimdBehaviour<N>>::vec_swizzle4::<X_SRC, Y_SRC, Z_SRC, W_SRC>(vec) }
    }

    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> bool
    where
        Self: PartialEq,
    {
        <T as SimdBehaviour<N>>::vec_eq(vec, other)
    }

    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, Simd>, other: Vector<N, Self, Simd>) -> bool
    where
        Self: PartialEq,
    {
        <T as SimdBehaviour<N>>::vec_ne(vec, other)
    }

    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Neg<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_neg(vec)
    }

    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Not<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_not(vec)
    }

    #[inline(always)]
    fn vec_add(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Add<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_add(vec, rhs)
    }

    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Sub<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_sub(vec, rhs)
    }

    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Mul<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_mul(vec, rhs)
    }

    #[inline(always)]
    fn vec_div(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Div<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_div(vec, rhs)
    }

    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Rem<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_rem(vec, rhs)
    }

    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Shl<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_shl(vec, rhs)
    }

    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: Shr<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_shr(vec, rhs)
    }

    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: BitAnd<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_bitand(vec, rhs)
    }

    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: BitOr<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_bitor(vec, rhs)
    }

    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self, Simd>, rhs: Vector<N, Self, Simd>) -> Vector<N, Self, Simd>
    where
        Self: BitXor<Output = Self>,
    {
        <T as SimdBehaviour<N>>::vec_bitxor(vec, rhs)
    }
}

impl<const N: usize, T: Scalar> VectorApi<N, NonSimd> for T
where
    VecLen<N>: SupportedVecLen,
{
    #[inline(always)]
    fn vec_from_array(array: [Self; N]) -> Vector<N, Self, NonSimd> {
        Vector(array)
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<N, Self, NonSimd> {
        Vector([value; N])
    }

    #[inline(always)]
    fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<N, Self, NonSimd>,
    ) -> Vector<2, Self, NonSimd> {
        const {
            assert!(X_SRC < N);
            assert!(Y_SRC < N);
        }

        Vector::<2, _, _>([vec[X_SRC], vec[Y_SRC]])
    }

    #[inline(always)]
    fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<N, Self, NonSimd>,
    ) -> Vector<3, Self, NonSimd> {
        const {
            assert!(X_SRC < N);
            assert!(Y_SRC < N);
            assert!(Z_SRC < N);
        }

        Vector::<3, _, _>([vec[X_SRC], vec[Y_SRC], vec[Z_SRC]])
    }

    #[inline(always)]
    fn vec_swizzle4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<N, Self, NonSimd>,
    ) -> Vector<4, Self, NonSimd> {
        const {
            assert!(X_SRC < N);
            assert!(Y_SRC < N);
            assert!(Z_SRC < N);
            assert!(W_SRC < N);
        }

        Vector::<4, _, _>([vec[X_SRC], vec[Y_SRC], vec[Z_SRC], vec[W_SRC]])
    }

    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, NonSimd>, other: Vector<N, Self, NonSimd>) -> bool
    where
        Self: PartialEq,
    {
        vec.iter().zip(other).all(|(a, b)| a == b)
    }

    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, NonSimd>, other: Vector<N, Self, NonSimd>) -> bool
    where
        Self: PartialEq,
    {
        vec.iter().zip(other).any(|(a, b)| a != b)
    }

    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, NonSimd>) -> Vector<N, Self, NonSimd>
    where
        Self: Neg<Output = Self>,
    {
        vec.map(Self::neg)
    }

    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, NonSimd>) -> Vector<N, Self, NonSimd>
    where
        Self: Not<Output = Self>,
    {
        vec.map(Self::not)
    }

    #[inline(always)]
    fn vec_add(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Add<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] + rhs[i])
    }

    #[inline(always)]
    fn vec_sub(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Sub<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] - rhs[i])
    }

    #[inline(always)]
    fn vec_mul(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Mul<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] * rhs[i])
    }

    #[inline(always)]
    fn vec_div(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Div<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] / rhs[i])
    }

    #[inline(always)]
    fn vec_rem(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Rem<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] % rhs[i])
    }

    #[inline(always)]
    fn vec_shl(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Shl<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] << rhs[i])
    }

    #[inline(always)]
    fn vec_shr(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: Shr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] >> rhs[i])
    }

    #[inline(always)]
    fn vec_bitand(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] & rhs[i])
    }

    #[inline(always)]
    fn vec_bitor(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] | rhs[i])
    }

    #[inline(always)]
    fn vec_bitxor(
        vec: Vector<N, Self, NonSimd>,
        rhs: Vector<N, Self, NonSimd>,
    ) -> Vector<N, Self, NonSimd>
    where
        Self: BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] ^ rhs[i])
    }
}

macro_rules! specialized_vector_api {
    {
        $Api:ident for <$N:expr, $T:ty, $S:ty>:

        $(
            $(#[$meta:meta])*
            $vis:vis fn $fn_name:ident$(<$(const $CONST_PARAM:ident: $ConstParam:ty),* $(,)?>)?(
                $($param:ident$(: $Param:ty)?),* $(,)?
            ) -> $return_tt:ty
            $(where
                $($BoundType:ty: $BoundTrait:path),*)?;
        )*
    } => {
        $(
            $(#[$meta])*
            $vis fn $fn_name$(<$(const $CONST_PARAM: $ConstParam),*>)?(
                $($param $(: $Param)?),*
            ) -> $return_tt
            $(where
                $($BoundType: $BoundTrait),*)?
            {
                (const {
                    match <$S>::IS_SIMD {
                        true => match $N {
                            2 => specialized_vector_api!(@transmute_fnptr($($param),*) <$T as $Api<2, crate::Simd>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                            3 => specialized_vector_api!(@transmute_fnptr($($param),*) <$T as $Api<3, crate::Simd>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                            4 => specialized_vector_api!(@transmute_fnptr($($param),*) <$T as $Api<4, crate::Simd>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                            ..2 | 5.. => panic!("N must be 2, 3, or 4"),
                        }
                        false => specialized_vector_api!(@transmute_fnptr($($param),*) <$T as $Api<N, crate::NonSimd>>::vec_$fn_name$(::<$($CONST_PARAM),*>)?),
                    }
                })($($param),*)
            }
        )*
    };

    { @transmute_fnptr($($param:ident),*) <$T:ty as $VectorApi:ident<$N:expr, $S:ty>>::vec_$fn_name:ident$(::<$($CONST_PARAM:ident),*>)? } => {
        crate::hidden::paste! {
            unsafe {
                let fnptr: fn($(specialized_vector_api!(@_ $param)),*) -> _ = <$T as $VectorApi<$N, $S>>::[<vec_$fn_name>]$(::<$($CONST_PARAM),*>)?;

                core::mem::transmute_copy::<
                    fn($(specialized_vector_api!(@_ $param)),*) -> _,
                    fn($(specialized_vector_api!(@_ $param)),*) -> _,
                >(&fnptr)
            }
        }
    };

    { @_ $_:tt } => { _ }
}

use specialized_vector_api;
