use wide::CmpNe;

use crate::{Alignment, Quaternion, Scalar, Vector, utils::WideTy};

#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Quaternion<Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// Creates an SoA (Structure of Arrays) quaternion from an array of lanes
    /// or scalar quaternions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Quat::from_xyzw(1, 2, 3, 4),
    ///     Quat::from_xyzw(5, 6, 7, 8),
    ///     Quat::from_xyzw(9, 10, 11, 12),
    ///     Quat::from_xyzw(13, 14, 15, 16),
    /// ];
    /// assert_eq!(
    ///     Quat::<i32x4>::from_lanes(&lanes),
    ///     Quat::from_xyzw(
    ///         i32x4::new([1, 5, 9, 13]),
    ///         i32x4::new([2, 6, 10, 14]),
    ///         i32x4::new([3, 7, 11, 15]),
    ///         i32x4::new([4, 8, 12, 16]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Quaternion<T, A>; LANES]) -> Self {
        Self::from_vector(Vector::from_lane_fn(|lane| lanes[lane].to_vector()))
    }

    /// Creates an SoA (Structure of Arrays) quaternion by calling function `f`
    /// for each lane index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Quat::from_xyzw(1, 2, 3, 4),
    ///     Quat::from_xyzw(5, 6, 7, 8),
    ///     Quat::from_xyzw(9, 10, 11, 12),
    ///     Quat::from_xyzw(13, 14, 15, 16),
    /// ];
    /// assert_eq!(
    ///     Quat::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Quat::from_xyzw(
    ///         i32x4::new([1, 5, 9, 13]),
    ///         i32x4::new([2, 6, 10, 14]),
    ///         i32x4::new([3, 7, 11, 15]),
    ///         i32x4::new([4, 8, 12, 16]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Quaternion<T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Converts an SoA (Structure of Arrays) quaternion to an array of lanes or
    /// scalar quaternions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// # use wide::i32x4;
    /// #
    /// let quat = Quat::from_xyzw(
    ///     i32x4::new([1, 5, 9, 13]),
    ///     i32x4::new([2, 6, 10, 14]),
    ///     i32x4::new([3, 7, 11, 15]),
    ///     i32x4::new([4, 8, 12, 16]),
    /// );
    /// assert_eq!(
    ///     quat.to_lanes(),
    ///     [
    ///         Quat::from_xyzw(1, 2, 3, 4),
    ///         Quat::from_xyzw(5, 6, 7, 8),
    ///         Quat::from_xyzw(9, 10, 11, 12),
    ///         Quat::from_xyzw(13, 14, 15, 16),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Quaternion<T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) quaternion and returns the lane at
    /// the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// # use wide::i32x4;
    /// #
    /// let quat = Quat::from_xyzw(
    ///     i32x4::new([1, 5, 9, 13]),
    ///     i32x4::new([2, 6, 10, 14]),
    ///     i32x4::new([3, 7, 11, 15]),
    ///     i32x4::new([4, 8, 12, 16]),
    /// );
    /// assert_eq!(quat.lane(1), Quat::from_xyzw(5, 6, 7, 8));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Quaternion<T, A> {
        Quaternion::from_vector(self.0.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) quaternion and sets the lane at the
    /// given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// # use wide::i32x4;
    /// #
    /// let mut quat = Quat::from_xyzw(
    ///     i32x4::new([1, 5, 9, 13]),
    ///     i32x4::new([2, 6, 10, 14]),
    ///     i32x4::new([3, 7, 11, 15]),
    ///     i32x4::new([4, 8, 12, 16]),
    /// );
    /// quat.set_lane(1, Quat::IDENTITY);
    /// assert_eq!(
    ///     quat,
    ///     Quat::from_xyzw(
    ///         i32x4::new([1, 0, 9, 13]),
    ///         i32x4::new([2, 0, 10, 14]),
    ///         i32x4::new([3, 0, 11, 15]),
    ///         i32x4::new([4, 1, 12, 16]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Quaternion<T, A>) {
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
    pub fn simd_ne(&self, other: &Self) -> Wide
    where
        Wide: CmpNe<Output = Wide>,
    {
        self.0.simd_ne(other.0)
    }
}

#[cfg(test)]
mod tests {
    use wide::{CmpEq, i32x4};

