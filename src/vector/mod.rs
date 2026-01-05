//! Items related to [`Vector`].

use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Index, IndexMut, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use crate::{cast, transmute, transmute_align, transmute_mut, transmute_ref, vec2, vec3, vec4};

mod constructor;
mod deref;
mod dir;
mod ops;
mod primitive_apis;
mod primitive_impls;
mod swizzle;
pub use dir::*;

/// A generic vector type.
///
/// This type is parameterized by:
/// - `N` - the number of elements in the vector.
/// - `T` - the type of elements in the vector.
/// - `A` - whether or not the vector is SIMD-aligned.
///
/// ## Type Aliases
///
/// For convenience, vectors of common primitives have type aliases (e.g.,
/// [`Vec2f`](crate::f32::Vec2f), [`Vec3fA`](crate::f32::Vec3fA), and
/// [`Vec4usizeA`](crate::usize::Vec4usizeA)).
///
/// ## Layout
///
/// The layout of `Vector<N, T, A>` is controlled by `A`, which can be either
/// [`Aligned`] or [`Unaligned`].
///
/// `Vector<N, T, Unaligned>` is stored as an array, meaning:
/// - `size_of::<Self>() = size_of::<T>() * N`
/// - `align_of::<Self>() = align_of::<T>()`
///
/// `Vector<N, T, Aligned>` is SIMD-aligned when appropriate, meaning:
/// - it may have increased alignment
/// - if `N == 3`, it may have increased size
///
/// The exact underlying type of a [`Vector`] is controlled by the
/// [`ScalarBackend`] trait.
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: Alignment>(VectorRepr<N, T, A>)
where
    Length<N>: SupportedLength;

type VectorRepr<const N: usize, T, A> = <A as Alignment>::Select<
    <Length<N> as SupportedLength>::Select<
        <<T as ScalarBackend<2, Aligned>>::VectorRepr as SoundVectorRepr<2, T>>::ActualRepr,
        <<T as ScalarBackend<3, Aligned>>::VectorRepr as SoundVectorRepr<3, T>>::ActualRepr,
        <<T as ScalarBackend<4, Aligned>>::VectorRepr as SoundVectorRepr<4, T>>::ActualRepr,
    >,
    <Length<N> as SupportedLength>::Select<
        <<T as ScalarBackend<2, Unaligned>>::VectorRepr as SoundVectorRepr<2, T>>::ActualRepr,
        <<T as ScalarBackend<3, Unaligned>>::VectorRepr as SoundVectorRepr<3, T>>::ActualRepr,
        <<T as ScalarBackend<4, Unaligned>>::VectorRepr as SoundVectorRepr<4, T>>::ActualRepr,
    >,
>;

