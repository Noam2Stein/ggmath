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
/// `Vector` is the generic form of:
///
/// - [`Vec2<T>`](crate::Vec2)
/// - [`Vec3<T>`](crate::Vec3)
/// - [`Vec4<T>`](crate::Vec4)
/// - [`Vec2U<T>`](crate::Vec2U)
/// - [`Vec3U<T>`](crate::Vec3U)
/// - [`Vec4U<T>`](crate::Vec4U)
///
/// `Vector` is generic over:
///
/// - `N`: Length (2, 3, or 4)
/// - `T`: Scalar type (see [`Scalar`])
/// - `A`: Alignment (see [`Alignment`])
///
/// To initialize vectors, use the macros [`vec2`](crate::vec2),
/// [`vec3`](crate::vec3), [`vec4`](crate::vec4). To initialize a vector of an
/// unknown length, use [`Vector::from_array`].
///
/// # Guarantees
///
/// `Vector<N, T, Unaligned>` is guaranteed to have the memory layout of
/// `[T; N]`.
///
/// `Vector<2, T, Aligned>` and `Vector<4, T, Aligned>` are guaranteed to have
/// the size of `[T; N]`, but may have additional alignment.
///
/// `Vector<3, T, Aligned>` is guaranteed to have the size of either `[T; 3]` or
/// `[T; 4]`, and may have additional alignment. When the size is of `[T; 4]`,
/// the padding is guaranteed to be an initialized value of type `T`.
///
/// Types containing `Vector` are not guaranteed to have the same memory layout
/// as types containing `[T; N]`. For example, `Option<Vector<2, T, Aligned>>`
/// is not guaranteed to have the same size as `Option<[T; 2]>`.
#[repr(transparent)]
pub struct Vector<const N: usize, T, A: Alignment>(VectorRepr<N, T, A>)
where
    Length<N>: SupportedLength,
    T: Scalar;

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates a vector from an array.
    ///
    /// The preferable way to create vectors is using the macros
    /// [`vec2`](crate::vec2), [`vec3`](crate::vec3), [`vec4`](crate::vec4).
    ///
    /// `Vector::from_array` should only be used when the length of the vector
    /// is unknown or when directly converting from an array.
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

    /// Creates a vector with all components set to the given value.
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

    /// Creates a vector by calling function `f` for each component index.
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
    ///
    /// See [`Alignment`] for more information.
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

    /// Converts the vector to [`Aligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(self) -> Vector<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the vector to [`Unaligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
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

    /// Returns a reference to the vector's components.
    #[inline]
    #[must_use]
    pub const fn as_array_ref(&self) -> &[T; N] {
        unsafe { transmute_ref::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns a mutable reference to the vector's components.
    #[inline]
    #[must_use]
    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute_mut::<Vector<N, T, A>, [T; N]>(self) }
    }

    /// Returns an iterator over the vector's components.
    ///
    /// This method returns an iterator over `T` and not `&T`. to iterate over
    /// references use `vec.as_array_ref().iter()`.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<T, N> {
        self.to_array().into_iter()
    }

    /// Returns an iterator over mutable references to the vector's components.
    #[inline]
    #[must_use = "iterators are lazy and do nothing unless consumed"]
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, T> {
        self.as_array_mut().iter_mut()
    }

    /// Creates a vector by calling function `f` for each component of the input
    /// vector.
    ///
    /// Equivalent to `(f(vec.x), f(vec.y), f(vec.z), ...)`.
    ///
    /// # Example
    ///
    /// ```
    /// use ggmath::{Vec3, vec3};
    ///
    /// let vec: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(vec.map(|x| x * 2.0), vec3!(2.0, 4.0, 6.0));
    ///
    /// assert_eq!(vec.map(|x| x.is_sign_negative()), vec3!(false, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn map<T2: Scalar>(self, f: impl Fn(T) -> T2) -> Vector<N, T2, A> {
        Vector::from_fn(|i| f(self[i]))
    }

    /// Returns the vector's components in reverse order.
    ///
    /// # Example
    ///
    /// ```
    /// use ggmath::{Vec3, vec3};
    ///
    /// let vec: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(vec.reverse(), vec3!(3.0, 2.0, 1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn reverse(self) -> Self {
        Self::from_fn(|i| self[N - 1 - i])
    }

    /// Returns the internal representation of the vector.
    ///
    /// The internal representation is controlled by the implementation of the
    /// [`ScalarBackend`] trait.
    ///
    /// This function should not be used outside the implementation of
    /// [`ScalarBackend`] because the specified internal representation could
    /// change silently and cause compile errors.
    #[inline]
    #[must_use]
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
    /// The internal representation is controlled by the implementation of the
    /// [`ScalarBackend`] trait.
    ///
    /// This function should not be used outside the implementation of
    /// [`ScalarBackend`] because the specified internal representation could
    /// change silently and cause compile errors.
    ///
    /// # Safety
    ///
    /// If the internal representation has less guarantees than the outer vector
    /// type, the input value must uphold the guarantees of the latter.
    #[inline]
    #[must_use]
    pub unsafe fn from_repr(repr: <T as ScalarBackend<N, A>>::VectorRepr) -> Self
    where
        T: ScalarBackend<N, A>,
    {
        unsafe {
            transmute_generic::<<T as ScalarBackend<N, A>>::VectorRepr, Vector<N, T, A>>(repr)
        }
    }
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar,
{
    #[inline]
    pub(crate) const fn new(x: T, y: T) -> Self {
        unsafe { transmute_generic::<Repr2<T>, Vector<2, T, A>>(Repr2(x, y)) }
    }
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar,
{
    #[inline]
    pub(crate) const fn new(x: T, y: T, z: T) -> Self {
        unsafe {
            match size_of::<Vector<3, T, A>>() / size_of::<T>() {
                3 => transmute_generic::<Repr3<T>, Vector<3, T, A>>(Repr3(x, y, z)),
                4 => transmute_generic::<Repr4<T>, Vector<3, T, A>>(Repr4(x, y, z, z)),
                _ => unreachable!(),
            }
        }
    }
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: Scalar,
{
    #[inline]
    pub(crate) const fn new(x: T, y: T, z: T, w: T) -> Self {
        unsafe { transmute_generic::<Repr4<T>, Vector<4, T, A>>(Repr4(x, y, z, w)) }
    }
}

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
        self.as_array_ref().index(index)
    }
}

