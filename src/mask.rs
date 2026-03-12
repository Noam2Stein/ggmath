use core::{
    fmt::{Debug, Display},
    hash::Hash,
    mem::transmute,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, ScalarRepr, SupportedLength, Unaligned, Vector,
    specialize::specialize,
};

/// An `N`-element vector mask optimized for type `T`.
///
/// `Mask<N, T, A>` is equivalent to a vector of booleans but is optimized
/// specifically for working with vectors of type `T`.
///
/// `A` controls SIMD alignment and is either [`Aligned`] or [`Unaligned`]. See
/// [`Alignment`] for more details.
///
/// # Type aliases
///
/// - [`Mask2<T>`] for `Mask<2, T, Aligned>`.
/// - [`Mask3<T>`] for `Mask<3, T, Aligned>`.
/// - [`Mask4<T>`] for `Mask<4, T, Aligned>`.
/// - [`Mask2U<T>`] for `Mask<2, T, Unaligned>`.
/// - [`Mask3U<T>`] for `Mask<3, T, Unaligned>`.
/// - [`Mask4U<T>`] for `Mask<4, T, Unaligned>`.
///
/// # Memory layout
///
/// `Mask<N, T, A>` does not have a stable representation, but does guarantee
/// certain properties.
///
/// `Mask<N, T, A>` does not contain any uninitialized bytes.
/// `Mask<N, T, A>` accepts the all-zero byte-pattern.
///
/// Masks of compatible [`Scalar::Repr`] types have the same representation,
/// size, and alignment. This means that they are transmutable (see
/// [`to_repr`]).
///
/// Types containing compatible masks may not have compatible layouts
/// themselves. For example, even though [`Mask2<i32>`] and [`Mask2<u32>`] have
/// compatible layouts, [`Option<Mask2<i32>>`] and [`Option<Mask2<u32>>`] may
/// not.
///
/// [`Mask2<T>`]: crate::Mask2
/// [`Mask3<T>`]: crate::Mask3
/// [`Mask4<T>`]: crate::Mask4
/// [`Mask2U<T>`]: crate::Mask2U
/// [`Mask3U<T>`]: crate::Mask3U
/// [`Mask4U<T>`]: crate::Mask4U
/// [`Mask2<i32>`]: crate::Mask2
/// [`Mask2<u32>`]: crate::Mask2
/// [`to_repr`]: Self::to_repr
#[repr(transparent)]
pub struct Mask<const N: usize, T, A: Alignment>(
    pub(crate) <T::Repr as ScalarRepr>::MaskRepr<N, A>,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

impl<const N: usize, T, A: Alignment> Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates a vector mask from an array.
    #[inline]
    #[must_use]
    pub fn from_array(array: [bool; N]) -> Self {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_from_array(array))
    }

    /// Creates a vector mask with all elements set to `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mask = Mask3::<f32>::splat(true);
    /// assert_eq!(mask, Mask3::new(true, true, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn splat(value: bool) -> Self {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_splat(value))
    }

    /// Creates a vector mask by calling function `f` for each element index.
    ///
    /// Equivalent to `(f(0), f(1), f(2), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// // indices are 0, 1, 2
    /// let mask = Mask3::<f32>::from_fn(|i| i % 2 == 0);
    /// assert_eq!(mask, Mask3::new(true, false, true));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> bool,
    {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_from_fn((f,)))
    }

    /// Conversion between [`Aligned`] and [`Unaligned`] storage.
    ///
    /// See [`align`] and [`unalign`] for scenarios where the output alignment
    /// is known.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Aligned, Unaligned, Mask3, Mask3U};
    /// #
    /// let aligned = Mask3::<f32>::new(false, true, false);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Mask3U::new(false, true, false));
    ///
    /// let unaligned = Mask3U::<f32>::new(false, true, false);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Mask3::new(false, true, false));
    /// ```
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub fn to_alignment<A2: Alignment>(self) -> Mask<N, T, A2> {
        (const {
            if A::IS_ALIGNED == A2::IS_ALIGNED {
                // `A` and `A2` are guaranteed to be the same type as long as
                // `A::IS_ALIGNED == A2::IS_ALIGNED` which was just checked.
                // Thus the transmuted types are the same type.
                unsafe {
                    transmute::<
                        fn(Mask<N, T, A>) -> Mask<N, T, A>,
                        fn(Mask<N, T, A>) -> Mask<N, T, A2>,
                    >(|mask| mask)
                }
            } else {
                |mask: Self| Mask::from_array(mask.to_array())
            }
        })(self)
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Mask3U};
    /// #
    /// let unaligned = Mask3U::<f32>::new(false, true, false);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Mask3::new(false, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn align(self) -> Mask<N, T, Aligned> {
        self.to_alignment()
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Mask3U};
    /// #
    /// let aligned = Mask3::<f32>::new(false, true, false);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Mask3U::new(false, true, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn unalign(self) -> Mask<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts the vector mask to an array.
    #[inline]
    #[must_use]
    pub fn to_array(self) -> [bool; N] {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_to_array(self))
    }

    /// Returns `true` if all elements of `self` are `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mask = Mask3::<f32>::new(true, true, false);
    /// assert_eq!(mask.all(), false);
    ///
    /// let mask = Mask3::<f32>::new(true, true, true);
    /// assert_eq!(mask.all(), true);
    /// ```
    #[inline]
    #[must_use]
    pub fn all(self) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_all(self))
    }

    /// Returns `true` if any element of `self` is `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mask = Mask3::<f32>::new(true, true, false);
    /// assert_eq!(mask.any(), true);
    ///
    /// let mask = Mask3::<f32>::new(false, false, false);
    /// assert_eq!(mask.any(), false);
    /// ```
    #[inline]
    #[must_use]
    pub fn any(self) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_any(self))
    }

    /// Selects between the elements of `if_true` and `if_false` based on the
    /// boolean elements of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask4, Vec4};
    /// #
    /// let mask = Mask4::new(true, false, false, true);
    /// let if_true = Vec4::new(1, 2, 3, 4);
    /// let if_false = Vec4::new(-1, -2, -3, -4);
    /// let result = mask.select(if_true, if_false);
    ///
    /// assert_eq!(result, Vec4::new(1, -2, -3, 4));
    /// ```
    #[inline]
    #[must_use]
    pub fn select(self, if_true: Vector<N, T, A>, if_false: Vector<N, T, A>) -> Vector<N, T, A> {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_select(
            self, if_true, if_false
        ))
    }

    /// Returns an iterator over the vector mask's elements.
    #[inline]
    #[must_use]
    pub fn iter(self) -> core::array::IntoIter<bool, N> {
        self.to_array().into_iter()
    }

    /// Returns the element at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn get(self, index: usize) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_get(self, index))
    }

    /// Sets the element at the given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements.
    #[inline]
    #[track_caller]
    pub fn set(&mut self, index: usize, value: bool) {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_set(self, index, value))
    }

    /// Raw transmutation between scalar types.
    ///
    /// This function's signature staticly guarantees that the types have
    /// compatible memory layouts.
    ///
    /// This function is used to make SIMD optimizations in implementations of
    /// [`Scalar`].
    ///
    /// # Examples
    ///
    /// Correct usage:
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<i32>::new(false, true, false);
    /// let b = a.to_repr::<u32>();
    ///
    /// assert_eq!(b, Mask3::<u32>::new(false, true, false));
    /// ```
    ///
    /// Incorrect usage:
    ///
    /// ```compile_fail
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<i32>::new(false, true, false);
    ///
    /// // This does not compile since `i32` and `i64` are not compatible.
    /// let _ = a.to_repr::<i64>();
    /// ```
    #[inline]
    #[must_use]
    pub const fn to_repr<T2>(self) -> Mask<N, T2, A>
    where
        T2: Scalar<Repr = T::Repr>,
    {
        Mask(self.0)
    }
}