impl<const N: usize, T: Scalar, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Converts an array into a [`Vector`].
    #[inline(always)]
    pub const fn from_array(array: [T; N]) -> Self {
        unsafe {
            match N {
                2 => transmute::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    array[0], array[1],
                )),
                3 => transmute::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    array[0], array[1], array[2],
                )),
                4 => transmute::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    array[0], array[1], array[2], array[3],
                )),
                _ => unreachable!(),
            }
        }
    }

    /// Creates a [`Vector`] with all elements set to `value`.
    #[inline(always)]
    pub const fn splat(value: T) -> Self {
        unsafe {
            match N {
                2 => transmute::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    value, value,
                )),
                3 => transmute::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    value, value, value,
                )),
                4 => transmute::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    value, value, value, value,
                )),
                _ => unreachable!(),
            }
        }
    }

    /// Creates a [`Vector`] by calling function `f` for the index of each element.
    #[inline(always)]
    pub fn from_fn(mut f: impl FnMut(usize) -> T) -> Self {
        match N {
            2 => cast::<Vector<2, T, A>, Vector<N, T, A>>(vec2!(f(0), f(1))),
            3 => cast::<Vector<3, T, A>, Vector<N, T, A>>(vec3!(f(0), f(1), f(2))),
            4 => cast::<Vector<4, T, A>, Vector<N, T, A>>(vec4!(f(0), f(1), f(2), f(3))),
            _ => unreachable!(),
        }
    }

    /// Returns the number of elements in the [`Vector`].
    ///
    /// This is a staticly known constant.
    #[inline(always)]
    pub const fn len(self) -> usize {
        N
    }

    /// Returns `true` for [`A = Aligned`](Aligned) and `false` for
    /// [`A = Unaligned`](Unaligned).
    #[inline(always)]
    pub const fn is_aligned(self) -> bool {
        A::IS_ALIGNED
    }

    /// Converts the [`Vector`] into the specified [`Alignment`].
    #[inline(always)]
    pub const fn to_alignment<A2: Alignment>(self) -> Vector<N, T, A2> {
        unsafe {
            match N {
                2 => transmute_align::<Vector<N, T, A>, Vector<N, T, A2>>(self),
                3 => transmute::<Vector<3, T, A2>, Vector<N, T, A2>>(Vector::<3, T, A2>::new(
                    self.as_array_ref()[0],
                    self.as_array_ref()[1],
                    self.as_array_ref()[2],
                )),
                4 => transmute_align::<Vector<N, T, A>, Vector<N, T, A2>>(self),
                _ => unreachable!(),
            }
        }
    }

    /// Converts the [`Vector`] into a SIMD-aligned [`Vector`].
    #[inline(always)]
    pub const fn align(self) -> Vector<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the [`Vector`] into a non SIMD-aligned [`Vector`].
    #[inline(always)]
    pub const fn unalign(self) -> Vector<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts the [`Vector`] into an array.
    #[inline(always)]
    pub const fn to_array(self) -> [T; N] {
        *self.as_array_ref()
    }

    /// Returns a reference to the [`Vector`]'s elements as an array.
    #[inline(always)]
    pub const fn as_array_ref(&self) -> &[T; N] {
        unsafe { transmute_ref::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns a mutable reference to the [`Vector`]'s elements as a mutable array.
    #[inline(always)]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute_mut::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns an iterator over the [`Vector`]'s elements.
    ///
    /// This returns an iterator over `T`, not `&T`. To iterate over `&T`, use
    /// `self.as_array_ref().iter()`.
    #[inline(always)]
    pub fn iter(self) -> core::array::IntoIter<T, N> {
        self.to_array().into_iter()
    }

    /// Returns an iterator over mutable references to the [`Vector`]'s elements.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_array_mut().iter_mut()
    }

    /// Creates a new [`Vector`] by applying function `f` to each element of `self`.
    #[inline(always)]
    pub fn map<T2: Scalar>(self, f: impl Fn(T) -> T2) -> Vector<N, T2, A> {
        Vector::from_fn(|i| f(self[i]))
    }

    /// Returns the element at `index`, or `None` if `index` is out of bounds.
    #[inline(always)]
    pub const fn get(self, index: usize) -> Option<T> {
        if index < N {
            Some(self.as_array_ref()[index])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at `index`, or `None` if `index`
    /// is out of bounds.
    #[inline(always)]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            Some(&mut self.as_array_mut()[index])
        } else {
            None
        }
    }

    /// Returns a new [`Vector`] with the elements of `self` in reverse order.
    #[inline(always)]
    pub fn reverse(self) -> Self {
        Self::from_fn(|i| self[N - 1 - i])
    }

    /// Returns the underlying type contained in the [`Vector`], as defined by the
    /// [`ScalarBackend`] trait.
    ///
    /// Do not use this function outside of [`ScalarBackend`] implementations, as
    /// the underlying type of a [`Scalar`]'s [`Vector`] may change quietly.
    #[inline(always)]
    pub const fn repr(self) -> <T as ScalarBackend<N, A>>::VectorRepr
    where
        T: ScalarBackend<N, A>,
    {
        unsafe { transmute::<VectorRepr<N, T, A>, <T as ScalarBackend<N, A>>::VectorRepr>(self.0) }
    }

    /// Creates a [`Vector`] from its underlying type, as defined by the
    /// [`ScalarBackend`] trait.
    ///
    /// Do not use this function outside of [`ScalarBackend`] implementations, as
    /// the underlying type of a [`Scalar`]'s [`Vector`] may change quietly.
    #[inline(always)]
    pub const fn from_repr(repr: <T as ScalarBackend<N, A>>::VectorRepr) -> Self
    where
        T: ScalarBackend<N, A>,
    {
        Vector(unsafe {
            transmute::<<T as ScalarBackend<N, A>>::VectorRepr, VectorRepr<N, T, A>>(repr)
        })
    }
}

