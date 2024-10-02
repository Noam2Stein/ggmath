use super::*;

mod get;
mod get_mut;
mod set;
mod with;
pub use get::*;
pub use get_mut::*;
pub use set::*;
pub use with::*;

pub trait VecNSwizzle<T: Element, const N: usize>:
    VecNGet<T, N> + VecNGetMut<T, N> + VecNSet<T, N> + VecNWith<T, N>
{
}
impl<T: Element> VecNSwizzle<T, 2> for Vec2<T> {}
impl<T: Element> VecNSwizzle<T, 3> for Vec3<T> {}
impl<T: Element> VecNSwizzle<T, 4> for Vec4<T> {}

pub trait ElementVecSwizzle: ElementVecGet + ElementVecWith {}
