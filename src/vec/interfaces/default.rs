use crate::vec::{ScalarCount, VecAlignment, VecLen, Vector};

ggmath_proc_macros::vec_interface!(
    ScalarDefault: Scalar + Default,
    VecLenDefault,
    VecAlignmentDefault,

    Default:

    fn default() -> Self {
        Self::from_array([<T as Default>::default(); N])
    }
);

impl<const N: usize, T: ScalarDefault, A: VecAlignment> Default for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    fn default() -> Self {
        Self::default()
    }
}
