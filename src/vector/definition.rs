use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, ScalarBackend, SupportedLength, Unaligned,
    utils::{specialize, transmute_generic, transmute_mut, transmute_ref},
};

/// A generic vector type.
///
/// This type is the generic form of these type aliases:
/// - [`Vec2<T>`](crate::Vec2), [`Vec3<T>`](crate::Vec3),
///   [`Vec4<T>`](crate::Vec4).
/// - [`Vec2U<T>`](crate::Vec2U), [`Vec3U<T>`](crate::Vec3U),
///   [`Vec4U<T>`](crate::Vec4U).
///
/// This type is generic over:
///
/// - `N`: Length (2, 3, or 4).
/// - `T`: Scalar type.
/// - `A`: Alignment (see [`Alignment`]).
///
/// # Memory Layout
///
/// | Type | Size | Alignment |
/// | ---- | ---- | --------- |
/// | [`Vec2<T>`](crate::Vec2) | `size_of::<T>() * 2` | See below |
/// | [`Vec3<T>`](crate::Vec3) | See below | See below |
/// | [`Vec4<T>`](crate::Vec4) | `size_of::<T>() * 4` | See below |
/// | [`Vec2U<T>`](crate::Vec2U) | `size_of::<T>() * 2` | `align_of::<T>()` |
/// | [`Vec3U<T>`](crate::Vec3U) | `size_of::<T>() * 3` | `align_of::<T>()` |
/// | [`Vec4U<T>`](crate::Vec4U) | `size_of::<T>() * 4` | `align_of::<T>()` |
///
/// The alignment of aligned vectors can be anything from the alignment of `T`
/// to the size of the vector.
///
/// The size of `Vec3<T>` can either be `size_of::<T>() * 3` or
/// `size_of::<T>() * 4`. If its size is times 4, the padding is guaranteed to
/// be an initialized value of `T`.
///
/// The specific representation of each vector type is controlled by the
/// [`ScalarBackend`] trait.
#[repr(transparent)]
pub struct Vector<const N: usize, T: Scalar, A: Alignment>(VectorRepr<N, T, A>)
where
    Length<N>: SupportedLength;

