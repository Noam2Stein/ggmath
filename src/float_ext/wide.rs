use wide::{CmpLe, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

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
    use crate::{
        FloatExt,
        utils::{assert_float_eq, for_parameters},
    };

    #[test]
    fn test_lerp() {
        for_parameters!(|Wide: WideFloat, value, other, t| {
            let _: [Wide; 3] = [value, other, t];

            assert_float_eq!(
                value.lerp(other, t),
                Wide::new(core::array::from_fn(
                    |i| value.to_array()[i].lerp(other.to_array()[i], t.to_array()[i])
                ))
            );
        });
    }

    #[test]
    fn test_move_towards() {
        for_parameters!(|Wide: WideFloat, value, target, max_delta| {
            let _: [Wide; 3] = [value, target, max_delta];

            assert_float_eq!(
                value.move_towards(target, max_delta),
                Wide::new(core::array::from_fn(|i| value.to_array()[i]
                    .move_towards(target.to_array()[i], max_delta.to_array()[i])))
            );
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|Wide: WideFloat, value, other, max_abs_diff| {
            let _: [Wide; 3] = [value, other, max_abs_diff];

            assert_eq!(
                value.abs_diff_eq(other, max_abs_diff),
                (0..LANES).all(|i| value.to_array()[i]
                    .abs_diff_eq(other.to_array()[i], max_abs_diff.to_array()[i]))
            );
        });
    }
}
