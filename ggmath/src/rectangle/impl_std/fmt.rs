use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Debug for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.resolve_repr_ref() {
            ReprResolvedRectangleRef::Centered(rect) => write!(
                f,
                "{{ center: {:?}, extents: {:?} }}",
                rect.center(),
                rect.extents()
            ),
            ReprResolvedRectangleRef::Cornered(rect) => {
                write!(f, "{{ min: {:?}, size: {:?} }}", rect.min(), rect.size())
            }
            ReprResolvedRectangleRef::MinMaxed(rect) => {
                write!(f, "{{ min: {:?}, max: {:?} }}", rect.min(), rect.max())
            }
        }
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Display for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.resolve_repr_ref() {
            ReprResolvedRectangleRef::Centered(rect) => write!(
                f,
                "{{ center: {}, extents: {} }}",
                rect.center(),
                rect.extents()
            ),
            ReprResolvedRectangleRef::Cornered(rect) => {
                write!(f, "{{ min: {}, size: {} }}", rect.min(), rect.size())
            }
            ReprResolvedRectangleRef::MinMaxed(rect) => {
                write!(f, "{{ min: {}, max: {} }}", rect.min(), rect.max())
            }
        }
    }
}
