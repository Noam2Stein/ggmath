use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const N: usize, T: AabbScalar + Debug, A: VecAlignment, R: AabbRepr> Debug for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.resolve_ref() {
            ResolvedAabbRef::Centered(rect) => write!(
                f,
                "{{ center: {:?}, extents: {:?} }}",
                rect.center(),
                rect.extents()
            ),
            ResolvedAabbRef::Cornered(rect) => {
                write!(f, "{{ min: {:?}, size: {:?} }}", rect.min(), rect.size())
            }
            ResolvedAabbRef::MinMaxed(rect) => {
                write!(f, "{{ min: {:?}, max: {:?} }}", rect.min(), rect.max())
            }
        }
    }
}

impl<const N: usize, T: AabbScalar + Display, A: VecAlignment, R: AabbRepr> Display
    for Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.resolve_ref() {
            ResolvedAabbRef::Centered(rect) => write!(
                f,
                "{{ center: {}, extents: {} }}",
                rect.center(),
                rect.extents()
            ),
            ResolvedAabbRef::Cornered(rect) => {
                write!(f, "{{ min: {}, size: {} }}", rect.min(), rect.size())
            }
            ResolvedAabbRef::MinMaxed(rect) => {
                write!(f, "{{ min: {}, max: {} }}", rect.min(), rect.max())
            }
        }
    }
}
