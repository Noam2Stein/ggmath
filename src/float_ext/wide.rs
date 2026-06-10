use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{FloatExt, float_ext::Sealed};

macro_rules! impl_wide_float {
    ($Wide:ident) => {
        impl FloatExt for $Wide {
            #[inline]
            fn lerp(self, other: Self, t: Self) -> Self {
                self * (Self::ONE - t) + other * t
            }

            #[inline]
            fn move_towards(self, target: Self, max_delta: Self) -> Self {
                let delta = target - self;
                let delta_abs = delta.abs();

                (delta_abs.simd_le(max_delta) | delta_abs.simd_le($Wide::splat(1e-4)))
                    .blend(target, self + max_delta * delta.signum())
            }

            #[inline]
            fn abs_diff_eq(self, other: Self, max_abs_diff: Self) -> bool {
                (self - other).abs().simd_le(max_abs_diff).all()
            }
        }

        impl Sealed for $Wide {}
    };
}
impl_wide_float!(f32x4);
impl_wide_float!(f32x8);
impl_wide_float!(f32x16);
impl_wide_float!(f64x2);
impl_wide_float!(f64x4);
impl_wide_float!(f64x8);

#[cfg(test)]
mod tests {
    use wide::f32x4;

    use crate::{
        FloatExt,
        utils::{assert_test_eq, random_iter},
    };

    #[test]
    fn test_lerp() {
        for [value, other, t] in random_iter::<[f32x4; 3]>() {
            assert_test_eq!(
                value.lerp(other, t),
                f32x4::new(core::array::from_fn(
                    |i| value.to_array()[i].lerp(other.to_array()[i], t.to_array()[i])
                ))
            );
        }
    }

    #[test]
    fn test_move_towards() {
        for [value, target, max_delta] in random_iter::<[f32x4; 3]>() {
            assert_test_eq!(
                value.move_towards(target, max_delta),
                f32x4::new(core::array::from_fn(|i| value.to_array()[i]
                    .move_towards(target.to_array()[i], max_delta.to_array()[i])))
            );
        }
    }

    #[test]
    fn test_abs_diff_eq() {
        for [value, other, max_abs_diff] in random_iter::<[f32x4; 3]>() {
            assert_eq!(
                value.abs_diff_eq(other, max_abs_diff),
                (0..4).all(|i| value.to_array()[i]
                    .abs_diff_eq(other.to_array()[i], max_abs_diff.to_array()[i]))
            );
        }
    }
}
