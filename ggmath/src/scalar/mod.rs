//! Scalars are mathamatical types that have magnitude but no direction.
//! - [f32] and [bool] are scalars.
//! - [Vec3](crate::vec::Vec3) is not a scalar.

use crate::{vector::*, *};

mod api;
mod impl_std;
mod num;
pub use api::*;
pub use impl_std::*;
pub use num::*;

pub use ggmath_proc_macros::inner_vectors;

mod primitive_impls;

pub unsafe trait ScalarInnerVectors {
    type InnerAlignedVec2: Construct;
    type InnerAlignedVec4: Construct;
}

pub trait Scalar: Construct + ScalarInnerVectors {
    #[inline(always)]
    fn vector_splat<const N: usize, A: VecAlignment>(value: Self) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_array([value; N])
    }

    #[inline(always)]
    fn vector_get<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
    ) -> Option<Self>
    where
        ScalarCount<N>: VecLen,
    {
        vec.get_ref(index).map(|output| *output)
    }
    #[inline(always)]
    fn vector_get_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
    ) -> Option<Vector<2, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.get_1_1_ref(indicies)
            .map(|(output0, output1)| Vector::<2, Self, A>::from_array([*output0, *output1]))
    }
    #[inline(always)]
    fn vector_get_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
    ) -> Option<Vector<3, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.get_1_1_1_ref(indicies)
            .map(|(output0, output1, output2)| {
                Vector::<3, Self, A>::from_array([*output0, *output1, *output2])
            })
    }
    #[inline(always)]
    fn vector_get_1_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
    ) -> Option<Vector<4, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        vec.get_1_1_1_1_ref(indicies)
            .map(|(output0, output1, output2, output3)| {
                Vector::<4, Self, A>::from_array([*output0, *output1, *output2, *output3])
            })
    }

    #[inline(always)]
    unsafe fn vector_get_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
    ) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        *vec.as_array().get_unchecked(index)
    }
    #[inline(always)]
    unsafe fn vector_get_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
    ) -> Vector<2, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::<2, Self, A>::from_array([
            vec.get_unchecked(indicies[0]),
            vec.get_unchecked(indicies[1]),
        ])
    }
    #[inline(always)]
    unsafe fn vector_get_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
    ) -> Vector<3, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::<3, Self, A>::from_array([
            vec.get_unchecked(indicies[0]),
            vec.get_unchecked(indicies[1]),
            vec.get_unchecked(indicies[2]),
        ])
    }
    #[inline(always)]
    unsafe fn vector_get_1_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
    ) -> Vector<4, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::<4, Self, A>::from_array([
            vec.get_unchecked(indicies[0]),
            vec.get_unchecked(indicies[1]),
            vec.get_unchecked(indicies[2]),
            vec.get_unchecked(indicies[3]),
        ])
    }

    fn vector_with<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        index: usize,
        value: Self,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if index >= N {
            None
        } else {
            Some(unsafe { vec.with_unchecked(index, value) })
        }
    }
    fn vector_with_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
        value: Vector<2, Self, impl VecAlignment>,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else {
            Some(unsafe { vec.with_1_1_unchecked(indicies, value) })
        }
    }
    fn vector_with_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
        value: Vector<3, Self, impl VecAlignment>,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else {
            Some(unsafe { vec.with_1_1_1_unchecked(indicies, value) })
        }
    }
    fn vector_with_1_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
        value: Vector<4, Self, impl VecAlignment>,
    ) -> Option<Vector<N, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[0] == indicies[3] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else if indicies[1] == indicies[3] {
            None
        } else if indicies[2] == indicies[3] {
            None
        } else {
            Some(unsafe { vec.with_1_1_1_1_unchecked(indicies, value) })
        }
    }

    #[inline(always)]
    unsafe fn vector_with_unchecked<const N: usize, A: VecAlignment>(
        mut vec: Vector<N, Self, A>,
        index: usize,
        value: Self,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        *vec.get_mut_unchecked(index) = value;
        vec
    }
    #[inline(always)]
    unsafe fn vector_with_1_1_unchecked<const N: usize, A: VecAlignment>(
        mut vec: Vector<N, Self, A>,
        indicies: [usize; 2],
        value: Vector<2, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        *vec.get_mut_unchecked(indicies[0]) = value.get_unchecked(0);
        *vec.get_mut_unchecked(indicies[1]) = value.get_unchecked(1);
        vec
    }
    #[inline(always)]
    unsafe fn vector_with_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        mut vec: Vector<N, Self, A>,
        indicies: [usize; 3],
        value: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        *vec.get_mut_unchecked(indicies[0]) = value.get_unchecked(0);
        *vec.get_mut_unchecked(indicies[1]) = value.get_unchecked(1);
        *vec.get_mut_unchecked(indicies[2]) = value.get_unchecked(2);
        vec
    }
    #[inline(always)]
    unsafe fn vector_with_1_1_1_1_unchecked<const N: usize, A: VecAlignment>(
        mut vec: Vector<N, Self, A>,
        indicies: [usize; 4],
        value: Vector<4, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        *vec.get_mut_unchecked(indicies[0]) = value.get_unchecked(0);
        *vec.get_mut_unchecked(indicies[1]) = value.get_unchecked(1);
        *vec.get_mut_unchecked(indicies[2]) = value.get_unchecked(2);
        *vec.get_mut_unchecked(indicies[3]) = value.get_unchecked(3);
        vec
    }
}
