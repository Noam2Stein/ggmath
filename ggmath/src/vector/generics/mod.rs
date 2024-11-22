use super::*;

mod alignment;
mod length;
pub use alignment::*;
pub use length::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn into_storage<AOutput: VecAlignment>(self) -> Vector<N, T, AOutput> {
        self.into_alignment()
    }
}
