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
/// - [`Mask2<T>`] for `Mask<2, T, Unaligned>`.
/// - [`Mask3<T>`] for `Mask<3, T, Unaligned>`.
/// - [`Mask4<T>`] for `Mask<4, T, Unaligned>`.
/// - [`Mask2A<T>`] for `Mask<2, T, Aligned>`.
/// - [`Mask3A<T>`] for `Mask<3, T, Aligned>`.
/// - [`Mask4A<T>`] for `Mask<4, T, Aligned>`.
///
/// # Memory layout
///
/// `Mask<N, T, A>` does not have a stable representation, but does guarantee
/// certain properties.
///
/// `Mask<N, T, A>` does not contain any uninitialized bytes.
/// `Mask<N, T, A>` accepts the all-zero byte-pattern.
#[repr(transparent)]
pub struct Mask<const N: usize, T, A: Alignment>(
    #[expect(clippy::type_complexity)]
    pub(crate)  <A as Alignment>::Select<
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
/// # No SIMD alignment
///
/// `Mask2<T>` does not have SIMD alignment. See [`Mask2A<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask2<T> = Mask<2, T, Unaligned>;

/// A 3-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask3<T>` does not have SIMD alignment. See [`Mask3A<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask3<T> = Mask<3, T, Unaligned>;

/// A 4-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask4<T>` does not have SIMD alignment. See [`Mask4A<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask4<T> = Mask<4, T, Unaligned>;

/// A 2-element vector mask.
///
/// # SIMD alignment
///
/// `Mask2A<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask2<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask2A<T> = Mask<2, T, Aligned>;

/// A 3-element vector mask.
///
/// # SIMD alignment
///
/// `Mask3A<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask3<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask3A<T> = Mask<3, T, Aligned>;

/// A 4-element vector mask.
///
/// # SIMD alignment
///
/// `Mask4A<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask4<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask4A<T> = Mask<4, T, Aligned>;

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
    /// # use ggmath::{Aligned, Unaligned, Mask3, Mask3A};
    /// #
    /// let unaligned = Mask3::<f32>::new(false, true, false);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Mask3A::new(false, true, false));
    ///
    /// let aligned = Mask3A::<f32>::new(false, true, false);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Mask3::new(false, true, false));
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
    /// # use ggmath::{Mask3, Mask3A};
    /// #
    /// let unaligned = Mask3::<f32>::new(false, true, false);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Mask3A::new(false, true, false));
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
    /// # use ggmath::{Mask3, Mask3A};
    /// #
    /// let aligned = Mask3A::<f32>::new(false, true, false);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Mask3::new(false, true, false));
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

    use std::{convert::identity, format, vec::Vec};

    use crate::{
        Aligned, Mask, Mask2, Mask3, Mask4, Unaligned, Vector,
        utils::{Repr2, Repr3, Repr4, assert_panic, for_types, random_iter},
    };

    #[test]
    fn test_layout() {
        for_types!(|N, T: PrimitiveNumber| {
            // This test relies on guarantees that the public API does not make.
            // This may need to be modified for future layout changes.

            assert!(
                size_of::<Mask<N, T, Aligned>>() == N && align_of::<Mask<N, T, Aligned>>() == 1
                    || size_of::<Mask<N, T, Aligned>>() == size_of::<Vector<N, T, Aligned>>()
                        && align_of::<Mask<N, T, Aligned>>() == align_of::<Vector<N, T, Aligned>>()
            );

            assert_eq!(size_of::<Mask<N, T, Unaligned>>(), N);
            assert_eq!(align_of::<Mask<N, T, Unaligned>>(), 1);
        });
    }

    #[test]
    fn test_from_array() {
        for_types!(|T: PrimitiveNumber, A| {
            for [x, y, z, w] in random_iter::<[bool; 4]>() {
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
            }
        });
    }

    #[test]
    fn test_splat() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for x in random_iter() {
                assert_eq!(Mask::<N, T, A>::splat(x), Mask::from_array([x; N]));
            }
        });
    }

    #[test]
    fn test_from_fn() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for array in random_iter::<[bool; N]>() {
                assert_eq!(
                    Mask::<N, T, A>::from_fn(|i| array[i]),
                    Mask::from_array(array)
                );
            }
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for array in random_iter::<[bool; N]>() {
                assert_eq!(
                    Mask::<N, T, A>::from_array(array).to_alignment(),
                    Mask::<N, T, Aligned>::from_array(array)
                );
                assert_eq!(
                    Mask::<N, T, A>::from_array(array).to_alignment(),
                    Mask::<N, T, Unaligned>::from_array(array)
                );
            }
        });
    }

    #[test]
    fn test_align() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for array in random_iter::<[bool; N]>() {
                assert_eq!(
                    Mask::<N, T, A>::from_array(array).align(),
                    Mask::<N, T, Aligned>::from_array(array)
                );
            }
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for array in random_iter::<[bool; N]>() {
                assert_eq!(
                    Mask::<N, T, A>::from_array(array).unalign(),
                    Mask::<N, T, Unaligned>::from_array(array)
                );
            }
        });
    }

    #[test]
    fn test_to_array() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for array in random_iter::<[bool; N]>() {
                assert_eq!(Mask::<N, T, A>::from_array(array).to_array(), array);
            }
        });
    }

    #[test]
    fn test_all() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in [Mask::splat(false), Mask::splat(true)]
                .into_iter()
                .chain(random_iter::<Mask<N, T, A>>())
            {
                assert_eq!(mask.all(), mask.iter().all(identity));
            }
        });
    }

    #[test]
    fn test_any() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in [Mask::splat(false), Mask::splat(true)]
                .into_iter()
                .chain(random_iter::<Mask<N, T, A>>())
            {
                assert_eq!(mask.any(), mask.iter().any(identity));
            }
        });
    }

    #[test]
    fn test_select() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let if_true = Vector::<N, T, A>::from_fn(T::as_from);
            let if_false = Vector::<N, T, A>::from_fn(|i| T::as_from(i + N));

            for mask in random_iter::<Mask<N, T, A>>() {
                assert_eq!(
                    mask.select(if_true, if_false),
                    Vector::from_fn(|i| if mask.get(i) { if_true[i] } else { if_false[i] })
                );
            }
        });
    }

    #[test]
    fn test_iter() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in random_iter::<Mask<N, T, A>>() {
                assert_eq!(
                    mask.iter().collect::<Vec<bool>>(),
                    Vec::from(mask.to_array())
                );
            }
        });
    }

    #[test]
    fn test_get() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in random_iter::<Mask<N, T, A>>() {
                for i in 0..N {
                    assert_eq!(mask.get(i), mask.to_array()[i]);
                }

                assert_panic!(mask.get(N));
                assert_panic!(mask.get(N + 1));
            }
        });
    }

    #[test]
    fn test_set() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in random_iter::<Mask<N, T, A>>() {
                for value in [false, true] {
                    for i in 0..N {
                        let mut result = mask;
                        result.set(i, value);

                        let mut expected = mask.to_array();
                        expected[i] = value;

                        assert_eq!(result, Mask::from_array(expected));
                    }

                    assert_panic!(mask.clone().set(N, value));
                    assert_panic!(mask.clone().set(N + 1, value));
                }
            }
        });
    }

    #[test]
    fn test_from_inner() {
        assert_eq!(
            Mask2::<u32>::from_inner(Repr2(false, true)),
            Mask2::new(false, true)
        );
        assert_eq!(
            Mask3::<u32>::from_inner(Repr3(false, true, false)),
            Mask3::new(false, true, false)
        );
        assert_eq!(
            Mask4::<u32>::from_inner(Repr4(false, true, false, true)),
            Mask4::new(false, true, false, true)
        );
    }

    #[test]
    fn test_inner() {
        assert_eq!(Mask2::<u32>::new(false, true).inner(), Repr2(false, true));
        assert_eq!(
            Mask3::<u32>::new(false, true, false).inner(),
            Repr3(false, true, false)
        );
        assert_eq!(
            Mask4::<u32>::new(false, true, false, true).inner(),
            Repr4(false, true, false, true)
        );
    }

    #[test]
    fn test_inner_mut() {
        assert_eq!(
            Mask2::<u32>::new(false, true).inner_mut(),
            &mut Repr2(false, true)
        );
        assert_eq!(
            Mask3::<u32>::new(false, true, false).inner_mut(),
            &mut Repr3(false, true, false)
        );
        assert_eq!(
            Mask4::<u32>::new(false, true, false, true).inner_mut(),
            &mut Repr4(false, true, false, true)
        );
    }

    #[test]
    fn test_into_iter() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in random_iter::<Mask<N, T, A>>() {
                assert_eq!(
                    mask.into_iter().collect::<Vec<bool>>(),
                    Vec::from(mask.to_array())
                );
            }
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            for [x, y, z, w] in random_iter::<[bool; 4]>() {
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
            }
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            for [x, y, z, w] in random_iter::<[bool; 4]>() {
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
            }
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>()
                .chain(random_iter().map(|mask| [mask, mask]))
                .chain(random_iter::<Mask<N, T, A>>().map(|mask| [mask, !mask]))
            {
                assert_eq!(mask_1 == mask_2, mask_1.to_array() == mask_2.to_array());
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>()
                .chain(random_iter().map(|mask| [mask, mask]))
                .chain(random_iter::<Mask<N, T, A>>().map(|mask| [mask, !mask]))
            {
                assert_eq!(mask_1 != mask_2, mask_1.to_array() != mask_2.to_array());
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(Mask::<N, T, A>::default(), Mask::splat(bool::default()));
        });
    }

    #[test]
    fn test_not() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for mask in random_iter::<Mask<N, T, A>>() {
                assert_eq!(!mask, Mask::from_array(mask.to_array().map(|x| !x)));
            }
        });
    }

    #[test]
    fn test_bitand() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>() {
                assert_eq!(
                    mask_1 & mask_2,
                    Mask::<N, T, A>::from_fn(|i| mask_1.get(i) & mask_2.get(i))
                );
            }
        });
    }

    #[test]
    fn test_bitor() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>() {
                assert_eq!(
                    mask_1 | mask_2,
                    Mask::<N, T, A>::from_fn(|i| mask_1.get(i) | mask_2.get(i))
                );
            }
        });
    }

    #[test]
    fn test_bitxor() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>() {
                assert_eq!(
                    mask_1 ^ mask_2,
                    Mask::<N, T, A>::from_fn(|i| mask_1.get(i) ^ mask_2.get(i))
                );
            }
        });
    }

    #[test]
    fn test_bitand_assign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>() {
                let mut result = mask_1;
                result &= mask_2;

                assert_eq!(result, mask_1 & mask_2);
            }
        });
    }

    #[test]
    fn test_bitor_assign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>() {
                let mut result = mask_1;
                result |= mask_2;

                assert_eq!(result, mask_1 | mask_2);
            }
        });
    }

    #[test]
    fn test_bitxor_assign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for [mask_1, mask_2] in random_iter::<[Mask<N, T, A>; 2]>() {
                let mut result = mask_1;
                result ^= mask_2;

                assert_eq!(result, mask_1 ^ mask_2);
            }
        });
    }
}
