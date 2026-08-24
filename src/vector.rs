use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
        DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, SupportedLength, Unaligned,
    backend::VectorBackend,
    utils::{specialize, transmute_mut, transmute_ref},
};

// These submodules have empty lines between them so that rustfmt does not
// incorrectly reorder them. The order is important since it impacts the order
// of `impl` blocks in rustdoc's output.
//
// The contents of the `generic` submodule *would* be simply put in this root
// module, but due to a rustdoc bug, that would cause functionality generic over
// `T` to be shown after all submodule functionality.

mod generic;

mod float;

mod integer;

mod signed;

mod unsigned;

mod bool;

mod swizzle;

#[cfg(feature = "wide")]
mod wide;

#[cfg(feature = "wide")]
mod wide_float;

#[cfg(feature = "wide")]
mod wide_integer;

#[cfg(feature = "wide")]
mod wide_signed;

#[cfg(feature = "wide")]
mod wide_unsigned;

/// An `N`-dimensional vector of type `T`.
///
/// `A` controls SIMD alignment and is either [`Unaligned`] or [`Aligned`]. See
/// [`Alignment`] for more details.
///
/// # Type aliases
///
/// - [`Vec2<T>`] for [`Vector<2, T, Unaligned>`].
/// - [`Vec3<T>`] for [`Vector<3, T, Unaligned>`].
/// - [`Vec4<T>`] for [`Vector<4, T, Unaligned>`].
/// - [`Vec2A<T>`] for [`Vector<2, T, Aligned>`].
/// - [`Vec3A<T>`] for [`Vector<3, T, Aligned>`].
/// - [`Vec4A<T>`] for [`Vector<4, T, Aligned>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector, exists for lengths `2`, `3`, `4`)
///
/// - `y: T` (the second element of the vector, exists for lengths `2`, `3`,
///   `4`)
///
/// - `z: T` (the third element of the vector, exists for lengths `3`, `4`)
///
/// - `w: T` (the fourth element of the vector, exists for length `4`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Vector<N, T, A>`] contains `N` consecutive values of `T` followed by
/// optional padding.
///
/// [`Vector<N, T, Unaligned>`] has the alignment of `T` and has no padding.
/// [`Vector<N, T, Aligned>`] may have higher alignment than `T`. [`Vec2A<T>`]
/// and [`Vec4A<T>`] have no padding. [`Vec3A<T>`] may have one padding element.
///
/// Padding is fully initialized and accepts all bit patterns. Unless `T`
/// accepts all bit patterns, it is not sound to assume padding contains valid
/// values of `T`.
#[repr(transparent)]
pub struct Vector<const N: usize, T, A: Alignment>(
    #[expect(clippy::type_complexity)]
    pub(crate)  <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<
            <T as VectorBackend<2, Aligned>>::Inner,
            <T as VectorBackend<3, Aligned>>::Inner,
            <T as VectorBackend<4, Aligned>>::Inner,
        >,
        <Length<N> as SupportedLength>::Select<
            <T as VectorBackend<2, Unaligned>>::Inner,
            <T as VectorBackend<3, Unaligned>>::Inner,
            <T as VectorBackend<4, Unaligned>>::Inner,
        >,
    >,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/// A 2D vector.
///
/// # No SIMD alignment
///
/// [`Vec2<T>`] does not have SIMD alignment, for that use [`Vec2A<T>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector)
/// - `y: T` (the second element of the vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Vec2<T> = Vector<2, T, Unaligned>;

/// A 3D vector.
///
/// # No SIMD alignment
///
/// [`Vec3<T>`] does not have SIMD alignment, for that use [`Vec3A<T>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector)
/// - `y: T` (the second element of the vector)
/// - `z: T` (the third element of the vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Vec3<T> = Vector<3, T, Unaligned>;

/// A 4D vector.
///
/// # No SIMD alignment
///
/// [`Vec4<T>`] does not have SIMD alignment, for that use [`Vec4A<T>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector)
/// - `y: T` (the second element of the vector)
/// - `z: T` (the third element of the vector)
/// - `w: T` (the fourth element of the vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Vec4<T> = Vector<4, T, Unaligned>;

/// A 2D vector.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Vec2A<T>`] has SIMD alignment. For no SIMD use
/// [`Vec2<T>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector)
/// - `y: T` (the second element of the vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Vec2A<T> = Vector<2, T, Aligned>;

/// A 3D vector.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Vec3A<T>`] has SIMD alignment. For no SIMD use
/// [`Vec3<T>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector)
/// - `y: T` (the second element of the vector)
/// - `z: T` (the third element of the vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Vec3A<T> = Vector<3, T, Aligned>;

/// A 4D vector.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Vec4A<T>`] has SIMD alignment. For no SIMD use
/// [`Vec4<T>`].
///
/// # Fields
///
/// - `x: T` (the first element of the vector)
/// - `y: T` (the second element of the vector)
/// - `z: T` (the third element of the vector)
/// - `w: T` (the fourth element of the vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Vec4A<T> = Vector<4, T, Aligned>;

impl<const N: usize, T, A: Alignment> Clone for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Index<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array().index(index)
    }
}

impl<const N: usize, T, A: Alignment> IndexMut<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_mut_array().index_mut(index)
    }
}

impl<const N: usize, T, A: Alignment> IntoIterator for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize, T, A: Alignment> IntoIterator for &Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T, A: Alignment> IntoIterator for &'a mut Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Vec2Fields<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
}

impl<T, A: Alignment> Deref for Vector<2, T, A>
where
    T: Scalar,
{
    type Target = Vec2Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Vector<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Xy<T>`.
        unsafe { transmute_ref::<Vector<2, T, A>, Vec2Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Vector<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Xy<T>`.
        unsafe { transmute_mut::<Vector<2, T, A>, Vec2Fields<T>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Vec3Fields<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
    /// The third element of the vector.
    pub z: T,
}

impl<T, A: Alignment> Deref for Vector<3, T, A>
where
    T: Scalar,
{
    type Target = Vec3Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Vector<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `T`, and so begin with `Xyz<T>`.
        unsafe { transmute_ref::<Vector<3, T, A>, Vec3Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Vector<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `T`, and so begin with `Xyz<T>`.
        unsafe { transmute_mut::<Vector<3, T, A>, Vec3Fields<T>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Vec4Fields<T> {
    /// The first element of the vector.
    pub x: T,
    /// The second element of the vector.
    pub y: T,
    /// The third element of the vector.
    pub z: T,
    /// The fourth element of the vector.
    pub w: T,
}

impl<T, A: Alignment> Deref for Vector<4, T, A>
where
    T: Scalar,
{
    type Target = Vec4Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Vector<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_ref::<Vector<4, T, A>, Vec4Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Vector<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_mut::<Vector<4, T, A>, Vec4Fields<T>>(self) }
    }
}

impl<T, A: Alignment> From<(T, T)> for Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T, A: Alignment> From<(T, T, T)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl<T, A: Alignment> From<(T, Vector<2, T, A>)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1.x, value.1.y)
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, T)> for Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, T)) -> Self {
        Self::new(value.0.x, value.0.y, value.1)
    }
}

