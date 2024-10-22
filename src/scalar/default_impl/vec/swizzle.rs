use std::mem::transmute;

use super::*;

impl<const N: usize, T: ScalarDefaultImpl, A: VecAlignment> ScalarVecSwizzleApi<N, A> for T
where
    ScalarCount<N>: VecLen<N>,
    T: ScalarVecArrayApi<2, A>,
    T: ScalarVecArrayApi<3, A>,
    T: ScalarVecArrayApi<4, A>,
{
    unsafe fn get_unchecked(vec: inner::InnerVector<N, Self, A>, index: usize) -> Self {
        let array: &[Self; N] = transmute(&vec);
        *array.get_unchecked(index)
    }
    unsafe fn get_2_unchecked(
        vec: inner::InnerVector<N, Self, A>,
        indicies: [usize; 2],
    ) -> crate::vec::inner::InnerVector<2, Self, A> {
        let array: &[Self; N] = transmute(&vec);
        <T as ScalarVecArrayApi<2, A>>::from_array(
            indicies.map(|index| *array.get_unchecked(index)),
        )
    }
    unsafe fn get_3_unchecked(
        vec: inner::InnerVector<N, Self, A>,
        indicies: [usize; 3],
    ) -> crate::vec::inner::InnerVector<3, Self, A> {
        let array: &[Self; N] = transmute(&vec);
        <T as ScalarVecArrayApi<3, A>>::from_array(
            indicies.map(|index| *array.get_unchecked(index)),
        )
    }
    unsafe fn get_4_unchecked(
        vec: inner::InnerVector<N, Self, A>,
        indicies: [usize; 4],
    ) -> crate::vec::inner::InnerVector<4, Self, A> {
        let array: &[Self; N] = transmute(&vec);
        <T as ScalarVecArrayApi<4, A>>::from_array(
            indicies.map(|index| *array.get_unchecked(index)),
        )
    }

    unsafe fn with_unchecked(
        vec: inner::InnerVector<N, Self, A>,
        index: usize,
        value: Self,
    ) -> crate::vec::inner::InnerVector<N, Self, A> {
    }
}