impl<const N: usize, T: Scalar, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Creates a vector from an array.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; N]) -> Self {
        unsafe {
            match N {
                2 => transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    array[0], array[1],
                )),
                3 => transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    array[0], array[1], array[2],
                )),
                4 => transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    array[0], array[1], array[2], array[3],
                )),
                _ => unreachable!(),
            }
        }
    }

    /// Creates a vector with all elements set to the given value.
    #[inline]
    #[must_use]
    pub const fn splat(value: T) -> Self {
        unsafe {
            match N {
                2 => transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    value, value,
                )),
                3 => transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    value, value, value,
                )),
                4 => transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    value, value, value, value,
                )),
                _ => unreachable!(),
            }
        }
    }

    /// Creates a vector by calling function `f` for each element index.
    ///
    /// Equivalent to `(f(0), f(1), f(2), ...)`.
    #[inline]
    #[must_use]
    pub fn from_fn(mut f: impl FnMut(usize) -> T) -> Self {
        unsafe {
            match N {
                2 => transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    f(0),
                    f(1),
                )),
                3 => transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    f(0),
                    f(1),
                    f(2),
                )),
                4 => transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    f(0),
                    f(1),
                    f(2),
                    f(3),
                )),
                _ => unreachable!(),
            }
        }
    }

    /// Converts the vector to the specified alignment.
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Vector<N, T, A2> {
        unsafe {
            match N {
                2 | 4 => transmute_generic::<Vector<N, T, A>, Vector<N, T, A2>>(self),
                3 => transmute_generic::<Vector<3, T, A2>, Vector<N, T, A2>>(
                    Vector::<3, T, A2>::new(
                        self.as_array_ref()[0],
                        self.as_array_ref()[1],
                        self.as_array_ref()[2],
                    ),
                ),
                _ => unreachable!(),
            }
        }
    }

    /// Converts the alignment of the vector to [`Aligned`].
    #[inline]
    #[must_use]
    pub const fn align(self) -> Vector<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the alignment of the vector to [`Unaligned`].
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Vector<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts the vector to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; N] {
        *self.as_array_ref()
    }

    /// Returns a reference to the vector's elements.
    #[inline]
    #[must_use]
    pub const fn as_array_ref(&self) -> &[T; N] {
        unsafe { transmute_ref::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns a mutable reference to the vector's elements.
    #[inline]
    #[must_use]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute_mut::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns an iterator over the vector's elements.
    ///
    /// This method returns an iterator over `T` and not `&T`. to iterate over
    /// references use `vec.as_array_ref().iter()`.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<T, N> {
        self.to_array().into_iter()
    }

    /// Returns an iterator over mutable references to the vector's elements.
    #[inline]
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_array_mut().iter_mut()
    }

    /// Calls function `f` for each element of the vector and returns the
    /// result.
    #[inline]
    #[must_use]
    pub fn map<T2: Scalar>(self, f: impl Fn(T) -> T2) -> Vector<N, T2, A> {
        Vector::from_fn(|i| f(self[i]))
    }

    /// Returns the element at the given index, or `None` if the index is out of
    /// bounds.
    #[inline]
    #[must_use]
    pub const fn get(self, index: usize) -> Option<T> {
        if index < N {
            Some(self.as_array_ref()[index])
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index, or `None`
    /// if the index is out of bounds.
    #[inline]
    #[must_use]
    pub const fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            Some(&mut self.as_array_mut()[index])
        } else {
            None
        }
    }

    /// Returns the vector with its components in reverse order.
    #[inline]
    #[must_use]
    pub fn reverse(self) -> Self {
        Self::from_fn(|i| self[N - 1 - i])
    }

    /// Returns the internal representation of the vector.
    ///
    /// The internal representation is controlled by the implementation for the
    /// [`ScalarBackend`] trait.
    ///
    /// This function should not be used outside the implementation for
    /// [`ScalarBackend`] because the internal representation could change
    /// silently and cause compile errors.
    #[inline]
    pub fn repr(self) -> <T as ScalarBackend<N, A>>::VectorRepr
    where
        T: ScalarBackend<N, A>,
    {
        unsafe {
            transmute_generic::<Vector<N, T, A>, <T as ScalarBackend<N, A>>::VectorRepr>(self)
        }
    }

    /// Creates a vector from its internal representation.
    ///
    /// The internal representation is controlled by the implementation for the
    /// [`ScalarBackend`] trait.
    ///
    /// This function should not be used outside the implementation for
    /// [`ScalarBackend`] because the internal representation could change
    /// silently and cause compile errors.
    ///
    /// # Safety
    ///
    /// The provided value must be valid for this vector type, because the
    /// internal type may have less memory safety requirements than `T`.
    #[inline]
    pub unsafe fn from_repr(repr: <T as ScalarBackend<N, A>>::VectorRepr) -> Self
    where
        T: ScalarBackend<N, A>,
    {
        unsafe {
            transmute_generic::<<T as ScalarBackend<N, A>>::VectorRepr, Vector<N, T, A>>(repr)
        }
    }
}

impl<T: Scalar, A: Alignment> Vector<2, T, A> {
    #[inline]
    pub(in crate::vector) const fn new(x: T, y: T) -> Self {
        unsafe { transmute_generic::<Repr2<T>, Vector<2, T, A>>(Repr2(x, y)) }
    }
}

impl<T: Scalar, A: Alignment> Vector<3, T, A> {
    #[inline]
    pub(in crate::vector) const fn new(x: T, y: T, z: T) -> Self {
        unsafe {
            match size_of::<Vector<3, T, A>>() / size_of::<T>() {
                3 => transmute_generic::<Repr3<T>, Vector<3, T, A>>(Repr3(x, y, z)),
                4 => transmute_generic::<Repr4<T>, Vector<3, T, A>>(Repr4(x, y, z, z)),
                _ => unreachable!(),
            }
        }
    }
}

impl<T: Scalar, A: Alignment> Vector<4, T, A> {
    #[inline]
    pub(in crate::vector) const fn new(x: T, y: T, z: T, w: T) -> Self {
        unsafe { transmute_generic::<Repr4<T>, Vector<4, T, A>>(Repr4(x, y, z, w)) }
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Clone for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Copy for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, T: Scalar, A: Alignment> Index<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.as_array_ref().index(index)
    }
}

impl<const N: usize, T: Scalar, A: Alignment> IndexMut<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline]
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

    #[inline]
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

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<const N: usize, T: Scalar + Debug, A: Alignment> Debug for Vector<N, T, A>
where
    Length<N>: SupportedLength,
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

