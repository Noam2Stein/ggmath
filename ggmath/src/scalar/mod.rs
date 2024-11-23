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
        if index >= N {
            None
        } else {
            Some(unsafe { vec.get_unchecked(index) })
        }
    }
    #[inline(always)]
    fn vector_get_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
    ) -> Option<Vector<2, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else {
            Some(unsafe { vec.get_2_unchecked(indicies) })
        }
    }
    #[inline(always)]
    fn vector_get_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
    ) -> Option<Vector<3, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else {
            Some(unsafe { vec.get_3_unchecked(indicies) })
        }
    }
    #[inline(always)]
    fn vector_get_1_1_1_1<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
    ) -> Option<Vector<4, Self, A>>
    where
        ScalarCount<N>: VecLen,
    {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else {
            Some(unsafe { vec.get_4_unchecked(indicies) })
        }
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
        Vector::from_array([
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
        Vector::from_array([
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
        Vector::from_array([
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
    ) -> Option<Self>
    where
        ScalarCount<N>: VecLen,
    {
        if index >= N {
            None
        } else {
            Some(unsafe { vec.with_unchecked(index, value) })
        }
    }
    fn vector_with_2<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 2],
        values: Vector<2, Self, A>,
    ) -> Option<Self> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else {
            Some(unsafe { vec.with_2_unchecked(indicies, values) })
        }
    }
    fn vector_with_3<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 3],
        values: Vector<3, Self, A>,
    ) -> Option<Self> {
        if indicies.into_iter().any(|index| index >= N) {
            None
        } else if indicies[0] == indicies[1] {
            None
        } else if indicies[0] == indicies[2] {
            None
        } else if indicies[1] == indicies[2] {
            None
        } else {
            Some(unsafe { vec.with_3_unchecked(indicies, values) })
        }
    }
    fn vector_with_4<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        indicies: [usize; 4],
        values: Vector<4, Self, A>,
    ) -> Option<Self> {
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
            Some(unsafe { vec.with_4_unchecked(indicies, values) })
        }
    }

    #[inline(always)]
    unsafe fn vector_with_unchecked<const N: usize, A: VecAlignment>(
        mut self,
        index: usize,
        value: Self,
    ) -> Self {
        *self.get_mut_unchecked(index) = value;
        self
    }
    #[inline(always)]
    unsafe fn vector_with_2_unchecked<const N: usize, A: VecAlignment>(
        mut self,
        indicies: [usize; 2],
        values: Vector<2, Self, A>,
    ) -> Vector<2, Self, A> {
        *self.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
        *self.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
        self
    }
    #[inline(always)]
    unsafe fn vector_with_3_unchecked<const N: usize, A: VecAlignment>(
        mut self,
        indicies: [usize; 3],
        values: Vector<3, Self, A>,
    ) -> Vector<3, Self, A> {
        *self.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
        *self.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
        *self.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
        self
    }
    #[inline(always)]
    unsafe fn vector_with_4_unchecked<const N: usize, A: VecAlignment>(
        mut self,
        indicies: [usize; 4],
        values: Vector<4, Self, A>,
    ) -> Vector<4, Self, A> {
        *self.get_mut_unchecked(indicies[0]) = values.get_unchecked(0);
        *self.get_mut_unchecked(indicies[1]) = values.get_unchecked(1);
        *self.get_mut_unchecked(indicies[2]) = values.get_unchecked(2);
        *self.get_mut_unchecked(indicies[3]) = values.get_unchecked(3);
        self
    }
}
