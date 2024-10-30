ggmath_proc_macros::vec_interface!(
    ScalarDefault: Scalar + Default,

    Default:

    fn default() -> Self {
        Vector::from_array([<T as Default>::default(); N])
    }
);

use crate::vector::{ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: ScalarDefault, A: VecAlignment> Default for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    fn default() -> Self {
        Self::default()
    }
}
