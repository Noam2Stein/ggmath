use wide::CmpNe;

use crate::{Affine, Alignment, Length, Scalar, SupportedLength, Vector, utils::WideTy};

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
    ///     Affine2::from_column_array(&[1, 2, 3, 4, 5, 6]),
    ///     Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]),
    ///     Affine2::from_column_array(&[13, 14, 15, 16, 17, 18]),
    ///     Affine2::from_column_array(&[19, 20, 21, 22, 23, 24]),
    /// ];
    /// assert_eq!(
    ///     Affine2::<i32x4>::from_lanes(&lanes),
    ///     Affine2::from_column_array(&[
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
        Self::from_column_fn(|i| Vector::from_lane_fn(|lane| lanes[lane].column(i)))
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
    ///     Affine2::from_column_array(&[1, 2, 3, 4, 5, 6]),
    ///     Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]),
    ///     Affine2::from_column_array(&[13, 14, 15, 16, 17, 18]),
    ///     Affine2::from_column_array(&[19, 20, 21, 22, 23, 24]),
    /// ];
    /// assert_eq!(
    ///     Affine2::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Affine2::from_column_array(&[
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
    /// let affine = Affine2::from_column_array(&[
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
    ///         Affine2::from_column_array(&[1, 2, 3, 4, 5, 6]),
    ///         Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]),
    ///         Affine2::from_column_array(&[13, 14, 15, 16, 17, 18]),
    ///         Affine2::from_column_array(&[19, 20, 21, 22, 23, 24]),
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
    /// let affine = Affine2::from_column_array(&[
    ///     i32x4::new([1, 7, 13, 19]),
    ///     i32x4::new([2, 8, 14, 20]),
    ///     i32x4::new([3, 9, 15, 21]),
    ///     i32x4::new([4, 10, 16, 22]),
    ///     i32x4::new([5, 11, 17, 23]),
    ///     i32x4::new([6, 12, 18, 24]),
    /// ]);
    /// assert_eq!(affine.lane(1), Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Affine<N, T, A> {
        Affine::from_column_fn(|i| self.column(i).lane(lane))
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
    /// let mut affine = Affine2::from_column_array(&[
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
    ///     Affine2::from_column_array(&[
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
        for i in 0..N + 1 {
            self.column_mut(i).set_lane(lane, value.column(i));
        }
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        self.submatrix.simd_eq(&other.submatrix) & self.translation.simd_eq(other.translation)
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, other: &Self) -> Wide
    where
        Wide: CmpNe<Output = Wide>,
    {
        self.submatrix.simd_ne(&other.submatrix) | self.translation.simd_ne(other.translation)
    }
}

#[cfg(test)]
mod tests {
    use wide::{CmpEq, i32x4};

