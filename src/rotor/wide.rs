use crate::{
    Alignment, Length, Rotor, Scalar,
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
    /// # use ggmath::Rot3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Rot3::new(1, 2, 3, 4),
    ///     Rot3::new(11, 12, 13, 14),
    ///     Rot3::new(21, 22, 23, 24),
    ///     Rot3::new(31, 32, 33, 34),
    /// ];
    /// assert_eq!(
    ///     Rot3::<i32x4>::from_lanes(&lanes),
    ///     Rot3::new(
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
    /// # use ggmath::Rot3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Rot3::new(1, 2, 3, 4),
    ///     Rot3::new(11, 12, 13, 14),
    ///     Rot3::new(21, 22, 23, 24),
    ///     Rot3::new(31, 32, 33, 34),
    /// ];
    /// assert_eq!(
    ///     Rot3::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Rot3::new(
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
    /// # use ggmath::Rot3;
    /// # use wide::i32x4;
    /// #
    /// let soa = Rot3::new(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// assert_eq!(
    ///     soa.to_lanes(),
    ///     [
    ///         Rot3::new(1, 2, 3, 4),
    ///         Rot3::new(11, 12, 13, 14),
    ///         Rot3::new(21, 22, 23, 24),
    ///         Rot3::new(31, 32, 33, 34),
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
    /// # use ggmath::Rot3;
    /// # use wide::i32x4;
    /// #
    /// let soa = Rot3::new(
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
    ///     Rot3::new(11, 12, 13, 14),
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
    /// # use ggmath::Rot3;
    /// # use wide::i32x4;
    /// #
    /// let mut soa = Rot3::new(
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    /// );
    /// soa.set_lane(1, Rot3::IDENTITY);
    /// assert_eq!(
    ///     soa,
    ///     Rot3::new(
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

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::i32x4;

    use crate::{
        Rot2, Rot3, Rotor, Vec2, Vec4,
        test_utils::{assert_panic, assert_test_eq, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Rot3::<i32x4>::from_lanes(&[
                Rot3::from_raw_elements(0, 1, 2, 3),
                Rot3::from_raw_elements(10, 11, 12, 13),
                Rot3::from_raw_elements(20, 21, 22, 23),
                Rot3::from_raw_elements(30, 31, 32, 33),
            ]),
            Rot3::from_raw_elements(
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
            Rot3::<i32x4>::from_lane_fn(|i| [
                Rot3::from_raw_elements(0, 1, 2, 3),
                Rot3::from_raw_elements(10, 11, 12, 13),
                Rot3::from_raw_elements(20, 21, 22, 23),
                Rot3::from_raw_elements(30, 31, 32, 33),
            ][i]),
            Rot3::from_raw_elements(
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
            Rot3::from_raw_elements(
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
            )
            .to_lanes(),
            [
                Rot3::from_raw_elements(0, 1, 2, 3),
                Rot3::from_raw_elements(10, 11, 12, 13),
                Rot3::from_raw_elements(20, 21, 22, 23),
                Rot3::from_raw_elements(30, 31, 32, 33),
            ],
        );
    }

    #[test]
    fn test_lane() {
        let rotor = Rot3::from_raw_elements(
            i32x4::new([0, 10, 20, 30]),
            i32x4::new([1, 11, 21, 31]),
            i32x4::new([2, 12, 22, 32]),
            i32x4::new([3, 13, 23, 33]),
        );

        assert_eq!(rotor.lane(0), Rot3::from_raw_elements(0, 1, 2, 3));
        assert_eq!(rotor.lane(1), Rot3::from_raw_elements(10, 11, 12, 13));
        assert_eq!(rotor.lane(2), Rot3::from_raw_elements(20, 21, 22, 23));
        assert_eq!(rotor.lane(3), Rot3::from_raw_elements(30, 31, 32, 33));
        assert_panic!(rotor.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut rotor = Rot3::from_raw_elements(
            i32x4::new([0, 10, 20, 30]),
            i32x4::new([1, 11, 21, 31]),
            i32x4::new([2, 12, 22, 32]),
            i32x4::new([3, 13, 23, 33]),
        );

        rotor.set_lane(0, Rot3::from_raw_elements(-1, -2, -3, -4));
        assert_eq!(
            rotor,
            Rot3::from_raw_elements(
                i32x4::new([-1, 10, 20, 30]),
                i32x4::new([-2, 11, 21, 31]),
                i32x4::new([-3, 12, 22, 32]),
                i32x4::new([-4, 13, 23, 33]),
            )
        );
        rotor.set_lane(1, Rot3::from_raw_elements(-10, -11, -12, -13));
        assert_eq!(
            rotor,
            Rot3::from_raw_elements(
                i32x4::new([-1, -10, 20, 30]),
                i32x4::new([-2, -11, 21, 31]),
                i32x4::new([-3, -12, 22, 32]),
                i32x4::new([-4, -13, 23, 33]),
            )
        );
        rotor.set_lane(2, Rot3::from_raw_elements(-20, -21, -22, -23));
        assert_eq!(
            rotor,
            Rot3::from_raw_elements(
                i32x4::new([-1, -10, -20, 30]),
                i32x4::new([-2, -11, -21, 31]),
                i32x4::new([-3, -12, -22, 32]),
                i32x4::new([-4, -13, -23, 33]),
            )
        );
        rotor.set_lane(3, Rot3::from_raw_elements(-30, -31, -32, -33));
        assert_eq!(
            rotor,
            Rot3::from_raw_elements(
                i32x4::new([-1, -10, -20, -30]),
                i32x4::new([-2, -11, -21, -31]),
                i32x4::new([-3, -12, -22, -32]),
                i32x4::new([-4, -13, -23, -33]),
            )
        );
        assert_panic!(rotor.clone().set_lane(4, Rotor::ZERO));
    }

    #[test]
    fn test_simd_eq() {
        for ([a, b], mask) in random_iter::<([Rot2<i32x4>; 2], Vec2<i32x4>)>() {
            let b: Rot2<i32x4> = Rotor(mask.negative_mask().blend(a.0, b.0));

            assert_test_eq!(
                a.simd_eq(&b),
                i32x4::new(std::array::from_fn(
                    |lane| if a.lane(lane) == b.lane(lane) { !0 } else { 0 }
                ))
            );
        }
        for ([a, b], mask) in random_iter::<([Rot3<i32x4>; 2], Vec4<i32x4>)>() {
            let b: Rot3<i32x4> = Rotor(mask.negative_mask().blend(a.0, b.0));

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
        for ([a, b], mask) in random_iter::<([Rot2<i32x4>; 2], Vec2<i32x4>)>() {
            let b: Rot2<i32x4> = Rotor(mask.negative_mask().blend(a.0, b.0));

            assert_test_eq!(
                a.simd_ne(&b),
                i32x4::new(std::array::from_fn(
                    |lane| if a.lane(lane) != b.lane(lane) { !0 } else { 0 }
                ))
            );
        }
        for ([a, b], mask) in random_iter::<([Rot3<i32x4>; 2], Vec4<i32x4>)>() {
            let b: Rot3<i32x4> = Rotor(mask.negative_mask().blend(a.0, b.0));

            assert_test_eq!(
                a.simd_ne(&b),
                i32x4::new(std::array::from_fn(
                    |lane| if a.lane(lane) != b.lane(lane) { !0 } else { 0 }
                ))
            );
        }
    }
}
