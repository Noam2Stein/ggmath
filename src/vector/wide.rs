use wide::Select;

use crate::{
    Alignment, Length, Scalar, SupportedLength, Vector,
    utils::{WideTy, specialize},
};

/// Functionality for [SoA] (Structure of Arrays) vectors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Vector<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) vector from an array of
    /// lanes or scalar vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Vec3::new(1, 2, 3),
    ///     Vec3::new(4, 5, 6),
    ///     Vec3::new(7, 8, 9),
    ///     Vec3::new(10, 11, 12),
    /// ];
    /// assert_eq!(
    ///     Vec3::<i32x4>::from_lanes(&lanes),
    ///     Vec3::new(
    ///         i32x4::new([1, 4, 7, 10]),
    ///         i32x4::new([2, 5, 8, 11]),
    ///         i32x4::new([3, 6, 9, 12]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Vector<N, T, A>; LANES]) -> Self {
        specialize!(Vector::<N, Wide, A>::from_lanes_backend(lanes))
    }

    /// Creates an SoA (Structure of Arrays) vector by calling function
    /// `f` for each lane index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Vec3::new(1, 2, 3),
    ///     Vec3::new(4, 5, 6),
    ///     Vec3::new(7, 8, 9),
    ///     Vec3::new(10, 11, 12),
    /// ];
    /// assert_eq!(
    ///     Vec3::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Vec3::new(
    ///         i32x4::new([1, 4, 7, 10]),
    ///         i32x4::new([2, 5, 8, 11]),
    ///         i32x4::new([3, 6, 9, 12]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) vector to an array of
    /// lanes or scalar vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let vector = Vec3::new(
    ///     i32x4::new([1, 4, 7, 10]),
    ///     i32x4::new([2, 5, 8, 11]),
    ///     i32x4::new([3, 6, 9, 12]),
    /// );
    /// assert_eq!(
    ///     vector.to_lanes(),
    ///     [
    ///         Vec3::new(1, 2, 3),
    ///         Vec3::new(4, 5, 6),
    ///         Vec3::new(7, 8, 9),
    ///         Vec3::new(10, 11, 12),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(self) -> [Vector<N, T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) vector and returns the lane
    /// at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of
    /// lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let vector = Vec3::new(
    ///     i32x4::new([1, 4, 7, 10]),
    ///     i32x4::new([2, 5, 8, 11]),
    ///     i32x4::new([3, 6, 9, 12]),
    /// );
    /// assert_eq!(vector.lane(1), Vec3::new(4, 5, 6));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(self, lane: usize) -> Vector<N, T, A> {
        specialize!(Vector::<N, Wide, A>::lane_backend(self, lane))
    }

    /// Takes an SoA (Structure of Arrays) vector and sets the lane at
    /// the given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of
    /// lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let mut vector = Vec3::new(
    ///     i32x4::new([1, 4, 7, 10]),
    ///     i32x4::new([2, 5, 8, 11]),
    ///     i32x4::new([3, 6, 9, 12]),
    /// );
    /// vector.set_lane(1, Vec3::ZERO);
    /// assert_eq!(
    ///     vector,
    ///     Vec3::new(
    ///         i32x4::new([1, 0, 7, 10]),
    ///         i32x4::new([2, 0, 8, 11]),
    ///         i32x4::new([3, 0, 9, 12]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Vector<N, T, A>) {
        specialize!(Vector::<N, Wide, A>::set_lane_backend(self, lane, value))
    }

    /// For each lane, returns `true` if all elements of `self` are `true`.
    #[inline]
    #[must_use]
    pub fn all(self) -> Wide {
        specialize!(Vector::<N, Wide, A>::all_backend(self))
    }

    /// For each lane, returns `true` if any element of `self` is `true`.
    #[inline]
    #[must_use]
    pub fn any(self) -> Wide {
        specialize!(Vector::<N, Wide, A>::any_backend(self))
    }

    /// Selects between the elements of `if_true` and `if_false` based on the
    /// boolean elements of `self`.
    ///
    /// This assumes each SIMD vector in `self` is a [mask].
    ///
    /// [mask]: https://docs.rs/wide/latest/wide/#masks
    #[inline]
    #[must_use]
    pub fn select<Output>(
        self,
        if_true: Vector<N, Output, A>,
        if_false: Vector<N, Output, A>,
    ) -> Vector<N, Output, A>
    where
        Output: Scalar,
        Wide: Select<Output>,
    {
        specialize!(Vector::<N, Wide, A>::select_backend(
            self, if_true, if_false
        ))
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(self, other: Self) -> Wide {
        specialize!(Vector::<N, Wide, A>::simd_eq_backend(self, other))
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(self, other: Self) -> Wide {
        specialize!(Vector::<N, Wide, A>::simd_ne_backend(self, other))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding elements of `self` and `other` are equal.
    ///
    /// Equivalent to `(self.x == other.x, self.y == other.y, ...)` for each
    /// lane.
    #[inline]
    #[must_use]
    pub fn simd_eq_mask(self, other: Self) -> Self {
        specialize!(Vector::<N, Wide, A>::simd_eq_mask_backend(self, other))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding elements of `self` and `other` are not equal.
    ///
    /// Equivalent to `(self.x != other.x, self.y != other.y, ...)` for each lane.
    #[inline]
    #[must_use]
    pub fn simd_ne_mask(self, other: Self) -> Self {
        specialize!(Vector::<N, Wide, A>::simd_ne_mask_backend(self, other))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is less than the corresponding element
    /// of `other`.
    ///
    /// Equivalent to `(self.x < other.x, self.y < other.y, ...)` for each lane.
    #[inline]
    #[must_use]
    pub fn simd_lt_mask(self, other: Self) -> Self {
        specialize!(Vector::<N, Wide, A>::simd_lt_mask_backend(self, other))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is greater than the corresponding
    /// element of `other`.
    ///
    /// Equivalent to `(self.x > other.x, self.y > other.y, ...)` for each lane.
    #[inline]
    #[must_use]
    pub fn simd_gt_mask(self, other: Self) -> Self {
        specialize!(Vector::<N, Wide, A>::simd_gt_mask_backend(self, other))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is less than or equal to the
    /// corresponding element of `other`.
    ///
    /// Equivalent to `(self.x <= other.x, self.y <= other.y, ...)` for each
    /// lane.
    #[inline]
    #[must_use]
    pub fn simd_le_mask(self, other: Self) -> Self {
        specialize!(Vector::<N, Wide, A>::simd_le_mask_backend(self, other))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is greater than or equal to the
    /// corresponding element of `other`.
    ///
    /// Equivalent to `(self.x >= other.x, self.y >= other.y, ...)` for each
    /// lane.
    #[inline]
    #[must_use]
    pub fn simd_ge_mask(self, other: Self) -> Self {
        specialize!(Vector::<N, Wide, A>::simd_ge_mask_backend(self, other))
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Vector<2, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Vector<2, T, A>; LANES]) -> Self {
        Self::new(
            Wide::new(lanes.map(|lane| lane.x)),
            Wide::new(lanes.map(|lane| lane.y)),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(self, lane: usize) -> Vector<2, T, A> {
        Vector::<2, T, A>::new(self.x.as_array()[lane], self.y.as_array()[lane])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Vector<2, T, A>) {
        self.x.as_mut_array()[lane] = value.x;
        self.y.as_mut_array()[lane] = value.y;
    }

    #[inline(always)]
    fn all_backend(self) -> Wide {
        self.x & self.y
    }

    #[inline(always)]
    fn any_backend(self) -> Wide {
        self.x | self.y
    }

    #[inline(always)]
    fn select_backend<Output>(
        self,
        if_true: Vector<2, Output, A>,
        if_false: Vector<2, Output, A>,
    ) -> Vector<2, Output, A>
    where
        Output: Scalar,
        Wide: Select<Output>,
    {
        Vector::<2, Output, A>::new(
            self.x.select(if_true.x, if_false.x),
            self.y.select(if_true.y, if_false.y),
        )
    }

    #[inline(always)]
    fn simd_eq_backend(self, other: Self) -> Wide {
        self.x.simd_eq(other.x) & self.y.simd_eq(other.y)
    }

    #[inline(always)]
    fn simd_ne_backend(self, other: Self) -> Wide {
        self.x.simd_ne(other.x) | self.y.simd_ne(other.y)
    }

    #[inline(always)]
    fn simd_eq_mask_backend(self, other: Self) -> Self {
        Self::new(self.x.simd_eq(other.x), self.y.simd_eq(other.y))
    }

    #[inline(always)]
    fn simd_ne_mask_backend(self, other: Self) -> Self {
        Self::new(self.x.simd_ne(other.x), self.y.simd_ne(other.y))
    }

    #[inline(always)]
    fn simd_lt_mask_backend(self, other: Self) -> Self {
        Self::new(self.x.simd_lt(other.x), self.y.simd_lt(other.y))
    }

    #[inline(always)]
    fn simd_gt_mask_backend(self, other: Self) -> Self {
        Self::new(self.x.simd_gt(other.x), self.y.simd_gt(other.y))
    }

    #[inline(always)]
    fn simd_le_mask_backend(self, other: Self) -> Self {
        Self::new(self.x.simd_le(other.x), self.y.simd_le(other.y))
    }

    #[inline(always)]
    fn simd_ge_mask_backend(self, other: Self) -> Self {
        Self::new(self.x.simd_ge(other.x), self.y.simd_ge(other.y))
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Vector<3, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Vector<3, T, A>; LANES]) -> Self {
        Self::new(
            Wide::new(lanes.map(|lane| lane.x)),
            Wide::new(lanes.map(|lane| lane.y)),
            Wide::new(lanes.map(|lane| lane.z)),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(self, lane: usize) -> Vector<3, T, A> {
        Vector::<3, T, A>::new(
            self.x.as_array()[lane],
            self.y.as_array()[lane],
            self.z.as_array()[lane],
        )
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Vector<3, T, A>) {
        self.x.as_mut_array()[lane] = value.x;
        self.y.as_mut_array()[lane] = value.y;
        self.z.as_mut_array()[lane] = value.z;
    }

    #[inline(always)]
    fn all_backend(self) -> Wide {
        self.x & self.y & self.z
    }

    #[inline(always)]
    fn any_backend(self) -> Wide {
        self.x | self.y | self.z
    }

    #[inline(always)]
    fn select_backend<Output>(
        self,
        if_true: Vector<3, Output, A>,
        if_false: Vector<3, Output, A>,
    ) -> Vector<3, Output, A>
    where
        Output: Scalar,
        Wide: Select<Output>,
    {
        Vector::<3, Output, A>::new(
            self.x.select(if_true.x, if_false.x),
            self.y.select(if_true.y, if_false.y),
            self.z.select(if_true.z, if_false.z),
        )
    }

    #[inline(always)]
    fn simd_eq_backend(self, other: Self) -> Wide {
        self.x.simd_eq(other.x) & self.y.simd_eq(other.y) & self.z.simd_eq(other.z)
    }

    #[inline(always)]
    fn simd_ne_backend(self, other: Self) -> Wide {
        self.x.simd_ne(other.x) | self.y.simd_ne(other.y) | self.z.simd_ne(other.z)
    }

    #[inline(always)]
    fn simd_eq_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_eq(other.x),
            self.y.simd_eq(other.y),
            self.z.simd_eq(other.z),
        )
    }

    #[inline(always)]
    fn simd_ne_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_ne(other.x),
            self.y.simd_ne(other.y),
            self.z.simd_ne(other.z),
        )
    }

    #[inline(always)]
    fn simd_lt_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_lt(other.x),
            self.y.simd_lt(other.y),
            self.z.simd_lt(other.z),
        )
    }

    #[inline(always)]
    fn simd_gt_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_gt(other.x),
            self.y.simd_gt(other.y),
            self.z.simd_gt(other.z),
        )
    }

    #[inline(always)]
    fn simd_le_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_le(other.x),
            self.y.simd_le(other.y),
            self.z.simd_le(other.z),
        )
    }

    #[inline(always)]
    fn simd_ge_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_ge(other.x),
            self.y.simd_ge(other.y),
            self.z.simd_ge(other.z),
        )
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Vector<4, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Vector<4, T, A>; LANES]) -> Self {
        Self::new(
            Wide::new(lanes.map(|lane| lane.x)),
            Wide::new(lanes.map(|lane| lane.y)),
            Wide::new(lanes.map(|lane| lane.z)),
            Wide::new(lanes.map(|lane| lane.w)),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(self, lane: usize) -> Vector<4, T, A> {
        Vector::<4, T, A>::new(
            self.x.as_array()[lane],
            self.y.as_array()[lane],
            self.z.as_array()[lane],
            self.w.as_array()[lane],
        )
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Vector<4, T, A>) {
        self.x.as_mut_array()[lane] = value.x;
        self.y.as_mut_array()[lane] = value.y;
        self.z.as_mut_array()[lane] = value.z;
        self.w.as_mut_array()[lane] = value.w;
    }

    #[inline(always)]
    fn all_backend(self) -> Wide {
        self.x & self.y & self.z & self.w
    }

    #[inline(always)]
    fn any_backend(self) -> Wide {
        self.x | self.y | self.z | self.w
    }

    #[inline(always)]
    fn select_backend<Output>(
        self,
        if_true: Vector<4, Output, A>,
        if_false: Vector<4, Output, A>,
    ) -> Vector<4, Output, A>
    where
        Output: Scalar,
        Wide: Select<Output>,
    {
        Vector::<4, Output, A>::new(
            self.x.select(if_true.x, if_false.x),
            self.y.select(if_true.y, if_false.y),
            self.z.select(if_true.z, if_false.z),
            self.w.select(if_true.w, if_false.w),
        )
    }

    #[inline(always)]
    fn simd_eq_backend(self, other: Self) -> Wide {
        self.x.simd_eq(other.x)
            & self.y.simd_eq(other.y)
            & self.z.simd_eq(other.z)
            & self.w.simd_eq(other.w)
    }

    #[inline(always)]
    fn simd_ne_backend(self, other: Self) -> Wide {
        self.x.simd_ne(other.x)
            | self.y.simd_ne(other.y)
            | self.z.simd_ne(other.z)
            | self.w.simd_ne(other.w)
    }

    #[inline(always)]
    fn simd_eq_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_eq(other.x),
            self.y.simd_eq(other.y),
            self.z.simd_eq(other.z),
            self.w.simd_eq(other.w),
        )
    }

    #[inline(always)]
    fn simd_ne_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_ne(other.x),
            self.y.simd_ne(other.y),
            self.z.simd_ne(other.z),
            self.w.simd_ne(other.w),
        )
    }

    #[inline(always)]
    fn simd_lt_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_lt(other.x),
            self.y.simd_lt(other.y),
            self.z.simd_lt(other.z),
            self.w.simd_lt(other.w),
        )
    }

    #[inline(always)]
    fn simd_gt_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_gt(other.x),
            self.y.simd_gt(other.y),
            self.z.simd_gt(other.z),
            self.w.simd_gt(other.w),
        )
    }

    #[inline(always)]
    fn simd_le_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_le(other.x),
            self.y.simd_le(other.y),
            self.z.simd_le(other.z),
            self.w.simd_le(other.w),
        )
    }

    #[inline(always)]
    fn simd_ge_mask_backend(self, other: Self) -> Self {
        Self::new(
            self.x.simd_ge(other.x),
            self.y.simd_ge(other.y),
            self.z.simd_ge(other.z),
            self.w.simd_ge(other.w),
        )
    }
}