    use crate::{
        Affine, Affine2,
        utils::{assert_float_eq, assert_panic, for_parameters},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Affine2::<i32x4>::from_lanes(&[
                Affine2::from_column_array(&[1, 2, 3, 4, 5, 6]),
                Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]),
                Affine2::from_column_array(&[13, 14, 15, 16, 17, 18]),
                Affine2::from_column_array(&[19, 20, 21, 22, 23, 24]),
            ]),
            Affine2::from_column_array(&[
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
            Affine2::<i32x4>::from_lane_fn(|i| [
                Affine2::from_column_array(&[1, 2, 3, 4, 5, 6]),
                Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]),
                Affine2::from_column_array(&[13, 14, 15, 16, 17, 18]),
                Affine2::from_column_array(&[19, 20, 21, 22, 23, 24]),
            ][i]),
            Affine2::from_column_array(&[
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
            Affine2::from_column_array(&[
                i32x4::new([1, 7, 13, 19]),
                i32x4::new([2, 8, 14, 20]),
                i32x4::new([3, 9, 15, 21]),
                i32x4::new([4, 10, 16, 22]),
                i32x4::new([5, 11, 17, 23]),
                i32x4::new([6, 12, 18, 24]),
            ])
            .to_lanes(),
            [
                Affine2::from_column_array(&[1, 2, 3, 4, 5, 6]),
                Affine2::from_column_array(&[7, 8, 9, 10, 11, 12]),
                Affine2::from_column_array(&[13, 14, 15, 16, 17, 18]),
                Affine2::from_column_array(&[19, 20, 21, 22, 23, 24]),
            ]
        );
    }

    #[test]
    fn test_lane() {
        let affine = Affine2::from_column_array(&[
            i32x4::new([1, 7, 13, 19]),
            i32x4::new([2, 8, 14, 20]),
            i32x4::new([3, 9, 15, 21]),
            i32x4::new([4, 10, 16, 22]),
            i32x4::new([5, 11, 17, 23]),
            i32x4::new([6, 12, 18, 24]),
        ]);

        assert_eq!(
            affine.lane(0),
            Affine2::from_column_array(&[1, 2, 3, 4, 5, 6])
        );
        assert_eq!(
            affine.lane(1),
            Affine2::from_column_array(&[7, 8, 9, 10, 11, 12])
        );
        assert_eq!(
            affine.lane(2),
            Affine2::from_column_array(&[13, 14, 15, 16, 17, 18])
        );
        assert_eq!(
            affine.lane(3),
            Affine2::from_column_array(&[19, 20, 21, 22, 23, 24])
        );
        assert_panic!(affine.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut affine = Affine2::from_column_array(&[
            i32x4::new([1, 7, 13, 19]),
            i32x4::new([2, 8, 14, 20]),
            i32x4::new([3, 9, 15, 21]),
            i32x4::new([4, 10, 16, 22]),
            i32x4::new([5, 11, 17, 23]),
            i32x4::new([6, 12, 18, 24]),
        ]);

        affine.set_lane(0, Affine2::from_column_array(&[-1, -2, -3, -4, -5, -6]));
        assert_eq!(
            affine,
            Affine2::from_column_array(&[
                i32x4::new([-1, 7, 13, 19]),
                i32x4::new([-2, 8, 14, 20]),
                i32x4::new([-3, 9, 15, 21]),
                i32x4::new([-4, 10, 16, 22]),
                i32x4::new([-5, 11, 17, 23]),
                i32x4::new([-6, 12, 18, 24]),
            ])
        );
        affine.set_lane(1, Affine2::from_column_array(&[-7, -8, -9, -10, -11, -12]));
        assert_eq!(
            affine,
            Affine2::from_column_array(&[
                i32x4::new([-1, -7, 13, 19]),
                i32x4::new([-2, -8, 14, 20]),
                i32x4::new([-3, -9, 15, 21]),
                i32x4::new([-4, -10, 16, 22]),
                i32x4::new([-5, -11, 17, 23]),
                i32x4::new([-6, -12, 18, 24]),
            ])
        );
        affine.set_lane(
            2,
            Affine2::from_column_array(&[-13, -14, -15, -16, -17, -18]),
        );
        assert_eq!(
            affine,
            Affine2::from_column_array(&[
                i32x4::new([-1, -7, -13, 19]),
                i32x4::new([-2, -8, -14, 20]),
                i32x4::new([-3, -9, -15, 21]),
                i32x4::new([-4, -10, -16, 22]),
                i32x4::new([-5, -11, -17, 23]),
                i32x4::new([-6, -12, -18, 24]),
            ])
        );
        affine.set_lane(
            3,
            Affine2::from_column_array(&[-19, -20, -21, -22, -23, -24]),
        );
        assert_eq!(
            affine,
            Affine2::from_column_array(&[
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
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, x, z]).simd_eq(
                    &Affine::<2, Wide, A>::from_column_array(&[z, y, z, w, x, z])
                ),
                x.simd_eq(z) & y.simd_eq(y) & z.simd_eq(z) & w.simd_eq(w) & x.simd_eq(x)
            );
            assert_float_eq!(
                Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, x, z]).simd_eq(
                    &Affine::<2, Wide, A>::from_column_array(&[z, w, x, y, z, x])
                ),
                x.simd_eq(z) & y.simd_eq(w)
            );

            assert_float_eq!(
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, x, y, w, x, y, z, x, y, z])
                    .simd_eq(&Affine::<3, Wide, A>::from_column_array(&[
                        x, y, w, x, y, w, x, y, z, x, y, z
                    ])),
                x.simd_eq(x) & y.simd_eq(y) & z.simd_eq(w) & w.simd_eq(w)
            );
            assert_float_eq!(
                Affine::<3, Wide, A>::from_column_array(&[x, y, z, z, w, y, x, y, z, z, x, y])
                    .simd_eq(&Affine::<3, Wide, A>::from_column_array(&[
                        z, w, y, x, y, z, z, w, y, y, y, w
                    ])),
                x.simd_eq(z) & y.simd_eq(w) & z.simd_eq(y)
            );

            assert_float_eq!(
                Affine::<4, Wide, A>::from_column_array(&[
                    x, y, z, w, x, y, z, w, z, y, x, w, x, y, z, y, x, y, z, w
                ])
                .simd_eq(&Affine::<4, Wide, A>::from_column_array(&[
                    w, y, z, w, x, y, z, w, z, y, x, w, x, y, z, y, x, y, z, w
                ])),
                x.simd_eq(w) & y.simd_eq(y) & z.simd_eq(z) & w.simd_eq(w)
            );
            assert_float_eq!(
                Affine::<4, Wide, A>::from_column_array(&[
                    x, y, z, w, z, w, y, x, x, y, z, w, z, w, y, x, z, y, x, w
                ])
                .simd_eq(&Affine::<4, Wide, A>::from_column_array(&[
                    z, w, y, x, x, y, z, w, z, w, y, x, x, y, z, w, x, y, z, x
                ])),
                x.simd_eq(z) & y.simd_eq(w) & z.simd_eq(y)
            );
        });
    }

    #[test]
    fn test_simd_ne() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let affine = Affine::<2, Wide, A>::from_column_array(&[x, y, z, w, x, z]);
            for other in [
                Affine::<2, Wide, A>::from_column_array(&[z, y, z, w, x, z]),
                Affine::<2, Wide, A>::from_column_array(&[z, w, x, y, z, x]),
            ] {
                assert_float_eq!(affine.simd_ne(&other), !affine.simd_eq(&other));
            }

            for (affine, other) in [
                (
                    Affine::<3, Wide, A>::from_column_array(&[x, y, z, x, y, w, x, y, z, x, y, z]),
                    Affine::<3, Wide, A>::from_column_array(&[x, y, w, x, y, w, x, y, z, x, y, z]),
                ),
                (
                    Affine::<3, Wide, A>::from_column_array(&[x, y, z, z, w, y, x, y, z, z, x, y]),
                    Affine::<3, Wide, A>::from_column_array(&[z, w, y, x, y, z, z, w, y, y, y, w]),
                ),
            ] {
                assert_float_eq!(affine.simd_ne(&other), !affine.simd_eq(&other));
            }

            for (affine, other) in [
                (
                    Affine::<4, Wide, A>::from_column_array(&[
                        x, y, z, w, x, y, z, w, z, y, x, w, x, y, z, y, x, y, z, w,
                    ]),
                    Affine::<4, Wide, A>::from_column_array(&[
                        w, y, z, w, x, y, z, w, z, y, x, w, x, y, z, y, x, y, z, w,
                    ]),
                ),
                (
                    Affine::<4, Wide, A>::from_column_array(&[
                        x, y, z, w, z, w, y, x, x, y, z, w, z, w, y, x, z, y, x, w,
                    ]),
                    Affine::<4, Wide, A>::from_column_array(&[
                        z, w, y, x, x, y, z, w, z, w, y, x, x, y, z, w, x, y, z, x,
                    ]),
                ),
            ] {
                assert_float_eq!(affine.simd_ne(&other), !affine.simd_eq(&other));
            }
        });
    }
}
