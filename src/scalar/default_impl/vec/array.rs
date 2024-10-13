use crate::vec::array::*;

use super::*;

impl<const N: usize, T: ScalarDefaultImpl> ScalarVecArrayApi<N, VecPacked> for T
where
    ScalarCount<N>: VecLen<N>,
{
    fn from_array(array: [Self; N]) -> InnerVector<N, VecPacked, Self> {
        array
    }
}
