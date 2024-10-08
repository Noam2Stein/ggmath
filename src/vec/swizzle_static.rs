use gomath_proc_macros::{
    vec_api, vec_cget_mut_wrappers, vec_cget_wrappers, vec_cset_wrappers, vec_cwith_wrappers,
};

use super::*;

vec_cget_wrappers!(Vec2: x, y);
vec_cget_wrappers!(Vec3: x, y, z);
vec_cget_wrappers!(Vec4: x, y, z, w);
vec_api!(
    ConstGet {
        unsafe fn cget<const X: usize>(self) -> T;
        unsafe fn cget2<const X: usize, const Y: usize>(self) -> Vec2<T>;
        unsafe fn cget3<const X: usize, const Y: usize, const Z: usize>(self) -> Vec3<T>;
        unsafe fn cget4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
            self,
        ) -> Vec4<T>;
    }
);

vec_cwith_wrappers!(Vec2: x, y);
vec_cwith_wrappers!(Vec3: x, y, z);
vec_cwith_wrappers!(Vec4: x, y, z, w);
vec_api!(
    ConstWith {
         unsafe fn cwith<const X: usize>(self, value: T) -> Self;
        unsafe fn cwith2<const X: usize, const Y: usize>(self, value: Vec2<T>) -> Self;
         unsafe fn cwith3<const X: usize, const Y: usize, const Z: usize>(
            self,
            value: Vec3<T>,
        ) -> Self;
         unsafe fn cwith4<const X: usize, const Y: usize, const Z: usize, const W: usize>(
            self,
            value: Vec4<T>,
        ) -> Self;
    }
);

vec_cget_mut_wrappers!(Vec2: x, y);
vec_cget_mut_wrappers!(Vec3: x, y, z);
vec_cget_mut_wrappers!(Vec4: x, y, z, w);
impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    pub unsafe fn cget_mut<const I0: usize>(&mut self) -> &mut T {
        self.get_unchecked_mut(I0)
    }
    #[inline(always)]
    pub unsafe fn cget_mut2<const I0: usize>(&mut self) -> &mut Vec2<T> {
        transmute(self.get_unchecked_mut(I0))
    }
    #[inline(always)]
    pub unsafe fn cget_mut3<const I0: usize>(&mut self) -> &mut Vec3<T> {
        transmute(self.get_unchecked_mut(I0))
    }
    #[inline(always)]
    pub unsafe fn cget_mut4<const I0: usize>(&mut self) -> &mut Vec4<T> {
        transmute(self.get_unchecked_mut(I0))
    }

    #[inline(always)]
    pub unsafe fn cget_mut_1_1<const I0: usize, const I1: usize>(&mut self) -> (&mut T, &mut T) {
        (self.cget_mut::<I0>(), self.cget_mut::<I1>())
    }
    #[inline(always)]
    pub unsafe fn cget_mut_2_1<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T) {
        (self.cget_mut2::<I0>(), self.cget_mut::<I1>())
    }
    #[inline(always)]
    pub unsafe fn cget_mut_3_1<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut Vec3<T>, &mut T) {
        (self.cget_mut3::<I0>(), self.cget_mut::<I1>())
    }
    #[inline(always)]
    pub unsafe fn cget_mut_1_2<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>) {
        (self.cget_mut::<I0>(), self.cget_mut2::<I1>())
    }
    #[inline(always)]
    pub unsafe fn cget_mut_2_2<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut Vec2<T>) {
        (self.cget_mut2::<I0>(), self.cget_mut2::<I1>())
    }
    #[inline(always)]
    pub unsafe fn cget_mut_1_3<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec3<T>) {
        (self.cget_mut::<I0>(), self.cget_mut3::<I1>())
    }

    #[inline(always)]
    pub unsafe fn cget_mut_1_1_1<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut T) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut::<I2>(),
        )
    }
    #[inline(always)]
    pub unsafe fn cget_mut_2_1_1<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T, &mut T) {
        (
            self.cget_mut2::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut::<I2>(),
        )
    }
    #[inline(always)]
    pub unsafe fn cget_mut_1_2_1<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>, &mut T) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut2::<I1>(),
            self.cget_mut::<I2>(),
        )
    }
    #[inline(always)]
    pub unsafe fn cget_mut_1_1_2<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut Vec2<T>) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut2::<I2>(),
        )
    }

    #[inline(always)]
    pub unsafe fn cget_mut_1_1_1_1<
        const I0: usize,
        const I1: usize,
        const I2: usize,
        const I3: usize,
    >(
        &mut self,
    ) -> (&mut T, &mut T, &mut T, &mut T) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut::<I2>(),
            self.cget_mut::<I3>(),
        )
    }
}

vec_cset_wrappers!(Vec2: x, y);
vec_cset_wrappers!(Vec3: x, y, z);
vec_cset_wrappers!(Vec4: x, y, z, w);
impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    pub unsafe fn cset<const X: usize>(&mut self, value: T) {
        *self.get_unchecked_mut(X) = value;
    }
    #[inline(always)]
    pub unsafe fn cset2<const X: usize, const Y: usize>(&mut self, value: Vec2<T>) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
    }
    #[inline(always)]
    pub unsafe fn cset3<const X: usize, const Y: usize, const Z: usize>(&mut self, value: Vec3<T>) {
        *self.get_unchecked_mut(X) = value.x();
        *self.get_unchecked_mut(Y) = value.y();
        *self.get_unchecked_mut(Z) = value.z();
    }
    #[inline(always)]
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