impl<T: Scalar, A: Alignment> Vector<2, T, A> {
    /// Returns `self` with the first element replaced with `value`.
    #[inline(always)]
    pub const fn with_x(mut self, value: T) -> Self {
        self.as_array_mut()[0] = value;
        self
    }

    /// Returns `self` with the second element replaced with `value`.
    #[inline(always)]
    pub const fn with_y(mut self, value: T) -> Self {
        self.as_array_mut()[1] = value;
        self
    }

    #[inline(always)]
    const fn new(x: T, y: T) -> Self {
        unsafe { transmute_align::<Xy<T>, Vector<2, T, A>>(Xy(x, y)) }
    }
}

impl<T: Scalar, A: Alignment> Vector<3, T, A> {
    /// Returns `self` with the first element replaced with `value`.
    #[inline(always)]
    pub const fn with_x(mut self, value: T) -> Self {
        self.as_array_mut()[0] = value;
        self
    }

    /// Returns `self` with the second element replaced with `value`.
    #[inline(always)]
    pub const fn with_y(mut self, value: T) -> Self {
        self.as_array_mut()[1] = value;
        self
    }

    /// Returns `self` with the third element replaced with `value`.
    #[inline(always)]
    pub const fn with_z(mut self, value: T) -> Self {
        self.as_array_mut()[2] = value;
        self
    }

    #[inline(always)]
    const fn new(x: T, y: T, z: T) -> Self {
        match size_of::<Vector<3, T, A>>() / size_of::<T>() {
            3 => unsafe { transmute_align::<Xyz<T>, Vector<3, T, A>>(Xyz(x, y, z)) },
            4 => unsafe { transmute_align::<Xyzw<T>, Vector<3, T, A>>(Xyzw(x, y, z, z)) },
            _ => unreachable!(),
        }
    }
}

impl<T: Scalar, A: Alignment> Vector<4, T, A> {
    /// Returns `self` with the first element replaced with `value`.
    #[inline(always)]
    pub const fn with_x(mut self, value: T) -> Self {
        self.as_array_mut()[0] = value;
        self
    }

    /// Returns `self` with the second element replaced with `value`.
    #[inline(always)]
    pub const fn with_y(mut self, value: T) -> Self {
        self.as_array_mut()[1] = value;
        self
    }

    /// Returns `self` with the third element replaced with `value`.
    #[inline(always)]
    pub const fn with_z(mut self, value: T) -> Self {
        self.as_array_mut()[2] = value;
        self
    }

    /// Returns `self` with the fourth element replaced with `value`.
    #[inline(always)]
    pub const fn with_w(mut self, value: T) -> Self {
        self.as_array_mut()[3] = value;
        self
    }

    #[inline(always)]
    const fn new(x: T, y: T, z: T, w: T) -> Self {
        unsafe { transmute_align::<Xyzw<T>, Vector<4, T, A>>(Xyzw(x, y, z, w)) }
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Clone for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Copy for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, T: Scalar, A: Alignment> From<[T; N]> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn from(value: [T; N]) -> Self {
        Self::from_array(value)
    }
}

impl<const N: usize, T: Scalar, A: Alignment> From<Vector<N, T, A>> for [T; N]
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn from(value: Vector<N, T, A>) -> Self {
        value.to_array()
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Index<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array_ref().index(index)
    }
}

