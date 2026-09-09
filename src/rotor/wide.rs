use wide::Select;

use crate::{Alignment, Length, Rotor, Scalar, Vector, length::Three, utils::WideTy};

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
    Length<N>: Three,
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
    ///     Rotor3::from_elements(1, 2, 3, 4),
    ///     Rotor3::from_elements(11, 12, 13, 14),
    ///     Rotor3::from_elements(21, 22, 23, 24),
    ///     Rotor3::from_elements(31, 32, 33, 34),
    /// ];
    /// assert_eq!(
    ///     Rotor3::<i32x4>::from_lanes(&lanes),
    ///     Rotor3::from_elements(
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
        Self(Vector::<4, Wide, A>::from_lane_fn(|lane| lanes[lane].0))
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
    ///     Rotor3::from_elements(1, 2, 3, 4),
    ///     Rotor3::from_elements(11, 12, 13, 14),
    ///     Rotor3::from_elements(21, 22, 23, 24),
    ///     Rotor3::from_elements(31, 32, 33, 34),
    /// ];
    /// assert_eq!(
    ///     Rotor3::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Rotor3::from_elements(
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
    /// let soa = Rotor3::from_elements(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// assert_eq!(
    ///     soa.to_lanes(),
    ///     [
    ///         Rotor3::from_elements(1, 2, 3, 4),
    ///         Rotor3::from_elements(11, 12, 13, 14),
    ///         Rotor3::from_elements(21, 22, 23, 24),
    ///         Rotor3::from_elements(31, 32, 33, 34),
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
    /// let soa = Rotor3::from_elements(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// assert_eq!(
    ///     soa.lane(1),
    ///     Rotor3::from_elements(11, 12, 13, 14),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Rotor<N, T, A> {
        Rotor(self.0.lane(lane))
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
    /// let mut soa = Rotor3::from_elements(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// soa.set_lane(1, Rotor3::IDENTITY);
    /// assert_eq!(
    ///     soa,
    ///     Rotor3::from_elements(
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
        self.0.set_lane(lane, value.0);
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        self.0.simd_eq(other.0)
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, other: &Self) -> Wide {
        self.0.simd_ne(other.0)
    }
}