    use crate::{
        QuatA, Quaternion,
        utils::{assert_float_eq, assert_panic, for_parameters},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            QuatA::<i32x4>::from_lanes(&[
                QuatA::from_xyzw(1, 2, 3, 4),
                QuatA::from_xyzw(5, 6, 7, 8),
                QuatA::from_xyzw(9, 10, 11, 12),
                QuatA::from_xyzw(13, 14, 15, 16),
            ]),
            QuatA::from_xyzw(
                i32x4::new([1, 5, 9, 13]),
                i32x4::new([2, 6, 10, 14]),
                i32x4::new([3, 7, 11, 15]),
                i32x4::new([4, 8, 12, 16]),
            ),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            QuatA::<i32x4>::from_lane_fn(|i| [
                QuatA::from_xyzw(1, 2, 3, 4),
                QuatA::from_xyzw(5, 6, 7, 8),
                QuatA::from_xyzw(9, 10, 11, 12),
                QuatA::from_xyzw(13, 14, 15, 16),
            ][i]),
            QuatA::from_xyzw(
                i32x4::new([1, 5, 9, 13]),
                i32x4::new([2, 6, 10, 14]),
                i32x4::new([3, 7, 11, 15]),
                i32x4::new([4, 8, 12, 16]),
            ),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            QuatA::from_xyzw(
                i32x4::new([1, 5, 9, 13]),
                i32x4::new([2, 6, 10, 14]),
                i32x4::new([3, 7, 11, 15]),
                i32x4::new([4, 8, 12, 16]),
            )
            .to_lanes(),
            [
                QuatA::from_xyzw(1, 2, 3, 4),
                QuatA::from_xyzw(5, 6, 7, 8),
                QuatA::from_xyzw(9, 10, 11, 12),
                QuatA::from_xyzw(13, 14, 15, 16),
            ],
        );
    }

    #[test]
    fn test_lane() {
        let quat = QuatA::from_xyzw(
            i32x4::new([1, 5, 9, 13]),
            i32x4::new([2, 6, 10, 14]),
            i32x4::new([3, 7, 11, 15]),
            i32x4::new([4, 8, 12, 16]),
        );

        assert_eq!(quat.lane(0), QuatA::from_xyzw(1, 2, 3, 4));
        assert_eq!(quat.lane(1), QuatA::from_xyzw(5, 6, 7, 8));
        assert_eq!(quat.lane(2), QuatA::from_xyzw(9, 10, 11, 12));
        assert_eq!(quat.lane(3), QuatA::from_xyzw(13, 14, 15, 16));
        assert_panic!(quat.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut quat = QuatA::from_xyzw(
            i32x4::new([1, 5, 9, 13]),
            i32x4::new([2, 6, 10, 14]),
            i32x4::new([3, 7, 11, 15]),
            i32x4::new([4, 8, 12, 16]),
        );

        quat.set_lane(0, QuatA::from_xyzw(-1, -2, -3, -4));
        assert_eq!(
            quat,
            QuatA::from_xyzw(
                i32x4::new([-1, 5, 9, 13]),
                i32x4::new([-2, 6, 10, 14]),
                i32x4::new([-3, 7, 11, 15]),
                i32x4::new([-4, 8, 12, 16]),
            )
        );
        quat.set_lane(1, QuatA::from_xyzw(-5, -6, -7, -8));
        assert_eq!(
            quat,
            QuatA::from_xyzw(
                i32x4::new([-1, -5, 9, 13]),
                i32x4::new([-2, -6, 10, 14]),
                i32x4::new([-3, -7, 11, 15]),
                i32x4::new([-4, -8, 12, 16]),
            )
        );
        quat.set_lane(2, QuatA::from_xyzw(-9, -10, -11, -12));
        assert_eq!(
            quat,
            QuatA::from_xyzw(
                i32x4::new([-1, -5, -9, 13]),
                i32x4::new([-2, -6, -10, 14]),
                i32x4::new([-3, -7, -11, 15]),
                i32x4::new([-4, -8, -12, 16]),
            )
        );
        quat.set_lane(3, QuatA::from_xyzw(-13, -14, -15, -16));
        assert_eq!(
            quat,
            QuatA::from_xyzw(
                i32x4::new([-1, -5, -9, -13]),
                i32x4::new([-2, -6, -10, -14]),
                i32x4::new([-3, -7, -11, -15]),
                i32x4::new([-4, -8, -12, -16]),
            )
        );
        assert_panic!(quat.clone().set_lane(4, QuatA::IDENTITY));
    }

    #[test]
    fn test_simd_eq() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w)
                    .simd_eq(&Quaternion::<Wide, A>::from_xyzw(z, y, z, w)),
                x.simd_eq(z) & y.simd_eq(y) & z.simd_eq(z) & w.simd_eq(w)
            );
            assert_float_eq!(
                Quaternion::<Wide, A>::from_xyzw(x, y, z, w)
                    .simd_eq(&Quaternion::<Wide, A>::from_xyzw(z, w, x, y)),
                x.simd_eq(z) & y.simd_eq(w)
            );
        });
    }

    #[test]
    fn test_simd_ne() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let quat = Quaternion::<Wide, A>::from_xyzw(x, y, z, w);
            for other in [
                Quaternion::<Wide, A>::from_xyzw(z, y, z, w),
                Quaternion::<Wide, A>::from_xyzw(z, w, x, y),
            ] {
                assert_float_eq!(quat.simd_ne(&other), !quat.simd_eq(&other));
            }
        });
    }
}