impl<const N: usize, T: Scalar, A: Alignment> IndexMut<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
    }
}

impl<const N: usize, T: Scalar, A: Alignment> IntoIterator for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    type Item = T;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, const N: usize, T: Scalar, A: Alignment> IntoIterator for &'a mut Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    type Item = &'a mut T;
    type IntoIter = <&'a mut [T; N] as IntoIterator>::IntoIter;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T: Scalar + Debug, A: Alignment> Debug for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({:?}, {:?})", self[0], self[1]),
            3 => write!(f, "({:?}, {:?}, {:?})", self[0], self[1], self[2]),
            4 => write!(
                f,
                "({:?}, {:?}, {:?}, {:?})",
                self[0], self[1], self[2], self[3]
            ),
            ..2 | 5.. => panic!("unsupported vector length"),
        }
    }
}

impl<const N: usize, T: Scalar + Display, A: Alignment> Display for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "({}, {})", self[0], self[1]),
            3 => write!(f, "({}, {}, {})", self[0], self[1], self[2]),
            4 => write!(f, "({}, {}, {}, {})", self[0], self[1], self[2], self[3]),
            ..2 | 5.. => panic!("unsupported vector length"),
        }
    }
}

impl<const N: usize, T: Scalar + PartialEq, A: Alignment> PartialEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        // This is a temporary workaround to optimize the function without creating
        // breaking changes.
        if align_of::<Self>() == align_of::<T>() {
            return match N {
                2 => self[0] == other[0] && self[1] == other[1],
                3 => self[0] == other[0] && self[1] == other[1] && self[2] == other[2],
                4 => {
                    self[0] == other[0]
                        && self[1] == other[1]
                        && self[2] == other[2]
                        && self[3] == other[3]
                }
                _ => unreachable!(),
            };
        }

        specialize!(<T as ScalarBackend<N, A>>::vec_eq(*self, *other))
    }

    #[inline(always)]
    fn ne(&self, other: &Self) -> bool {
        // This is a temporary workaround to optimize the function without creating
        // breaking changes.
        if align_of::<Self>() == align_of::<T>() {
            return !(self == other);
        }

        specialize!(<T as ScalarBackend<N, A>>::vec_ne(*self, *other))
    }
}

impl<const N: usize, T: Scalar + Eq, A: Alignment> Eq for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, T: Scalar + Hash, A: Alignment> Hash for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        <[T; N] as Hash>::hash(self.as_array_ref(), state);
    }
}

impl<const N: usize, T: Scalar + Default, A: Alignment> Default for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

////////////////////////////////////////////////////////////////////////////////
// Scalar
////////////////////////////////////////////////////////////////////////////////

/// A trait for [`Vector`] element types.
///
/// To implement this trait, you must implement the [`ScalarBackend`].
///
/// ## Example
///
/// ```
/// use ggmath::vector::{Alignment, Length, Scalar, ScalarBackend, SupportedLength};
///
/// #[derive(Clone, Copy)]
/// struct MyScalar;
///
/// impl Scalar for MyScalar {}
///
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for MyScalar
/// where
///     Length<N>: SupportedLength,
/// {
///     type VectorRepr = [MyScalar; N];
/// }
///
/// // You can then use MyScalar in Vectors
/// ```
pub trait Scalar:
    Send
    + Sync
    + Copy
    + 'static
    + ScalarBackend<2, Aligned>
    + ScalarBackend<3, Aligned>
    + ScalarBackend<4, Aligned>
    + ScalarBackend<2, Unaligned>
    + ScalarBackend<3, Unaligned>
    + ScalarBackend<4, Unaligned>
{
}

