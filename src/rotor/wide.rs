use wide::Select;

use crate::{
    Alignment, Length, Rotor, Scalar, Vector,
    length::TwoOrThree,
    utils::{WideTy, specialize_23},
};

/// Functionality for [SoA] (Structure of Arrays) rotors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Rotor<N, Wide, A>
where
    Length<N>: TwoOrThree,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) rotor from an array of regular,
    /// non-SoA rotors corresponding to each output lane.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rotor3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Rotor3::new(1, 2, 3, 4),
    ///     Rotor3::new(11, 12, 13, 14),
    ///     Rotor3::new(21, 22, 23, 24),
    ///     Rotor3::new(31, 32, 33, 34),
    /// ];
    /// assert_eq!(
    ///     Rotor3::<i32x4>::from_lanes(&lanes),
    ///     Rotor3::new(
    ///         i32x4::new([1, 11, 21, 31]),
    ///         i32x4::new([2, 12, 22, 32]),
    ///         i32x4::new([3, 13, 23, 33]),
    ///         i32x4::new([4, 14, 24, 34]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Rotor<N, T, A>; LANES]) -> Self {
        specialize_23!(Rotor::<N, Wide, A>::from_lanes_backend(lanes))
    }

    /// Creates an SoA (Structure of Arrays) rotor by calling function `f` for
    /// each output lane.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rotor3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Rotor3::new(1, 2, 3, 4),
    ///     Rotor3::new(11, 12, 13, 14),
    ///     Rotor3::new(21, 22, 23, 24),
    ///     Rotor3::new(31, 32, 33, 34),
    /// ];
    /// assert_eq!(
    ///     Rotor3::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Rotor3::new(
    ///         i32x4::new([1, 11, 21, 31]),
    ///         i32x4::new([2, 12, 22, 32]),
    ///         i32x4::new([3, 13, 23, 33]),
    ///         i32x4::new([4, 14, 24, 34]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Rotor<N, T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) rotor to an array of regular,
    /// non-SoA rotors corresponding to each input lane.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rotor3;
    /// # use wide::i32x4;
    /// #
    /// let soa = Rotor3::new(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// assert_eq!(
    ///     soa.to_lanes(),
    ///     [
    ///         Rotor3::new(1, 2, 3, 4),
    ///         Rotor3::new(11, 12, 13, 14),
    ///         Rotor3::new(21, 22, 23, 24),
    ///         Rotor3::new(31, 32, 33, 34),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Rotor<N, T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) rotor transform and returns the
    /// lane at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rotor3;
    /// # use wide::i32x4;
    /// #
    /// let soa = Rotor3::new(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    ///     i32x4::new([5, 15, 25, 35]),
    ///     i32x4::new([6, 16, 26, 36]),
    ///     i32x4::new([7, 17, 27, 37]),
    ///     i32x4::new([8, 18, 28, 38]),
    ///     i32x4::new([9, 19, 29, 39]),
    /// );
    /// assert_eq!(
    ///     soa.lane(1),
    ///     Rotor3::new(11, 12, 13, 14),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Rotor<N, T, A> {
        specialize_23!(Rotor::<N, Wide, A>::lane_backend(self, lane))
    }

    /// Takes an SoA (Structure of Arrays) rotor and sets the lane at the given
    /// index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of
    /// lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rotor3;
    /// # use wide::i32x4;
    /// #
    /// let mut soa = Rotor3::new(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// soa.set_lane(1, Rotor3::IDENTITY);
    /// assert_eq!(
    ///     soa,
    ///     Rotor3::new(
    ///         i32x4::new([1, 0, 21, 31]),
    ///         i32x4::new([2, 0, 22, 32]),
    ///         i32x4::new([3, 0, 23, 33]),
    ///         i32x4::new([4, 1, 24, 34]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Rotor<N, T, A>) {
        specialize_23!(Rotor::<N, Wide, A>::set_lane_backend(self, lane, value))
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        specialize_23!(Rotor::<N, Wide, A>::simd_eq_backend(self, other))
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, other: &Self) -> Wide {
        specialize_23!(Rotor::<N, Wide, A>::simd_ne_backend(self, other))
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Rotor<2, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Rotor<2, T, A>; LANES]) -> Self {
        Self::from_raw_elements(
            Wide::new(lanes.map(|lane| lane.xy)),
            Wide::new(lanes.map(|lane| lane.s)),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Rotor<2, T, A> {
        Rotor::<2, T, A>::from_raw_elements(self.xy.as_array()[lane], self.s.as_array()[lane])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Rotor<2, T, A>) {
        self.xy.as_mut_array()[lane] = value.xy;
        self.s.as_mut_array()[lane] = value.s;
    }

    #[track_caller]
    #[inline(always)]
    fn simd_eq_backend(&self, other: &Self) -> Wide {
        self.xy.simd_eq(other.xy) & self.s.simd_eq(other.s)
    }

    #[track_caller]
    #[inline(always)]
    fn simd_ne_backend(&self, other: &Self) -> Wide {
        self.xy.simd_ne(other.xy) | self.s.simd_ne(other.s)
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Rotor<3, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Rotor<3, T, A>; LANES]) -> Self {
        Self::from_raw_elements(
            Wide::new(lanes.map(|lane| lane.xy)),
            Wide::new(lanes.map(|lane| lane.xz)),
            Wide::new(lanes.map(|lane| lane.yz)),
            Wide::new(lanes.map(|lane| lane.s)),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Rotor<3, T, A> {
        Rotor::<3, T, A>::from_raw_elements(
            self.xy.as_array()[lane],
            self.xz.as_array()[lane],
            self.yz.as_array()[lane],
            self.s.as_array()[lane],
        )
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Rotor<3, T, A>) {
        self.xy.as_mut_array()[lane] = value.xy;
        self.xy.as_mut_array()[lane] = value.xz;
        self.yz.as_mut_array()[lane] = value.yz;
        self.s.as_mut_array()[lane] = value.s;
    }

    #[track_caller]
    #[inline(always)]
    fn simd_eq_backend(&self, other: &Self) -> Wide {
        self.xy.simd_eq(other.xy)
            & self.xz.simd_eq(other.xz)
            & self.yz.simd_eq(other.yz)
            & self.s.simd_eq(other.s)
    }

    #[track_caller]
    #[inline(always)]
    fn simd_ne_backend(&self, other: &Self) -> Wide {
        self.xy.simd_ne(other.xy)
            | self.xz.simd_ne(other.xz)
            | self.yz.simd_ne(other.yz)
            | self.s.simd_ne(other.s)
    }
}

#[expect(private_bounds)]
impl<Wide, A: Alignment> Rotor<2, Wide, A>
where
    Wide: WideTy,
{
    #[inline(always)]
    fn scalar_select_backend<Mask>(mask: Mask, if_true: Self, if_false: Self) -> Self
    where
        Mask: Copy + Select<Wide> + Select<Vector<2, Wide, A>>,
    {
        Self::from_raw_elements(
            mask.select(if_true.xy, if_false.xy),
            mask.select(if_true.s, if_false.s),
        )
    }
}

#[expect(private_bounds)]
impl<Wide, A: Alignment> Rotor<3, Wide, A>
where
    Wide: WideTy,
{
    #[inline(always)]
    fn scalar_select_backend<Mask>(mask: Mask, if_true: Self, if_false: Self) -> Self
    where
        Mask: Copy + Select<Wide> + Select<Vector<4, Wide, A>>,
    {
        Self::from_raw_elements(
            mask.select(if_true.xy, if_false.xy),
            mask.select(if_true.xz, if_false.xz),
            mask.select(if_true.yz, if_false.yz),
            mask.select(if_true.s, if_false.s),
        )
    }
}

/// Unfortunately this cannot be done with a generic `Mask` type due to orphan
/// rules.
macro_rules! impl_select {
    ($Mask:ident) => {
        impl<const N: usize, Wide, A: Alignment> Select<Rotor<N, Wide, A>> for wide::$Mask
        where
            Length<N>: TwoOrThree,
            wide::$Mask: Select<Wide>,
            Wide: WideTy,
        {
            #[inline]
            fn select(
                self,
                if_true: Rotor<N, Wide, A>,
                if_false: Rotor<N, Wide, A>,
            ) -> Rotor<N, Wide, A> {
                specialize_23!(Rotor::<N, Wide, A>::scalar_select_backend::<wide::$Mask>(
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

    #[test]
    fn test_from_lanes() {
        todo!()
    }

    #[test]
    fn test_from_lane_fn() {
        todo!()
    }

    #[test]
    fn test_to_lanes() {
        todo!()
    }

    #[test]
    fn test_lane() {
        todo!()
    }

    #[test]
    fn test_set_lane() {
        todo!()
    }

    #[test]
    fn test_simd_eq() {
        todo!()
    }

    #[test]
    fn test_simd_ne() {
        todo!()
    }

    #[test]
    fn test_scalar_select() {
        todo!()
    }
}
