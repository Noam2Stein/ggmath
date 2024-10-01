use gomath_proc_macros::impl_vec_set_shortnames;

use super::*;

impl_vec_set_shortnames!(Vec2: x, y);
impl_vec_set_shortnames!(Vec3: x, y, z);
impl_vec_set_shortnames!(Vec4: x, y, z, w);

pub trait VecNSet<T: Element, const N: usize>: VecNArray<T, N> {
    fn set<const X: usize>(&mut self, value: T) {
        self[X] = value;
    }
    fn set2<const X: usize, const Y: usize>(&mut self, value: Vec2<T>) {
        self[X] = value.cget::<0>();
        self[Y] = value.cget::<1>();
    }
    fn set3<const X: usize, const Y: usize, const Z: usize>(&mut self, value: Vec3<T>) {
        self[X] = value.cget::<0>();
        self[Y] = value.cget::<1>();
        self[Z] = value.cget::<2>();
    }
    fn set4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
        &mut self,
        value: Vec4<T>,
    ) {
        self[X] = value.cget::<0>();
        self[Y] = value.cget::<1>();
        self[Z] = value.cget::<2>();
        self[W] = value.cget::<3>();
    }
}

impl<T: Element> VecNSet<T, 2> for Vec2<T> {}
impl<T: Element> VecNSet<T, 3> for Vec3<T> {}
impl<T: Element> VecNSet<T, 4> for Vec4<T> {}