/// Specifies the underlying implementation of a [`Vector`].
///
/// This trait is where SIMD optimizations are implemented, but the trait can
/// also be implemented quickly without optimizations.
///
/// ## Example
///
/// ```
/// use ggmath::vector::{Alignment, Length, Scalar, ScalarBackend, ScalarWrapper, SupportedLength};
///
/// #[derive(Clone, Copy)]
/// struct MyScalar;
///
/// impl Scalar for MyScalar {}
///
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for MyScalar
/// where
///     Length<N>: SupportedLength,
/// {
///     type VectorRepr = [MyScalar; N];
/// }
///
/// // You can then use MyScalar in Vectors
/// ```
///
/// ## SIMD Optimizations
///
/// To implement SIMD optimizations for a [`Scalar`] type, it must be a wrapper
/// of an existing type. This is specified with the [`ScalarWrapper`] trait.
///
/// When `Self` is a `ScalarWrapper<T>`, you can use `Vector<N, T, A>` as the
/// `<Self as ScalarBackend<N, A>>::VectorRepr` type. This inherits the SIMD
/// alignment of `T`.
///
/// After that, we can override the implementations of [`Vector`] functions
/// to optimize them using the [`Vector::repr`] and [`Vector::from_repr`]
/// functions.
///
/// ### Example
///
/// ```
/// use core::ops::Add;
///
/// use ggmath::vector::{Alignment, Length, Scalar, ScalarBackend, ScalarWrapper, SupportedLength, Vector};
///
/// #[repr(transparent)]
/// #[derive(Clone, Copy)]
/// struct MyScalar(f32);
///
/// impl Add for MyScalar {
///     type Output = Self;
///
///     #[inline(always)]
///     fn add(self, rhs: Self) -> Self::Output {
///         Self(self.0 + rhs.0)
///     }
/// }
///
/// impl Scalar for MyScalar {}
///
/// // SAFETY: `MyScalar` is a transparent wrapper of `f32`
/// unsafe impl ScalarWrapper<f32> for MyScalar {}
///
/// impl<const N: usize, A: Alignment> ScalarBackend<N, A> for MyScalar
/// where
///     Length<N>: SupportedLength,
/// {
///     type VectorRepr = Vector<N, f32, A>;
///
///     #[inline(always)]
///     fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A> {
///         Vector::from_repr(vec.repr() + rhs.repr())
///     }
/// }
///
/// // You can then use MyScalar in Vectors
/// ```
pub trait ScalarBackend<const N: usize, A: Alignment>: Send + Sync + Copy + 'static
where
    Length<N>: SupportedLength,
{
    /// The underlying representation of a [`Vector<N, Self, A>`].
    ///
    /// This type can be one of two things:
    /// - `[Self; N]`
    /// - `Vector<N, TInner, A>` where `Self: ScalarWrapper<TInner>`
    ///
    /// To have SIMD alignment, use the second option as it inherits the alignment
    /// of `TInner` vectors.
    #[expect(private_bounds)]
    type VectorRepr: Send + Sync + Copy + 'static + SoundVectorRepr<N, Self>;

    /// Overridable implementation of [`Vector::eq`].
    #[inline(always)]
    fn vec_eq(vec: Vector<N, Self, A>, other: Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        vec.iter().zip(other).all(|(a, b)| a == b)
    }

    /// Overridable implementation of [`Vector::ne`].
    #[inline(always)]
    fn vec_ne(vec: Vector<N, Self, A>, other: Vector<N, Self, A>) -> bool
    where
        Self: Scalar + PartialEq,
    {
        vec.iter().zip(other).any(|(a, b)| a != b)
    }

    /// Overridable implementation of [`Vector::neg`].
    #[inline(always)]
    fn vec_neg(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Neg<Output = Self>,
    {
        vec.map(Self::neg)
    }

    /// Overridable implementation of [`Vector::not`].
    #[inline(always)]
    fn vec_not(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Not<Output = Self>,
    {
        vec.map(Self::not)
    }

    /// Overridable implementation of [`Vector::add`].
    #[inline(always)]
    fn vec_add(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Add<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] + rhs[i])
    }

    /// Overridable implementation of [`Vector::sub`].
    #[inline(always)]
    fn vec_sub(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Sub<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] - rhs[i])
    }

    /// Overridable implementation of [`Vector::mul`].
    #[inline(always)]
    fn vec_mul(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Mul<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] * rhs[i])
    }

    /// Overridable implementation of [`Vector::div`].
    #[inline(always)]
    fn vec_div(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Div<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] / rhs[i])
    }

    /// Overridable implementation of [`Vector::rem`].
    #[inline(always)]
    fn vec_rem(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Rem<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] % rhs[i])
    }

    /// Overridable implementation of [`Vector::shl`].
    #[inline(always)]
    fn vec_shl(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shl<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] << rhs[i])
    }

    /// Overridable implementation of [`Vector::shr`].
    #[inline(always)]
    fn vec_shr(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + Shr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] >> rhs[i])
    }

    /// Overridable implementation of [`Vector::bitand`].
    #[inline(always)]
    fn vec_bitand(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitAnd<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] & rhs[i])
    }

    /// Overridable implementation of [`Vector::bitor`].
    #[inline(always)]
    fn vec_bitor(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitOr<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] | rhs[i])
    }

    /// Overridable implementation of [`Vector::bitxor`].
    #[inline(always)]
    fn vec_bitxor(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: Scalar + BitXor<Output = Self>,
    {
        Vector::from_fn(|i| vec[i] ^ rhs[i])
    }
}

