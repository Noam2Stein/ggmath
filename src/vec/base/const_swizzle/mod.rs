use super::*;

mod cget;
mod cget_mut;
mod cset;
mod cwith;
pub use cget::*;
pub use cget_mut::*;
pub use cset::*;
pub use cwith::*;

pub trait VecNConstSwizzle<T: Element, const N: usize>:
    VecNConstGet<T, N> + VecNConstGetMut<T, N> + VecNConstSet<T, N> + VecNConstWith<T, N>
{
}
impl<T: Element> VecNConstSwizzle<T, 2> for Vec2<T> {}
impl<T: Element> VecNConstSwizzle<T, 3> for Vec3<T> {}
impl<T: Element> VecNConstSwizzle<T, 4> for Vec4<T> {}

pub trait ElementVecConstSwizzle: ElementVecConstGet + ElementVecConstWith {}
