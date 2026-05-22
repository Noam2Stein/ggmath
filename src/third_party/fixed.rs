use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
    types::extra::{
        IsLessOrEqual, True, U6, U7, U14, U15, U30, U31, U62, U63, U126, U127, Unsigned,
    },
};

use crate::{
    Aligned, Backend, DefaultBackend, Length, Mask, Scalar, SupportedLength, Unaligned, Vector,
    constants::{Max, Min, NegOne, One, Zero},
};

macro_rules! impl_fixed {
    ($T:ident, $Fixed:ident, $OneMaxFrac:ident) => {
        impl<Frac> Scalar for $Fixed<Frac> {}

        impl<const N: usize, Frac> DefaultBackend<N, Unaligned> for $Fixed<Frac> {}

        // SAFETY: Because `$Fixed<Frac>` is a transparent wrapper over `$T`
        // that accepts all bit-patterns, `Vector<N, $T, Aligned>` and
        // `Mask<N, $T, Aligned>` satisfy all requirements.
        unsafe impl<const N: usize, Frac> Backend<N, Aligned> for $Fixed<Frac>
        where
            Length<N>: SupportedLength,
        {
            type Vector = Vector<N, $T, Aligned>;
            type Mask = Mask<N, $T, Aligned>;

            #[inline]
            fn mask_from_array(array: [bool; N]) -> Mask<N, Self, Aligned> {
                Mask::from_inner(Mask::from_array(array))
            }

            #[inline]
            fn mask_to_array(mask: Mask<N, Self, Aligned>) -> [bool; N] {
                mask.inner().to_array()
            }
        }

        impl<Frac> Zero for $Fixed<Frac> {
            const ZERO: Self = Self::ZERO;
        }

        impl<Frac> One for $Fixed<Frac>
        where
            Frac: IsLessOrEqual<$OneMaxFrac, Output = True> + Unsigned,
        {
            const ONE: Self = Self::ONE;
        }

        impl<Frac> Min for $Fixed<Frac> {
            const MIN: Self = Self::MIN;
        }

        impl<Frac> Max for $Fixed<Frac> {
            const MAX: Self = Self::MAX;
        }
    };
}
impl_fixed!(i8, FixedI8, U6);
impl_fixed!(i16, FixedI16, U14);
impl_fixed!(i32, FixedI32, U30);
impl_fixed!(i64, FixedI64, U62);
impl_fixed!(i128, FixedI128, U126);
impl_fixed!(u8, FixedU8, U7);
impl_fixed!(u16, FixedU16, U15);
impl_fixed!(u32, FixedU32, U31);
impl_fixed!(u64, FixedU64, U63);
impl_fixed!(u128, FixedU128, U127);

macro_rules! impl_fixed_signed {
    ($Fixed:ident, $NegOneMaxFrac:ident) => {
        impl<Frac> NegOne for $Fixed<Frac>
        where
            Frac: IsLessOrEqual<$NegOneMaxFrac, Output = True> + Unsigned,
        {
            const NEG_ONE: Self = Self::NEG_ONE;
        }
    };
}
impl_fixed_signed!(FixedI8, U7);
impl_fixed_signed!(FixedI16, U15);
impl_fixed_signed!(FixedI32, U31);
impl_fixed_signed!(FixedI64, U63);
impl_fixed_signed!(FixedI128, U127);

#[cfg(test)]
mod tests {
    use fixed::types::{I4F4, I8F8, I24F8, I32F32, I64F64, U4F4, U8F8, U24F8, U32F32, U64F64};

    use crate::Vec3;

    #[test]
    fn test_constants() {
        assert_eq!(Vec3::ZERO, Vec3::splat(I4F4::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(I8F8::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(I24F8::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(I32F32::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(I64F64::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(U4F4::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(U8F8::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(U24F8::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(U32F32::ZERO));
        assert_eq!(Vec3::ZERO, Vec3::splat(U64F64::ZERO));

        assert_eq!(Vec3::ONE, Vec3::splat(I4F4::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(I8F8::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(I24F8::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(I32F32::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(I64F64::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(U4F4::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(U8F8::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(U24F8::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(U32F32::ONE));
        assert_eq!(Vec3::ONE, Vec3::splat(U64F64::ONE));

        assert_eq!(Vec3::NEG_ONE, Vec3::splat(I4F4::NEG_ONE));
        assert_eq!(Vec3::NEG_ONE, Vec3::splat(I8F8::NEG_ONE));
        assert_eq!(Vec3::NEG_ONE, Vec3::splat(I24F8::NEG_ONE));
        assert_eq!(Vec3::NEG_ONE, Vec3::splat(I32F32::NEG_ONE));
        assert_eq!(Vec3::NEG_ONE, Vec3::splat(I64F64::NEG_ONE));

        assert_eq!(Vec3::MIN, Vec3::splat(I4F4::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(I8F8::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(I24F8::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(I32F32::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(I64F64::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(U4F4::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(U8F8::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(U24F8::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(U32F32::MIN));
        assert_eq!(Vec3::MIN, Vec3::splat(U64F64::MIN));

        assert_eq!(Vec3::MAX, Vec3::splat(I4F4::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(I8F8::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(I24F8::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(I32F32::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(I64F64::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(U4F4::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(U8F8::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(U24F8::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(U32F32::MAX));
        assert_eq!(Vec3::MAX, Vec3::splat(U64F64::MAX));
    }
}
