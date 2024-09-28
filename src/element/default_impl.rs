use super::*;

pub trait ElementDefaultImpl:
    fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display
{
}
unsafe impl<T: ElementDefaultImpl> Element for T {
    type Vec2Inner = [T; 2];
    type Vec3Inner = [T; 4];
    type Vec4Inner = [T; 4];
}
