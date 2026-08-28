use wide::Select;

use crate::{
    Affine, Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    utils::{WideTy, specialize},
};

/// Functionality for [SoA] (Structure of Arrays) affine transforms.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Affine<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) affine transform from an array of
    /// lanes or scalar affine transforms.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Affine2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Affine2::from_row_array(&[1, 2, 3, 4, 5, 6]),
    ///     Affine2::from_row_array(&[7, 8, 9, 10, 11, 12]),
    ///     Affine2::from_row_array(&[13, 14, 15, 16, 17, 18]),
    ///     Affine2::from_row_array(&[19, 20, 21, 22, 23, 24]),
    /// ];
    /// assert_eq!(
    ///     Affine2::<i32x4>::from_lanes(&lanes),
    ///     Affine2::from_row_array(&[
    ///         i32x4::new([1, 7, 13, 19]),
    ///         i32x4::new([2, 8, 14, 20]),
    ///         i32x4::new([3, 9, 15, 21]),
    ///         i32x4::new([4, 10, 16, 22]),
    ///         i32x4::new([5, 11, 17, 23]),
    ///         i32x4::new([6, 12, 18, 24]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Affine<N, T, A>; LANES]) -> Self {
        specialize!(Affine::<N, Wide, A>::from_lanes_backend(lanes))
    }

    /// Creates an SoA (Structure of Arrays) affine transform by calling
    /// function `f` for each lane index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Affine2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Affine2::from_row_array(&[1, 2, 3, 4, 5, 6]),
    ///     Affine2::from_row_array(&[7, 8, 9, 10, 11, 12]),
    ///     Affine2::from_row_array(&[13, 14, 15, 16, 17, 18]),
    ///     Affine2::from_row_array(&[19, 20, 21, 22, 23, 24]),
    /// ];
    /// assert_eq!(
    ///     Affine2::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Affine2::from_row_array(&[
    ///         i32x4::new([1, 7, 13, 19]),
    ///         i32x4::new([2, 8, 14, 20]),
    ///         i32x4::new([3, 9, 15, 21]),
    ///         i32x4::new([4, 10, 16, 22]),
    ///         i32x4::new([5, 11, 17, 23]),
    ///         i32x4::new([6, 12, 18, 24]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Affine<N, T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) affine transform to an array of
    /// lanes or scalar affine transforms.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Affine2;
    /// # use wide::i32x4;
    /// #
    /// let affine = Affine2::from_row_array(&[
    ///     i32x4::new([1, 7, 13, 19]),
    ///     i32x4::new([2, 8, 14, 20]),
    ///     i32x4::new([3, 9, 15, 21]),
    ///     i32x4::new([4, 10, 16, 22]),
    ///     i32x4::new([5, 11, 17, 23]),
    ///     i32x4::new([6, 12, 18, 24]),
    /// ]);
    /// assert_eq!(
    ///     affine.to_lanes(),
    ///     [
    ///         Affine2::from_row_array(&[1, 2, 3, 4, 5, 6]),
    ///         Affine2::from_row_array(&[7, 8, 9, 10, 11, 12]),
    ///         Affine2::from_row_array(&[13, 14, 15, 16, 17, 18]),
    ///         Affine2::from_row_array(&[19, 20, 21, 22, 23, 24]),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Affine<N, T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) affine transform and returns the lane
    /// at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Affine2;
    /// # use wide::i32x4;
    /// #
    /// let affine = Affine2::from_row_array(&[
    ///     i32x4::new([1, 7, 13, 19]),
    ///     i32x4::new([2, 8, 14, 20]),
    ///     i32x4::new([3, 9, 15, 21]),
    ///     i32x4::new([4, 10, 16, 22]),
    ///     i32x4::new([5, 11, 17, 23]),
    ///     i32x4::new([6, 12, 18, 24]),
    /// ]);
    /// assert_eq!(affine.lane(1), Affine2::from_row_array(&[7, 8, 9, 10, 11, 12]));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Affine<N, T, A> {
        Affine::from_matrix_translation(&self.matrix.lane(lane), self.translation.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) affine transform and sets the lane at
    /// the given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Affine2;
    /// # use wide::i32x4;
    /// #
    /// let mut affine = Affine2::from_row_array(&[
    ///     i32x4::new([1, 7, 13, 19]),
    ///     i32x4::new([2, 8, 14, 20]),
    ///     i32x4::new([3, 9, 15, 21]),
    ///     i32x4::new([4, 10, 16, 22]),
    ///     i32x4::new([5, 11, 17, 23]),
    ///     i32x4::new([6, 12, 18, 24]),
    /// ]);
    /// affine.set_lane(1, Affine2::ZERO);
    /// assert_eq!(
    ///     affine,
    ///     Affine2::from_row_array(&[
    ///         i32x4::new([1, 0, 13, 19]),
    ///         i32x4::new([2, 0, 14, 20]),
    ///         i32x4::new([3, 0, 15, 21]),
    ///         i32x4::new([4, 0, 16, 22]),
    ///         i32x4::new([5, 0, 17, 23]),
    ///         i32x4::new([6, 0, 18, 24]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Affine<N, T, A>) {
        self.matrix.set_lane(lane, value.matrix);
        self.translation.set_lane(lane, value.translation);
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        self.matrix.simd_eq(&other.matrix) & self.translation.simd_eq(other.translation)
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, other: &Self) -> Wide {
        self.matrix.simd_ne(&other.matrix) | self.translation.simd_ne(other.translation)
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Affine<2, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Affine<2, T, A>; LANES]) -> Self {
        Self::from_row_array(&[
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.y)),
        ])
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Affine<3, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Affine<3, T, A>; LANES]) -> Self {
        Self::from_row_array(&[
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.z)),
        ])
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Affine<4, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Affine<4, T, A>; LANES]) -> Self {
        Self::from_row_array(&[
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.x_axis.w)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.y_axis.w)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.z_axis.w)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.w_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.w_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.w_axis.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].matrix.w_axis.w)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.z)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].translation.w)),
        ])
    }
}

