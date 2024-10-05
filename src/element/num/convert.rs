use super::*;

pub trait ElementNumConvert:
    Element
    + NumCast
    + ElementFromNum<f32>
    + ElementFromNum<f64>
    + ElementFromNum<u8>
    + ElementFromNum<u16>
    + ElementFromNum<u32>
    + ElementFromNum<u64>
    + ElementFromNum<u128>
    + ElementFromNum<usize>
    + ElementFromNum<i8>
    + ElementFromNum<i16>
    + ElementFromNum<i32>
    + ElementFromNum<i64>
    + ElementFromNum<i128>
    + ElementFromNum<isize>
{
    fn as_num<N: NumElement>(self) -> N;
    fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N>;
    fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N>;
    fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N>;
}
pub trait ElementFromNum<N: NumElement>: Element {
    fn from_num(value: N) -> Self;
    fn vec2_from_num(value: Vec2<N>) -> Vec2<Self>;
    fn vec3_from_num(value: Vec3<N>) -> Vec3<Self>;
    fn vec4_from_num(value: Vec4<N>) -> Vec4<Self>;
}

macro_rules! impl_trait {
    ($ty:ident) => {
        impl ElementNumConvert for $ty {
            fn as_num<N: NumElement>(self) -> N {
                N::from_num(self)
            }
            fn vec2_as_num<N: NumElement>(value: Vec2<Self>) -> Vec2<N> {
                N::vec2_from_num(value)
            }
            fn vec3_as_num<N: NumElement>(value: Vec3<Self>) -> Vec3<N> {
                N::vec3_from_num(value)
            }
            fn vec4_as_num<N: NumElement>(value: Vec4<Self>) -> Vec4<N> {
                N::vec4_from_num(value)
            }
        }
    };
}
impl_trait!(f32);
impl_trait!(f64);
impl_trait!(u8);
impl_trait!(u16);
impl_trait!(u32);
impl_trait!(u64);
impl_trait!(u128);
impl_trait!(usize);
impl_trait!(i8);
impl_trait!(i16);
impl_trait!(i32);
impl_trait!(i64);
impl_trait!(i128);
impl_trait!(isize);
