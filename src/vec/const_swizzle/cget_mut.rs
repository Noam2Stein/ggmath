use std::mem::transmute;

use gomath_proc_macros::vec_cget_mut_wrappers;

use super::*;

vec_cget_mut_wrappers!(Vec2: x, y);
vec_cget_mut_wrappers!(Vec3: x, y, z);
vec_cget_mut_wrappers!(Vec4: x, y, z, w);

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    pub unsafe fn cget_mut<const V0: usize>(&mut self) -> &mut T {
        self.get_unchecked_mut(V0)
    }
    pub unsafe fn cget_mut2<const V0: usize>(&mut self) -> &mut Vec2<T> {
        transmute(self.get_unchecked_mut(V0))
    }
    pub unsafe fn cget_mut3<const V0: usize>(&mut self) -> &mut Vec3<T> {
        transmute(self.get_unchecked_mut(V0))
    }
    pub unsafe fn cget_mut4<const V0: usize>(&mut self) -> &mut Vec4<T> {
        transmute(self.get_unchecked_mut(V0))
    }

    pub unsafe fn cget_mut_1_1<const V0: usize, const V1: usize>(&mut self) -> (&mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
        )
    }
    pub unsafe fn cget_mut_2_1<const V0: usize, const V1: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
        )
    }
    pub unsafe fn cget_mut_3_1<const V0: usize, const V1: usize>(
        &mut self,
    ) -> (&mut Vec3<T>, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
        )
    }
    pub unsafe fn cget_mut_1_2<const V0: usize, const V1: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
        )
    }
    pub unsafe fn cget_mut_2_2<const V0: usize, const V1: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut Vec2<T>) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
        )
    }
    pub unsafe fn cget_mut_1_3<const V0: usize, const V1: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec3<T>) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
        )
    }

    pub unsafe fn cget_mut_1_1_1<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
            transmute(self.get_unchecked_mut(V2)),
        )
    }
    pub unsafe fn cget_mut_2_1_1<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
            transmute(self.get_unchecked_mut(V2)),
        )
    }
    pub unsafe fn cget_mut_1_2_1<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
            transmute(self.get_unchecked_mut(V2)),
        )
    }
    pub unsafe fn cget_mut_1_1_2<const V0: usize, const V1: usize, const V2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut Vec2<T>) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
            transmute(self.get_unchecked_mut(V2)),
        )
    }

    pub unsafe fn cget_mut_1_1_1_1<
        const V0: usize,
        const V1: usize,
        const V2: usize,
        const V3: usize,
    >(
        &mut self,
    ) -> (&mut T, &mut T, &mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(V0)),
            transmute(self.get_unchecked_mut(V1)),
            transmute(self.get_unchecked_mut(V2)),
            transmute(self.get_unchecked_mut(V3)),
        )
    }
}
