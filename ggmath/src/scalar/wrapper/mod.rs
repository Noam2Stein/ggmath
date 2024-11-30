use std::mem::transmute_copy;

use super::*;

pub use ggmath_proc_macros::WrapperScalar;

pub unsafe trait WrapperScalar: Construct {
    type InnerScalar: Scalar;

    fn wrap(inner: Self::InnerScalar) -> Self {
        unsafe { transmute_copy(&inner) }
    }

    fn unwrap(self) -> Self::InnerScalar {
        unsafe { transmute_copy(&self) }
    }
}

unsafe impl<T: WrapperScalar> ScalarInnerAlignedVecs for T {
    type InnerAlignedVec2 = <T::InnerScalar as ScalarInnerAlignedVecs>::InnerAlignedVec2;
    type InnerAlignedVec4 = <T::InnerScalar as ScalarInnerAlignedVecs>::InnerAlignedVec4;
}
impl<T: WrapperScalar> Scalar for T {
    fn vector_splat<const N: usize, A: VecAlignment>(value: Self) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::wrap(Vector::splat(value.unwrap()))
    }

    fn vector_get<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
    ) -> Option<Self>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap().get(index).map(|output| T::wrap(output))
    }
    fn vector_get_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
    ) -> Option<Vector<2, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .get_1_1(indicies)
            .map(|output| Vector::<2, Self, A>::wrap(output))
    }
    fn vector_get_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
    ) -> Option<Vector<3, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .get_1_1_1(indicies)
            .map(|output| Vector::<3, Self, A>::wrap(output))
    }
    fn vector_get_1_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
    ) -> Option<Vector<4, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .get_1_1_1_1(indicies)
            .map(|output| Vector::<4, Self, A>::wrap(output))
    }

    unsafe fn vector_get_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
    ) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        Self::wrap(vec.unwrap().get_unchecked(index))
    }
    unsafe fn vector_get_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
    ) -> Vector<2, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::<2, Self, A>::wrap(vec.unwrap().get_1_1_unchecked(indicies))
    }
    unsafe fn vector_get_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
    ) -> Vector<3, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::<3, Self, A>::wrap(vec.unwrap().get_1_1_1_unchecked(indicies))
    }
    unsafe fn vector_get_1_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
    ) -> Vector<4, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::<4, Self, A>::wrap(vec.unwrap().get_1_1_1_1_unchecked(indicies))
    }

    fn vector_with<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
        value: Self,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .with(index, value.unwrap())
            .map(|output| Vector::wrap(output))
    }
    fn vector_with_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
        value: Vector<2, Self, impl VecAlignment>,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .with_1_1(indicies, value.unwrap())
            .map(|output| Vector::wrap(output))
    }
    fn vector_with_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
        value: Vector<3, Self, impl VecAlignment>,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .with_1_1_1(indicies, value.unwrap())
            .map(|output| Vector::wrap(output))
    }
    fn vector_with_1_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
        value: Vector<4, Self, impl VecAlignment>,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.unwrap()
            .with_1_1_1_1(indicies, value.unwrap())
            .map(|output| Vector::wrap(output))
    }

    unsafe fn vector_with_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
        value: Self,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::wrap(vec.unwrap().with_unchecked(index, value.unwrap()))
    }
    unsafe fn vector_with_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
        value: Vector<2, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::wrap(vec.unwrap().with_1_1_unchecked(indicies, value.unwrap()))
    }
    unsafe fn vector_with_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
        value: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::wrap(vec.unwrap().with_1_1_1_unchecked(indicies, value.unwrap()))
    }
    unsafe fn vector_with_1_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
        value: Vector<4, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::wrap(
            vec.unwrap()
                .with_1_1_1_1_unchecked(indicies, value.unwrap()),
        )
    }
}

impl<const N: usize, T: WrapperScalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn wrap(inner: Vector<N, T::InnerScalar, A>) -> Self {
        unsafe { transmute_copy(&inner) }
    }

    pub fn unwrap(self) -> Vector<N, T::InnerScalar, A> {
        unsafe { transmute_copy(&self) }
    }
}
