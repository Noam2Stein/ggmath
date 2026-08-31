use wide::Select;

use crate::{Alignment, Length, Rotor, Scalar, length::TwoOrThree, utils::WideTy};

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
    pub fn from_lanes(_lanes: &[Rotor<N, T, A>; LANES]) -> Self {
        todo!()
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
    pub fn from_lane_fn<F>(_f: F) -> Self
    where
        F: FnMut(usize) -> Rotor<N, T, A>,
    {
        todo!()
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
        todo!()
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
    pub fn lane(&self, _lane: usize) -> Rotor<N, T, A> {
        todo!()
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
    pub fn set_lane(&mut self, _lane: usize, _value: Rotor<N, T, A>) {
        todo!()
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, _other: &Self) -> Wide {
        todo!()
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, _other: &Self) -> Wide {
        todo!()
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
                _if_true: Rotor<N, Wide, A>,
                _if_false: Rotor<N, Wide, A>,
            ) -> Rotor<N, Wide, A> {
                todo!()
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