#[expect(private_bounds)]
impl<Wide, A: Alignment> Vector<2, Wide, A>
where
    Wide: WideTy,
{
    #[inline(always)]
    fn scalar_select_backend<Mask>(mask: Mask, if_true: Self, if_false: Self) -> Self
    where
        Mask: Copy + Select<Wide>,
    {
        Self::new(
            mask.select(if_true.x, if_false.x),
            mask.select(if_true.y, if_false.y),
        )
    }
}

#[expect(private_bounds)]
impl<Wide, A: Alignment> Vector<3, Wide, A>
where
    Wide: WideTy,
{
    #[inline(always)]
    fn scalar_select_backend<Mask>(mask: Mask, if_true: Self, if_false: Self) -> Self
    where
        Mask: Copy + Select<Wide>,
    {
        Self::new(
            mask.select(if_true.x, if_false.x),
            mask.select(if_true.y, if_false.y),
            mask.select(if_true.z, if_false.z),
        )
    }
}

#[expect(private_bounds)]
impl<Wide, A: Alignment> Vector<4, Wide, A>
where
    Wide: WideTy,
{
    #[inline(always)]
    fn scalar_select_backend<Mask>(mask: Mask, if_true: Self, if_false: Self) -> Self
    where
        Mask: Copy + Select<Wide>,
    {
        Self::new(
            mask.select(if_true.x, if_false.x),
            mask.select(if_true.y, if_false.y),
            mask.select(if_true.z, if_false.z),
            mask.select(if_true.w, if_false.w),
        )
    }
}

