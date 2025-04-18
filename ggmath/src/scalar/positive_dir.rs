use super::*;

pub trait PositiveDir: Scalar {
    const POSITIVE_DIR_X: PositiveDirX;
    const POSITIVE_DIR_Y: PositiveDirY;
    const POSITIVE_DIR_Z: PositiveDirZ;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositiveDirX {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositiveDirY {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PositiveDirZ {
    Forward,
    Backward,
}

macro_rules! prim_impl {
    ($type:ty) => {
        #[cfg(any(feature = "positive_right", feature = "positive_left"))]
        #[cfg(any(feature = "positive_up", feature = "positive_down"))]
        #[cfg(any(feature = "positive_forward", feature = "positive_backward"))]
        impl PositiveDir for $type {
            const POSITIVE_DIR_X: PositiveDirX = {
                #[cfg(feature = "positive_right")]
                return PositiveDirX::Right;

                #[cfg(feature = "positive_left")]
                return PositiveDirX::Left;
            };

            const POSITIVE_DIR_Y: PositiveDirY = {
                #[cfg(feature = "positive_up")]
                return PositiveDirY::Up;

                #[cfg(feature = "positive_down")]
                return PositiveDirY::Down;
            };

            const POSITIVE_DIR_Z: PositiveDirZ = {
                #[cfg(feature = "positive_forward")]
                return PositiveDirZ::Forward;

                #[cfg(feature = "positive_backward")]
                return PositiveDirZ::Backward;
            };
        }
    };
}
prim_impl!(u8);
prim_impl!(u16);
prim_impl!(u32);
prim_impl!(u64);
prim_impl!(u128);
prim_impl!(usize);
prim_impl!(i8);
prim_impl!(i16);
prim_impl!(i32);
prim_impl!(i64);
prim_impl!(i128);
prim_impl!(isize);
prim_impl!(f32);
prim_impl!(f64);