impl<T, A: Alignment> Mask<2, T, A>
where
    T: Scalar,
{
    /// Creates a 2-element vector mask.
    #[inline]
    #[must_use]
    pub fn new(x: bool, y: bool) -> Self {
        Self::from_array([x, y])
    }
}

impl<T, A: Alignment> Mask<3, T, A>
where
    T: Scalar,
{
    /// Creates a 3-element vector mask.
    #[inline]
    #[must_use]
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        Self::from_array([x, y, z])
    }
}

impl<T, A: Alignment> Mask<4, T, A>
where
    T: Scalar,
{
    /// Creates a 4-element vector mask.
    #[inline]
    #[must_use]
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        Self::from_array([x, y, z, w])
    }
}

impl<const N: usize, T, A: Alignment> Clone for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> IntoIterator for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = bool;
    type IntoIter = core::array::IntoIter<bool, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize, T, A: Alignment> IntoIterator for &Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Item = bool;
    type IntoIter = core::array::IntoIter<bool, N>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const N: usize, T, A: Alignment> Debug for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", Vector::<N, bool, A>::from_array(self.to_array()))
    }
}

impl<const N: usize, T, A: Alignment> Display for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", Vector::<N, bool, A>::from_array(self.to_array()))
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_eq(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Hash for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.to_array().hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn default() -> Self {
        Self::splat(bool::default())
    }
}

