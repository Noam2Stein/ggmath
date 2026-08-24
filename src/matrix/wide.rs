use crate::{
    Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    utils::{WideTy, specialize},
};

/// Functionality for [SoA] (Structure of Arrays) matrices.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Matrix<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) matrix from an array of lanes or
    /// scalar matrices.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mat2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Mat2::from_row_array(&[1, 2, 3, 4]),
    ///     Mat2::from_row_array(&[5, 6, 7, 8]),
    ///     Mat2::from_row_array(&[9, 10, 11, 12]),
    ///     Mat2::from_row_array(&[13, 14, 15, 16]),
    /// ];
    /// assert_eq!(
    ///     Mat2::<i32x4>::from_lanes(&lanes),
    ///     Mat2::from_row_array(&[
    ///         i32x4::new([1, 5, 9, 13]),
    ///         i32x4::new([2, 6, 10, 14]),
    ///         i32x4::new([3, 7, 11, 15]),
    ///         i32x4::new([4, 8, 12, 16]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Matrix<N, T, A>; LANES]) -> Self {
        specialize!(Matrix::<N, Wide, A>::from_lanes_backend(lanes))
    }

    /// Creates an SoA (Structure of Arrays) matrix by calling function `f` for
    /// each lane index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mat2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Mat2::from_row_array(&[1, 2, 3, 4]),
    ///     Mat2::from_row_array(&[5, 6, 7, 8]),
    ///     Mat2::from_row_array(&[9, 10, 11, 12]),
    ///     Mat2::from_row_array(&[13, 14, 15, 16]),
    /// ];
    /// assert_eq!(
    ///     Mat2::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Mat2::from_row_array(&[
    ///         i32x4::new([1, 5, 9, 13]),
    ///         i32x4::new([2, 6, 10, 14]),
    ///         i32x4::new([3, 7, 11, 15]),
    ///         i32x4::new([4, 8, 12, 16]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Matrix<N, T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) matrix to an array of lanes or
    /// scalar matrices.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mat2;
    /// # use wide::i32x4;
    /// #
    /// let matrix = Mat2::from_row_array(&[
    ///     i32x4::new([1, 5, 9, 13]),
    ///     i32x4::new([2, 6, 10, 14]),
    ///     i32x4::new([3, 7, 11, 15]),
    ///     i32x4::new([4, 8, 12, 16]),
    /// ]);
    /// assert_eq!(
    ///     matrix.to_lanes(),
    ///     [
    ///         Mat2::from_row_array(&[1, 2, 3, 4]),
    ///         Mat2::from_row_array(&[5, 6, 7, 8]),
    ///         Mat2::from_row_array(&[9, 10, 11, 12]),
    ///         Mat2::from_row_array(&[13, 14, 15, 16]),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Matrix<N, T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) matrix and returns the lane at the
    /// given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Mat2;
    /// # use wide::i32x4;
    /// #
    /// let matrix = Mat2::from_row_array(&[
    ///     i32x4::new([1, 5, 9, 13]),
    ///     i32x4::new([2, 6, 10, 14]),
    ///     i32x4::new([3, 7, 11, 15]),
    ///     i32x4::new([4, 8, 12, 16]),
    /// ]);
    /// assert_eq!(matrix.lane(1), Mat2::from_row_array(&[5, 6, 7, 8]));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Matrix<N, T, A> {
        specialize!(Matrix::<N, Wide, A>::lane_backend(self, lane))
    }

    /// Takes an SoA (Structure of Arrays) matrix and sets the lane at the given
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
    /// # use ggmath::Mat2;
    /// # use wide::i32x4;
    /// #
    /// let mut matrix = Mat2::from_row_array(&[
    ///     i32x4::new([1, 5, 9, 13]),
    ///     i32x4::new([2, 6, 10, 14]),
    ///     i32x4::new([3, 7, 11, 15]),
    ///     i32x4::new([4, 8, 12, 16]),
    /// ]);
    /// matrix.set_lane(1, Mat2::ZERO);
    /// assert_eq!(
    ///     matrix,
    ///     Mat2::from_row_array(&[
    ///         i32x4::new([1, 0, 9, 13]),
    ///         i32x4::new([2, 0, 10, 14]),
    ///         i32x4::new([3, 0, 11, 15]),
    ///         i32x4::new([4, 0, 12, 16]),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Matrix<N, T, A>) {
        specialize!(Matrix::<N, Wide, A>::set_lane_backend(self, lane, value))
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        specialize!(Matrix::<N, Wide, A>::simd_eq_backend(self, other))
    }

    /// For each lane, returns `true` if `self` is not equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) != other.lane(0), self.lane(1) != other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, other: &Self) -> Wide {
        specialize!(Matrix::<N, Wide, A>::simd_ne_backend(self, other))
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Matrix<2, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Matrix<2, T, A>; LANES]) -> Self {
        Self::from_row_array(&[
            Wide::new(core::array::from_fn(|lane| lanes[lane].x_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].x_axis.y)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].y_axis.x)),
            Wide::new(core::array::from_fn(|lane| lanes[lane].y_axis.y)),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Matrix<2, T, A> {
        Matrix::from_rows(&[self.x_axis.lane(lane), self.y_axis.lane(lane)])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Matrix<2, T, A>) {
        self.x_axis.set_lane(lane, value.x_axis);
        self.y_axis.set_lane(lane, value.y_axis);
    }

    #[track_caller]
    #[inline(always)]
    fn simd_eq_backend(&self, other: &Self) -> Wide {
        self.x_axis.simd_eq(other.x_axis) & self.y_axis.simd_eq(other.y_axis)
    }

    #[track_caller]
    #[inline(always)]
    fn simd_ne_backend(&self, other: &Self) -> Wide {
        self.x_axis.simd_ne(other.x_axis) | self.y_axis.simd_ne(other.y_axis)
    }
}

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Matrix<3, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Matrix<3, T, A>; LANES]) -> Self {
        Self::from_rows(&[
            Vector::from_lane_fn(|lane| lanes[lane].x_axis),
            Vector::from_lane_fn(|lane| lanes[lane].y_axis),
            Vector::from_lane_fn(|lane| lanes[lane].z_axis),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Matrix<3, T, A> {
        Matrix::from_rows(&[
            self.x_axis.lane(lane),
            self.y_axis.lane(lane),
            self.z_axis.lane(lane),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Matrix<3, T, A>) {
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
impl<Wide, T, const LANES: usize, A: Alignment> Matrix<4, Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    #[inline(always)]
    fn from_lanes_backend(lanes: &[Matrix<4, T, A>; LANES]) -> Self {
        Self::from_rows(&[
            Vector::from_lane_fn(|lane| lanes[lane].x_axis),
            Vector::from_lane_fn(|lane| lanes[lane].y_axis),
            Vector::from_lane_fn(|lane| lanes[lane].z_axis),
            Vector::from_lane_fn(|lane| lanes[lane].w_axis),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn lane_backend(&self, lane: usize) -> Matrix<4, T, A> {
        Matrix::from_rows(&[
            self.x_axis.lane(lane),
            self.y_axis.lane(lane),
            self.z_axis.lane(lane),
            self.w_axis.lane(lane),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn set_lane_backend(&mut self, lane: usize, value: Matrix<4, T, A>) {
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
        Mat2A, Matrix, Unaligned,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Mat2A::<i32x4>::from_lanes(&[
                Mat2A::from_row_array(&[1, 2, 3, 4]),
                Mat2A::from_row_array(&[5, 6, 7, 8]),
                Mat2A::from_row_array(&[9, 10, 11, 12]),
                Mat2A::from_row_array(&[13, 14, 15, 16]),
            ]),
            Mat2A::from_row_array(&[
                i32x4::new([1, 5, 9, 13]),
                i32x4::new([2, 6, 10, 14]),
                i32x4::new([3, 7, 11, 15]),
                i32x4::new([4, 8, 12, 16]),
            ]),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Mat2A::<i32x4>::from_lane_fn(|i| [
                Mat2A::from_row_array(&[1, 2, 3, 4]),
                Mat2A::from_row_array(&[5, 6, 7, 8]),
                Mat2A::from_row_array(&[9, 10, 11, 12]),
                Mat2A::from_row_array(&[13, 14, 15, 16]),
            ][i]),
            Mat2A::from_row_array(&[
                i32x4::new([1, 5, 9, 13]),
                i32x4::new([2, 6, 10, 14]),
                i32x4::new([3, 7, 11, 15]),
                i32x4::new([4, 8, 12, 16]),
            ]),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Mat2A::from_row_array(&[
                i32x4::new([1, 5, 9, 13]),
                i32x4::new([2, 6, 10, 14]),
                i32x4::new([3, 7, 11, 15]),
                i32x4::new([4, 8, 12, 16]),
            ])
            .to_lanes(),
            [
                Mat2A::from_row_array(&[1, 2, 3, 4]),
                Mat2A::from_row_array(&[5, 6, 7, 8]),
                Mat2A::from_row_array(&[9, 10, 11, 12]),
                Mat2A::from_row_array(&[13, 14, 15, 16]),
            ],
        );
    }

    #[test]
    fn test_lane() {
        let matrix = Mat2A::from_row_array(&[
            i32x4::new([1, 5, 9, 13]),
            i32x4::new([2, 6, 10, 14]),
            i32x4::new([3, 7, 11, 15]),
            i32x4::new([4, 8, 12, 16]),
        ]);

        assert_eq!(matrix.lane(0), Mat2A::from_row_array(&[1, 2, 3, 4]));
        assert_eq!(matrix.lane(1), Mat2A::from_row_array(&[5, 6, 7, 8]));
        assert_eq!(matrix.lane(2), Mat2A::from_row_array(&[9, 10, 11, 12]));
        assert_eq!(matrix.lane(3), Mat2A::from_row_array(&[13, 14, 15, 16]));
        assert_panic!(matrix.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut matrix = Mat2A::from_row_array(&[
            i32x4::new([1, 5, 9, 13]),
            i32x4::new([2, 6, 10, 14]),
            i32x4::new([3, 7, 11, 15]),
            i32x4::new([4, 8, 12, 16]),
        ]);

        matrix.set_lane(0, Mat2A::from_row_array(&[-1, -2, -3, -4]));
        assert_eq!(
            matrix,
            Mat2A::from_row_array(&[
                i32x4::new([-1, 5, 9, 13]),
                i32x4::new([-2, 6, 10, 14]),
                i32x4::new([-3, 7, 11, 15]),
                i32x4::new([-4, 8, 12, 16]),
            ])
        );
        matrix.set_lane(1, Mat2A::from_row_array(&[-5, -6, -7, -8]));
        assert_eq!(
            matrix,
            Mat2A::from_row_array(&[
                i32x4::new([-1, -5, 9, 13]),
                i32x4::new([-2, -6, 10, 14]),
                i32x4::new([-3, -7, 11, 15]),
                i32x4::new([-4, -8, 12, 16]),
            ])
        );
        matrix.set_lane(2, Mat2A::from_row_array(&[-9, -10, -11, -12]));
        assert_eq!(
            matrix,
            Mat2A::from_row_array(&[
                i32x4::new([-1, -5, -9, 13]),
                i32x4::new([-2, -6, -10, 14]),
                i32x4::new([-3, -7, -11, 15]),
                i32x4::new([-4, -8, -12, 16]),
            ])
        );
        matrix.set_lane(3, Mat2A::from_row_array(&[-13, -14, -15, -16]));
        assert_eq!(
            matrix,
            Mat2A::from_row_array(&[
                i32x4::new([-1, -5, -9, -13]),
                i32x4::new([-2, -6, -10, -14]),
                i32x4::new([-3, -7, -11, -15]),
                i32x4::new([-4, -8, -12, -16]),
            ])
        );
        assert_panic!(matrix.clone().set_lane(4, Matrix::ZERO));
    }

    #[test]
    fn test_simd_eq() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b, mask] in random_iter::<[Matrix<N, Wide, Unaligned>; 3]>() {
                let mask = Matrix::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Matrix::from_row_fn(|r| mask[r].blend(a[r], b[r]));

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
            for [a, b, mask] in random_iter::<[Matrix<N, Wide, Unaligned>; 3]>() {
                let mask = Matrix::from_row_fn(|r| mask[r].sign_negative_mask());
                let b = Matrix::from_row_fn(|r| mask[r].blend(a[r], b[r]));

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