impl<T, A: Alignment> From<(T, T, T, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl<T, A: Alignment> From<(T, T, Vector<2, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, T, Vector<2, T, A>)) -> Self {
        Self::new(value.0, value.1, value.2.x, value.2.y)
    }
}

impl<T, A: Alignment> From<(T, Vector<2, T, A>, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<2, T, A>, T)) -> Self {
        Self::new(value.0, value.1.x, value.1.y, value.2)
    }
}

impl<T, A: Alignment> From<(T, Vector<3, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (T, Vector<3, T, A>)) -> Self {
        Self::new(value.0, value.1.x, value.1.y, value.1.z)
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, T, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, T, T)) -> Self {
        Self::new(value.0.x, value.0.y, value.1, value.2)
    }
}

impl<T, A: Alignment> From<(Vector<2, T, A>, Vector<2, T, A>)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<2, T, A>, Vector<2, T, A>)) -> Self {
        Self::new(value.0.x, value.0.y, value.1.x, value.1.y)
    }
}

impl<T, A: Alignment> From<(Vector<3, T, A>, T)> for Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn from(value: (Vector<3, T, A>, T)) -> Self {
        Self::new(value.0.x, value.0.y, value.0.z, value.1)
    }
}

impl<const N: usize, T, A: Alignment> Debug for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({:?}, {:?})", self[0], self[1]),
            3 => write!(f, "({:?}, {:?}, {:?})", self[0], self[1], self[2]),
            4 => write!(
                f,
                "({:?}, {:?}, {:?}, {:?})",
                self[0], self[1], self[2], self[3]
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> Display for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({}, {})", self[0], self[1]),
            3 => write!(f, "({}, {}, {})", self[0], self[1], self[2]),
            4 => write!(f, "({}, {}, {}, {})", self[0], self[1], self[2], self[3]),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize!(<T as VectorBackend<N, A>>::vector_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as VectorBackend<N, A>>::vector_ne(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_array().hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Default,
{
    #[inline]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

macro_rules! impl_unary_operator {
    ($Op:ident, $op:ident, $vector_op:ident, $(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self) -> Self::Output {
                specialize!(<T as VectorBackend<N, A>>::$vector_op(self))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self) -> Self::Output {
                Vector::$op(*self)
            }
        }
    };
}
impl_unary_operator!(
    Neg,
    neg,
    vector_neg,
    /// Performs the unary `-` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = -Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(-1, -2, -3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
);
impl_unary_operator!(
    Not,
    not,
    vector_not,
    /// Performs the unary `!` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = !Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(!1, !2, !3));
    /// ```
);

macro_rules! impl_binary_operator {
    ($Op:ident, $op:ident, $vector_op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: Self) -> Self::Output {
                specialize!(<T as VectorBackend<N, A>>::$vector_op(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: T) -> Self::Output {
                self.$op(Self::splat(rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&Vector<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &Vector<N, T, A>) -> Self::Output {
                self.$op(*rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &T) -> Self::Output {
                self.$op(Self::splat(*rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<Vector<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: Vector<N, T, A>) -> Self::Output {
                Vector::$op(*self, rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<T> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: T) -> Self::Output {
                Vector::$op(*self, Vector::splat(rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&Vector<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &Vector<N, T, A>) -> Self::Output {
                Vector::$op(*self, *rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&T> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op(self, rhs: &T) -> Self::Output {
                Vector::$op(*self, Vector::splat(*rhs))
            }
        }
    };
}
impl_binary_operator!(
    Add,
    add,
    vector_add,
    /// Performs the `+` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a + Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 + 4, 2 + 5, 3 + 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `+` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a + 4;
    /// assert_eq!(b, Vec3::new(1 + 4, 2 + 4, 3 + 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector + splat(scalar)`.
);
impl_binary_operator!(
    Sub,
    sub,
    vector_sub,
    /// Performs the `-` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a - Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 - 4, 2 - 5, 3 - 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `-` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a - 4;
    /// assert_eq!(b, Vec3::new(1 - 4, 2 - 4, 3 - 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector - splat(scalar)`.
);
impl_binary_operator!(
    Mul,
    mul,
    vector_mul,
    /// Performs the `*` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a * Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 * 4, 2 * 5, 3 * 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `*` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a * 4;
    /// assert_eq!(b, Vec3::new(1 * 4, 2 * 4, 3 * 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector * splat(scalar)`.
);
impl_binary_operator!(
    Div,
    div,
    vector_div,
    /// Performs the `/` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 10, 12);
    /// let b = a / Vec3::new(2, 5, 3);
    /// assert_eq!(b, Vec3::new(8 / 2, 10 / 5, 12 / 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ,
    /// Performs the `/` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 10, 12);
    /// let b = a / 2;
    /// assert_eq!(b, Vec3::new(8 / 2, 10 / 2, 12 / 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector / splat(scalar)`.
);
impl_binary_operator!(
    Rem,
    rem,
    vector_rem,
    /// Performs the `%` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(5, 7, 9);
    /// let b = a % Vec3::new(2, 3, 4);
    /// assert_eq!(b, Vec3::new(5 % 2, 7 % 3, 9 % 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ,
    /// Performs the `%` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(5, 7, 9);
    /// let b = a % 2;
    /// assert_eq!(b, Vec3::new(5 % 2, 7 % 2, 9 % 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ///
    /// This operation is fully consistent with `vector % splat(scalar)`.
);
impl_binary_operator!(
    Shl,
    shl,
    vector_shl,
    /// Performs the `<<` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a << Vec3::new(1, 2, 3);
    /// assert_eq!(b, Vec3::new(1 << 1, 2 << 2, 3 << 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ,
    /// Performs the `<<` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a << 1;
    /// assert_eq!(b, Vec3::new(1 << 1, 2 << 1, 3 << 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector << splat(scalar)`.
);
impl_binary_operator!(
    Shr,
    shr,
    vector_shr,
    /// Performs the `>>` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 16, 32);
    /// let b = a >> Vec3::new(1, 2, 3);
    /// assert_eq!(b, Vec3::new(8 >> 1, 16 >> 2, 32 >> 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ,
    /// Performs the `>>` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(8, 16, 32);
    /// let b = a >> 1;
    /// assert_eq!(b, Vec3::new(8 >> 1, 16 >> 1, 32 >> 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector >> splat(scalar)`.
);
impl_binary_operator!(
    BitAnd,
    bitand,
    vector_bitand,
    /// Performs the `&` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a & Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 & 4, 2 & 5, 3 & 6));
    /// ```
    ,
    /// Performs the `&` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a & 4;
    /// assert_eq!(b, Vec3::new(1 & 4, 2 & 4, 3 & 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector & splat(scalar)`.
);
impl_binary_operator!(
    BitOr,
    bitor,
    vector_bitor,
    /// Performs the `|` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a | Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 | 4, 2 | 5, 3 | 6));
    /// ```
    ,
    /// Performs the `|` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a | 4;
    /// assert_eq!(b, Vec3::new(1 | 4, 2 | 4, 3 | 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector | splat(scalar)`.
);
impl_binary_operator!(
    BitXor,
    bitxor,
    vector_bitxor,
    /// Performs the `^` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a ^ Vec3::new(4, 5, 6);
    /// assert_eq!(b, Vec3::new(1 ^ 4, 2 ^ 5, 3 ^ 6));
    /// ```
    ,
    /// Performs the `^` operation for each vector element and the scalar `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1, 2, 3);
    /// let b = a ^ 4;
    /// assert_eq!(b, Vec3::new(1 ^ 4, 2 ^ 4, 3 ^ 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector ^ splat(scalar)`.
);

macro_rules! impl_assign_operator {
    ($Op:ident, $OpAssign:ident, $op_assign:ident, $op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
        impl<const N: usize, T, A: Alignment> $OpAssign for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: T) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<&Vector<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: &Vector<N, T, A>) {
                *self = self.$op(*rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<&T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            $(#[$doc_scalar])*
            #[inline]
            #[track_caller]
            fn $op_assign(&mut self, rhs: &T) {
                *self = self.$op(*rhs);
            }
        }
    };
}
impl_assign_operator!(
    Add,
    AddAssign,
    add_assign,
    add,
    /// Performs the `+=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector += Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 + 4, 2 + 5, 3 + 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector + vector`.
    ,
    /// Performs the `+=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector += 4;
    /// assert_eq!(vector, Vec3::new(1 + 4, 2 + 4, 3 + 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector + vector`.
);
impl_assign_operator!(
    Sub,
    SubAssign,
    sub_assign,
    sub,
    /// Performs the `-=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector -= Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(5 - 1, 7 - 2, 9 - 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector - vector`.
    ,
    /// Performs the `-=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector -= 2;
    /// assert_eq!(vector, Vec3::new(5 - 2, 7 - 2, 9 - 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector - vector`.
);
impl_assign_operator!(
    Mul,
    MulAssign,
    mul_assign,
    mul,
    /// Performs the `*=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector *= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 * 4, 2 * 5, 3 * 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector * vector`.
    ,
    /// Performs the `*=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector *= 4;
    /// assert_eq!(vector, Vec3::new(1 * 4, 2 * 4, 3 * 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector * vector`.
);
impl_assign_operator!(
    Div,
    DivAssign,
    div_assign,
    div,
    /// Performs the `/=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(8, 10, 12);
    /// vector /= Vec3::new(2, 5, 3);
    /// assert_eq!(vector, Vec3::new(8 / 2, 10 / 5, 12 / 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector / vector`.
    ,
    /// Performs the `/=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(8, 10, 12);
    /// vector /= 2;
    /// assert_eq!(vector, Vec3::new(8 / 2, 10 / 2, 12 / 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `vector / vector`.
);
impl_assign_operator!(
    Rem,
    RemAssign,
    rem_assign,
    rem,
    /// Performs the `%=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector %= Vec3::new(2, 3, 4);
    /// assert_eq!(vector, Vec3::new(5 % 2, 7 % 3, 9 % 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ///
    /// This operation is fully consistent with `vector % vector`.
    ,
    /// Performs the `%=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(5, 7, 9);
    /// vector %= 2;
    /// assert_eq!(vector, Vec3::new(5 % 2, 7 % 2, 9 % 2));
    /// ```
    ///
    /// # Consistency
    ///
    /// For integers this operation is fully consistent with the scalar
    /// operation, including panics.
    ///
    /// For floats this operation may be inconsistent with the scalar operation,
    /// regarding precision and NaN propagation.
    ///
    /// This operation is fully consistent with `vector % vector`.
);
impl_assign_operator!(
    Shl,
    ShlAssign,
    shl_assign,
    shl,
    /// Performs the `<<=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector <<= Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(1 << 1, 2 << 2, 3 << 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector << vector`.
    ,
    /// Performs the `<<=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector <<= 1;
    /// assert_eq!(vector, Vec3::new(1 << 1, 2 << 1, 3 << 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector << vector`.
);
impl_assign_operator!(
    Shr,
    ShrAssign,
    shr_assign,
    shr,
    /// Performs the `>>=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(8, 16, 32);
    /// vector >>= Vec3::new(1, 2, 3);
    /// assert_eq!(vector, Vec3::new(8 >> 1, 16 >> 2, 32 >> 3));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector >> vector`.
    ,
    /// Performs the `>>=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(8, 16, 32);
    /// vector >>= 1;
    /// assert_eq!(vector, Vec3::new(8 >> 1, 16 >> 1, 32 >> 1));
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
    ///
    /// This operation is fully consistent with `vector >> vector`.
);
impl_assign_operator!(
    BitAnd,
    BitAndAssign,
    bitand_assign,
    bitand,
    /// Performs the `&=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector &= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 & 4, 2 & 5, 3 & 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector & vector`.
    ,
    /// Performs the `&=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector &= 4;
    /// assert_eq!(vector, Vec3::new(1 & 4, 2 & 4, 3 & 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector & vector`.
);
impl_assign_operator!(
    BitOr,
    BitOrAssign,
    bitor_assign,
    bitor,
    /// Performs the `|=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector |= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 | 4, 2 | 5, 3 | 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector | vector`.
    ,
    /// Performs the `|=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector |= 4;
    /// assert_eq!(vector, Vec3::new(1 | 4, 2 | 4, 3 | 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector | vector`.
);
impl_assign_operator!(
    BitXor,
    BitXorAssign,
    bitxor_assign,
    bitxor,
    /// Performs the `^=` operation for each vector element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector ^= Vec3::new(4, 5, 6);
    /// assert_eq!(vector, Vec3::new(1 ^ 4, 2 ^ 5, 3 ^ 6));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector ^ vector`.
    ,
    /// Performs the `^=` operation for each vector element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let mut vector = Vec3::new(1, 2, 3);
    /// vector ^= 4;
    /// assert_eq!(vector, Vec3::new(1 ^ 4, 2 ^ 4, 3 ^ 4));
    /// ```
    ///
    /// # Consistency
    ///
    /// This operation is fully consistent with `vector ^ vector`.
);

// SAFETY: Vectors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Send` the list also is, and the padding is `Send` too.
unsafe impl<const N: usize, T, A: Alignment> Send for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Send,
{
}

// SAFETY: Vectors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::{format, vec::Vec};

    use crate::{
        Aligned, Mask, Matrix, Unaligned, Vec2, Vec2A, Vec3, Vec3A, Vec4, Vec4A, Vector,
        test_utils::{assert_panic, assert_panic_test_eq, assert_test_eq, for_types, random_iter},
        utils::{Repr2, Repr3, Repr4},
    };

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Vec2A<T>>(), size_of::<T>() * 2);
            assert!(
                align_of::<Vec2A<T>>() == align_of::<T>()
                    || align_of::<Vec2A<T>>() == size_of::<T>() * 2
            );

            assert!(
                size_of::<Vec3A<T>>() == size_of::<T>() * 3
                    && align_of::<Vec3A<T>>() == align_of::<T>()
                    || size_of::<Vec3A<T>>() == size_of::<T>() * 4
                        && align_of::<Vec3A<T>>() == size_of::<T>() * 4
            );

            assert_eq!(size_of::<Vec4A<T>>(), size_of::<T>() * 4);
            assert!(
                align_of::<Vec4A<T>>() == align_of::<T>()
                    || align_of::<Vec4A<T>>() == size_of::<T>() * 4
            );

            assert_eq!(size_of::<Vec2<T>>(), size_of::<T>() * 2);
            assert_eq!(align_of::<Vec2<T>>(), align_of::<T>());

            assert_eq!(size_of::<Vec3<T>>(), size_of::<T>() * 3);
            assert_eq!(align_of::<Vec3<T>>(), align_of::<T>());

            assert_eq!(size_of::<Vec4<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Vec4<T>>(), align_of::<T>());
        });
    }

    #[test]
    fn test_zero() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(Vector::<N, T, A>::ZERO, Vector::splat(T::as_from(0)));
        });
    }

    #[test]
    fn test_one() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(Vector::<N, T, A>::ONE, Vector::splat(T::as_from(1)));
        });
    }

    #[test]
    fn test_neg_one() {
        for_types!(|N, T: PrimitiveFloat, A| {
            assert_eq!(Vector::<N, T, A>::NEG_ONE, Vector::splat(-1.0));
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            assert_eq!(Vector::<N, T, A>::NEG_ONE, Vector::splat(-1));
        });
    }

    #[test]
    fn test_from_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::from_array([x, y]),
                Vector::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Vector::<3, T, A>::from_array([x, y, z]),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<4, T, A>::from_array([x, y, z, w]),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_splat() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let x = T::as_from(5);

            assert_eq!(Vector::<N, T, A>::splat(x), Vector::from_array([x; N]));
        });
    }

    #[test]
    fn test_from_fn() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vector::<N, T, A>::from_fn(|i| array[i]),
                Vector::from_array(array)
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let vector = Vector::<N, T, A>::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                vector.to_alignment(),
                Vector::<N, T, Aligned>::from_array(vector.to_array())
            );
            assert_eq!(
                vector.to_alignment(),
                Vector::<N, T, Unaligned>::from_array(vector.to_array())
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let vector = Vector::<N, T, A>::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                vector.align(),
                Vector::<N, T, Aligned>::from_array(vector.to_array())
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let vector = Vector::<N, T, A>::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                vector.unalign(),
                Vector::<N, T, Unaligned>::from_array(vector.to_array())
            );
        });
    }

    #[test]
    fn test_to_array() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(Vector::<N, T, A>::from_array(array).to_array(), array);
        });
    }

    #[test]
    fn test_as_array() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(Vector::<N, T, A>::from_array(array).as_array(), &array);
        });
    }

    #[test]
    fn test_as_mut_array() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let mut array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vector::<N, T, A>::from_array(array).as_mut_array(),
                &mut array
            );
        });
    }

    #[test]
    fn test_iter() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vec::from_iter(Vector::<N, T, A>::from_array(array).iter()),
                Vec::from(array)
            );
        });
    }

    #[test]
    fn test_iter_mut() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let mut array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vec::from_iter(Vector::<N, T, A>::from_array(array).iter_mut()),
                Vec::from_iter(array.iter_mut())
            );
        });
    }

    #[test]
    fn test_map() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vector::<N, T, A>::from_array(array).map(T::as_to),
                Vector::<N, usize, A>::from_array(array.map(T::as_to))
            );
        });
    }

    #[test]
    fn test_reverse() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).reverse(),
                Vector::<2, T, A>::new(y, x)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).reverse(),
                Vector::<3, T, A>::new(z, y, x)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).reverse(),
                Vector::<4, T, A>::new(w, z, y, x)
            );
        });
    }

    #[test]
    fn test_element_sum() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in [0.0, -0.0]
                .into_iter()
                .flat_map(|x| [0.0, -0.0].map(|y| [x, y]))
                .flat_map(|[x, y]| [0.0, -0.0].map(|z| [x, y, z]))
                .flat_map(|[x, y, z]| [0.0, -0.0].map(|w| [x, y, z, w]))
                .map(Vector::<4, T, A>::from_array)
                .chain(random_iter())
            {
                let [x, y, z, w] = vector.to_array();

                assert_test_eq!(vector.xy().element_sum(), x + y);
                assert_test_eq!(vector.xyz().element_sum(), x + y + z);
                assert_test_eq!(vector.element_sum(), x + y + (z + w));
            }
        });
        for_types!(|T: PrimitiveInteger, A| {
            for vector in random_iter::<Vector<4, T, A>>() {
                let [x, y, z, w] = vector.to_array();

                assert_panic_test_eq!(vector.xy().element_sum(), x + y);
                assert_panic_test_eq!(vector.xyz().element_sum(), x + y + z);
                assert_panic_test_eq!(vector.element_sum(), x + y + (z + w));
            }
        });
    }

    #[test]
    fn test_element_product() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in [0.0, -0.0, 1.0, -1.0]
                .into_iter()
                .flat_map(|x| [0.0, -0.0, 1.0, -1.0].map(|y| [x, y]))
                .flat_map(|[x, y]| [0.0, -0.0, 1.0, -1.0].map(|z| [x, y, z]))
                .flat_map(|[x, y, z]| [0.0, -0.0, 1.0, -1.0].map(|w| [x, y, z, w]))
                .map(Vector::<4, T, A>::from_array)
                .chain(random_iter())
            {
                let [x, y, z, w] = vector.to_array();

                assert_test_eq!(vector.xy().element_product(), x * y);
                assert_test_eq!(vector.xyz().element_product(), x * y * z);
                assert_test_eq!(
                    vector.element_product(),
                    x * y * (z * w),
                    "  vector: {vector:?}"
                );
            }
        });
        for_types!(|T: PrimitiveInteger, A| {
            for vector in random_iter::<Vector<4, T, A>>() {
                let [x, y, z, w] = vector.to_array();

                assert_panic_test_eq!(vector.xy().element_product(), x * y);
                assert_panic_test_eq!(vector.xyz().element_product(), x * y * z);
                assert_panic_test_eq!(vector.element_product(), x * y * (z * w));
            }
        });
    }

    #[test]
    fn test_eq_mask() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(
                    vector.eq_mask(other),
                    Mask::from_fn(|i| vector[i] == other[i])
                );
            }
        });
    }

    #[test]
    fn test_ne_mask() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(
                    vector.ne_mask(other),
                    Mask::from_fn(|i| vector[i] != other[i])
                );
            }
        });
    }

    #[test]
    fn test_lt_mask() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(
                    vector.lt_mask(other),
                    Mask::from_fn(|i| vector[i] < other[i])
                );
            }
        });
    }

    #[test]
    fn test_gt_mask() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(
                    vector.gt_mask(other),
                    Mask::from_fn(|i| vector[i] > other[i])
                );
            }
        });
    }

    #[test]
    fn test_le_mask() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(
                    vector.le_mask(other),
                    Mask::from_fn(|i| vector[i] <= other[i])
                );
            }
        });
    }

    #[test]
    fn test_ge_mask() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(
                    vector.ge_mask(other),
                    Mask::from_fn(|i| vector[i] >= other[i])
                );
            }
        });
    }

    #[test]
    fn test_dot() {
        for_types!(|T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<4, T, A>; 2]>() {
                let [x1, y1, z1, w1] = vector.to_array();
                let [x2, y2, z2, w2] = other.to_array();

                assert_test_eq!(vector.xy().dot(other.xy()), x1 * x2 + y1 * y2);
                assert_test_eq!(vector.xyz().dot(other.xyz()), x1 * x2 + y1 * y2 + z1 * z2);
                assert_test_eq!(vector.dot(other), x1 * x2 + y1 * y2 + (z1 * z2 + w1 * w2));
            }
        });
        for_types!(|T: PrimitiveInteger, A| {
            for [vector, other] in random_iter::<[Vector<4, T, A>; 2]>() {
                let [x1, y1, z1, w1] = vector.to_array();
                let [x2, y2, z2, w2] = other.to_array();

                assert_panic_test_eq!(vector.xy().dot(other.xy()), x1 * x2 + y1 * y2);
                assert_panic_test_eq!(vector.xyz().dot(other.xyz()), x1 * x2 + y1 * y2 + z1 * z2);
                assert_panic_test_eq!(vector.dot(other), x1 * x2 + y1 * y2 + (z1 * z2 + w1 * w2));
            }
        });
    }

    #[test]
    fn test_length_squared() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<4, T, A>>() {
                let [x, y, z, w] = vector.to_array();

                assert_test_eq!(vector.xy().length_squared(), x * x + y * y);
                assert_test_eq!(vector.xyz().length_squared(), x * x + y * y + z * z);
                assert_test_eq!(vector.length_squared(), x * x + y * y + (z * z + w * w));
            }
        });
        for_types!(|T: PrimitiveInteger, A| {
            for vector in random_iter::<Vector<4, T, A>>() {
                let [x, y, z, w] = vector.to_array();

                assert_panic_test_eq!(vector.xy().length_squared(), x * x + y * y);
                assert_panic_test_eq!(vector.xyz().length_squared(), x * x + y * y + z * z);
                assert_panic_test_eq!(vector.length_squared(), x * x + y * y + (z * z + w * w));
            }
        });
    }

    #[test]
    fn test_distance_squared() {
        for_types!(|T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<4, T, A>; 2]>() {
                let [x1, y1, z1, w1] = vector.to_array();
                let [x2, y2, z2, w2] = other.to_array();

                assert_test_eq!(
                    vector.xy().distance_squared(other.xy()),
                    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
                );
                assert_test_eq!(
                    vector.xyz().distance_squared(other.xyz()),
                    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)
                );
                assert_test_eq!(
                    vector.distance_squared(other),
                    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
                        + ((z1 - z2) * (z1 - z2) + (w1 - w2) * (w1 - w2))
                );
            }
        });
        for_types!(|T: PrimitiveSigned, A| {
            for [vector, other] in random_iter::<[Vector<4, T, A>; 2]>() {
                let [x1, y1, z1, w1] = vector.to_array();
                let [x2, y2, z2, w2] = other.to_array();

                assert_panic_test_eq!(
                    vector.xy().distance_squared(other.xy()),
                    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
                );
                assert_panic_test_eq!(
                    vector.xyz().distance_squared(other.xyz()),
                    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)
                );
                assert_panic_test_eq!(
                    vector.distance_squared(other),
                    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
                        + ((z1 - z2) * (z1 - z2) + (w1 - w2) * (w1 - w2))
                );
            }
        });
    }

    #[test]
    fn test_from_inner() {
        assert_eq!(Vec2::<u32>::from_inner(Repr2(0, 1)), Vec2::new(0, 1));
        assert_eq!(Vec3::<u32>::from_inner(Repr3(0, 1, 2)), Vec3::new(0, 1, 2));
        assert_eq!(
            Vec4::<u32>::from_inner(Repr4(0, 1, 2, 3)),
            Vec4::new(0, 1, 2, 3)
        );
    }

    #[test]
    fn test_inner() {
        assert_eq!(Vec2::<u32>::new(0, 1).inner(), Repr2(0, 1));
        assert_eq!(Vec3::<u32>::new(0, 1, 2).inner(), Repr3(0, 1, 2));
        assert_eq!(Vec4::<u32>::new(0, 1, 2, 3).inner(), Repr4(0, 1, 2, 3));
    }

    #[test]
    fn test_inner_mut() {
        assert_eq!(Vec2::<u32>::new(0, 1).inner_mut(), &mut Repr2(0, 1));
        assert_eq!(Vec3::<u32>::new(0, 1, 2).inner_mut(), &mut Repr3(0, 1, 2));
        assert_eq!(
            Vec4::<u32>::new(0, 1, 2, 3).inner_mut(),
            &mut Repr4(0, 1, 2, 3)
        );
    }

    #[test]
    fn test_axes() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Vector::<2, T, A>::X,
                Vector::<2, T, A>::new(T::as_from(1), T::as_from(0))
            );
            assert_eq!(
                Vector::<2, T, A>::Y,
                Vector::<2, T, A>::new(T::as_from(0), T::as_from(1))
            );

            assert_eq!(
                Vector::<3, T, A>::X,
                Vector::<3, T, A>::new(T::as_from(1), T::as_from(0), T::as_from(0))
            );
            assert_eq!(
                Vector::<3, T, A>::Y,
                Vector::<3, T, A>::new(T::as_from(0), T::as_from(1), T::as_from(0))
            );
            assert_eq!(
                Vector::<3, T, A>::Z,
                Vector::<3, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(1))
            );

            assert_eq!(
                Vector::<4, T, A>::X,
                Vector::<4, T, A>::new(T::as_from(1), T::as_from(0), T::as_from(0), T::as_from(0))
            );
            assert_eq!(
                Vector::<4, T, A>::Y,
                Vector::<4, T, A>::new(T::as_from(0), T::as_from(1), T::as_from(0), T::as_from(0))
            );
            assert_eq!(
                Vector::<4, T, A>::Z,
                Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(1), T::as_from(0))
            );
            assert_eq!(
                Vector::<4, T, A>::W,
                Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(0), T::as_from(1))
            );
        });
    }

    #[test]
    fn test_neg_axes() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::NEG_X, Vector::<2, T, A>::new(-1.0, 0.0));
            assert_eq!(Vector::<2, T, A>::NEG_Y, Vector::<2, T, A>::new(0.0, -1.0));

            assert_eq!(
                Vector::<3, T, A>::NEG_X,
                Vector::<3, T, A>::new(-1.0, 0.0, 0.0)
            );
            assert_eq!(
                Vector::<3, T, A>::NEG_Y,
                Vector::<3, T, A>::new(0.0, -1.0, 0.0)
            );
            assert_eq!(
                Vector::<3, T, A>::NEG_Z,
                Vector::<3, T, A>::new(0.0, 0.0, -1.0)
            );

            assert_eq!(
                Vector::<4, T, A>::NEG_X,
                Vector::<4, T, A>::new(-1.0, 0.0, 0.0, 0.0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Y,
                Vector::<4, T, A>::new(0.0, -1.0, 0.0, 0.0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Z,
                Vector::<4, T, A>::new(0.0, 0.0, -1.0, 0.0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_W,
                Vector::<4, T, A>::new(0.0, 0.0, 0.0, -1.0)
            );
        });
        for_types!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::NEG_X, Vector::<2, T, A>::new(-1, 0));
            assert_eq!(Vector::<2, T, A>::NEG_Y, Vector::<2, T, A>::new(0, -1));

            assert_eq!(Vector::<3, T, A>::NEG_X, Vector::<3, T, A>::new(-1, 0, 0));
            assert_eq!(Vector::<3, T, A>::NEG_Y, Vector::<3, T, A>::new(0, -1, 0));
            assert_eq!(Vector::<3, T, A>::NEG_Z, Vector::<3, T, A>::new(0, 0, -1));

            assert_eq!(
                Vector::<4, T, A>::NEG_X,
                Vector::<4, T, A>::new(-1, 0, 0, 0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Y,
                Vector::<4, T, A>::new(0, -1, 0, 0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_Z,
                Vector::<4, T, A>::new(0, 0, -1, 0)
            );
            assert_eq!(
                Vector::<4, T, A>::NEG_W,
                Vector::<4, T, A>::new(0, 0, 0, -1)
            );
        });
    }

    #[test]
    fn test_extend() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).extend(z),
                Vector::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).extend(w),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_perp() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_eq!(Vector::<2, T, A>::X.perp(), Vector::<2, T, A>::Y);
            assert_eq!(Vector::<2, T, A>::Y.perp(), Vector::<2, T, A>::NEG_X);
            assert_eq!(Vector::<2, T, A>::NEG_X.perp(), Vector::<2, T, A>::NEG_Y);
            assert_eq!(Vector::<2, T, A>::NEG_Y.perp(), Vector::<2, T, A>::X);
        });
        for_types!(|T: PrimitiveSigned, A| {
            assert_eq!(Vector::<2, T, A>::X.perp(), Vector::<2, T, A>::Y);
            assert_eq!(Vector::<2, T, A>::Y.perp(), Vector::<2, T, A>::NEG_X);
            assert_eq!(Vector::<2, T, A>::NEG_X.perp(), Vector::<2, T, A>::NEG_Y);
            assert_eq!(Vector::<2, T, A>::NEG_Y.perp(), Vector::<2, T, A>::X);
        });
    }

    #[test]
    fn test_wedge() {
        for_types!(|T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<2, T, A>; 2]>() {
                assert_test_eq!(
                    vector.wedge(other),
                    Matrix::from_rows(&[vector, other]).determinant()
                );
            }
        });
        for_types!(|T: PrimitiveSigned, A| {
            for [vector, other] in random_iter::<[Vector<2, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector.wedge(other),
                    Matrix::from_rows(&[vector, other]).determinant()
                );
            }
        });
    }

    #[test]
    fn test_truncate() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).truncate(),
                Vector::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).truncate(),
                Vector::<3, T, A>::new(x, y, z)
            );
        });
    }

    #[test]
    fn test_to_homogeneous() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z] = std::array::from_fn(T::as_from);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).to_homogeneous(),
                Vector::<3, T, A>::new(x, y, T::ONE)
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).to_homogeneous(),
                Vector::<4, T, A>::new(x, y, z, T::ONE)
            );
        });
    }

    #[test]
    fn test_cross() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Vector::<3, T, A>::X.cross(Vector::<3, T, A>::Y),
                Vector::<3, T, A>::Z
            );
            assert_eq!(
                Vector::<3, T, A>::Y.cross(Vector::<3, T, A>::Z),
                Vector::<3, T, A>::X
            );
            assert_eq!(
                Vector::<3, T, A>::Z.cross(Vector::<3, T, A>::X),
                Vector::<3, T, A>::Y
            );

            for a in [
                Vector::<3, T, A>::X,
                Vector::<3, T, A>::Y,
                Vector::<3, T, A>::Z,
            ] {
                assert_eq!(a.cross(a), Vector::ZERO);

                for b in [
                    Vector::<3, T, A>::X,
                    Vector::<3, T, A>::Y,
                    Vector::<3, T, A>::Z,
                ] {
                    assert_eq!(b.cross(a), -a.cross(b));
                    assert_eq!((-a).cross(b), -a.cross(b));
                    assert_eq!(a.cross(-b), -a.cross(b));
                }
            }
        });
        for_types!(|T: PrimitiveSigned, A| {
            assert_eq!(
                Vector::<3, T, A>::X.cross(Vector::<3, T, A>::Y),
                Vector::<3, T, A>::Z
            );
            assert_eq!(
                Vector::<3, T, A>::Y.cross(Vector::<3, T, A>::Z),
                Vector::<3, T, A>::X
            );
            assert_eq!(
                Vector::<3, T, A>::Z.cross(Vector::<3, T, A>::X),
                Vector::<3, T, A>::Y
            );

            for a in [
                Vector::<3, T, A>::X,
                Vector::<3, T, A>::Y,
                Vector::<3, T, A>::Z,
            ] {
                assert_eq!(a.cross(a), Vector::ZERO);

                for b in [
                    Vector::<3, T, A>::X,
                    Vector::<3, T, A>::Y,
                    Vector::<3, T, A>::Z,
                ] {
                    assert_eq!(b.cross(a), -a.cross(b));
                    assert_eq!((-a).cross(b), -a.cross(b));
                    assert_eq!(a.cross(-b), -a.cross(b));
                }
            }
        });
    }

    #[test]
    fn test_index() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let vector = Vector::<N, T, A>::from_fn(|i| T::as_from(i + 1));

            for i in 0..N {
                assert_eq!(vector[i], vector.to_array()[i]);
            }
            assert_panic!(vector[N]);
            assert_panic!(vector[N + 1]);
        });
    }

    #[test]
    #[expect(clippy::clone_on_copy)]
    fn test_index_mut() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let vector = Vector::<N, T, A>::from_fn(|i| T::as_from(i + 1));

            for i in 0..N {
                assert_eq!(&mut vector.clone()[i], &mut vector.to_array()[i]);
            }
            assert_panic!(&mut vector.clone()[N]);
            assert_panic!(&mut vector.clone()[N + 1]);
        });
    }

    #[test]
    fn test_into_iter() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vec::from_iter(Vector::<N, T, A>::from_array(array).into_iter()),
                Vec::from(array)
            );
        });
    }

    #[test]
    fn test_mut_into_iter() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let mut array = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Vec::from_iter((&mut Vector::<N, T, A>::from_array(array)).into_iter()),
                Vec::from_iter(array.iter_mut())
            );
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Vector::<2, T, A>::new(x, y).x, x);
            assert_eq!(Vector::<2, T, A>::new(x, y).y, y);

            assert_eq!(Vector::<3, T, A>::new(x, y, z).x, x);
            assert_eq!(Vector::<3, T, A>::new(x, y, z).y, y);
            assert_eq!(Vector::<3, T, A>::new(x, y, z).z, z);

            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).x, x);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).y, y);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).z, z);
            assert_eq!(Vector::<4, T, A>::new(x, y, z, w).w, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(&mut Vector::<2, T, A>::new(x, y).x, &mut x);
            assert_eq!(&mut Vector::<2, T, A>::new(x, y).y, &mut y);

            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z).x, &mut x);
            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z).y, &mut y);
            assert_eq!(&mut Vector::<3, T, A>::new(x, y, z).z, &mut z);

            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).x, &mut x);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).y, &mut y);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).z, &mut z);
            assert_eq!(&mut Vector::<4, T, A>::new(x, y, z, w).w, &mut w);
        });
    }

    #[test]
    fn test_from_tuples() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));
            let xy = Vector::<2, T, A>::new(x, y);
            let xyz = Vector::<3, T, A>::new(x, y, z);
            let yz = Vector::<2, T, A>::new(y, z);
            let xyzw = Vector::<4, T, A>::new(x, y, z, w);
            let zw = Vector::<2, T, A>::new(z, w);
            let yzw = Vector::<3, T, A>::new(y, z, w);

            assert_eq!(Vector::<2, T, A>::from((x, y)), xy);

            assert_eq!(Vector::<3, T, A>::from((x, y, z)), xyz);
            assert_eq!(Vector::<3, T, A>::from((x, yz)), xyz);
            assert_eq!(Vector::<3, T, A>::from((xy, z)), xyz);

            assert_eq!(Vector::<4, T, A>::from((x, y, z, w)), xyzw);
            assert_eq!(Vector::<4, T, A>::from((x, y, zw)), xyzw);
            assert_eq!(Vector::<4, T, A>::from((x, yz, w)), xyzw);
            assert_eq!(Vector::<4, T, A>::from((x, yzw)), xyzw);
            assert_eq!(Vector::<4, T, A>::from((xy, z, w)), xyzw);
            assert_eq!(Vector::<4, T, A>::from((xy, zw)), xyzw);
            assert_eq!(Vector::<4, T, A>::from((xyz, w)), xyzw);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{:?}", Vector::<2, T, A>::new(x, y)),
                format!("({x:?}, {y:?})")
            );
            assert_eq!(
                format!("{:?}", Vector::<3, T, A>::new(x, y, z)),
                format!("({x:?}, {y:?}, {z:?})")
            );
            assert_eq!(
                format!("{:?}", Vector::<4, T, A>::new(x, y, z, w)),
                format!("({x:?}, {y:?}, {z:?}, {w:?})")
            );
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{}", Vector::<2, T, A>::new(x, y)),
                format!("({x}, {y})")
            );
            assert_eq!(
                format!("{}", Vector::<3, T, A>::new(x, y, z)),
                format!("({x}, {y}, {z})")
            );
            assert_eq!(
                format!("{}", Vector::<4, T, A>::new(x, y, z, w)),
                format!("({x}, {y}, {z}, {w})")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(vector == other, vector.to_array() == other.to_array());
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [vector, other] in random_iter::<([Vector<N, T, A>; 2], Mask<N, T, A>)>()
                .map(|([vector, other], mask)| [vector, mask.select(vector, other)])
            {
                assert_eq!(vector != other, vector.to_array() != other.to_array());
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(Vector::<N, T, A>::default(), Vector::splat(T::default()));
        });
    }

    #[test]
    fn test_neg() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(-vector, vector.map(|x| -x));
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_panic_test_eq!(-vector, vector.map(|x| -x));
            }
        });
    }

    #[test]
    fn test_not() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_panic_test_eq!(!vector, vector.map(|x| !x));
            }
        });
    }

    #[test]
    fn test_add() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector_a + vector_b,
                    Vector::from_fn(|i| vector_a[i] + vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[2, 3], [T::MAX - 1, 3], [T::MAX - 1, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a + vector_b,
                    Vector::from_fn(|i| vector_a[i] + vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MIN + 1, -3], [T::MIN + 1, -1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a + vector_b,
                    Vector::from_fn(|i| vector_a[i] + vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_sub() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector_a - vector_b,
                    Vector::from_fn(|i| vector_a[i] - vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MIN + 1, 3], [T::MIN + 1, 1], [T::MIN, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a - vector_b,
                    Vector::from_fn(|i| vector_a[i] - vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -3], [T::MAX - 1, -1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a - vector_b,
                    Vector::from_fn(|i| vector_a[i] - vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_mul() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector_a * vector_b,
                    Vector::from_fn(|i| vector_a[i] * vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in [[3, 2], [T::MAX - 1, 2], [T::MAX, 1], [T::MAX, 0]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a * vector_b,
                    Vector::from_fn(|i| vector_a[i] * vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX - 1, -2], [T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a * vector_b,
                    Vector::from_fn(|i| vector_a[i] * vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_div() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector_a / vector_b,
                    Vector::from_fn(|i| vector_a[i] / vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveUnsigned, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a / vector_b,
                    Vector::from_fn(|i| vector_a[i] / vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
                .chain(random_iter())
            {
                assert_panic_test_eq!(
                    vector_a / vector_b,
                    Vector::from_fn(|i| vector_a[i] / vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_rem() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Vector<N, T, A>; 2]>() {
                let [a, b] = [a, b].map(|v| {
                    (v.gt_mask(Vector::splat(0.1)) & v.lt_mask(Vector::splat(1e4))
                        | !v.finite_mask())
                    .select(v, Vector::ZERO)
                });

                assert_test_eq!(
                    a % b,
                    Vector::from_fn(|i| a[i] % b[i]),
                    abs <= (a * b).abs().map(|x| x.max(1.0)) * 1e-4,
                    INFINITY = NAN
                );
            }
        });
        for_types!(|N, T: PrimitiveUnsigned, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a % vector_b,
                    Vector::from_fn(|i| vector_a[i] % vector_b[i])
                );
            }
        });
        for_types!(|N, T: PrimitiveSigned, A| {
            for [vector_a, vector_b] in [[T::MAX, -1], [T::MIN, -1]]
                .into_iter()
                .map(|values| values.map(Vector::<N, T, A>::splat))
            {
                assert_panic_test_eq!(
                    vector_a % vector_b,
                    Vector::from_fn(|i| vector_a[i] % vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_shl() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a << vector_b,
                    Vector::from_fn(|i| vector_a[i] << vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_shr() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_panic_test_eq!(
                    vector_a >> vector_b,
                    Vector::from_fn(|i| vector_a[i] >> vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_bitand() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_eq!(
                    vector_a & vector_b,
                    Vector::from_fn(|i| vector_a[i] & vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_bitor() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_eq!(
                    vector_a | vector_b,
                    Vector::from_fn(|i| vector_a[i] | vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_bitxor() {
        for_types!(|N, T: PrimitiveInteger, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_eq!(
                    vector_a ^ vector_b,
                    Vector::from_fn(|i| vector_a[i] ^ vector_b[i])
                );
            }
        });
    }

    #[test]
    fn test_add_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, f32, A>; 2]>() {
                let mut result = vector_a;
                result += vector_b;

                assert_test_eq!(result, vector_a + vector_b);
            }
        });
    }

    #[test]
    fn test_sub_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, f32, A>; 2]>() {
                let mut result = vector_a;
                result -= vector_b;

                assert_test_eq!(result, vector_a - vector_b);
            }
        });
    }

    #[test]
    fn test_mul_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, f32, A>; 2]>() {
                let mut result = vector_a;
                result *= vector_b;

                assert_test_eq!(result, vector_a * vector_b);
            }
        });
    }

    #[test]
    fn test_div_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, f32, A>; 2]>() {
                let mut result = vector_a;
                result /= vector_b;

                assert_test_eq!(result, vector_a / vector_b);
            }
        });
    }

    #[test]
    fn test_rem_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, f32, A>; 2]>() {
                let mut result = vector_a;
                result %= vector_b;

                assert_test_eq!(result, vector_a % vector_b);
            }
        });
    }

    #[test]
    fn test_shl_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, i32, A>; 2]>() {
                assert_panic_test_eq!(
                    {
                        let mut result = vector_a;
                        result <<= vector_b;
                        result
                    },
                    vector_a << vector_b
                );
            }
        });
    }

    #[test]
    fn test_shr_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, i32, A>; 2]>() {
                assert_panic_test_eq!(
                    {
                        let mut result = vector_a;
                        result >>= vector_b;
                        result
                    },
                    vector_a >> vector_b
                );
            }
        });
    }

    #[test]
    fn test_bitand_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, i32, A>; 2]>() {
                let mut result = vector_a;
                result &= vector_b;

                assert_eq!(result, vector_a & vector_b);
            }
        });
    }

    #[test]
    fn test_bitor_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, i32, A>; 2]>() {
                let mut result = vector_a;
                result |= vector_b;

                assert_eq!(result, vector_a | vector_b);
            }
        });
    }

    #[test]
    fn test_bitxor_assign() {
        for_types!(|N, A| {
            for [vector_a, vector_b] in random_iter::<[Vector<N, i32, A>; 2]>() {
                let mut result = vector_a;
                result ^= vector_b;

                assert_eq!(result, vector_a ^ vector_b);
            }
        });
    }
}