/// Marks a [`Scalar`] type as a transparent wrapper of another [`Scalar`] type.
///
/// This trait is used to add SIMD alignment to user defined [`Scalar`] types in
/// [`ScalarBackend`].
///
/// ## Safety
///
/// Implementations must ensure that `Self` is a transparent wrapper of `T`,
/// meaning:
/// - `size_of::<Self>() == size_of::<T>()`
/// - `align_of::<Self>() == align_of::<T>()`
pub unsafe trait ScalarWrapper<T> {}

/// This is an internal trait used by [`ScalarBackend`].
///
/// ## Safety
///
/// If `N == 2` or `N == 4`, `Self::ActualRepr` must contain exactly `N` `T`
/// elements, meaning:
/// - `size_of::<Self::ActualRepr>()` is `N * size_of::<T>()`
/// - `align_of::<Self::ActualRepr>()` is either `align_of::<T>()` or `N * size_of::<T>()`
///
/// If `N == 3`, either `Self::ActualRepr` contains exactly `N` `T` elements
/// with only the alignment of `T`, or `Self::ActualRepr` contains 4 `T`s with
/// an alignment of `4 * size_of::<T>()`.
#[diagnostic::on_unimplemented(message = "`Vector<{N}, {T}, A>` cannot be represented by `{Self}`")]
unsafe trait SoundVectorRepr<const N: usize, T>: Send + Sync + Copy + 'static {
    type ActualRepr: Send + Sync + Copy + 'static;
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Xy<T>(T, T);

#[repr(C)]
#[derive(Copy, Clone)]
struct Xyz<T>(T, T, T);

#[repr(C)]
#[derive(Copy, Clone)]
struct Xyzw<T>(T, T, T, T);

unsafe impl<const N: usize, T: Scalar> SoundVectorRepr<N, T> for [T; N]
where
    Length<N>: SupportedLength,
{
    type ActualRepr = <Length<N> as SupportedLength>::Select<Xy<T>, Xyz<T>, Xyzw<T>>;
}

unsafe impl<const N: usize, T, TInner: Scalar, A: Alignment> SoundVectorRepr<N, T>
    for Vector<N, TInner, A>
where
    Length<N>: SupportedLength,
    T: ScalarWrapper<TInner>,
{
    type ActualRepr = VectorRepr<N, TInner, A>;
}

