use wide::Select;

use crate::{Alignment, Element, Rotation2, Vector, utils::WideTy};

/// Functionality for [SoA] (Structure of Arrays) 2D rotations.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[expect(private_bounds)]
impl<Wide, T, const LANES: usize, A: Alignment> Rotation2<Wide, A>
where
    Wide: WideTy<Array = [T; LANES]>,
    T: Element,
{
    /// Creates an SoA (Structure of Arrays) 2D rotation from an array of lanes
    /// or scalar 2D rotations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rot2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Rot2::from_cos_sin(1, 2),
    ///     Rot2::from_cos_sin(3, 4),
    ///     Rot2::from_cos_sin(5, 6),
    ///     Rot2::from_cos_sin(7, 8),
    /// ];
    /// assert_eq!(
    ///     Rot2::<i32x4>::from_lanes(&lanes),
    ///     Rot2::from_cos_sin(
    ///         i32x4::new([1, 3, 5, 7]),
    ///         i32x4::new([2, 4, 6, 8]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_lanes(lanes: &[Rotation2<T, A>; LANES]) -> Self {
        Self(Vector::from_lane_fn(|lane| lanes[lane].0))
    }

    /// Converts an SoA (Structure of Arrays) 2D rotation to an array of lanes or
    /// scalar 2D rotations.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rot2;
    /// # use wide::i32x4;
    /// #
    /// let rotation = Rot2::from_cos_sin(
    ///     i32x4::new([1, 3, 5, 7]),
    ///     i32x4::new([2, 4, 6, 8]),
    /// );
    /// assert_eq!(
    ///     rotation.to_lanes(),
    ///     [
    ///         Rot2::from_cos_sin(1, 2),
    ///         Rot2::from_cos_sin(3, 4),
    ///         Rot2::from_cos_sin(5, 6),
    ///         Rot2::from_cos_sin(7, 8),
    ///     ],
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Rotation2<T, A>; LANES] {
        core::array::from_fn(|lane| self.lane(lane))
    }

    /// Creates an SoA (Structure of Arrays) 2D rotation by calling function `f`
    /// for each lane index.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rot2;
    /// # use wide::i32x4;
    /// #
    /// let lanes = [
    ///     Rot2::from_cos_sin(1, 2),
    ///     Rot2::from_cos_sin(3, 4),
    ///     Rot2::from_cos_sin(5, 6),
    ///     Rot2::from_cos_sin(7, 8),
    /// ];
    /// assert_eq!(
    ///     Rot2::<i32x4>::from_lane_fn(|lane_index| lanes[lane_index]),
    ///     Rot2::from_cos_sin(
    ///         i32x4::new([1, 3, 5, 7]),
    ///         i32x4::new([2, 4, 6, 8]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Rotation2<T, A>,
    {
        Self::from_lanes(&core::array::from_fn(f))
    }

    /// Takes an SoA (Structure of Arrays) 2D rotation and returns the lane at
    /// the given index.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rot2;
    /// # use wide::i32x4;
    /// #
    /// let rotation = Rot2::from_cos_sin(
    ///     i32x4::new([1, 3, 5, 7]),
    ///     i32x4::new([2, 4, 6, 8]),
    /// );
    /// assert_eq!(rotation.lane(1), Rot2::from_cos_sin(3, 4));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, lane: usize) -> Rotation2<T, A> {
        Rotation2::from_vector(self.0.lane(lane))
    }

    /// Takes an SoA (Structure of Arrays) 2D rotation and sets the lane at the
    /// given index to `value`.
    ///
    /// # Panics
    ///
    /// Panics if `lane` is greater than or equal to the number of lanes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Rot2;
    /// # use wide::i32x4;
    /// #
    /// let mut rotation = Rot2::from_cos_sin(
    ///     i32x4::new([1, 3, 5, 7]),
    ///     i32x4::new([2, 4, 6, 8]),
    /// );
    /// rotation.set_lane(1, Rot2::IDENTITY);
    /// assert_eq!(
    ///     rotation,
    ///     Rot2::from_cos_sin(
    ///         i32x4::new([1, 1, 5, 7]),
    ///         i32x4::new([2, 0, 6, 8]),
    ///     ),
    /// );
    /// ```
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, lane: usize, value: Rotation2<T, A>) {
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
        impl<Wide, A: Alignment> Select<Rotation2<Wide, A>> for wide::$Mask
        where
            wide::$Mask: Select<Wide>,
            Wide: WideTy,
        {
            #[inline]
            fn select(
                self,
                if_true: Rotation2<Wide, A>,
                if_false: Rotation2<Wide, A>,
            ) -> Rotation2<Wide, A> {
                Rotation2(self.select::<Vector<2, Wide, A>>(if_true.0, if_false.0))
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
    use wide::{f32x4, i32x4};

    use crate::{
        Rot2, Rot2A, Vec2,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_from_lanes() {
        assert_eq!(
            Rot2A::<i32x4>::from_lanes(&[
                Rot2A::from_cos_sin(1, 2),
                Rot2A::from_cos_sin(3, 4),
                Rot2A::from_cos_sin(5, 6),
                Rot2A::from_cos_sin(7, 8),
            ]),
            Rot2A::from_cos_sin(i32x4::new([1, 3, 5, 7]), i32x4::new([2, 4, 6, 8])),
        );
    }

    #[test]
    fn test_from_lane_fn() {
        assert_eq!(
            Rot2A::<i32x4>::from_lane_fn(|i| [
                Rot2A::from_cos_sin(1, 2),
                Rot2A::from_cos_sin(3, 4),
                Rot2A::from_cos_sin(5, 6),
                Rot2A::from_cos_sin(7, 8),
            ][i]),
            Rot2A::from_cos_sin(i32x4::new([1, 3, 5, 7]), i32x4::new([2, 4, 6, 8])),
        );
    }

    #[test]
    fn test_to_lanes() {
        assert_eq!(
            Rot2A::from_cos_sin(i32x4::new([1, 3, 5, 7]), i32x4::new([2, 4, 6, 8])).to_lanes(),
            [
                Rot2A::from_cos_sin(1, 2),
                Rot2A::from_cos_sin(3, 4),
                Rot2A::from_cos_sin(5, 6),
                Rot2A::from_cos_sin(7, 8),
            ],
        );
    }

    #[test]
    fn test_lane() {
        let rotation = Rot2A::from_cos_sin(i32x4::new([1, 3, 5, 7]), i32x4::new([2, 4, 6, 8]));

        assert_eq!(rotation.lane(0), Rot2A::from_cos_sin(1, 2));
        assert_eq!(rotation.lane(1), Rot2A::from_cos_sin(3, 4));
        assert_eq!(rotation.lane(2), Rot2A::from_cos_sin(5, 6));
        assert_eq!(rotation.lane(3), Rot2A::from_cos_sin(7, 8));
        assert_panic!(rotation.lane(4));
    }

    #[test]
    fn test_set_lane() {
        let mut rotation = Rot2A::from_cos_sin(i32x4::new([1, 3, 5, 7]), i32x4::new([2, 4, 6, 8]));

        rotation.set_lane(0, Rot2A::from_cos_sin(-1, -2));
        assert_eq!(
            rotation,
            Rot2A::from_cos_sin(i32x4::new([-1, 3, 5, 7]), i32x4::new([-2, 4, 6, 8]))
        );
        rotation.set_lane(1, Rot2A::from_cos_sin(-3, -4));
        assert_eq!(
            rotation,
            Rot2A::from_cos_sin(i32x4::new([-1, -3, 5, 7]), i32x4::new([-2, -4, 6, 8]))
        );
        rotation.set_lane(2, Rot2A::from_cos_sin(-5, -6));
        assert_eq!(
            rotation,
            Rot2A::from_cos_sin(i32x4::new([-1, -3, -5, 7]), i32x4::new([-2, -4, -6, 8]))
        );
        rotation.set_lane(3, Rot2A::from_cos_sin(-7, -8));
        assert_eq!(
            rotation,
            Rot2A::from_cos_sin(i32x4::new([-1, -3, -5, -7]), i32x4::new([-2, -4, -6, -8]))
        );
        assert_panic!(rotation.clone().set_lane(4, Rot2A::IDENTITY));
    }

    #[test]
    fn test_simd_eq() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], mask) in random_iter::<([Rot2<Wide>; 2], Vec2<Wide>)>() {
                let mask = mask.sign_negative_mask();
                let b = Rot2::from_vector(mask.select(a.to_vector(), b.to_vector()));

                assert_test_eq!(a.simd_eq(&b), a.to_vector().simd_eq(b.to_vector()));
            }
        });
    }

    #[test]
    fn test_simd_ne() {
        for_types!(|Wide: WideFloat| {
            for ([a, b], mask) in random_iter::<([Rot2<Wide>; 2], Vec2<Wide>)>() {
                let mask = mask.sign_negative_mask();
                let b = Rot2::from_vector(mask.select(a.to_vector(), b.to_vector()));

                assert_test_eq!(a.simd_ne(&b), a.to_vector().simd_ne(b.to_vector()));
            }
        });
    }

    #[test]
    fn test_scalar_select() {
        for (mask, [if_true, if_false]) in random_iter::<(i32x4, [Rot2<f32x4>; 2])>() {
            let mask = mask.is_negative();

            assert_test_eq!(
                mask.select(if_true, if_false),
                Rot2::from_lane_fn(|lane| if mask.as_array()[lane].is_negative() {
                    if_true.lane(lane)
                } else {
                    if_false.lane(lane)
                })
            );
        }
    }
}
