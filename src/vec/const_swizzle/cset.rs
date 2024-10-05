use gomath_proc_macros::vec_cset_wrappers;

use super::*;

vec_cset_wrappers!(Vec2: x, y);
vec_cset_wrappers!(Vec3: x, y, z);
vec_cset_wrappers!(Vec4: x, y, z, w);

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    pub unsafe fn cset<const X: usize>(&mut self, value: T) {
        *self.get_unchecked_mut(X) = value;
    }
    pub unsafe fn cset2<const X: usize, const Y: usize>(&mut self, value: Vec2<T>) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
    }
    pub unsafe fn cset3<const X: usize, const Y: usize, const Z: usize>(&mut self, value: Vec3<T>) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
        *self.get_unchecked_mut(Z) = value.z();
    }
    pub unsafe fn cset4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        &mut self,
        value: Vec4<T>,
    ) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
        *self.get_unchecked_mut(Z) = value.z();
        *self.get_unchecked_mut(W) = value.w();
    }
}
