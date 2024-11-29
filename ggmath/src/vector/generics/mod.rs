use super::*;

mod alignment;
mod length;
pub use alignment::*;
pub use length::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    /// Converts the math-type into the specified storage generics.
    /// In the case of a vector only ```A: VecAlignment```.
    ///
    /// This function is present for every math-type.
    ///
    /// Because a vector's storage generics are only ```A```,
    /// and ```into_alignment``` does the same thing as this function,
    /// use this function when the math-type may change,
    /// For example into a matrix.
    #[inline(always)]
    pub fn into_storage<AOutput: VecAlignment>(self) -> Vector<N, T, AOutput> {
        self.into_alignment()
    }
}
