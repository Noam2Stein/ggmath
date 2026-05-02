use core::{
    fmt::{Debug, Display},
    hash::Hash,
    mem::transmute,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Backend, Length, Scalar, SupportedLength, Unaligned, Vector,
    utils::{specialize, transmute_generic, transmute_mut},
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
/// [`Mask2<T>`]: crate::Mask2
/// [`Mask3<T>`]: crate::Mask3
/// [`Mask4<T>`]: crate::Mask4
/// [`Mask2U<T>`]: crate::Mask2U
/// [`Mask3U<T>`]: crate::Mask3U
/// [`Mask4U<T>`]: crate::Mask4U
/// [`Mask2<i32>`]: crate::Mask2
/// [`Mask2<u32>`]: crate::Mask2
#[repr(transparent)]
pub struct Mask<const N: usize, T, A: Alignment>(
    /// The internal representation of the vector mask.
    ///
    /// This field's insane type corresponds to [`<T as Backend<N, A>>::Mask`]
    /// which cannot be used directly because of type system limitations. In
    /// generic contexts this field will not work, in which case you should use
    /// [`from_inner`], [`inner`] and [`inner_mut`].
    ///
    /// This field should only be accessed from the crate defining `T`, else its
    /// type may change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>::Mask`]: Backend
    /// [`from_inner`]: Self::from_inner
    /// [`inner`]: Self::inner
    /// [`inner_mut`]: Self::inner_mut
    #[expect(clippy::type_complexity)]
    pub  <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<
            <T as Backend<2, Aligned>>::Mask,
            <T as Backend<3, Aligned>>::Mask,
            <T as Backend<4, Aligned>>::Mask,
        >,
        <Length<N> as SupportedLength>::Select<
            <T as Backend<2, Unaligned>>::Mask,
            <T as Backend<3, Unaligned>>::Mask,
            <T as Backend<4, Unaligned>>::Mask,
        >,
    >,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/// A 2-element vector mask.
///
/// # SIMD alignment
///
/// `Mask2<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask2U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask2<T> = Mask<2, T, Aligned>;

/// A 3-element vector mask.
///
/// # SIMD alignment
///
/// `Mask3<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask3U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask3<T> = Mask<3, T, Aligned>;

/// A 4-element vector mask.
///
/// # SIMD alignment
///
/// `Mask4<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask4U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask4<T> = Mask<4, T, Aligned>;

/// A 2-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask2U<T>` does not have SIMD alignment. See [`Mask2<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask2U<T> = Mask<2, T, Unaligned>;

/// A 3-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask3U<T>` does not have SIMD alignment. See [`Mask3<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask3U<T> = Mask<3, T, Unaligned>;

/// A 4-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask4U<T>` does not have SIMD alignment. See [`Mask4<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask4U<T> = Mask<4, T, Unaligned>;

impl<const N: usize, T, A: Alignment> Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates a vector mask from an array.
    #[inline]
    #[must_use]
    pub fn from_array(array: [bool; N]) -> Self {
        specialize!(<T as Backend<N, A>>::mask_from_array(array))
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
        specialize!(<T as Backend<N, A>>::mask_splat(value))
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
        (const {
            // SAFETY: All transmutations are between types that are previously
            // checked to be the same type.
            unsafe {
                match (N, A::IS_ALIGNED) {
                    (2, true) => transmute::<fn(F) -> Mask<2, T, Aligned>, fn(F) -> Mask<N, T, A>>(
                        <T as Backend<2, Aligned>>::mask_from_fn,
                    ),
                    (3, true) => transmute::<fn(F) -> Mask<3, T, Aligned>, fn(F) -> Mask<N, T, A>>(
                        <T as Backend<3, Aligned>>::mask_from_fn,
                    ),
                    (4, true) => transmute::<fn(F) -> Mask<4, T, Aligned>, fn(F) -> Mask<N, T, A>>(
                        <T as Backend<4, Aligned>>::mask_from_fn,
                    ),
                    (2, false) => {
                        transmute::<fn(F) -> Mask<2, T, Unaligned>, fn(F) -> Mask<N, T, A>>(
                            <T as Backend<2, Unaligned>>::mask_from_fn,
                        )
                    }
                    (3, false) => {
                        transmute::<fn(F) -> Mask<3, T, Unaligned>, fn(F) -> Mask<N, T, A>>(
                            <T as Backend<3, Unaligned>>::mask_from_fn,
                        )
                    }
                    (4, false) => {
                        transmute::<fn(F) -> Mask<4, T, Unaligned>, fn(F) -> Mask<N, T, A>>(
                            <T as Backend<4, Unaligned>>::mask_from_fn,
                        )
                    }
                    _ => unreachable!(),
                }
            }
        })(f)
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
        specialize!(<T as Backend<N, A>>::mask_to_array(self))
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
        specialize!(<T as Backend<N, A>>::mask_all(self))
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
        specialize!(<T as Backend<N, A>>::mask_any(self))
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
        specialize!(<T as Backend<N, A>>::mask_select(self, if_true, if_false))
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
        specialize!(<T as Backend<N, A>>::mask_get(self, index))
    }

    /// Sets the element at the given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of elements.
    #[inline]
    #[track_caller]
    pub fn set(&mut self, index: usize, value: bool) {
        specialize!(<T as Backend<N, A>>::mask_set(self, index, value))
    }

    /// Creates a vector mask from its internal representation.
    ///
    /// Equivalent to `Mask(inner)` but works in generic contexts.
    ///
    /// The input type is specified by [`<T as Backend<N, A>>`]. This should
    /// only be called from the crate defining `T`, else the input type may
    /// change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>`]: Backend
    #[inline]
    #[must_use]
    pub const fn from_inner(inner: <T as Backend<N, A>>::Mask) -> Self
    where
        T: Backend<N, A>,
    {
        // SAFETY: `Mask<N, T, A>` is a transparent wrapper over
        // `<T as Backend<N, A>>::Mask`.
        unsafe { transmute_generic::<<T as Backend<N, A>>::Mask, Mask<N, T, A>>(inner) }
    }

    /// Returns the internal representation of `self`.
    ///
    /// Equivalent to `self.0` but works in generic contexts.
    ///
    /// The resulting type is specified by [`<T as Backend<N, A>>`]. This should
    /// only be called from the crate defining `T`, else the resulting type may
    /// change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>`]: Backend
    #[inline]
    #[must_use]
    pub const fn inner(self) -> <T as Backend<N, A>>::Mask
    where
        T: Backend<N, A>,
    {
        // SAFETY: `Mask<N, T, A>` is a transparent wrapper over
        // `<T as Backend<N, A>>::Mask`.
        unsafe { transmute_generic::<Mask<N, T, A>, <T as Backend<N, A>>::Mask>(self) }
    }

    /// Returns a mutable reference to the internal representation of `self`.
    ///
    /// Equivalent to `&mut self.0` but works in generic contexts.
    ///
    /// The resulting type is specified by [`<T as Backend<N, A>>`]. This should
    /// only be called from the crate defining `T`, else the resulting type may
    /// change silently as it is considered an implementation detail.
    ///
    /// [`<T as Backend<N, A>>`]: Backend
    #[inline]
    #[must_use]
    pub const fn inner_mut(&mut self) -> &mut <T as Backend<N, A>>::Mask
    where
        T: Backend<N, A>,
    {
        // SAFETY: `Mask<N, T, A>` is a transparent wrapper over
        // `<T as Backend<N, A>>::Mask`.
        unsafe { transmute_mut::<Mask<N, T, A>, <T as Backend<N, A>>::Mask>(self) }
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
        specialize!(<T as Backend<N, A>>::mask_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as Backend<N, A>>::mask_ne(self, other))
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

macro_rules! impl_not {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Not for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            fn not(self) -> Self::Output {
                specialize!(<T as Backend<N, A>>::mask_not(self))
            }
        }

        impl<const N: usize, T, A: Alignment> Not for &Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Mask<N, T, A>;

            $(#[$doc])*
            #[inline]
            fn not(self) -> Self::Output {
                Mask::not(*self)
            }
        }
    };
}
impl_not!(
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
);

macro_rules! impl_binary_operator {
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
                specialize!(<T as Backend<N, A>>::$mask_op(self, rhs))
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

        impl<const N: usize, T, A: Alignment> $Op<&Mask<N, T, A>> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            fn $op(self, rhs: &Mask<N, T, A>) -> Self::Output {
                Self::$op(self, *rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&bool> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Self;

            $(#[$doc_scalar])*
            #[inline]
            fn $op(self, rhs: &bool) -> Self::Output {
                self.$op(Self::splat(*rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<Mask<N, T, A>> for &Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Mask<N, T, A>;

            $(#[$doc])*
            #[inline]
            fn $op(self, rhs: Mask<N, T, A>) -> Self::Output {
                Mask::$op(*self, rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<bool> for &Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Mask<N, T, A>;

            $(#[$doc_scalar])*
            #[inline]
            fn $op(self, rhs: bool) -> Self::Output {
                Mask::$op(*self, Mask::splat(rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&Mask<N, T, A>> for &Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Mask<N, T, A>;

            $(#[$doc])*
            #[inline]
            fn $op(self, rhs: &Mask<N, T, A>) -> Self::Output {
                Mask::$op(*self, *rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> $Op<&bool> for &Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            type Output = Mask<N, T, A>;

            $(#[$doc_scalar])*
            #[inline]
            fn $op(self, rhs: &bool) -> Self::Output {
                Mask::$op(*self, Mask::splat(*rhs))
            }
        }
    };
}
impl_binary_operator!(
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
impl_binary_operator!(
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
impl_binary_operator!(
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

        impl<const N: usize, T, A: Alignment> $OpAssign<&Mask<N, T, A>> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: &Mask<N, T, A>) {
                *self = self.$op(*rhs);
            }
        }

        impl<const N: usize, T, A: Alignment> $OpAssign<&bool> for Mask<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar,
        {
            #[inline]
            fn $op_assign(&mut self, rhs: &bool) {
                *self = self.$op(*rhs);
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

// SAFETY: Mask representations implement `Send` and `Sync`.
unsafe impl<const N: usize, T, A: Alignment> Send for Mask<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

// SAFETY: Mask representations implement `Send` and `Sync`.
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::{format, vec};

    use itertools::Itertools;

    use crate::{
        Aligned, Mask, Mask2, Mask2U, Mask3, Mask3U, Mask4, Mask4U, Unaligned, Vec2, Vec3, Vec4,
        Vector,
        utils::{Repr2, Repr3, Repr4, assert_panic, for_parameters},
    };

    #[test]
    fn test_layout() {
        for_parameters!(|T: PrimitiveNumber| {
            // This test relies on guarantees that the public API does not make.
            // This may need to be modified for future layout changes.

            assert!(
                size_of::<Mask2<T>>() == 2 && align_of::<Mask2<T>>() == 1
                    || size_of::<Mask2<T>>() == size_of::<Vec2<T>>()
                        && align_of::<Mask2<T>>() == align_of::<Vec2<T>>()
            );
            assert!(
                size_of::<Mask3<T>>() == 3 && align_of::<Mask3<T>>() == 1
                    || size_of::<Mask3<T>>() == size_of::<Vec3<T>>()
                        && align_of::<Mask3<T>>() == align_of::<Vec3<T>>()
            );
            assert!(
                size_of::<Mask4<T>>() == 4 && align_of::<Mask4<T>>() == 1
                    || size_of::<Mask4<T>>() == size_of::<Vec4<T>>()
                        && align_of::<Mask4<T>>() == align_of::<Vec4<T>>()
            );

            assert_eq!(size_of::<Mask2U<T>>(), 2);
            assert_eq!(align_of::<Mask2U<T>>(), 1);

            assert_eq!(size_of::<Mask3U<T>>(), 3);
            assert_eq!(align_of::<Mask3U<T>>(), 1);

            assert_eq!(size_of::<Mask4U<T>>(), 4);
            assert_eq!(align_of::<Mask4U<T>>(), 1);
        });
    }

    #[test]
    fn test_from_array() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                Mask::<2, T, A>::from_array([x, y]),
                Mask::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Mask::<3, T, A>::from_array([x, y, z]),
                Mask::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Mask::<4, T, A>::from_array([x, y, z, w]),
                Mask::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_splat() {
        for_parameters!(|T: PrimitiveNumber, A, x| {
            assert_eq!(Mask::<2, T, A>::splat(x), Mask::<2, T, A>::new(x, x));
            assert_eq!(Mask::<3, T, A>::splat(x), Mask::<3, T, A>::new(x, x, x));
            assert_eq!(Mask::<4, T, A>::splat(x), Mask::<4, T, A>::new(x, x, x, x));
        });
    }

    #[test]
    fn test_from_fn() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                Mask::<2, T, A>::from_fn(|i| [x, y][i]),
                Mask::<2, T, A>::new(x, y)
            );
            assert_eq!(
                Mask::<3, T, A>::from_fn(|i| [x, y, z][i]),
                Mask::<3, T, A>::new(x, y, z)
            );
            assert_eq!(
                Mask::<4, T, A>::from_fn(|i| [x, y, z, w][i]),
                Mask::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y).to_alignment(),
                Mask::<2, T, Aligned>::new(x, y)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).to_alignment(),
                Mask::<3, T, Aligned>::new(x, y, z)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).to_alignment(),
                Mask::<4, T, Aligned>::new(x, y, z, w)
            );

            assert_eq!(
                Mask::<2, T, A>::new(x, y).to_alignment(),
                Mask::<2, T, Unaligned>::new(x, y)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).to_alignment(),
                Mask::<3, T, Unaligned>::new(x, y, z)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).to_alignment(),
                Mask::<4, T, Unaligned>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_align() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y).align(),
                Mask::<2, T, Aligned>::new(x, y)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).align(),
                Mask::<3, T, Aligned>::new(x, y, z)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).align(),
                Mask::<4, T, Aligned>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y).unalign(),
                Mask::<2, T, Unaligned>::new(x, y)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).unalign(),
                Mask::<3, T, Unaligned>::new(x, y, z)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).unalign(),
                Mask::<4, T, Unaligned>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_array() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(Mask::<2, T, A>::new(x, y).to_array(), [x, y]);
            assert_eq!(Mask::<3, T, A>::new(x, y, z).to_array(), [x, y, z]);
            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).to_array(), [x, y, z, w]);
        });
    }

    #[test]
    fn test_all() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(Mask::<2, T, A>::new(x, y).all(), x && y);
            assert_eq!(Mask::<3, T, A>::new(x, y, z).all(), x && y && z);
            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).all(), x && y && z && w);
        });
    }

    #[test]
    fn test_any() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(Mask::<2, T, A>::new(x, y).any(), x || y);
            assert_eq!(Mask::<3, T, A>::new(x, y, z).any(), x || y || z);
            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).any(), x || y || z || w);
        });
    }

    #[test]
    fn test_select() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            let [a, b, c, d, e, f, g, h] = std::array::from_fn(T::as_from);

            assert_eq!(
                Mask::<2, T, A>::new(x, y)
                    .select(Vector::<2, T, A>::new(a, b), Vector::<2, T, A>::new(c, d)),
                Vector::<2, T, A>::new(if x { a } else { c }, if y { b } else { d })
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).select(
                    Vector::<3, T, A>::new(a, b, c),
                    Vector::<3, T, A>::new(d, e, f)
                ),
                Vector::<3, T, A>::new(
                    if x { a } else { d },
                    if y { b } else { e },
                    if z { c } else { f }
                )
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).select(
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h)
                ),
                Vector::<4, T, A>::new(
                    if x { a } else { e },
                    if y { b } else { f },
                    if z { c } else { g },
                    if w { d } else { h }
                )
            );
        });
    }

    #[test]
    fn test_iter() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(Mask::<2, T, A>::new(x, y).iter().collect_vec(), vec![x, y]);
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).iter().collect_vec(),
                vec![x, y, z]
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).iter().collect_vec(),
                vec![x, y, z, w]
            );
        });
    }

    #[test]
    fn test_get() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(Mask::<2, T, A>::new(x, y).get(0), x);
            assert_eq!(Mask::<2, T, A>::new(x, y).get(1), y);
            assert_panic!(Mask::<2, T, A>::new(x, y).get(2));

            assert_eq!(Mask::<3, T, A>::new(x, y, z).get(0), x);
            assert_eq!(Mask::<3, T, A>::new(x, y, z).get(1), y);
            assert_eq!(Mask::<3, T, A>::new(x, y, z).get(2), z);
            assert_panic!(Mask::<3, T, A>::new(x, y, z).get(3));

            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(0), x);
            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(1), y);
            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(2), z);
            assert_eq!(Mask::<4, T, A>::new(x, y, z, w).get(3), w);
            assert_panic!(Mask::<4, T, A>::new(x, y, z, w).get(4));
        });
    }

    #[test]
    fn test_set() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            let mut mask = Mask::<2, T, A>::new(x, y);
            mask.set(0, z);
            assert_eq!(mask, Mask::<2, T, A>::new(z, y));
            mask.set(1, w);
            assert_eq!(mask, Mask::<2, T, A>::new(z, w));
            assert_panic!(mask.clone().set(2, x));

            let mut mask = Mask::<3, T, A>::new(x, y, z);
            mask.set(0, z);
            assert_eq!(mask, Mask::<3, T, A>::new(z, y, z));
            mask.set(1, w);
            assert_eq!(mask, Mask::<3, T, A>::new(z, w, z));
            mask.set(2, x);
            assert_eq!(mask, Mask::<3, T, A>::new(z, w, x));
            assert_panic!(mask.clone().set(3, x));

            let mut mask = Mask::<4, T, A>::new(x, y, z, w);
            mask.set(0, z);
            assert_eq!(mask, Mask::<4, T, A>::new(z, y, z, w));
            mask.set(1, w);
            assert_eq!(mask, Mask::<4, T, A>::new(z, w, z, w));
            mask.set(2, x);
            assert_eq!(mask, Mask::<4, T, A>::new(z, w, x, w));
            mask.set(3, y);
            assert_eq!(mask, Mask::<4, T, A>::new(z, w, x, y));
            assert_panic!(mask.clone().set(4, x));
        });
    }

    #[test]
    fn test_from_inner() {
        assert_eq!(
            Mask2U::<u32>::from_inner(Repr2(false, true)),
            Mask2U::new(false, true)
        );
        assert_eq!(
            Mask3U::<u32>::from_inner(Repr3(false, true, false)),
            Mask3U::new(false, true, false)
        );
        assert_eq!(
            Mask4U::<u32>::from_inner(Repr4(false, true, false, true)),
            Mask4U::new(false, true, false, true)
        );
    }

    #[test]
    fn test_inner() {
        assert_eq!(Mask2U::<u32>::new(false, true).inner(), Repr2(false, true));
        assert_eq!(
            Mask3U::<u32>::new(false, true, false).inner(),
            Repr3(false, true, false)
        );
        assert_eq!(
            Mask4U::<u32>::new(false, true, false, true).inner(),
            Repr4(false, true, false, true)
        );
    }

    #[test]
    fn test_inner_mut() {
        assert_eq!(
            Mask2U::<u32>::new(false, true).inner_mut(),
            &mut Repr2(false, true)
        );
        assert_eq!(
            Mask3U::<u32>::new(false, true, false).inner_mut(),
            &mut Repr3(false, true, false)
        );
        assert_eq!(
            Mask4U::<u32>::new(false, true, false, true).inner_mut(),
            &mut Repr4(false, true, false, true)
        );
    }

    #[test]
    fn test_into_iter() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y).into_iter().collect_vec(),
                vec![x, y]
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z).into_iter().collect_vec(),
                vec![x, y, z]
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w).into_iter().collect_vec(),
                vec![x, y, z, w]
            );
        });
    }

    #[test]
    fn test_debug() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                format!("{:?}", Mask::<2, T, A>::new(x, y)),
                format!("({x:?}, {y:?})")
            );
            assert_eq!(
                format!("{:?}", Mask::<3, T, A>::new(x, y, z)),
                format!("({x:?}, {y:?}, {z:?})")
            );
            assert_eq!(
                format!("{:?}", Mask::<4, T, A>::new(x, y, z, w)),
                format!("({x:?}, {y:?}, {z:?}, {w:?})")
            );
        });
    }

    #[test]
    fn test_display() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(
                format!("{}", Mask::<2, T, A>::new(x, y)),
                format!("({x}, {y})")
            );
            assert_eq!(
                format!("{}", Mask::<3, T, A>::new(x, y, z)),
                format!("({x}, {y}, {z})")
            );
            assert_eq!(
                format!("{}", Mask::<4, T, A>::new(x, y, z, w)),
                format!("({x}, {y}, {z}, {w})")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y) == Mask::<2, T, A>::new(z, w),
                x == z && y == w
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z) == Mask::<3, T, A>::new(w, a, b),
                x == w && y == a && z == b
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w) == Mask::<4, T, A>::new(a, b, c, d),
                x == a && y == b && z == c && w == d
            );
        });
    }

    #[test]
    fn test_ne() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y) != Mask::<2, T, A>::new(z, w),
                x != z || y != w
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z) != Mask::<3, T, A>::new(w, a, b),
                x != w || y != a || z != b
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w) != Mask::<4, T, A>::new(a, b, c, d),
                x != a || y != b || z != c || w != d
            );
        });
    }

    #[test]
    fn test_default() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Mask::<2, T, A>::default(), Mask::splat(bool::default()));
            assert_eq!(Mask::<3, T, A>::default(), Mask::splat(bool::default()));
            assert_eq!(Mask::<4, T, A>::default(), Mask::splat(bool::default()));
        });
    }

    #[test]
    fn test_not() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w| {
            assert_eq!(!Mask::<2, T, A>::new(x, y), Mask::<2, T, A>::new(!x, !y));
            assert_eq!(
                !Mask::<3, T, A>::new(x, y, z),
                Mask::<3, T, A>::new(!x, !y, !z)
            );
            assert_eq!(
                !Mask::<4, T, A>::new(x, y, z, w),
                Mask::<4, T, A>::new(!x, !y, !z, !w)
            );
        });
    }

    #[test]
    fn test_bitand() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y) & Mask::<2, T, A>::new(z, w),
                Mask::<2, T, A>::new(x && z, y && w)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z) & Mask::<3, T, A>::new(w, a, b),
                Mask::<3, T, A>::new(x && w, y && a, z && b)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w) & Mask::<4, T, A>::new(a, b, c, d),
                Mask::<4, T, A>::new(x && a, y && b, z && c, w && d)
            );
        });
    }

    #[test]
    fn test_bitor() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y) | Mask::<2, T, A>::new(z, w),
                Mask::<2, T, A>::new(x || z, y || w)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z) | Mask::<3, T, A>::new(w, a, b),
                Mask::<3, T, A>::new(x || w, y || a, z || b)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w) | Mask::<4, T, A>::new(a, b, c, d),
                Mask::<4, T, A>::new(x || a, y || b, z || c, w || d)
            );
        });
    }

    #[test]
    fn test_bitxor() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            assert_eq!(
                Mask::<2, T, A>::new(x, y) ^ Mask::<2, T, A>::new(z, w),
                Mask::<2, T, A>::new(x ^ z, y ^ w)
            );
            assert_eq!(
                Mask::<3, T, A>::new(x, y, z) ^ Mask::<3, T, A>::new(w, a, b),
                Mask::<3, T, A>::new(x ^ w, y ^ a, z ^ b)
            );
            assert_eq!(
                Mask::<4, T, A>::new(x, y, z, w) ^ Mask::<4, T, A>::new(a, b, c, d),
                Mask::<4, T, A>::new(x ^ a, y ^ b, z ^ c, w ^ d)
            );
        });
    }

    #[test]
    fn test_bitand_assign() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            let mut mask = Mask::<2, T, A>::new(x, y);
            mask &= Mask::<2, T, A>::new(z, w);
            assert_eq!(mask, Mask::<2, T, A>::new(x && z, y && w));

            let mut mask = Mask::<3, T, A>::new(x, y, z);
            mask &= Mask::<3, T, A>::new(w, a, b);
            assert_eq!(mask, Mask::<3, T, A>::new(x && w, y && a, z && b));

            let mut mask = Mask::<4, T, A>::new(x, y, z, w);
            mask &= Mask::<4, T, A>::new(a, b, c, d);
            assert_eq!(mask, Mask::<4, T, A>::new(x && a, y && b, z && c, w && d));
        });
    }

    #[test]
    fn test_bitor_assign() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            let mut mask = Mask::<2, T, A>::new(x, y);
            mask |= Mask::<2, T, A>::new(z, w);
            assert_eq!(mask, Mask::<2, T, A>::new(x || z, y || w));

            let mut mask = Mask::<3, T, A>::new(x, y, z);
            mask |= Mask::<3, T, A>::new(w, a, b);
            assert_eq!(mask, Mask::<3, T, A>::new(x || w, y || a, z || b));

            let mut mask = Mask::<4, T, A>::new(x, y, z, w);
            mask |= Mask::<4, T, A>::new(a, b, c, d);
            assert_eq!(mask, Mask::<4, T, A>::new(x || a, y || b, z || c, w || d));
        });
    }

    #[test]
    fn test_bitxor_assign() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z, w, a, b, c, d| {
            let mut mask = Mask::<2, T, A>::new(x, y);
            mask ^= Mask::<2, T, A>::new(z, w);
            assert_eq!(mask, Mask::<2, T, A>::new(x ^ z, y ^ w));

            let mut mask = Mask::<3, T, A>::new(x, y, z);
            mask ^= Mask::<3, T, A>::new(w, a, b);
            assert_eq!(mask, Mask::<3, T, A>::new(x ^ w, y ^ a, z ^ b));

            let mut mask = Mask::<4, T, A>::new(x, y, z, w);
            mask ^= Mask::<4, T, A>::new(a, b, c, d);
            assert_eq!(mask, Mask::<4, T, A>::new(x ^ a, y ^ b, z ^ c, w ^ d));
        });
    }
}