impl<const N: usize, T, A: Alignment> IndexMut<usize> for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_array_mut().index_mut(index)
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
        specialize!(<T as ScalarBackend<N, A>>::vec_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::vec_ne(self, other))
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
        self.as_array_ref().hash(state);
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

macro_rules! impl_unary_op {
    ($Op:ident $op:ident $vec_op:ident) => {
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
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
        impl<const N: usize, T, A: Alignment> $Op for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            type Output = Self;

            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                specialize!(<T as ScalarBackend<N, A>>::$vec_op(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
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
        impl<const N: usize, T, A: Alignment> $OpAssign for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<T> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + $Op<Output = T>,
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

unsafe impl<const N: usize, T, A: Alignment> Send for Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Send,
{
}

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

/// Selects the correct internal representation for `Vector<N, T, A>`.
///
/// For `A = Aligned` this is `<T as ScalarBackend<N, Aligned>>::VectorRepr`.
///
/// For `A = Unaligned` this is one of the structs [`Repr2<T>`], [`Repr3<T>`],
/// [`Repr4<T>`]. This trick lets implementations of
/// [`ScalarBackend::VectorRepr`] use `Vector<N, Self, Unaligned>`.
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
struct Repr2<T>(T, T);

#[repr(C)]
#[derive(Clone, Copy)]
struct Repr3<T>(T, T, T);

#[repr(C)]
#[derive(Clone, Copy)]
struct Repr4<T>(T, T, T, T);
