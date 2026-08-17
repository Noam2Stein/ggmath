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
    pub fn from_lanes(lanes: &[Projective<N, T, A>; LANES]) -> Self {
        specialize_23!(Projective::<N, Wide, A>::from_lanes_backend(lanes))
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
        F: FnMut(usize) -> Projective<N, T, A>,
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
    pub fn to_lanes(&self) -> [Projective<N, T, A>; LANES] {
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
    pub fn lane(&self, lane: usize) -> Projective<N, T, A> {
        specialize_23!(Projective::<N, Wide, A>::lane_backend(self, lane))
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
