use super::*;

// Comparison

impl<T: Scalar + PartialEq<T2>, A: VecAlignment, T2: Scalar, A2: VecAlignment>
    PartialEq<Quaternion<T2, A2>> for Quaternion<T, A>
{
    #[inline(always)]
    fn eq(&self, other: &Quaternion<T2, A2>) -> bool {
        self.inner == other.inner
    }
}