/// Unfortunately this cannot be done with a generic `Mask` type due to orphan
/// rules.
macro_rules! impl_select {
    ($Mask:ident) => {
        impl<const N: usize, Wide, A: Alignment> Select<Vector<N, Wide, A>> for wide::$Mask
        where
            Length<N>: SupportedLength,
            wide::$Mask: Select<Wide>,
            Wide: WideTy,
        {
            #[inline]
            fn select(
                self,
                if_true: Vector<N, Wide, A>,
                if_false: Vector<N, Wide, A>,
            ) -> Vector<N, Wide, A> {
                specialize!(Vector::<N, Wide, A>::scalar_select_backend::<wide::$Mask>(
                    self, if_true, if_false
                ))
            }
        }
    };
}
impl_select!(f32x4);
impl_select!(f32x8);
impl_select!(f32x16);
impl_select!(f64x2);
impl_select!(f64x4);
impl_select!(f64x8);
impl_select!(i8x16);
impl_select!(i8x32);
impl_select!(i8x64);
impl_select!(i16x8);
impl_select!(i16x16);
impl_select!(i16x32);
impl_select!(i32x4);
impl_select!(i32x8);
impl_select!(i32x16);
impl_select!(i64x2);
impl_select!(i64x4);
impl_select!(i64x8);
impl_select!(u8x16);
impl_select!(u8x32);
impl_select!(u8x64);
impl_select!(u16x8);
impl_select!(u16x16);
impl_select!(u16x32);
impl_select!(u32x4);
impl_select!(u32x8);
impl_select!(u32x16);
impl_select!(u64x2);
impl_select!(u64x4);
impl_select!(u64x8);

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::{f32x4, i32x4};

    use crate::{
        Unaligned, Vec2, Vec3, Vec3A, Vec4, Vector,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Vec3A::<i32x4>::from_lanes(&[
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(7, 8, 9),
                Vec3A::new(10, 11, 12),
            ]),
            Vec3A::new(
                i32x4::new([1, 4, 7, 10]),
                i32x4::new([2, 5, 8, 11]),
                i32x4::new([3, 6, 9, 12]),
            ),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Vec3A::<i32x4>::from_lane_fn(|lane| [
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(7, 8, 9),
                Vec3A::new(10, 11, 12),
            ][lane]),
            Vec3A::new(
                i32x4::new([1, 4, 7, 10]),
                i32x4::new([2, 5, 8, 11]),
                i32x4::new([3, 6, 9, 12]),
            ),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Vec3A::new(
                i32x4::new([1, 4, 7, 10]),
                i32x4::new([2, 5, 8, 11]),
                i32x4::new([3, 6, 9, 12]),
            )
            .to_lanes(),
            [
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(7, 8, 9),
                Vec3A::new(10, 11, 12),
            ]
        );
    }

    #[test]
    fn test_lane() {
        let vector = Vec3A::new(
            i32x4::new([1, 4, 7, 10]),
            i32x4::new([2, 5, 8, 11]),
            i32x4::new([3, 6, 9, 12]),
        );

        assert_eq!(vector.lane(0), Vec3A::new(1, 2, 3));
        assert_eq!(vector.lane(1), Vec3A::new(4, 5, 6));
        assert_eq!(vector.lane(2), Vec3A::new(7, 8, 9));
        assert_eq!(vector.lane(3), Vec3A::new(10, 11, 12));
        assert_panic!(vector.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut vector = Vec3A::new(
            i32x4::new([1, 4, 7, 10]),
            i32x4::new([2, 5, 8, 11]),
            i32x4::new([3, 6, 9, 12]),
        );

        vector.set_lane(0, Vec3A::new(-1, -2, -3));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, 4, 7, 10]),
                i32x4::new([-2, 5, 8, 11]),
                i32x4::new([-3, 6, 9, 12]),
            )
        );
        vector.set_lane(1, Vec3A::new(-4, -5, -6));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, -4, 7, 10]),
                i32x4::new([-2, -5, 8, 11]),
                i32x4::new([-3, -6, 9, 12]),
            )
        );
        vector.set_lane(2, Vec3A::new(-7, -8, -9));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, -4, -7, 10]),
                i32x4::new([-2, -5, -8, 11]),
                i32x4::new([-3, -6, -9, 12]),
            )
        );
        vector.set_lane(3, Vec3A::new(-10, -11, -12));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, -4, -7, -10]),
                i32x4::new([-2, -5, -8, -11]),
                i32x4::new([-3, -6, -9, -12]),
            )
        );
        assert_panic!(vector.clone().set_lane(4, Vector::ZERO));
    }

    #[test]
    fn test_all() {
        for [x, y, z, w] in random_iter::<[f32x4; 4]>() {
            assert_test_eq!(Vec2::new(x, y).all(), x & y);
            assert_test_eq!(Vec3::new(x, y, z).all(), x & y & z);
            assert_test_eq!(Vec4::new(x, y, z, w).all(), x & y & z & w);
        }
    }

    #[test]
    fn test_any() {
        for [x, y, z, w] in random_iter::<[f32x4; 4]>() {
            assert_test_eq!(Vec2::new(x, y).any(), x | y);
            assert_test_eq!(Vec3::new(x, y, z).any(), x | y | z);
            assert_test_eq!(Vec4::new(x, y, z, w).any(), x | y | z | w);
        }
    }

    #[test]
    fn test_select() {
        for_types!(|N| {
            for [mask, if_true, if_false] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                assert_test_eq!(
                    mask.select(if_true, if_false),
                    Vector::from_fn(|i| mask[i].select(if_true[i], if_false[i]))
                );
            }
        });
    }

    #[test]
    fn test_simd_eq() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(
                    a.simd_eq(b),
                    f32x4::new(std::array::from_fn(
                        |lane| if a.lane(lane) == b.lane(lane) {
                            f32::from_bits(!0)
                        } else {
                            0.0
                        }
                    ))
                );
            }
        });
    }

    #[test]
    fn test_simd_ne() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(
                    a.simd_ne(b),
                    f32x4::new(std::array::from_fn(
                        |lane| if a.lane(lane) != b.lane(lane) {
                            f32::from_bits(!0)
                        } else {
                            0.0
                        }
                    ))
                );
            }
        });
    }

    #[test]
    fn test_simd_eq_mask() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(a.simd_eq_mask(b), Vector::from_fn(|i| a[i].simd_eq(b[i])));
            }
        });
    }

    #[test]
    fn test_simd_ne_mask() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(a.simd_ne_mask(b), Vector::from_fn(|i| a[i].simd_ne(b[i])));
            }
        });
    }

    #[test]
    fn test_simd_lt_mask() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(a.simd_lt_mask(b), Vector::from_fn(|i| a[i].simd_lt(b[i])));
            }
        });
    }

    #[test]
    fn test_simd_gt_mask() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(a.simd_gt_mask(b), Vector::from_fn(|i| a[i].simd_gt(b[i])));
            }
        });
    }

    #[test]
    fn test_simd_le_mask() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(a.simd_le_mask(b), Vector::from_fn(|i| a[i].simd_le(b[i])));
            }
        });
    }

    #[test]
    fn test_simd_ge_mask() {
        for_types!(|N| {
            for [a, b, mask] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let mask = mask.sign_negative_mask();
                let b = mask.select(a, b);

                assert_test_eq!(a.simd_ge_mask(b), Vector::from_fn(|i| a[i].simd_ge(b[i])));
            }
        });
    }

    #[test]
    fn test_scalar_select() {
        for_types!(|N| {
            for (mask, [if_true, if_false]) in
                random_iter::<(i32x4, [Vector<N, f32x4, Unaligned>; 2])>()
            {
                let mask = mask.is_negative();

                assert_test_eq!(
                    mask.select(if_true, if_false),
                    Vector::from_lane_fn(|lane| if mask.as_array()[lane].is_negative() {
                        if_true.lane(lane)
                    } else {
                        if_false.lane(lane)
                    })
                );
            }
        });
    }
}