/// Unfortunately this cannot be done with a generic `Mask` type due to orphan
/// rules.
macro_rules! impl_select {
    ($Mask:ident) => {
        impl<const N: usize, Wide, A: Alignment> Select<Affine<N, Wide, A>> for wide::$Mask
        where
            Length<N>: SupportedLength,
            wide::$Mask: Select<Wide>,
            Wide: WideTy,
        {
            #[inline]
            fn select(
                self,
                if_true: Affine<N, Wide, A>,
                if_false: Affine<N, Wide, A>,
            ) -> Affine<N, Wide, A> {
                Affine::from_matrix_translation(
                    &self.select::<Matrix<N, Wide, A>>(if_true.matrix, if_false.matrix),
                    self.select::<Vector<N, Wide, A>>(if_true.translation, if_false.translation),
                )
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
        Affine, Affine2A, Unaligned,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Affine2A::<i32x4>::from_lanes(&[
                Affine2A::from_row_array(&[1, 2, 3, 4, 5, 6]),
                Affine2A::from_row_array(&[7, 8, 9, 10, 11, 12]),
                Affine2A::from_row_array(&[13, 14, 15, 16, 17, 18]),
                Affine2A::from_row_array(&[19, 20, 21, 22, 23, 24]),
            ]),
            Affine2A::from_row_array(&[
                i32x4::new([1, 7, 13, 19]),
                i32x4::new([2, 8, 14, 20]),
                i32x4::new([3, 9, 15, 21]),
                i32x4::new([4, 10, 16, 22]),
                i32x4::new([5, 11, 17, 23]),
                i32x4::new([6, 12, 18, 24]),
            ]),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Affine2A::<i32x4>::from_lane_fn(|i| [
                Affine2A::from_row_array(&[1, 2, 3, 4, 5, 6]),
                Affine2A::from_row_array(&[7, 8, 9, 10, 11, 12]),
                Affine2A::from_row_array(&[13, 14, 15, 16, 17, 18]),
                Affine2A::from_row_array(&[19, 20, 21, 22, 23, 24]),
            ][i]),
            Affine2A::from_row_array(&[
                i32x4::new([1, 7, 13, 19]),
                i32x4::new([2, 8, 14, 20]),
                i32x4::new([3, 9, 15, 21]),
                i32x4::new([4, 10, 16, 22]),
                i32x4::new([5, 11, 17, 23]),
                i32x4::new([6, 12, 18, 24]),
            ]),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Affine2A::from_row_array(&[
                i32x4::new([1, 7, 13, 19]),
                i32x4::new([2, 8, 14, 20]),
                i32x4::new([3, 9, 15, 21]),
                i32x4::new([4, 10, 16, 22]),
                i32x4::new([5, 11, 17, 23]),
                i32x4::new([6, 12, 18, 24]),
            ])
            .to_lanes(),
            [
                Affine2A::from_row_array(&[1, 2, 3, 4, 5, 6]),
                Affine2A::from_row_array(&[7, 8, 9, 10, 11, 12]),
                Affine2A::from_row_array(&[13, 14, 15, 16, 17, 18]),
                Affine2A::from_row_array(&[19, 20, 21, 22, 23, 24]),
            ]
        );
    }

    #[test]
    fn test_lane() {
        let affine = Affine2A::from_row_array(&[
            i32x4::new([1, 7, 13, 19]),
            i32x4::new([2, 8, 14, 20]),
            i32x4::new([3, 9, 15, 21]),
            i32x4::new([4, 10, 16, 22]),
            i32x4::new([5, 11, 17, 23]),
            i32x4::new([6, 12, 18, 24]),
        ]);

        assert_eq!(
            affine.lane(0),
            Affine2A::from_row_array(&[1, 2, 3, 4, 5, 6])
        );
        assert_eq!(
            affine.lane(1),
            Affine2A::from_row_array(&[7, 8, 9, 10, 11, 12])
        );
        assert_eq!(
            affine.lane(2),
            Affine2A::from_row_array(&[13, 14, 15, 16, 17, 18])
        );
        assert_eq!(
            affine.lane(3),
            Affine2A::from_row_array(&[19, 20, 21, 22, 23, 24])
        );
        assert_panic!(affine.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut affine = Affine2A::from_row_array(&[
            i32x4::new([1, 7, 13, 19]),
            i32x4::new([2, 8, 14, 20]),
            i32x4::new([3, 9, 15, 21]),
            i32x4::new([4, 10, 16, 22]),
            i32x4::new([5, 11, 17, 23]),
            i32x4::new([6, 12, 18, 24]),
        ]);

        affine.set_lane(0, Affine2A::from_row_array(&[-1, -2, -3, -4, -5, -6]));
        assert_eq!(
            affine,
            Affine2A::from_row_array(&[
                i32x4::new([-1, 7, 13, 19]),
                i32x4::new([-2, 8, 14, 20]),
                i32x4::new([-3, 9, 15, 21]),
                i32x4::new([-4, 10, 16, 22]),
                i32x4::new([-5, 11, 17, 23]),
                i32x4::new([-6, 12, 18, 24]),
            ])
        );
        affine.set_lane(1, Affine2A::from_row_array(&[-7, -8, -9, -10, -11, -12]));
        assert_eq!(
            affine,
            Affine2A::from_row_array(&[
                i32x4::new([-1, -7, 13, 19]),
                i32x4::new([-2, -8, 14, 20]),
                i32x4::new([-3, -9, 15, 21]),
                i32x4::new([-4, -10, 16, 22]),
                i32x4::new([-5, -11, 17, 23]),
                i32x4::new([-6, -12, 18, 24]),
            ])
        );
        affine.set_lane(2, Affine2A::from_row_array(&[-13, -14, -15, -16, -17, -18]));
        assert_eq!(
            affine,
            Affine2A::from_row_array(&[
                i32x4::new([-1, -7, -13, 19]),
                i32x4::new([-2, -8, -14, 20]),
                i32x4::new([-3, -9, -15, 21]),
                i32x4::new([-4, -10, -16, 22]),
                i32x4::new([-5, -11, -17, 23]),
                i32x4::new([-6, -12, -18, 24]),
            ])
        );
        affine.set_lane(3, Affine2A::from_row_array(&[-19, -20, -21, -22, -23, -24]));
        assert_eq!(
            affine,
            Affine2A::from_row_array(&[
                i32x4::new([-1, -7, -13, -19]),
                i32x4::new([-2, -8, -14, -20]),
                i32x4::new([-3, -9, -15, -21]),
                i32x4::new([-4, -10, -16, -22]),
                i32x4::new([-5, -11, -17, -23]),
                i32x4::new([-6, -12, -18, -24]),
            ])
        );
        assert_panic!(affine.clone().set_lane(4, Affine::ZERO));
    }

    #[test]
    fn test_simd_eq() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b, mask] in random_iter::<[Affine<N, Wide, Unaligned>; 3]>() {
                let mask = Affine::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Affine::from_row_fn(|r| mask[r].select(a[r], b[r]));

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
        for_types!(|N, Wide: WideFloat| {
            for [a, b, mask] in random_iter::<[Affine<N, Wide, Unaligned>; 3]>() {
                let mask = Affine::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Affine::from_row_fn(|r| mask[r].select(a[r], b[r]));

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

    #[test]
    fn test_scalar_select() {
        for_types!(|N| {
            for (mask, [if_true, if_false]) in
                random_iter::<(i32x4, [Affine<N, f32x4, Unaligned>; 2])>()
            {
                let mask = mask.is_negative();

                assert_test_eq!(
                    mask.select(if_true, if_false),
                    Affine::from_lane_fn(|lane| if mask.as_array()[lane].is_negative() {
                        if_true.lane(lane)
                    } else {
                        if_false.lane(lane)
                    })
                );
            }
        });
    }
}
