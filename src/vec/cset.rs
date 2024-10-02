use gomath_proc_macros::vec_cset_wrappers;

use super::*;

vec_cset_wrappers!(Vec2: x, y);
vec_cset_wrappers!(Vec3: x, y, z);
vec_cset_wrappers!(Vec4: x, y, z, w);

pub trait VecNCSet<T: Element, const N: usize>: VecNArray<T, N> {
    unsafe fn cset<const X: usize>(&mut self, value: T) {
        *self.get_unchecked_mut(X) = value;
    }
    unsafe fn cset2<const X: usize, const Y: usize>(&mut self, value: Vec2<T>) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
    }
    unsafe fn cset3<const X: usize, const Y: usize, const Z: usize>(&mut self, value: Vec3<T>) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
        *self.get_unchecked_mut(Z) = value.z();
    }
    unsafe fn cset4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        &mut self,
        value: Vec4<T>,
    ) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
        *self.get_unchecked_mut(Z) = value.z();
        *self.get_unchecked_mut(W) = value.w();
    }
}

impl<T: Element> VecNCSet<T, 2> for Vec2<T> {}
impl<T: Element> VecNCSet<T, 3> for Vec3<T> {}
impl<T: Element> VecNCSet<T, 4> for Vec4<T> {}
