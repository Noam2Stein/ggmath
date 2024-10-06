use vec::swizzle::ElementVecWith;

use super::*;

impl<const N: usize, T: ElementDefaultImpl> ElementVecWith<N> for T
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn vec_with(
        vec: vec::inner::InnerVec<N, Self>,
        index: usize,
        value: Self,
    ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = value,
            None => return Err("x is out of vec2 bounds"),
        };
        Ok(vec)
    }
    fn vec_with2(
        vec: vec::inner::InnerVec<N, Self>,
        indicies: [usize; 2],
        value: Self::InnerVec2,
    ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
        match vec.get_mut(x) {
            Some(some) => *some = unsafe { T::vec2_cget::<0>(value) },
            None => return Err("x is out of vec2 bounds"),
        };
        match vec.get_mut(y) {
            Some(some) => *some = unsafe { T::vec2_cget::<1>(value) },
            None => return Err("y is out of vec2 bounds"),
        };
        Ok(vec)
    }
    fn vec_with3(
        vec: vec::inner::InnerVec<N, Self>,
        indicies: [usize; 3],
        value: Self::InnerVec3,
    ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
        if x < 3 {
            *unsafe { vec.get_unchecked_mut(x) } = value;
        } else {
            return Err("x is out of vec3 bounds");
        };
        Ok(vec)
    }
    fn vec_with4(
        vec: vec::inner::InnerVec<N, Self>,
        indicies: [usize; 4],
        value: Self::InnerVec4,
    ) -> Result<vec::inner::InnerVec<N, Self>, &'static str> {
        if x < 3 {
            *unsafe { vec.get_unchecked_mut(x) } = unsafe { T::vec2_cget::<0>(value) };
        } else {
            return Err("x is out of vec3 bounds");
        };
        if y < 3 {
            *unsafe { vec.get_unchecked_mut(y) } = unsafe { T::vec2_cget::<1>(value) };
        } else {
            return Err("y is out of vec3 bounds");
        };
        Ok(vec)
    }

    unsafe fn vec_with_unchecked(
        vec: Self::InnerVec4,
        index: usize,
        value: Self,
    ) -> Self::InnerVec4 {
    }
    unsafe fn vec_with2_unchecked(
        vec: Self::InnerVec4,
        indicies: [usize; 2],
        value: Self::InnerVec2,
    ) -> Self::InnerVec4 {
    }
    unsafe fn vec_with3_unchecked(
        vec: Self::InnerVec4,
        indicies: [usize; 3],
        value: Self::InnerVec3,
    ) -> Self::InnerVec4 {
    }
    unsafe fn vec_with4_unchecked(
        vec: Self::InnerVec4,
        indicies: [usize; 4],
        value: Self::InnerVec4,
    ) -> Self::InnerVec4 {
    }
}