impl<const N: usize, T, A: Alignment> Not for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = Self;

    /// Performs the unary `!` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mask = Mask3::<f32>::new(false, true, false);
    /// assert_eq!(!mask, Mask3::new(true, false, true));
    /// ```
    #[inline]
    fn not(self) -> Self::Output {
        specialize!(<T::Repr as MaskBackend<N, A>>::mask_not(self))
    }
}

macro_rules! impl_binary_operators {
    ($Op:ident, $op:ident, $mask_op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
        impl<const N: usize, T, A: Alignment> $Op for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            fn $op(self, rhs: Self) -> Self::Output {
                specialize!(<T::Repr as MaskBackend<N, A>>::$mask_op(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<bool> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            $(#[$doc_scalar])*
            #[inline]
            fn $op(self, rhs: bool) -> Self::Output {
                self.$op(Self::splat(rhs))
            }
        }
    };
}
impl_binary_operators!(
    BitAnd,
    bitand,
    mask_bitand,
    /// Performs the `&` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<f32>::new(true, true, false);
    /// let b = a & Mask3::new(false, true, true);
    ///
    /// assert_eq!(b, Mask3::new(true & false, true & true, false & true));
    /// ```
    ,
    /// Performs the `&` operation for each vector mask element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<f32>::new(true, false, true);
    /// let b = a & false;
    ///
    /// assert_eq!(b, Mask3::new(true & false, false & false, true & false));
    /// ```
);
impl_binary_operators!(
    BitOr,
    bitor,
    mask_bitor,
    /// Performs the `|` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<f32>::new(true, true, false);
    /// let b = a | Mask3::new(false, true, true);
    ///
    /// assert_eq!(b, Mask3::new(true | false, true | true, false | true));
    /// ```
    ,
    /// Performs the `|` operation for each vector mask element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<f32>::new(true, false, true);
    /// let b = a | false;
    ///
    /// assert_eq!(b, Mask3::new(true | false, false | false, true | false));
    /// ```
);
impl_binary_operators!(
    BitXor,
    bitxor,
    mask_bitxor,
    /// Performs the `^` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<f32>::new(true, true, false);
    /// let b = a ^ Mask3::new(false, true, true);
    ///
    /// assert_eq!(b, Mask3::new(true ^ false, true ^ true, false ^ true));
    /// ```
    ,
    /// Performs the `^` operation for each vector mask element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let a = Mask3::<f32>::new(true, false, true);
    /// let b = a ^ false;
    ///
    /// assert_eq!(b, Mask3::new(true ^ false, false ^ false, true ^ false));
    /// ```
);

macro_rules! impl_assign_operator {
    ($OpAssign:ident, $op_assign:ident, $op:ident, $(#[$doc:meta])*, $(#[$doc_scalar:meta])*) => {
        impl<const N: usize, T, A: Alignment> $OpAssign for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: Self) {
                *self = self.$op(rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<bool> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: bool) {
                *self = self.$op(rhs);
            }
        }
    };
}
impl_assign_operator!(
    BitAndAssign,
    bitand_assign,
    bitand,
    /// Performs the `&=` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mut mask = Mask3::<f32>::new(true, true, false);
    /// mask &= Mask3::new(false, true, true);
    ///
    /// assert_eq!(mask, Mask3::new(true & false, true & true, false & true));
    /// ```
    ,
    /// Performs the `&=` operation for each vector mask element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mut mask = Mask3::<f32>::new(true, true, false);
    /// mask &= false;
    ///
    /// assert_eq!(mask, Mask3::new(true & false, true & false, false & false);
    /// ```
);
impl_assign_operator!(
    BitOrAssign,
    bitor_assign,
    bitor,
    /// Performs the `|=` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mut mask = Mask3::<f32>::new(true, true, false);
    /// mask |= Mask3::new(false, true, true);
    ///
    /// assert_eq!(mask, Mask3::new(true | false, true | true, false | true));
    /// ```
    ,
    /// Performs the `|=` operation for each vector mask element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mut mask = Mask3::<f32>::new(true, true, false);
    /// mask |= false;
    ///
    /// assert_eq!(mask, Mask3::new(true | false, true | false, false | false);
    /// ```
);
impl_assign_operator!(
    BitXorAssign,
    bitxor_assign,
    bitxor,
    /// Performs the `^=` operation for each vector mask element.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mut mask = Mask3::<f32>::new(true, true, false);
    /// mask ^= Mask3::new(false, true, true);
    ///
    /// assert_eq!(mask, Mask3::new(true ^ false, true ^ true, false ^ true));
    /// ```
    ,
    /// Performs the `^=` operation for each vector mask element and the scalar
    /// `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mask3;
    /// #
    /// let mut mask = Mask3::<f32>::new(true, true, false);
    /// mask ^= false;
    ///
    /// assert_eq!(mask, Mask3::new(true ^ false, true ^ false, false ^ false);
    /// ```
);

// SAFETY: Mask representations must be either equivalent to `[bool; N]` or be
// simple intrinsic types. Both are `Send`.
unsafe impl<const N: usize, T, A: Alignment> Send for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

// SAFETY: Mask representations must be either equivalent to `[bool; N]` or be
// simple intrinsic types. Both are `Sync`.
unsafe impl<const N: usize, T, A: Alignment> Sync for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

/// Controls the implementation of vector mask functions.
///
/// Unlike [`ScalarBackend<N, A>`], `MaskBackend<N, A>` is implemented for
/// [`T::Repr`] instead of `T`.
///
/// Unlike [`ScalarBackend<N, A>`], `MaskBackend<N, A>` functions have no
/// default implementation. This is because there are not enough guarantees
/// about the representation of vector masks to make a default implementation.
///
/// [`ScalarBackend<N, A>`]: crate::ScalarBackend
/// [`T::Repr`]: Scalar::Repr
pub(crate) trait MaskBackend<const N: usize, A: Alignment>
where
    Length<N>: SupportedLength,
{
    fn mask_from_array<T>(array: [bool; N]) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_splat<T>(value: bool) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    /// A one member tuple is used to fix type inference in the macro
    /// [`specialize`], which fails for generic function types.
    fn mask_from_fn<T, F>(f: (F,)) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool;

    fn mask_to_array<T>(mask: Mask<N, T, A>) -> [bool; N]
    where
        T: Scalar<Repr = Self>;

    fn mask_all<T>(mask: Mask<N, T, A>) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_any<T>(mask: Mask<N, T, A>) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_select<T>(
        mask: Mask<N, T, A>,
        if_true: Vector<N, T, A>,
        if_false: Vector<N, T, A>,
    ) -> Vector<N, T, A>
    where
        T: Scalar<Repr = Self>;

    #[track_caller]
    fn mask_get<T>(mask: Mask<N, T, A>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>;

    #[track_caller]
    fn mask_set<T>(mask: &mut Mask<N, T, A>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>;

    fn mask_eq<T>(mask: &Mask<N, T, A>, other: &Mask<N, T, A>) -> bool
    where
        T: Scalar<Repr = Self>;

    fn mask_not<T>(mask: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_bitand<T>(mask: Mask<N, T, A>, rhs: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_bitor<T>(mask: Mask<N, T, A>, rhs: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;

    fn mask_bitxor<T>(mask: Mask<N, T, A>, rhs: Mask<N, T, A>) -> Mask<N, T, A>
    where
        T: Scalar<Repr = Self>;
}
