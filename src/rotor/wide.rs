use wide::Select;

use crate::{Alignment, Length, Rotor, Scalar, length::TwoOrThree, utils::WideTy};

#[expect(private_bounds)]
impl<const N: usize, Wide, T, const LANES: usize, A: Alignment> Rotor<N, Wide, A>
where
    Length<N>: TwoOrThree,
    Wide: WideTy<Array = [T; LANES]>,
    T: Scalar,
{
    /// TODO
    #[inline]
    #[must_use]
    pub fn from_lanes(_lanes: &[Rotor<N, T, A>; LANES]) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_lane_fn<F>(_f: F) -> Self
    where
        F: FnMut(usize) -> Rotor<N, T, A>,
    {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn to_lanes(&self) -> [Rotor<N, T, A>; LANES] {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lane(&self, _lane: usize) -> Rotor<N, T, A> {
        todo!()
    }

    /// TODO
    #[inline]
    #[track_caller]
    pub fn set_lane(&mut self, _lane: usize, _value: Rotor<N, T, A>) {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn simd_eq(&self, _other: &Self) -> Wide {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn simd_ne(&self, _other: &Self) -> Wide {
        todo!()
    }
}

/// Unfortunately this cannot be done with a generic `Mask` type due to orphan
/// rules.
macro_rules! impl_select {
    ($Mask:ident) => {
        impl<const N: usize, Wide, A: Alignment> Select<Rotor<N, Wide, A>> for wide::$Mask
        where
            Length<N>: TwoOrThree,
            wide::$Mask: Select<Wide>,
            Wide: WideTy,
        {
            #[inline]
            fn select(
                self,
                _if_true: Rotor<N, Wide, A>,
                _if_false: Rotor<N, Wide, A>,
            ) -> Rotor<N, Wide, A> {
                todo!()
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
    extern crate std;

    #[test]
    fn test_from_lanes() {
        todo!()
    }

    #[test]
    fn test_from_lane_fn() {
        todo!()
    }

    #[test]
    fn test_to_lanes() {
        todo!()
    }

    #[test]
    fn test_lane() {
        todo!()
    }

    #[test]
    fn test_set_lane() {
        todo!()
    }

    #[test]
    fn test_simd_eq() {
        todo!()
    }

    #[test]
    fn test_simd_ne() {
        todo!()
    }

    #[test]
    fn test_scalar_select() {
        todo!()
    }
}
