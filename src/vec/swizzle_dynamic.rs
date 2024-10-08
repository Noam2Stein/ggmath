use std::mem::transmute;

use gomath_proc_macros::vec_api;

use super::*;

vec_api!(
    Get {
        pub fn get(self, index: usize) -> Result<T, &'static str>;
        pub fn get2(self, indicies: [usize; 2]) -> Result<Vec2<T>, &'static str>;
        pub fn get3(self, indicies: [usize; 3]) -> Result<Vec3<T>, &'static str>;
        pub fn get4(self, indicies: [usize; 4]) -> Result<Vec4<T>, &'static str>;

        pub unsafe fn get_unchecked(self, index: usize) -> T;
        pub unsafe fn get2_unchecked(self, indicies: [usize; 2]) -> Vec2<T>;
        pub unsafe fn get3_unchecked(self, indicies: [usize; 3]) -> Vec3<T>;
        pub unsafe fn get4_unchecked(self, indicies: [usize; 4]) -> Vec4<T>;
    }
);
pub(super) trait 

vec_api!(
    With {
        pub fn with(self, index: usize, value: T) -> Result<Self, &'static str>;
        pub fn with2(self, indicies: [usize; 2], value: Vec2<T>) -> Result<Self, &'static str>;
        pub fn with3(self, indicies: [usize; 3], value: Vec3<T>) -> Result<Self, &'static str>;
        pub fn with4(self, indicies: [usize; 4], value: Vec4<T>) -> Result<Self, &'static str>;

        pub unsafe fn with_unchecked(self, index: usize, value: T) -> Self;
        pub unsafe fn with2_unchecked(self, indicies: [usize; 2], value: Vec2<T>) -> Self;
        pub unsafe fn with3_unchecked(self, indicies: [usize; 3], value: Vec3<T>) -> Self;
        pub unsafe fn with4_unchecked(self, indicies: [usize; 4], value: Vec4<T>) -> Self;
    }
);

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_array_mut().get_mut(index)
    }
    #[inline(always)]
    pub fn get_mut_n<const N2: usize>(&mut self, index: usize) -> Option<&mut VecN<N2, T>>
    where
        MaybeVecNum<N2>: VecNum<N2>,
    {
        if index + N2 <= N {
            Some(unsafe { self.get_mut_n_unchecked(index) })
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn get_mut_n<const N2: usize>(&mut self, index: usize) -> Option<&mut VecN<N2, T>>
    where
        MaybeVecNum<N2>: VecNum<N2>,
    {
        if index + N2 <= N {
            Some(unsafe { self.get_mut_n_unchecked(index) })
        } else {
            None
        }
    }
    
    #[inline(always)]
    pub fn get_mut2(&mut self, index: usize) -> Option<&mut Vec2<T>> {
        self.get_mut_n(index)
    }
    #[inline(always)]
    pub fn get_mut3(&mut self, index: usize) -> Option<&mut Vec3<T>> {
        self.get_mut_n(index)
    }
    #[inline(always)]
    pub fn get_mut4(&mut self, index: usize) -> Option<&mut Vec4<T>> {
        self.get_mut_n(index)
    }

    #[inline(always)]
    pub unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut T {
        self.as_array_mut().get_unchecked_mut(index)
    }
    #[inline(always)]
    pub unsafe fn get_mut_n_unchecked<const N2: usize>(&mut self, index: usize) -> &mut T
    where
        MaybeVecNum<N2>: VecNum<N2>,
    {
        transmute(self.get_mut_unchecked(index))
    }
    #[inline(always)]
    pub unsafe fn get_mut2_unchecked(&mut self, index: usize) -> &mut Vec2<T> {
        self.get_mut_n_unchecked(index)
    }
    #[inline(always)]
    pub unsafe fn get_mut3_unchecked(&mut self, index: usize) -> &mut Vec3<T> {
        self.get_mut_n_unchecked(index)
    }
    #[inline(always)]
    pub unsafe fn get_mut4_unchecked(&mut self, index: usize) -> &mut Vec4<T> {
        self.get_mut_n_unchecked(index)
    }

    #[inline(always)]
    pub unsafe fn get_mut_1_1<const I0: usize, const I1: usize>(&mut self) -> (&mut T, &mut T) {
        (self.cget_mut::<I0>(), self.cget_mut::<I1>())
    }
    #[inline(always)]
    pub unsafe fn get_mut_2_1<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T) {
        (self.cget_mut2::<I0>(), self.cget_mut::<I1>())
    }
    #[inline(always)]
    pub unsafe fn get_mut_3_1<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut Vec3<T>, &mut T) {
        (self.cget_mut3::<I0>(), self.cget_mut::<I1>())
    }
    #[inline(always)]
    pub unsafe fn get_mut_1_2<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>) {
        (self.cget_mut::<I0>(), self.cget_mut2::<I1>())
    }
    #[inline(always)]
    pub unsafe fn get_mut_2_2<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut Vec2<T>) {
        (self.cget_mut2::<I0>(), self.cget_mut2::<I1>())
    }
    #[inline(always)]
    pub unsafe fn get_mut_1_3<const I0: usize, const I1: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec3<T>) {
        (self.cget_mut::<I0>(), self.cget_mut3::<I1>())
    }

    #[inline(always)]
    pub unsafe fn get_mut_1_1_1<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut T) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut::<I2>(),
        )
    }
    #[inline(always)]
    pub unsafe fn get_mut_2_1_1<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut Vec2<T>, &mut T, &mut T) {
        (
            self.cget_mut2::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut::<I2>(),
        )
    }
    #[inline(always)]
    pub unsafe fn get_mut_1_2_1<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut T, &mut Vec2<T>, &mut T) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut2::<I1>(),
            self.cget_mut::<I2>(),
        )
    }
    #[inline(always)]
    pub unsafe fn get_mut_1_1_2<const I0: usize, const I1: usize, const I2: usize>(
        &mut self,
    ) -> (&mut T, &mut T, &mut Vec2<T>) {
        (
            self.cget_mut::<I0>(),
            self.cget_mut::<I1>(),
            self.cget_mut2::<I2>(),
        )
    }

    #[inline(always)]
    pub unsafe fn get_mut_1_1_1_1<
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
