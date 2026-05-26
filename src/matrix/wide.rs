use wide::CmpNe;

use crate::{Alignment, Length, Matrix, Scalar, SupportedLength, Vector, utils::WideTy};

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
        Self::from_row_fn(|i| Vector::from_lane_fn(|lane| lanes[lane][i]))
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
        Matrix::from_row_fn(|i| self[i].lane(lane))
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
        for i in 0..N {
            self[i].set_lane(lane, value[i]);
        }
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, other: &Self) -> Wide {
        match N {
            2 => self[0].simd_eq(other[0]) & self[1].simd_eq(other[1]),
            3 => self[0].simd_eq(other[0]) & self[1].simd_eq(other[1]) & self[2].simd_eq(other[2]),
            4 => {
                self[0].simd_eq(other[0])
                    & self[1].simd_eq(other[1])
                    & self[2].simd_eq(other[2])
                    & self[3].simd_eq(other[3])
            }
            _ => unreachable!(),
        }
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
        match N {
            2 => self[0].simd_ne(other[0]) | self[1].simd_ne(other[1]),
            3 => self[0].simd_ne(other[0]) | self[1].simd_ne(other[1]) | self[2].simd_ne(other[2]),
            4 => {
                self[0].simd_ne(other[0])
                    | self[1].simd_ne(other[1])
                    | self[2].simd_ne(other[2])
                    | self[3].simd_ne(other[3])
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use wide::{CmpEq, i32x4};

    use crate::{
        Mat2A, Matrix,
        utils::{assert_float_eq, assert_panic, for_parameters},
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
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Matrix::<2, Wide, A>::from_row_array(&[x, y, z, w])
                    .simd_eq(&Matrix::<2, Wide, A>::from_row_array(&[z, y, z, w])),
                x.simd_eq(z) & y.simd_eq(y) & z.simd_eq(z) & w.simd_eq(w)
            );
            assert_float_eq!(
                Matrix::<2, Wide, A>::from_row_array(&[x, y, z, w])
                    .simd_eq(&Matrix::<2, Wide, A>::from_row_array(&[z, w, x, y])),
                x.simd_eq(z) & y.simd_eq(w)
            );

            assert_float_eq!(
                Matrix::<3, Wide, A>::from_row_array(&[x, y, z, x, y, w, x, y, z]).simd_eq(
                    &Matrix::<3, Wide, A>::from_row_array(&[x, y, w, x, y, w, x, y, z])
                ),
                x.simd_eq(x) & y.simd_eq(y) & z.simd_eq(w) & w.simd_eq(w) & z.simd_eq(z)
            );
            assert_float_eq!(
                Matrix::<3, Wide, A>::from_row_array(&[x, y, z, z, w, y, x, y, z]).simd_eq(
                    &Matrix::<3, Wide, A>::from_row_array(&[z, w, y, x, y, z, z, w, y])
                ),
                x.simd_eq(z) & y.simd_eq(w) & z.simd_eq(y)
            );

            assert_float_eq!(
                Matrix::<4, Wide, A>::from_row_array(&[
                    x, y, z, w, x, y, z, y, x, y, y, w, x, y, z, x
                ])
                .simd_eq(&Matrix::<4, Wide, A>::from_row_array(&[
                    w, y, z, w, x, y, z, y, x, y, y, w, x, y, z, x
                ])),
                x.simd_eq(w) & y.simd_eq(y) & z.simd_eq(z) & w.simd_eq(w)
            );
            assert_float_eq!(
                Matrix::<4, Wide, A>::from_row_array(&[
                    x, y, z, w, z, w, y, x, x, y, z, w, z, w, y, x
                ])
                .simd_eq(&Matrix::<4, Wide, A>::from_row_array(&[
                    z, w, y, x, x, y, z, w, z, w, y, x, x, y, z, w
                ])),
                x.simd_eq(z) & y.simd_eq(w) & z.simd_eq(y) & w.simd_eq(x)
            );
        });
    }

    #[test]
    fn test_simd_ne() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let matrix = Matrix::<2, Wide, A>::from_row_array(&[x, y, z, w]);
            for other in [
                Matrix::<2, Wide, A>::from_row_array(&[z, y, z, w]),
                Matrix::<2, Wide, A>::from_row_array(&[z, w, x, y]),
            ] {
                assert_float_eq!(matrix.simd_ne(&other), !matrix.simd_eq(&other));
            }

            for (matrix, other) in [
                (
                    Matrix::<3, Wide, A>::from_row_array(&[x, y, z, x, y, w, x, y, z]),
                    Matrix::<3, Wide, A>::from_row_array(&[x, y, w, x, y, w, x, y, z]),
                ),
                (
                    Matrix::<3, Wide, A>::from_row_array(&[x, y, z, z, w, y, x, y, z]),
                    Matrix::<3, Wide, A>::from_row_array(&[z, w, y, x, y, z, z, w, y]),
                ),
            ] {
                assert_float_eq!(matrix.simd_ne(&other), !matrix.simd_eq(&other));
            }

            for (matrix, other) in [
                (
                    Matrix::<4, Wide, A>::from_row_array(&[
                        x, y, z, w, x, y, z, y, x, y, y, w, x, y, z, x,
                    ]),
                    Matrix::<4, Wide, A>::from_row_array(&[
                        w, y, z, w, x, y, z, y, x, y, y, w, x, y, z, x,
                    ]),
                ),
                (
                    Matrix::<4, Wide, A>::from_row_array(&[
                        x, y, z, w, z, w, y, x, x, y, z, w, z, w, y, x,
                    ]),
                    Matrix::<4, Wide, A>::from_row_array(&[
                        z, w, y, x, x, y, z, w, z, w, y, x, x, y, z, w,
                    ]),
                ),
            ] {
                assert_float_eq!(matrix.simd_ne(&other), !matrix.simd_eq(&other));
            }
        });
    }
}