/// Unfortunately this cannot be done with a generic `Mask` type due to orphan
/// rules.
macro_rules! impl_select {
    ($Mask:ident) => {
        impl<const N: usize, Wide, A: Alignment> Select<Rotor<N, Wide, A>> for wide::$Mask
        where
            Length<N>: Three,
            wide::$Mask: Select<Wide>,
            Wide: WideTy,
        {
            #[inline]
            fn select(
                self,
                if_true: Rotor<N, Wide, A>,
                if_false: Rotor<N, Wide, A>,
            ) -> Rotor<N, Wide, A> {
                Rotor(self.select::<Vector<4, Wide, A>>(if_true.0, if_false.0))
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
        Rotor3, Vec4,
        test_utils::{assert_panic, assert_test_eq, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Rotor3::<i32x4>::from_lanes(&[
                Rotor3::from_elements(0, 1, 2, 3),
                Rotor3::from_elements(10, 11, 12, 13),
                Rotor3::from_elements(20, 21, 22, 23),
                Rotor3::from_elements(30, 31, 32, 33),
            ]),
            Rotor3::from_elements(
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
            ),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Rotor3::<i32x4>::from_lane_fn(|i| [
                Rotor3::from_elements(0, 1, 2, 3),
                Rotor3::from_elements(10, 11, 12, 13),
                Rotor3::from_elements(20, 21, 22, 23),
                Rotor3::from_elements(30, 31, 32, 33),
            ][i]),
            Rotor3::from_elements(
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
            ),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Rotor3::from_elements(
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
            )
            .to_lanes(),
            [
                Rotor3::from_elements(0, 1, 2, 3),
                Rotor3::from_elements(10, 11, 12, 13),
                Rotor3::from_elements(20, 21, 22, 23),
                Rotor3::from_elements(30, 31, 32, 33),
            ],
        );
    }

    #[test]
    fn test_lane() {
        let rotor = Rotor3::from_elements(
            i32x4::new([0, 10, 20, 30]),
            i32x4::new([1, 11, 21, 31]),
            i32x4::new([2, 12, 22, 32]),
            i32x4::new([3, 13, 23, 33]),
        );

        assert_eq!(rotor.lane(0), Rotor3::from_elements(0, 1, 2, 3));
        assert_eq!(rotor.lane(1), Rotor3::from_elements(10, 11, 12, 13));
        assert_eq!(rotor.lane(2), Rotor3::from_elements(20, 21, 22, 23));
        assert_eq!(rotor.lane(3), Rotor3::from_elements(30, 31, 32, 33));
        assert_panic!(rotor.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut rotor = Rotor3::from_elements(
            i32x4::new([0, 10, 20, 30]),
            i32x4::new([1, 11, 21, 31]),
            i32x4::new([2, 12, 22, 32]),
            i32x4::new([3, 13, 23, 33]),
        );

        rotor.set_lane(0, Rotor3::from_elements(-1, -2, -3, -4));
        assert_eq!(
            rotor,
            Rotor3::from_elements(
                i32x4::new([-1, 10, 20, 30]),
                i32x4::new([-2, 11, 21, 31]),
                i32x4::new([-3, 12, 22, 32]),
                i32x4::new([-4, 13, 23, 33]),
            )
        );
        rotor.set_lane(1, Rotor3::from_elements(-10, -11, -12, -13));
        assert_eq!(
            rotor,
            Rotor3::from_elements(
                i32x4::new([-1, -10, 20, 30]),
                i32x4::new([-2, -11, 21, 31]),
                i32x4::new([-3, -12, 22, 32]),
                i32x4::new([-4, -13, 23, 33]),
            )
        );
        rotor.set_lane(2, Rotor3::from_elements(-20, -21, -22, -23));
        assert_eq!(
            rotor,
            Rotor3::from_elements(
                i32x4::new([-1, -10, -20, 30]),
                i32x4::new([-2, -11, -21, 31]),
                i32x4::new([-3, -12, -22, 32]),
                i32x4::new([-4, -13, -23, 33]),
            )
        );
        rotor.set_lane(3, Rotor3::from_elements(-30, -31, -32, -33));
        assert_eq!(
            rotor,
            Rotor3::from_elements(
                i32x4::new([-1, -10, -20, -30]),
                i32x4::new([-2, -11, -21, -31]),
                i32x4::new([-3, -12, -22, -32]),
                i32x4::new([-4, -13, -23, -33]),
            )
        );
        assert_panic!(rotor.clone().set_lane(4, Rotor3::IDENTITY));
    }

    #[test]
    fn test_simd_eq() {
        for ([a, b], mask) in random_iter::<([Rotor3<i32x4>; 2], Vec4<i32x4>)>() {
            let b = Rotor3::from_raw_vector(mask.negative_mask().select(a.0, b.0));

            assert_test_eq!(
                a.simd_eq(&b),
                i32x4::new(std::array::from_fn(
                    |lane| if a.lane(lane) == b.lane(lane) { !0 } else { 0 }
                ))
            );
        }
    }

    #[test]
    fn test_simd_ne() {
        for ([a, b], mask) in random_iter::<([Rotor3<i32x4>; 2], Vec4<i32x4>)>() {
            let b = Rotor3::from_raw_vector(mask.negative_mask().select(a.0, b.0));

            assert_test_eq!(
                a.simd_ne(&b),
                i32x4::new(std::array::from_fn(
                    |lane| if a.lane(lane) != b.lane(lane) { !0 } else { 0 }
                ))
            );
        }
    }

    #[test]
    fn test_scalar_select() {
        for (mask, [if_true, if_false]) in random_iter::<(i32x4, [Rotor3<f32x4>; 2])>() {
            let mask = mask.is_negative();

            assert_test_eq!(
                mask.select(if_true, if_false),
                Rotor3::from_lane_fn(|lane| if mask.as_array()[lane].is_negative() {
                    if_true.lane(lane)
                } else {
                    if_false.lane(lane)
                })
            );
        }
    }
}
