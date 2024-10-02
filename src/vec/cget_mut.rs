use gomath_proc_macros::impl_vec_cget_mut_shortnames;

use super::*;

impl_vec_cget_mut_shortnames!(Vec2: x, y);
impl_vec_cget_mut_shortnames!(Vec3: x, y, z);
impl_vec_cget_mut_shortnames!(Vec4: x, y, z, w);

pub trait VecNCGetMut<T: Element, const N: usize>: VecNArray<T, N> {
    fn cget_mut<const V0: usize>(&mut self) -> &mut T {
        &mut self[V0]
    }
    fn cget_mut2<const V0: usize>(&mut self) -> &mut Vec2<T> {
        assert!(N >= V0 + 2);
        unsafe { std::mem::transmute(self.get_unchecked_mut(V0)) }
    }
    fn cget_mut3<const V0: usize>(&mut self) -> &mut Vec3<T> {
        assert!(N >= V0 + 3);
        unsafe { std::mem::transmute(self.get_unchecked_mut(V0)) }
    }
    fn cget_mut4<const V0: usize>(&mut self) -> &mut Vec4<T> {
        assert!(N >= V0 + 4);
        unsafe { std::mem::transmute(self.get_unchecked_mut(V0)) }
    }

    fn cget_mut_1_1<const V0: usize, const V1: usize>(&mut self) -> (&mut T, &mut T) {
        assert!(N >= V0 + 1 + V1 + 1);
        unsafe { std::mem::transmute((self.get_unchecked(V0), self.get_unchecked(V0 + 1 + V1))) }
    }
    fn cget_mut_2_1<const V0: usize, const V1: usize>(&mut self) -> (&mut Vec2<T>, &mut T) {
        assert!(N >= V0 + 2 + V1 + 1);
        unsafe { std::mem::transmute((self.get_unchecked(V0), self.get_unchecked(V0 + 2 + V1))) }
    }
    fn cget_mut_3_1<const V0: usize, const V1: usize>(&mut self) -> (&mut Vec3<T>, &mut T) {
        assert!(N >= V0 + 3 + V1 + 1);
        unsafe { std::mem::transmute((self.get_unchecked(V0), self.get_unchecked(V0 + 3 + V1))) }
    }
    fn cget_mut_1_2<const V0: usize, const V1: usize>(&mut self) -> (&mut T, &mut Vec2<T>) {
        assert!(N >= V0 + 1 + V1 + 2);
        unsafe { std::mem::transmute((self.get_unchecked(V0), self.get_unchecked(V0 + 1 + V1))) }
    }
    fn cget_mut_2_2<const V0: usize, const V1: usize>(&mut self) -> (&mut Vec2<T>, &mut Vec2<T>) {
        assert!(N >= V0 + 2 + V1 + 2);
        unsafe { std::mem::transmute((self.get_unchecked(V0), self.get_unchecked(V0 + 2 + V1))) }
    }
    fn cget_mut_1_3<const V0: usize, const V1: usize>(&mut self) -> (&mut T, &mut Vec3<T>) {
        assert!(N >= V0 + 1 + V1 + 3);
        unsafe { std::mem::transmute((self.get_unchecked(V0), self.get_unchecked(V0 + 1 + V1))) }
    }

    fn cget_mut_1_1_1<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut T) {
        assert!(N >= V0 + 1 + V1 + 1 + V2 + 1);
        unsafe {
            std::mem::transmute((
                self.get_unchecked(V0),
                self.get_unchecked(V0 + 1 + V1),
                self.get_unchecked(V0 + 1 + V1 + 1 + V2),
            ))
        }
    }
    fn cget_mut_2_1_1<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T, &mut T) {
        assert!(N >= V0 + 2 + V1 + 1 + V2 + 1);
        unsafe {
            std::mem::transmute((
                self.get_unchecked(V0),
                self.get_unchecked(V0 + 2 + V1),
                self.get_unchecked(V0 + 2 + V1 + 1 + V2),
            ))
        }
    }
    fn cget_mut_1_2_1<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>, &mut T) {
        assert!(N >= V0 + 1 + V1 + 2 + V2 + 1);
        unsafe {
            std::mem::transmute((
                self.get_unchecked(V0),
                self.get_unchecked(V0 + 1 + V1),
                self.get_unchecked(V0 + 1 + V1 + 2 + V2),
            ))
        }
    }
    fn cget_mut_1_1_2<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut Vec2<T>) {
        assert!(N >= V0 + 1 + V1 + 1 + V2 + 2);
        unsafe {
            std::mem::transmute((
                self.get_unchecked(V0),
                self.get_unchecked(V0 + 1 + V1),
                self.get_unchecked(V0 + 1 + V1 + 1 + V2),
            ))
        }
    }

    fn cget_mut_1_1_1_1<const V0: usize, const V1: usize, const V2: usize, const V3: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut T, &mut T) {
        assert!(N >= V0 + 1 + V1 + 1 + V2 + 1 + V3 + 1);
        unsafe {
            std::mem::transmute((
                self.get_unchecked(V0),
                self.get_unchecked(V0 + 1 + V1),
                self.get_unchecked(V0 + 1 + V1 + 1 + V2),
                self.get_unchecked(V0 + 1 + V1 + 1 + V2 + 1 + V3),
            ))
        }
    }
}

impl<T: Element> VecNCGetMut<T, 2> for Vec2<T> {}
impl<T: Element> VecNCGetMut<T, 3> for Vec3<T> {}
impl<T: Element> VecNCGetMut<T, 4> for Vec4<T> {}
