use crate::{
    Alignment, Length, Projective, Scalar, Vector,
    length::TwoOrThree,
    utils::{WideTy, specialize_23},
};

#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Projective<N, Wide, A>
where
    Length<N>: TwoOrThree,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) projective transform from an array
    /// of regular, non-SoA projective transforms corresponding to each output
    /// lane.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Proj2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Proj2::from_row_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ///     Proj2::from_row_array(&[11, 12, 13, 14, 15, 16, 17, 18, 19]),
    ///     Proj2::from_row_array(&[21, 22, 23, 24, 25, 26, 27, 28, 29]),
    ///     Proj2::from_row_array(&[31, 32, 33, 34, 35, 36, 37, 38, 39]),
    /// ];
    /// assert_eq!(
    ///     Proj2::<i32x4>::from_lanes(&lanes),
    ///     Proj2::from_row_array(&[
    ///         i32x4::new([1, 11, 21, 31]),
    ///         i32x4::new([2, 12, 22, 32]),
    ///         i32x4::new([3, 13, 23, 33]),
    ///         i32x4::new([4, 14, 24, 34]),
    ///         i32x4::new([5, 15, 25, 35]),
    ///         i32x4::new([6, 16, 26, 36]),
    ///         i32x4::new([7, 17, 27, 37]),
    ///         i32x4::new([8, 18, 28, 38]),
    ///         i32x4::new([9, 19, 29, 39]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Projective<N, T, A>; LANES]) -> Self {
        specialize_23!(Projective::<N, Wide, A>::from_lanes_backend(lanes))
    }

    /// Creates an SoA (Structure of Arrays) projective transform by calling
    /// function `f` for each output lane.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Proj2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Proj2::from_row_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ///     Proj2::from_row_array(&[11, 12, 13, 14, 15, 16, 17, 18, 19]),
    ///     Proj2::from_row_array(&[21, 22, 23, 24, 25, 26, 27, 28, 29]),
    ///     Proj2::from_row_array(&[31, 32, 33, 34, 35, 36, 37, 38, 39]),
    /// ];
    /// assert_eq!(
    ///     Proj2::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Proj2::from_row_array(&[
    ///         i32x4::new([1, 11, 21, 31]),
    ///         i32x4::new([2, 12, 22, 32]),
    ///         i32x4::new([3, 13, 23, 33]),
    ///         i32x4::new([4, 14, 24, 34]),
    ///         i32x4::new([5, 15, 25, 35]),
    ///         i32x4::new([6, 16, 26, 36]),
    ///         i32x4::new([7, 17, 27, 37]),
    ///         i32x4::new([8, 18, 28, 38]),
    ///         i32x4::new([9, 19, 29, 39]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Projective<N, T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) projective transform to an array
    /// of regular, non-SoA projective transforms corresponding to each input
    /// lane.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Proj2;
    /// # use wide::i32x4;
    /// #
    /// let soa = Proj2::from_row_array(&[
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    ///     i32x4::new([5, 15, 25, 35]),
    ///     i32x4::new([6, 16, 26, 36]),
    ///     i32x4::new([7, 17, 27, 37]),
    ///     i32x4::new([8, 18, 28, 38]),
    ///     i32x4::new([9, 19, 29, 39]),
    /// ]);
    /// assert_eq!(
    ///     soa.to_lanes(),
    ///     [
    ///         Proj2::from_row_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9]),
    ///         Proj2::from_row_array(&[11, 12, 13, 14, 15, 16, 17, 18, 19]),
    ///         Proj2::from_row_array(&[21, 22, 23, 24, 25, 26, 27, 28, 29]),
    ///         Proj2::from_row_array(&[31, 32, 33, 34, 35, 36, 37, 38, 39]),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Projective<N, T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) projective transform and returns the
    /// lane at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Proj2;
    /// # use wide::i32x4;
    /// #
    /// let soa = Proj2::from_row_array(&[
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    ///     i32x4::new([5, 15, 25, 35]),
    ///     i32x4::new([6, 16, 26, 36]),
    ///     i32x4::new([7, 17, 27, 37]),
    ///     i32x4::new([8, 18, 28, 38]),
    ///     i32x4::new([9, 19, 29, 39]),
    /// ]);
    /// assert_eq!(
    ///     soa.lane(1),
    ///     Proj2::from_row_array(&[11, 12, 13, 14, 15, 16, 17, 18, 19]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Projective<N, T, A> {
        specialize_23!(Projective::<N, Wide, A>::lane_backend(self, lane))
    }

    /// Takes an SoA (Structure of Arrays) projective transform and sets the
    /// lane at the given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of
    /// lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Proj2;
    /// # use wide::i32x4;
    /// #
    /// let mut soa = Proj2::from_row_array(&[
    ///     i32x4::new([1, 11, 21, 31]),
    ///     i32x4::new([2, 12, 22, 32]),
    ///     i32x4::new([3, 13, 23, 33]),
    ///     i32x4::new([4, 14, 24, 34]),
    ///     i32x4::new([5, 15, 25, 35]),
    ///     i32x4::new([6, 16, 26, 36]),
    ///     i32x4::new([7, 17, 27, 37]),
    ///     i32x4::new([8, 18, 28, 38]),
    ///     i32x4::new([9, 19, 29, 39]),
    /// ]);
    /// soa.set_lane(1, Proj2::ZERO);
    /// assert_eq!(
    ///     matrix,
    ///     Proj2::from_row_array(&[
    ///         i32x4::new([1, 0, 21, 31]),
    ///         i32x4::new([2, 0, 22, 32]),
    ///         i32x4::new([3, 0, 23, 33]),
    ///         i32x4::new([4, 0, 24, 34]),
    ///         i32x4::new([5, 0, 25, 35]),
    ///         i32x4::new([6, 0, 26, 36]),
    ///         i32x4::new([7, 0, 27, 37]),
    ///         i32x4::new([8, 0, 28, 38]),
    ///         i32x4::new([9, 0, 29, 39]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Projective<N, T, A>) {
        specialize_23!(Projective::<N, Wide, A>::set_lane_backend(
            self, lane, value
        ))
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        specialize_23!(Projective::<N, Wide, A>::simd_eq_backend(self, other))
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, other: &Self) -> Wide {
        specialize_23!(Projective::<N, Wide, A>::simd_ne_backend(self, other))
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Projective<2, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Projective<2, T, A>; LANES]) -> Self {
        Self::from_rows(&[
            Vector::from_lane_fn(|lane| lanes[lane].x_axis),
            Vector::from_lane_fn(|lane| lanes[lane].y_axis),
            Vector::from_lane_fn(|lane| lanes[lane].z_axis),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Projective<2, T, A> {
        Projective::<2, T, A>::from_rows(&[
            self.x_axis.lane(lane),
            self.y_axis.lane(lane),
            self.z_axis.lane(lane),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Projective<2, T, A>) {
        self.x_axis.set_lane(lane, value.x_axis);
        self.y_axis.set_lane(lane, value.y_axis);
        self.z_axis.set_lane(lane, value.z_axis);
    }

    #[track_caller]
    #[inline(always)]
    fn simd_eq_backend(&self, other: &Self) -> Wide {
        self.x_axis.simd_eq(other.x_axis)
            & self.y_axis.simd_eq(other.y_axis)
            & self.z_axis.simd_eq(other.z_axis)
    }

    #[track_caller]
    #[inline(always)]
    fn simd_ne_backend(&self, other: &Self) -> Wide {
        self.x_axis.simd_ne(other.x_axis)
            | self.y_axis.simd_ne(other.y_axis)
            | self.z_axis.simd_ne(other.z_axis)
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Projective<3, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Projective<3, T, A>; LANES]) -> Self {
        Self::from_rows(&[
            Vector::from_lane_fn(|lane| lanes[lane].x_axis),
            Vector::from_lane_fn(|lane| lanes[lane].y_axis),
            Vector::from_lane_fn(|lane| lanes[lane].z_axis),
            Vector::from_lane_fn(|lane| lanes[lane].w_axis),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Projective<3, T, A> {
        Projective::<3, T, A>::from_rows(&[
            self.x_axis.lane(lane),
            self.y_axis.lane(lane),
            self.z_axis.lane(lane),
            self.w_axis.lane(lane),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Projective<3, T, A>) {
        self.x_axis.set_lane(lane, value.x_axis);
        self.y_axis.set_lane(lane, value.y_axis);
        self.z_axis.set_lane(lane, value.z_axis);
        self.w_axis.set_lane(lane, value.w_axis);
    }

    #[track_caller]
    #[inline(always)]
    fn simd_eq_backend(&self, other: &Self) -> Wide {
        self.x_axis.simd_eq(other.x_axis)
            & self.y_axis.simd_eq(other.y_axis)
            & self.z_axis.simd_eq(other.z_axis)
            & self.w_axis.simd_eq(other.w_axis)
    }

    #[track_caller]
    #[inline(always)]
    fn simd_ne_backend(&self, other: &Self) -> Wide {
        self.x_axis.simd_ne(other.x_axis)
            | self.y_axis.simd_ne(other.y_axis)
            | self.z_axis.simd_ne(other.z_axis)
            | self.w_axis.simd_ne(other.w_axis)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::i32x4;

    use crate::{
        Proj2, Proj3, Projective,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Proj2::<i32x4>::from_lanes(&[
                Proj2::from_row_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8]),
                Proj2::from_row_array(&[10, 11, 12, 13, 14, 15, 16, 17, 18]),
                Proj2::from_row_array(&[20, 21, 2, 23, 24, 25, 26, 27, 28]),
                Proj2::from_row_array(&[30, 31, 32, 33, 34, 35, 36, 37, 38]),
            ]),
            Proj2::from_row_array(&[
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
                i32x4::new([4, 14, 24, 34]),
                i32x4::new([5, 15, 25, 35]),
                i32x4::new([6, 16, 26, 36]),
                i32x4::new([7, 17, 27, 37]),
                i32x4::new([8, 18, 28, 38]),
            ]),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Proj2::<i32x4>::from_lane_fn(|i| [
                Proj2::from_row_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8]),
                Proj2::from_row_array(&[10, 11, 12, 13, 14, 15, 16, 17, 18]),
                Proj2::from_row_array(&[20, 21, 2, 23, 24, 25, 26, 27, 28]),
                Proj2::from_row_array(&[30, 31, 32, 33, 34, 35, 36, 37, 38]),
            ][i]),
            Proj2::from_row_array(&[
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
                i32x4::new([4, 14, 24, 34]),
                i32x4::new([5, 15, 25, 35]),
                i32x4::new([6, 16, 26, 36]),
                i32x4::new([7, 17, 27, 37]),
                i32x4::new([8, 18, 28, 38]),
            ]),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Proj2::from_row_array(&[
                i32x4::new([0, 10, 20, 30]),
                i32x4::new([1, 11, 21, 31]),
                i32x4::new([2, 12, 22, 32]),
                i32x4::new([3, 13, 23, 33]),
                i32x4::new([4, 14, 24, 34]),
                i32x4::new([5, 15, 25, 35]),
                i32x4::new([6, 16, 26, 36]),
                i32x4::new([7, 17, 27, 37]),
                i32x4::new([8, 18, 28, 38]),
            ])
            .to_lanes(),
            [
                Proj2::from_row_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8]),
                Proj2::from_row_array(&[10, 11, 12, 13, 14, 15, 16, 17, 18]),
                Proj2::from_row_array(&[20, 21, 2, 23, 24, 25, 26, 27, 28]),
                Proj2::from_row_array(&[30, 31, 32, 33, 34, 35, 36, 37, 38]),
            ],
        );
    }

    #[test]
    fn test_lane() {
        let projective = Proj2::from_row_array(&[
            i32x4::new([0, 10, 20, 30]),
            i32x4::new([1, 11, 21, 31]),
            i32x4::new([2, 12, 22, 32]),
            i32x4::new([3, 13, 23, 33]),
            i32x4::new([4, 14, 24, 34]),
            i32x4::new([5, 15, 25, 35]),
            i32x4::new([6, 16, 26, 36]),
            i32x4::new([7, 17, 27, 37]),
            i32x4::new([8, 18, 28, 38]),
        ]);

        assert_eq!(
            projective.lane(0),
            Proj2::from_row_array(&[0, 1, 2, 3, 4, 5, 6, 7, 8])
        );
        assert_eq!(
            projective.lane(1),
            Proj2::from_row_array(&[10, 11, 12, 13, 14, 15, 16, 17, 18])
        );
        assert_eq!(
            projective.lane(2),
            Proj2::from_row_array(&[20, 21, 2, 23, 24, 25, 26, 27, 28])
        );
        assert_eq!(
            projective.lane(3),
            Proj2::from_row_array(&[30, 31, 32, 33, 34, 35, 36, 37, 38])
        );
        assert_panic!(projective.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut projective = Proj2::from_row_array(&[
            i32x4::new([0, 10, 20, 30]),
            i32x4::new([1, 11, 21, 31]),
            i32x4::new([2, 12, 22, 32]),
            i32x4::new([3, 13, 23, 33]),
            i32x4::new([4, 14, 24, 34]),
            i32x4::new([5, 15, 25, 35]),
            i32x4::new([6, 16, 26, 36]),
            i32x4::new([7, 17, 27, 37]),
            i32x4::new([8, 18, 28, 38]),
        ]);

        projective.set_lane(
            0,
            Proj2::from_row_array(&[-1, -2, -3, -4, -5, -6, -7, -8, -9]),
        );
        assert_eq!(
            projective,
            Proj2::from_row_array(&[
                i32x4::new([-1, 10, 20, 30]),
                i32x4::new([-2, 11, 21, 31]),
                i32x4::new([-3, 12, 22, 32]),
                i32x4::new([-4, 13, 23, 33]),
                i32x4::new([-5, 14, 24, 34]),
                i32x4::new([-6, 15, 25, 35]),
                i32x4::new([-7, 16, 26, 36]),
                i32x4::new([-8, 17, 27, 37]),
                i32x4::new([-9, 18, 28, 38]),
            ])
        );
        projective.set_lane(
            1,
            Proj2::from_row_array(&[-10, -11, -12, -13, -14, -15, -16, -17, -18]),
        );
        assert_eq!(
            projective,
            Proj2::from_row_array(&[
                i32x4::new([-1, -10, 20, 30]),
                i32x4::new([-2, -11, 21, 31]),
                i32x4::new([-3, -12, 22, 32]),
                i32x4::new([-4, -13, 23, 33]),
                i32x4::new([-5, -14, 24, 34]),
                i32x4::new([-6, -15, 25, 35]),
                i32x4::new([-7, -16, 26, 36]),
                i32x4::new([-8, -17, 27, 37]),
                i32x4::new([-9, -18, 28, 38]),
            ])
        );
        projective.set_lane(
            2,
            Proj2::from_row_array(&[-20, -21, -22, -23, -24, -25, -26, -27, -28]),
        );
        assert_eq!(
            projective,
            Proj2::from_row_array(&[
                i32x4::new([-1, -10, -20, 30]),
                i32x4::new([-2, -11, -21, 31]),
                i32x4::new([-3, -12, -22, 32]),
                i32x4::new([-4, -13, -23, 33]),
                i32x4::new([-5, -14, -24, 34]),
                i32x4::new([-6, -15, -25, 35]),
                i32x4::new([-7, -16, -26, 36]),
                i32x4::new([-8, -17, -27, 37]),
                i32x4::new([-9, -18, -28, 38]),
            ])
        );
        projective.set_lane(
            3,
            Proj2::from_row_array(&[-30, -31, -32, -33, -34, -35, -36, -37, -38]),
        );
        assert_eq!(
            projective,
            Proj2::from_row_array(&[
                i32x4::new([-1, -10, -20, -30]),
                i32x4::new([-2, -11, -21, -31]),
                i32x4::new([-3, -12, -22, -32]),
                i32x4::new([-4, -13, -23, -33]),
                i32x4::new([-5, -14, -24, -34]),
                i32x4::new([-6, -15, -25, -35]),
                i32x4::new([-7, -16, -26, -36]),
                i32x4::new([-8, -17, -27, -37]),
                i32x4::new([-9, -18, -28, -38]),
            ])
        );
        assert_panic!(projective.clone().set_lane(4, Projective::ZERO));
    }

    #[test]
    fn test_simd_eq() {
        for_types!(|Wide: WideFloat| {
            for [a, b, mask] in random_iter::<[Proj2<Wide>; 3]>() {
                let mask = Proj2::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Proj2::from_row_fn(|r| mask[r].blend(a[r], b[r]));

                assert_test_eq!(
                    a.simd_eq(&b),
                    Wide::new(std::array::from_fn(
                        |lane| if a.lane(lane) == b.lane(lane) {
                            T::from_bits(!0)
                        } else {
                            0.0
                        }
                    ))
                );
            }

            for [a, b, mask] in random_iter::<[Proj3<Wide>; 3]>() {
                let mask = Proj3::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Proj3::from_row_fn(|r| mask[r].blend(a[r], b[r]));

                assert_test_eq!(
                    a.simd_eq(&b),
                    Wide::new(std::array::from_fn(
                        |lane| if a.lane(lane) == b.lane(lane) {
                            T::from_bits(!0)
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
        for_types!(|Wide: WideFloat| {
            for [a, b, mask] in random_iter::<[Proj2<Wide>; 3]>() {
                let mask = Proj2::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Proj2::from_row_fn(|r| mask[r].blend(a[r], b[r]));

                assert_test_eq!(
                    a.simd_ne(&b),
                    Wide::new(std::array::from_fn(
                        |lane| if a.lane(lane) != b.lane(lane) {
                            T::from_bits(!0)
                        } else {
                            0.0
                        }
                    ))
                );
            }

            for [a, b, mask] in random_iter::<[Proj3<Wide>; 3]>() {
                let mask = Proj3::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Proj3::from_row_fn(|r| mask[r].blend(a[r], b[r]));

                assert_test_eq!(
                    a.simd_ne(&b),
                    Wide::new(std::array::from_fn(
                        |lane| if a.lane(lane) != b.lane(lane) {
                            T::from_bits(!0)
                        } else {
                            0.0
                        }
                    ))
                );
            }
        });
    }
}