impl<const N: usize, T: Scalar + Display, A: Alignment> Display for Vector<N, T, A>
where
    Length<N>: SupportedLength,
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

impl<const N: usize, T: Scalar + PartialEq, A: Alignment> PartialEq for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::vec_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::vec_ne(self, other))
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
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_array_ref().hash(state);
    }
}

impl<const N: usize, T: Scalar + Default, A: Alignment> Default for Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn default() -> Self {
        Self::splat(T::default())
    }
}

macro_rules! impl_unary_op {
    ($Op:ident $op:ident $vec_op:ident) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
        {
            type Output = Self;

            #[inline]
            fn $op(self) -> Self::Output {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self))
            }
        }
    };
}
impl_unary_op!(Neg neg vec_neg);
impl_unary_op!(Not not vec_not);

macro_rules! impl_binary_op {
    ($Op:ident $op:ident $vec_op:ident) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self, rhs))
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>, A: Alignment> $Op<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: T) -> Self::Output {
                self.$op(Self::splat(rhs))
            }
        }
    };
}
impl_binary_op!(Add add vec_add);
impl_binary_op!(Sub sub vec_sub);
impl_binary_op!(Mul mul vec_mul);
impl_binary_op!(Div div vec_div);
impl_binary_op!(Rem rem vec_rem);
impl_binary_op!(Shl shl vec_shl);
impl_binary_op!(Shr shr vec_shr);
impl_binary_op!(BitAnd bitand vec_bitand);
impl_binary_op!(BitOr bitor vec_bitor);
impl_binary_op!(BitXor bitxor vec_bitxor);

macro_rules! impl_assign_op {
    ($Op:ident $OpAssign:ident $op_assign:ident $op:ident) => {
        impl<const N: usize, T: Scalar + $Op<Output = T>, A: Alignment> $OpAssign
            for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T: Scalar + $Op<Output = T>, A: Alignment> $OpAssign<T>
            for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: T) {
                *self = self.$op(rhs);
            }
        }
    };
}
impl_assign_op!(Add AddAssign add_assign add);
impl_assign_op!(Sub SubAssign sub_assign sub);
impl_assign_op!(Mul MulAssign mul_assign mul);
impl_assign_op!(Div DivAssign div_assign div);
impl_assign_op!(Rem RemAssign rem_assign rem);
impl_assign_op!(Shl ShlAssign shl_assign shl);
impl_assign_op!(Shr ShrAssign shr_assign shr);
impl_assign_op!(BitAnd BitAndAssign bitand_assign bitand);
impl_assign_op!(BitOr BitOrAssign bitor_assign bitor);
impl_assign_op!(BitXor BitXorAssign bitxor_assign bitxor);

unsafe impl<const N: usize, T: Scalar + Send, A: Alignment> Send for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

unsafe impl<const N: usize, T: Scalar + Sync, A: Alignment> Sync for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, T: Scalar + Unpin, A: Alignment> Unpin for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, T: Scalar + UnwindSafe, A: Alignment> UnwindSafe for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

impl<const N: usize, T: Scalar + RefUnwindSafe, A: Alignment> RefUnwindSafe for Vector<N, T, A> where
    Length<N>: SupportedLength
{
}

/// This type is indirection for:
///
/// - `<T as ScalarBackend<N, A>>::VectorRepr` for [`Aligned`].
/// - `Repr2<T>`, `Repr3<T>`, or `Repr4<T>` for [`Aligned`].
///
/// Indirection must be used because the type system cannot prove that this
/// condition is met:
/// ```ignore
/// for<const N: usize, T: Scalar, A: Alignment> where Length<N>: SupportedLength {
///     T: ScalarBackend<N, A>,
/// }
/// ```
type VectorRepr<const N: usize, T, A> = <A as Alignment>::Select<
    <Length<N> as SupportedLength>::Select<
        <T as ScalarBackend<2, Aligned>>::VectorRepr,
        <T as ScalarBackend<3, Aligned>>::VectorRepr,
        <T as ScalarBackend<4, Aligned>>::VectorRepr,
    >,
    <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>,
>;

#[repr(C)]
#[derive(Clone, Copy)]
struct Repr2<T>(pub T, pub T);

#[repr(C)]
#[derive(Clone, Copy)]
struct Repr3<T>(pub T, pub T, pub T);

#[repr(C)]
#[derive(Clone, Copy)]
struct Repr4<T>(pub T, pub T, pub T, pub T);