/// Calls the scalar backend function for the correct length and alignment.
///
/// ## Safety
///
/// The caller must ensure that the type of the function and the call site
/// expression is the same, as that is not checked automatically.
macro_rules! specialize {
    (<$T:ty as $Backend:ident<$N:tt, $A:ident>>::$f:ident$(::<$($ARG:ty),*$(,)?>)?($($arg:expr),*$(,)?)) => {
        match ($N, $A::IS_ALIGNED) {
            (2, true) => $crate::cast(<$T as $Backend<2, $crate::vector::Aligned>>::$f$(::<$($ARG),*>)?(
                $($crate::cast($arg)),*
            )),
            (3, true) => $crate::cast(<$T as $Backend<3, $crate::vector::Aligned>>::$f$(::<$($ARG),*>)?(
                $($crate::cast($arg)),*
            )),
            (4, true) => $crate::cast(<$T as $Backend<4, $crate::vector::Aligned>>::$f$(::<$($ARG),*>)?(
                $($crate::cast($arg)),*
            )),
            (2, false) => $crate::cast(<$T as $Backend<2, $crate::vector::Unaligned>>::$f$(::<$($ARG),*>)?(
                $($crate::cast($arg)),*
            )),
            (3, false) => $crate::cast(<$T as $Backend<3, $crate::vector::Unaligned>>::$f$(::<$($ARG),*>)?(
                $($crate::cast($arg)),*
            )),
            (4, false) => $crate::cast(<$T as $Backend<4, $crate::vector::Unaligned>>::$f$(::<$($ARG),*>)?(
                $($crate::cast($arg)),*
            )),
            _ => unreachable!(),
        }
    };
}

use specialize;

////////////////////////////////////////////////////////////////////////////////
// Length
////////////////////////////////////////////////////////////////////////////////

/// A marker type the length of a [`Vector`].
pub struct Length<const N: usize>;

/// A marker trait for supported [`Vector`] lengths (`2`, `3`, and `4`).
pub trait SupportedLength {
    #[doc(hidden)]
    type Select<
        T2: Send + Sync + Copy + 'static,
        T3: Send + Sync + Copy + 'static,
        T4: Send + Sync + Copy + 'static,
    >: Send + Sync + Copy + 'static;
}

impl SupportedLength for Length<2> {
    type Select<
        T2: Send + Sync + Copy + 'static,
        T3: Send + Sync + Copy + 'static,
        T4: Send + Sync + Copy + 'static,
    > = T2;
}

impl SupportedLength for Length<3> {
    type Select<
        T2: Send + Sync + Copy + 'static,
        T3: Send + Sync + Copy + 'static,
        T4: Send + Sync + Copy + 'static,
    > = T3;
}

impl SupportedLength for Length<4> {
    type Select<
        T2: Send + Sync + Copy + 'static,
        T3: Send + Sync + Copy + 'static,
        T4: Send + Sync + Copy + 'static,
    > = T4;
}

////////////////////////////////////////////////////////////////////////////////
// Alignment
////////////////////////////////////////////////////////////////////////////////

/// A marker type for SIMD-aligned [`Vector`]s.
pub struct Aligned;

/// A marker type for non SIMD-aligned [`Vector`]s.
pub struct Unaligned;

/// A marker trait for SIMD-aligned and non SIMD-aligned [`Vector`]s.
pub trait Alignment: 'static {
    /// Is `true` for [`Aligned`] and `false` for [`Unaligned`].
    const IS_ALIGNED: bool;

    #[doc(hidden)]
    type Select<TAligned: Send + Sync + Copy + 'static, TUnaligned: Send + Sync + Copy + 'static>:
        Send + Sync + Copy + 'static;
}

impl Alignment for Aligned {
    const IS_ALIGNED: bool = true;

    type Select<TAligned: Send + Sync + Copy + 'static, TUnaligned: Send + Sync + Copy + 'static> =
        TAligned;
}

impl Alignment for Unaligned {
    const IS_ALIGNED: bool = false;

    type Select<TAligned: Send + Sync + Copy + 'static, TUnaligned: Send + Sync + Copy + 'static> =
        TUnaligned;
}
