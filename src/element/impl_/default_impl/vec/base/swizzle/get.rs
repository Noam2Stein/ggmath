use vec::swizzle::ElementVecGet;

use super::*;

impl<const N: usize, T: ElementDefaultImpl> ElementVecGet<N> for T
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn vec_get(vec: vec::inner::InnerVecN<N, Self>, index: usize) -> Result<Self, &'static str> {
        Ok(match vec.get(index) {
            Some(some) => *some,
            None => return Err("x is out of vec2 bounds"),
        })
    }
    fn vec_get2(
        vec: vec::inner::InnerVecN<N, Self>,
        indicies: [usize; 2],
    ) -> Result<Self::InnerVec2, &'static str> {
        Ok([
            match vec.get(indicies[0]) {
                Some(some) => *some,
                None => return Err("x is out of vec2 bounds"),
            },
            match vec.get(indicies[1]) {
                Some(some) => *some,
                None => return Err("y is out of vec2 bounds"),
            },
        ])
    }
    fn vec_get3(
        vec: vec::inner::InnerVecN<N, Self>,
        indicies: [usize; 3],
    ) -> Result<Self::InnerVec3, &'static str> {
        Ok([
            match vec.get(indicies[0]) {
                Some(some) => *some,
                None => return Err("x is out of vec2 bounds"),
            },
            match vec.get(indicies[1]) {
                Some(some) => *some,
                None => return Err("y is out of vec2 bounds"),
            },
            match vec.get(indicies[2]) {
                Some(some) => *some,
                None => return Err("z is out of vec2 bounds"),
            },
            T::default(),
        ])
    }
    fn vec_get4(
        vec: vec::inner::InnerVecN<N, Self>,
        indicies: [usize; 4],
    ) -> Result<Self::InnerVec4, &'static str> {
        Ok([
            match vec.get(indicies[0]) {
                Some(some) => *some,
                None => return Err("x is out of vec2 bounds"),
            },
            match vec.get(indicies[1]) {
                Some(some) => *some,
                None => return Err("y is out of vec2 bounds"),
            },
            match vec.get(indicies[2]) {
                Some(some) => *some,
                None => return Err("z is out of vec2 bounds"),
            },
            match vec.get(indicies[3]) {
                Some(some) => *some,
                None => return Err("w is out of vec2 bounds"),
            },
        ])
    }

    unsafe fn vec_get_unchecked(vec: vec::inner::InnerVecN<N, Self>, index: usize) -> Self {
        *vec.get_unchecked(index)
    }
    unsafe fn vec_get2_unchecked(
        vec: vec::inner::InnerVecN<N, Self>,
        indicies: [usize; 2],
    ) -> Self::InnerVec2 {
        indicies.map(|i| vec.get_unchecked(indicies[i]))
    }
    unsafe fn vec_get3_unchecked(
        vec: vec::inner::InnerVecN<N, Self>,
        indicies: [usize; 3],
    ) -> Self::InnerVec3 {
        [
            *vec.get_unchecked(indicies[0]),
            *vec.get_unchecked(indicies[1]),
            *vec.get_unchecked(indicies[2]),
            T::default(),
        ]
    }
    unsafe fn vec_get4_unchecked(
        vec: vec::inner::InnerVecN<N, Self>,
        indicies: [usize; 4],
    ) -> Self::InnerVec4 {
        indicies.map(|i| vec.get_unchecked(indicies[i]))
    }
}
