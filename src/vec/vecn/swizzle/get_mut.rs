use std::mem::transmute;

use super::*;

pub trait VecNGetMut<T: Element, const N: usize>: VecNArray<T, N> {
    fn get_mut(&mut self, v0: usize) -> Result<&mut T, &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else {
            Ok(unsafe { self.get_unchecked_mut(v0) })
        }
    }
    fn get_mut2(&mut self, v0: usize) -> Result<&mut Vec2<T>, &'static str> {
        if v0 + 2 > N {
            Err("v0 outside of vec bounds")
        } else {
            Ok(unsafe { self.get_unchecked_mut2(v0) })
        }
    }
    fn get_mut3(&mut self, v0: usize) -> Result<&mut Vec3<T>, &'static str> {
        if v0 + 3 > N {
            Err("v0 outside of vec bounds")
        } else {
            Ok(unsafe { self.get_unchecked_mut3(v0) })
        }
    }
    fn get_mut4(&mut self, v0: usize) -> Result<&mut Vec4<T>, &'static str> {
        if v0 + 4 > N {
            Err("v0 outside of vec bounds")
        } else {
            Ok(unsafe { self.get_unchecked_mut4(v0) })
        }
    }

    fn get_mut_1_1(&mut self, v0: usize, v1: usize) -> Result<(&mut T, &mut T), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v1 == v0 {
            Err("v1 conflicts with v0")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                )
            })
        }
    }
    fn get_mut_2_1(
        &mut self,
        v0: usize,
        v1: usize,
    ) -> Result<(&mut Vec2<T>, &mut T), &'static str> {
        if v0 + 2 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v1 == v0 || v1 == v0 + 1 {
            Err("v1 conflicts with v0")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                )
            })
        }
    }
    fn get_mut_3_1(
        &mut self,
        v0: usize,
        v1: usize,
    ) -> Result<(&mut Vec3<T>, &mut T), &'static str> {
        if v0 + 3 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v1 == v0 || v1 == v0 + 1 || v1 == v0 + 2 {
            Err("v1 conflicts with v0")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                )
            })
        }
    }
    fn get_mut_1_2(
        &mut self,
        v0: usize,
        v1: usize,
    ) -> Result<(&mut T, &mut Vec2<T>), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 2 > N {
            Err("v1 outside of vec bounds")
        } else if v1 == v0 || v1 + 1 == v0 {
            Err("v1 conflicts with v0")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                )
            })
        }
    }
    fn get_mut_2_2(
        &mut self,
        v0: usize,
        v1: usize,
    ) -> Result<(&mut Vec2<T>, &mut Vec2<T>), &'static str> {
        if v0 + 2 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 2 > N {
            Err("v1 outside of vec bounds")
        } else if v1 == v0 || v1 + 1 == v0 || v1 == v0 + 1 {
            Err("v1 conflicts with v0")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                )
            })
        }
    }
    fn get_mut_1_3(
        &mut self,
        v0: usize,
        v1: usize,
    ) -> Result<(&mut T, &mut Vec3<T>), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 3 > N {
            Err("v1 outside of vec bounds")
        } else if v1 == v0 || v1 + 1 == v0 || v1 + 2 == v0 {
            Err("v1 conflicts with v0")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                )
            })
        }
    }

    fn get_mut_1_1_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> Result<(&mut T, &mut T, &mut T), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v2 + 1 > N {
            Err("v2 outside of vec bounds")
        } else if v1 == v0 {
            Err("v1 conflicts with v0")
        } else if v2 == v0 {
            Err("v2 conflicts with v0")
        } else if v2 == v1 {
            Err("v2 conflicts with v1")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                    transmute(self.get_unchecked_mut(v2)),
                )
            })
        }
    }
    fn get_mut_2_1_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> Result<(&mut Vec2<T>, &mut T, &mut T), &'static str> {
        if v0 + 2 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v2 + 1 > N {
            Err("v2 outside of vec bounds")
        } else if v1 == v0 || v1 == v0 + 1 {
            Err("v1 conflicts with v0")
        } else if v2 == v0 || v2 == v0 + 1 {
            Err("v2 conflicts with v0")
        } else if v2 == v1 {
            Err("v2 conflicts with v1")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                    transmute(self.get_unchecked_mut(v2)),
                )
            })
        }
    }
    fn get_mut_1_2_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> Result<(&mut T, &mut Vec2<T>, &mut T), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 2 > N {
            Err("v1 outside of vec bounds")
        } else if v2 + 1 > N {
            Err("v2 outside of vec bounds")
        } else if v1 == v0 || v1 + 1 == v0 {
            Err("v1 conflicts with v0")
        } else if v2 == v0 {
            Err("v2 conflicts with v0")
        } else if v2 == v1 || v2 == v1 + 1 {
            Err("v2 conflicts with v1")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                    transmute(self.get_unchecked_mut(v2)),
                )
            })
        }
    }
    fn get_mut_1_1_2(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> Result<(&mut T, &mut T, &mut Vec2<T>), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v2 + 2 > N {
            Err("v2 outside of vec bounds")
        } else if v1 == v0 {
            Err("v1 conflicts with v0")
        } else if v2 == v0 || v2 + 1 == v0 {
            Err("v2 conflicts with v0")
        } else if v2 == v1 || v2 + 1 == v1 {
            Err("v2 conflicts with v1")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                    transmute(self.get_unchecked_mut(v2)),
                )
            })
        }
    }

    fn get_mut_1_1_1_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
        v3: usize,
    ) -> Result<(&mut T, &mut T, &mut T, &mut T), &'static str> {
        if v0 + 1 > N {
            Err("v0 outside of vec bounds")
        } else if v1 + 1 > N {
            Err("v1 outside of vec bounds")
        } else if v2 + 1 > N {
            Err("v2 outside of vec bounds")
        } else if v3 + 1 > N {
            Err("v3 outside of vec bounds")
        } else if v1 == v0 {
            Err("v1 conflicts with v0")
        } else if v2 == v0 {
            Err("v2 conflicts with v0")
        } else if v3 == v0 {
            Err("v3 conflicts with v0")
        } else if v2 == v1 {
            Err("v2 conflicts with v1")
        } else if v3 == v1 {
            Err("v3 conflicts with v1")
        } else if v3 == v2 {
            Err("v3 conflicts with v2")
        } else {
            Ok(unsafe {
                (
                    transmute(self.get_unchecked_mut(v0)),
                    transmute(self.get_unchecked_mut(v1)),
                    transmute(self.get_unchecked_mut(v2)),
                    transmute(self.get_unchecked_mut(v3)),
                )
            })
        }
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, v0: usize) -> &mut T {
        self.as_array_mut().get_unchecked_mut(v0)
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut2(&mut self, v0: usize) -> &mut Vec2<T> {
        transmute(self.get_unchecked_mut(v0))
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut3(&mut self, v0: usize) -> &mut Vec3<T> {
        transmute(self.get_unchecked_mut(v0))
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut4(&mut self, v0: usize) -> &mut Vec4<T> {
        transmute(self.get_unchecked_mut(v0))
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut_1_1(&mut self, v0: usize, v1: usize) -> (&mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_2_1(&mut self, v0: usize, v1: usize) -> (&mut Vec2<T>, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_3_1(&mut self, v0: usize, v1: usize) -> (&mut Vec3<T>, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_1_2(&mut self, v0: usize, v1: usize) -> (&mut T, &mut Vec2<T>) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_2_2(
        &mut self,
        v0: usize,
        v1: usize,
    ) -> (&mut Vec2<T>, &mut Vec2<T>) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_1_3(&mut self, v0: usize, v1: usize) -> (&mut T, &mut Vec3<T>) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
        )
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut_1_1_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> (&mut T, &mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
            transmute(self.get_unchecked_mut(v2)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_2_1_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> (&mut Vec2<T>, &mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
            transmute(self.get_unchecked_mut(v2)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_1_2_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> (&mut T, &mut Vec2<T>, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
            transmute(self.get_unchecked_mut(v2)),
        )
    }
    #[inline(always)]
    unsafe fn get_unchecked_mut_1_1_2(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
    ) -> (&mut T, &mut T, &mut Vec2<T>) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
            transmute(self.get_unchecked_mut(v2)),
        )
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut_1_1_1_1(
        &mut self,
        v0: usize,
        v1: usize,
        v2: usize,
        v3: usize,
    ) -> (&mut T, &mut T, &mut T, &mut T) {
        (
            transmute(self.get_unchecked_mut(v0)),
            transmute(self.get_unchecked_mut(v1)),
            transmute(self.get_unchecked_mut(v2)),
            transmute(self.get_unchecked_mut(v3)),
        )
    }
}

impl<T: Element> VecNGetMut<T, 2> for Vec2<T> {}
impl<T: Element> VecNGetMut<T, 3> for Vec3<T> {}
impl<T: Element> VecNGetMut<T, 4> for Vec4<T> {}
