use super::*;

mod resolve;
pub use resolve::*;

mod alignment;
mod length;
pub use alignment::*;
pub use length::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Converts the vector into the specified memory-layout generics.
    /// In the case of a vector, the only memory-layout generics it has is its alignment.
    #[inline(always)]
    pub const fn to_layout<AOutput: VecAlignment>(self) -> Vector<N, T, AOutput> {
        Vector::from_array(self.to_array())
    }
}
