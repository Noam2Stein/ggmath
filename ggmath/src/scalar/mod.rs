//! Scalars are mathamatical types that have magnitude but no direction.
//! - [f32] and [bool] are scalars.
//! - [Vec3](crate::vec::Vec3) is not a scalar.

use std::ops::*;

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
    // ********************************************************************************
    // ********************************************************************************
    // ************************************* Core *************************************
    // ********************************************************************************
    // ********************************************************************************

    #[inline(always)]
    fn vector_splat<const N: usize, A: VecAlignment>(value: Self) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_array([value; N])
    }

    // Vector: Get

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

    // Vector: With

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

    // ********************************************************************************
    // ********************************************************************************
    // ************************************* STD **************************************
    // ********************************************************************************
    // ********************************************************************************

    #[inline(always)]
    fn vector_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: Default,
    {
        Vector::splat(Self::default())
    }

    // Vector: Self Ops

    fn vector_neg<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Neg<Output: Scalar>,
    {
        vec.map(|c| c.neg())
    }

    fn vector_not<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Not<Output: Scalar>,
    {
        vec.map(|c| c.not())
    }

    // Vector: Rhs Ops

    fn vector_add<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Add<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].add(rhs[i]))
    }

    fn vector_bitand<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitAnd<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitand(rhs[i]))
    }

    fn vector_bitor<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitOr<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitor(rhs[i]))
    }

    fn vector_bitxor<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: BitXor<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].bitxor(rhs[i]))
    }

    fn vector_div<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Div<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].div(rhs[i]))
    }

    fn vector_mul<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Mul<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].mul(rhs[i]))
    }

    fn vector_rem<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Rem<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].rem(rhs[i]))
    }

    fn vector_shl<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Shl<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shl(rhs[i]))
    }

    fn vector_shr<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Shr<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].shr(rhs[i]))
    }

    fn vector_sub<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: Sub<TRhs, Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].sub(rhs[i]))
    }

    // Vector: Assign Ops

    fn vector_add_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: AddAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].add_assign(rhs[i]);
        }
    }

    fn vector_bitand_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitAndAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitand_assign(rhs[i]);
        }
    }

    fn vector_bitor_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitOrAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitor_assign(rhs[i]);
        }
    }

    fn vector_bitxor_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: BitXorAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].bitxor_assign(rhs[i]);
        }
    }

    fn vector_div_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: DivAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].div_assign(rhs[i]);
        }
    }

    fn vector_mul_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: MulAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].mul_assign(rhs[i]);
        }
    }

    fn vector_rem_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: RemAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].rem_assign(rhs[i]);
        }
    }

    fn vector_shl_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: ShlAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].shl_assign(rhs[i]);
        }
    }

    fn vector_shr_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: ShrAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].shr_assign(rhs[i]);
        }
    }

    fn vector_sub_assign<const N: usize, TRhs: Scalar>(
        vec: &mut Vector<N, Self, impl VecAlignment>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) where
        ScalarCount<N>: VecLen,
        Self: SubAssign<TRhs>,
    {
        for i in 0..N {
            vec[i].sub_assign(rhs[i]);
        }
    }

    // ********************************************************************************
    // ********************************************************************************
    // ************************************* API **************************************
    // ********************************************************************************
    // ********************************************************************************
    #[inline(always)]
    fn vector_csum<const N: usize>(vec: Vector<N, Self, impl VecAlignment>) -> Self
    where
        Self: Add<Output = Self>,
        ScalarCount<N>: VecLen,
    {
        match vec.resolve_length() {
            LengthResolvedVector::Vec2(vec) => vec.x() + vec.y(),
            LengthResolvedVector::Vec3(vec) => vec.x() + vec.y() + vec.z(),
            LengthResolvedVector::Vec4(vec) => vec.x() + vec.y() + vec.z() + vec.w(),
        }
    }

    #[inline(always)]
    fn vector_dot<const N: usize>(
        vec: Vector<N, Self, impl VecAlignment>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        Self: Mul<Output = Self> + Add<Output = Self>,
        ScalarCount<N>: VecLen,
    {
        (vec * other).csum()
    }

    #[inline(always)]
    fn vector_cross<A: VecAlignment>(
        vec: Vector<3, Self, A>,
        other: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<3, Self, A>
    where
        Self: Mul<Output = Self> + Sub<Output = Self>,
    {
        (vec.zxy() * other - vec * other.zxy()).zxy()
    }
}
