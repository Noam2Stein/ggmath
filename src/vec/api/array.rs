use std::mem::{transmute, transmute_copy};

ggmath_proc_macros::vec_api!(
    /// ```Vector``` API for array conversions
    Array:

    /// Constructs a ```Vector``` from an array of scalars.
    fn from_array(array: [T; N]) -> Self;

    /// Converts the ```Vector``` into an array of scalars.
    #[inline(always)]
    fn into_array(self) -> [T; N] {
        unsafe { transmute_copy(&self) }
    }
    /// Interprets a reference to a ```Vector``` as a reference to an array of scalars.
    /// - Array references can't be interpreted back as ```Vector``` because a vector may have a larger size and alignment than an array.
    #[inline(always)]
    fn as_array(&self) -> &[T; N] {
        unsafe { transmute(self) }
    }
    /// Interprets a reference to a ```Vector``` as a reference to an array of scalars.
    #[inline(always)]
    fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { transmute(self) }
    }
);

impl<const N: usize, T: ScalarInnerVecs> ScalarVecArrayApi<N, VecPacked> for T
where
    ScalarCount<N>: VecLen<N>,
{
    fn from_array(array: [Self; N]) -> InnerVector<N, Self, VecPacked> {
        array
    }
}
