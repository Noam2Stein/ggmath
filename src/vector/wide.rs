use wide::{CmpGe, CmpLe, CmpNe};

use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::WideTy};

#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Vector<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) vector from an array of
    /// lanes or scalar vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Vec3::new(1, 2, 3),
    ///     Vec3::new(4, 5, 6),
    ///     Vec3::new(7, 8, 9),
    ///     Vec3::new(10, 11, 12),
    /// ];
    /// assert_eq!(
    ///     Vec3::<i32x4>::from_lanes(&lanes),
    ///     Vec3::new(
    ///         i32x4::new([1, 4, 7, 10]),
    ///         i32x4::new([2, 5, 8, 11]),
    ///         i32x4::new([3, 6, 9, 12]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Vector<N, T, A>; LANES]) -> Self {
        Self::from_fn(|i| Wide::new(lanes.map(|lane| lane[i])))
    }

    /// Creates an SoA (Structure of Arrays) vector by calling function
    /// `f` for each lane index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Vec3::new(1, 2, 3),
    ///     Vec3::new(4, 5, 6),
    ///     Vec3::new(7, 8, 9),
    ///     Vec3::new(10, 11, 12),
    /// ];
    /// assert_eq!(
    ///     Vec3::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Vec3::new(
    ///         i32x4::new([1, 4, 7, 10]),
    ///         i32x4::new([2, 5, 8, 11]),
    ///         i32x4::new([3, 6, 9, 12]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) vector to an array of
    /// lanes or scalar vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let vector = Vec3::new(
    ///     i32x4::new([1, 4, 7, 10]),
    ///     i32x4::new([2, 5, 8, 11]),
    ///     i32x4::new([3, 6, 9, 12]),
    /// );
    /// assert_eq!(
    ///     vector.to_lanes(),
    ///     [
    ///         Vec3::new(1, 2, 3),
    ///         Vec3::new(4, 5, 6),
    ///         Vec3::new(7, 8, 9),
    ///         Vec3::new(10, 11, 12),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(self) -> [Vector<N, T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) vector and returns the lane
    /// at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of
    /// lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let vector = Vec3::new(
    ///     i32x4::new([1, 4, 7, 10]),
    ///     i32x4::new([2, 5, 8, 11]),
    ///     i32x4::new([3, 6, 9, 12]),
    /// );
    /// assert_eq!(vector.lane(1), Vec3::new(4, 5, 6));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(self, lane: usize) -> Vector<N, T, A> {
        Vector::from_fn(|i| self[i].to_array()[lane])
    }

    /// Takes an SoA (Structure of Arrays) vector and sets the lane at
    /// the given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of
    /// lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// # use wide::i32x4;
    /// #
    /// let mut vector = Vec3::new(
    ///     i32x4::new([1, 4, 7, 10]),
    ///     i32x4::new([2, 5, 8, 11]),
    ///     i32x4::new([3, 6, 9, 12]),
    /// );
    /// vector.set_lane(1, Vec3::ZERO);
    /// assert_eq!(
    ///     vector,
    ///     Vec3::new(
    ///         i32x4::new([1, 0, 7, 10]),
    ///         i32x4::new([2, 0, 8, 11]),
    ///         i32x4::new([3, 0, 9, 12]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Vector<N, T, A>) {
        for i in 0..N {
            self[i].as_mut_array()[lane] = value[i];
        }
    }

    /// For each lane, returns `true` if all elements of `self` are `true`.
    #[inline]
    #[must_use]
    pub fn all(self) -> Wide {
        match N {
            2 => self[0] & self[1],
            3 => self[0] & self[1] & self[2],
            4 => self[0] & self[1] & self[2] & self[3],
            _ => unreachable!(),
        }
    }

    /// For each lane, returns `true` if any element of `self` is `true`.
    #[inline]
    #[must_use]
    pub fn any(self) -> Wide {
        match N {
            2 => self[0] | self[1],
            3 => self[0] | self[1] | self[2],
            4 => self[0] | self[1] | self[2] | self[3],
            _ => unreachable!(),
        }
    }

    /// For each lane, selects between the elements of `if_true` and `if_false`
    /// based on the boolean elements of `self`.
    #[inline]
    #[must_use]
    pub fn blend(self, if_true: Self, if_false: Self) -> Self {
        Vector::from_fn(|i| self[i].blend(if_true[i], if_false[i]))
    }

    /// For each lane, returns `true` if `self` is equal to `other`.
    ///
    /// Equivalent to
    /// `(self.lane(0) == other.lane(0), self.lane(1) == other.lane(1), ...)`.
    #[inline]
    #[must_use]
    pub fn simd_eq(self, other: Self) -> Wide {
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
    pub fn simd_ne(self, other: Self) -> Wide
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

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding elements of `self` and `other` are equal.
    ///
    /// Equivalent to `(self.x == other.x, self.y == other.y, ...)` for each
    /// lane.
    #[inline]
    #[must_use]
    pub fn simd_eq_mask(self, other: Self) -> Self {
        Self::from_fn(|i| self[i].simd_eq(other[i]))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding elements of `self` and `other` are not equal.
    ///
    /// Equivalent to `(self.x != other.x, self.y != other.y, ...)` for each lane.
    #[inline]
    #[must_use]
    pub fn simd_ne_mask(self, other: Self) -> Self
    where
        Wide: CmpNe<Output = Wide>,
    {
        Self::from_fn(|i| self[i].simd_ne(other[i]))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is less than the corresponding element
    /// of `other`.
    ///
    /// Equivalent to `(self.x < other.x, self.y < other.y, ...)` for each lane.
    #[inline]
    #[must_use]
    pub fn simd_lt_mask(self, other: Self) -> Self {
        Self::from_fn(|i| self[i].simd_lt(other[i]))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is greater than the corresponding
    /// element of `other`.
    ///
    /// Equivalent to `(self.x > other.x, self.y > other.y, ...)` for each lane.
    #[inline]
    #[must_use]
    pub fn simd_gt_mask(self, other: Self) -> Self {
        Self::from_fn(|i| self[i].simd_gt(other[i]))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is less than or equal to the
    /// corresponding element of `other`.
    ///
    /// Equivalent to `(self.x <= other.x, self.y <= other.y, ...)` for each
    /// lane.
    #[inline]
    #[must_use]
    pub fn simd_le_mask(self, other: Self) -> Self
    where
        Wide: CmpLe<Output = Wide>,
    {
        Self::from_fn(|i| self[i].simd_le(other[i]))
    }

    /// For each lane, returns a vector mask where each element is `true` if the
    /// corresponding element of `self` is greater than or equal to the
    /// corresponding element of `other`.
    ///
    /// Equivalent to `(self.x >= other.x, self.y >= other.y, ...)` for each
    /// lane.
    #[inline]
    #[must_use]
    pub fn simd_ge_mask(self, other: Self) -> Self
    where
        Wide: CmpGe<Output = Wide>,
    {
        Self::from_fn(|i| self[i].simd_ge(other[i]))
    }
}

#[cfg(test)]
mod tests {
    use wide::{CmpEq, CmpGe, CmpGt, CmpLe, CmpLt, CmpNe, i32x4};

    use crate::{
        Vec3A, Vector,
        utils::{assert_float_eq, assert_panic, for_parameters},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Vec3A::<i32x4>::from_lanes(&[
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(7, 8, 9),
                Vec3A::new(10, 11, 12),
            ]),
            Vec3A::new(
                i32x4::new([1, 4, 7, 10]),
                i32x4::new([2, 5, 8, 11]),
                i32x4::new([3, 6, 9, 12]),
            ),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Vec3A::<i32x4>::from_lane_fn(|lane| [
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(7, 8, 9),
                Vec3A::new(10, 11, 12),
            ][lane]),
            Vec3A::new(
                i32x4::new([1, 4, 7, 10]),
                i32x4::new([2, 5, 8, 11]),
                i32x4::new([3, 6, 9, 12]),
            ),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Vec3A::new(
                i32x4::new([1, 4, 7, 10]),
                i32x4::new([2, 5, 8, 11]),
                i32x4::new([3, 6, 9, 12]),
            )
            .to_lanes(),
            [
                Vec3A::new(1, 2, 3),
                Vec3A::new(4, 5, 6),
                Vec3A::new(7, 8, 9),
                Vec3A::new(10, 11, 12),
            ]
        );
    }

    #[test]
    fn test_lane() {
        let vector = Vec3A::new(
            i32x4::new([1, 4, 7, 10]),
            i32x4::new([2, 5, 8, 11]),
            i32x4::new([3, 6, 9, 12]),
        );

        assert_eq!(vector.lane(0), Vec3A::new(1, 2, 3));
        assert_eq!(vector.lane(1), Vec3A::new(4, 5, 6));
        assert_eq!(vector.lane(2), Vec3A::new(7, 8, 9));
        assert_eq!(vector.lane(3), Vec3A::new(10, 11, 12));
        assert_panic!(vector.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut vector = Vec3A::new(
            i32x4::new([1, 4, 7, 10]),
            i32x4::new([2, 5, 8, 11]),
            i32x4::new([3, 6, 9, 12]),
        );

        vector.set_lane(0, Vec3A::new(-1, -2, -3));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, 4, 7, 10]),
                i32x4::new([-2, 5, 8, 11]),
                i32x4::new([-3, 6, 9, 12]),
            )
        );
        vector.set_lane(1, Vec3A::new(-4, -5, -6));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, -4, 7, 10]),
                i32x4::new([-2, -5, 8, 11]),
                i32x4::new([-3, -6, 9, 12]),
            )
        );
        vector.set_lane(2, Vec3A::new(-7, -8, -9));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, -4, -7, 10]),
                i32x4::new([-2, -5, -8, 11]),
                i32x4::new([-3, -6, -9, 12]),
            )
        );
        vector.set_lane(3, Vec3A::new(-10, -11, -12));
        assert_eq!(
            vector,
            Vec3A::new(
                i32x4::new([-1, -4, -7, -10]),
                i32x4::new([-2, -5, -8, -11]),
                i32x4::new([-3, -6, -9, -12]),
            )
        );
        assert_panic!(vector.clone().set_lane(4, Vector::ZERO));
    }

    #[test]
    fn test_all() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(Vector::<2, Wide, A>::new(x, y).all(), x & y);
            assert_float_eq!(Vector::<3, Wide, A>::new(x, y, z).all(), x & y & z);
            assert_float_eq!(Vector::<4, Wide, A>::new(x, y, z, w).all(), x & y & z & w);
        });
    }

    #[test]
    fn test_any() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(Vector::<2, Wide, A>::new(x, y).any(), x | y);
            assert_float_eq!(Vector::<3, Wide, A>::new(x, y, z).any(), x | y | z);
            assert_float_eq!(Vector::<4, Wide, A>::new(x, y, z, w).any(), x | y | z | w);
        });
    }

    #[test]
    fn test_blend() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).blend(
                    Vector::<2, Wide, A>::new(y, z),
                    Vector::<2, Wide, A>::new(z, w)
                ),
                Vector::<2, Wide, A>::new(x.blend(y, z), y.blend(z, w))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).blend(
                    Vector::<3, Wide, A>::new(y, z, w),
                    Vector::<3, Wide, A>::new(z, w, x)
                ),
                Vector::<3, Wide, A>::new(x.blend(y, z), y.blend(z, w), z.blend(w, x))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).blend(
                    Vector::<4, Wide, A>::new(y, z, w, x),
                    Vector::<4, Wide, A>::new(z, w, x, y)
                ),
                Vector::<4, Wide, A>::new(
                    x.blend(y, z),
                    y.blend(z, w),
                    z.blend(w, x),
                    w.blend(x, y)
                )
            );
        });
    }

    #[test]
    fn test_simd_eq() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_eq(Vector::<2, Wide, A>::new(a, b)),
                x.simd_eq(a) & y.simd_eq(b)
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_eq(Vector::<3, Wide, A>::new(a, b, c)),
                x.simd_eq(a) & y.simd_eq(b) & z.simd_eq(c)
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_eq(Vector::<4, Wide, A>::new(a, b, c, d)),
                x.simd_eq(a) & y.simd_eq(b) & z.simd_eq(c) & w.simd_eq(d)
            );
        });
    }

    #[test]
    fn test_simd_ne() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_ne(Vector::<2, Wide, A>::new(a, b)),
                x.simd_ne(a) | y.simd_ne(b)
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_ne(Vector::<3, Wide, A>::new(a, b, c)),
                x.simd_ne(a) | y.simd_ne(b) | z.simd_ne(c)
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_ne(Vector::<4, Wide, A>::new(a, b, c, d)),
                x.simd_ne(a) | y.simd_ne(b) | z.simd_ne(c) | w.simd_ne(d)
            );
        });
    }

    #[test]
    fn test_simd_eq_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_eq_mask(Vector::<2, Wide, A>::new(a, b)),
                Vector::<2, Wide, A>::new(x.simd_eq(a), y.simd_eq(b))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_eq_mask(Vector::<3, Wide, A>::new(a, b, c)),
                Vector::<3, Wide, A>::new(x.simd_eq(a), y.simd_eq(b), z.simd_eq(c))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_eq_mask(Vector::<4, Wide, A>::new(a, b, c, d)),
                Vector::<4, Wide, A>::new(x.simd_eq(a), y.simd_eq(b), z.simd_eq(c), w.simd_eq(d))
            );
        });
    }

    #[test]
    fn test_simd_ne_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_ne_mask(Vector::<2, Wide, A>::new(a, b)),
                Vector::<2, Wide, A>::new(x.simd_ne(a), y.simd_ne(b))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_ne_mask(Vector::<3, Wide, A>::new(a, b, c)),
                Vector::<3, Wide, A>::new(x.simd_ne(a), y.simd_ne(b), z.simd_ne(c))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_ne_mask(Vector::<4, Wide, A>::new(a, b, c, d)),
                Vector::<4, Wide, A>::new(x.simd_ne(a), y.simd_ne(b), z.simd_ne(c), w.simd_ne(d))
            );
        });
    }

    #[test]
    fn test_simd_lt_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_lt_mask(Vector::<2, Wide, A>::new(a, b)),
                Vector::<2, Wide, A>::new(x.simd_lt(a), y.simd_lt(b))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_lt_mask(Vector::<3, Wide, A>::new(a, b, c)),
                Vector::<3, Wide, A>::new(x.simd_lt(a), y.simd_lt(b), z.simd_lt(c))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_lt_mask(Vector::<4, Wide, A>::new(a, b, c, d)),
                Vector::<4, Wide, A>::new(x.simd_lt(a), y.simd_lt(b), z.simd_lt(c), w.simd_lt(d))
            );
        });
    }

    #[test]
    fn test_simd_gt_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_gt_mask(Vector::<2, Wide, A>::new(a, b)),
                Vector::<2, Wide, A>::new(x.simd_gt(a), y.simd_gt(b))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_gt_mask(Vector::<3, Wide, A>::new(a, b, c)),
                Vector::<3, Wide, A>::new(x.simd_gt(a), y.simd_gt(b), z.simd_gt(c))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_gt_mask(Vector::<4, Wide, A>::new(a, b, c, d)),
                Vector::<4, Wide, A>::new(x.simd_gt(a), y.simd_gt(b), z.simd_gt(c), w.simd_gt(d))
            );
        });
    }

    #[test]
    fn test_simd_le_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_le_mask(Vector::<2, Wide, A>::new(a, b)),
                Vector::<2, Wide, A>::new(x.simd_le(a), y.simd_le(b))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_le_mask(Vector::<3, Wide, A>::new(a, b, c)),
                Vector::<3, Wide, A>::new(x.simd_le(a), y.simd_le(b), z.simd_le(c))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_le_mask(Vector::<4, Wide, A>::new(a, b, c, d)),
                Vector::<4, Wide, A>::new(x.simd_le(a), y.simd_le(b), z.simd_le(c), w.simd_le(d))
            );
        });
    }

    #[test]
    fn test_simd_ge_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).simd_ge_mask(Vector::<2, Wide, A>::new(a, b)),
                Vector::<2, Wide, A>::new(x.simd_ge(a), y.simd_ge(b))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).simd_ge_mask(Vector::<3, Wide, A>::new(a, b, c)),
                Vector::<3, Wide, A>::new(x.simd_ge(a), y.simd_ge(b), z.simd_ge(c))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .simd_ge_mask(Vector::<4, Wide, A>::new(a, b, c, d)),
                Vector::<4, Wide, A>::new(x.simd_ge(a), y.simd_ge(b), z.simd_ge(c), w.simd_ge(d))
            );
        });
    }
}
