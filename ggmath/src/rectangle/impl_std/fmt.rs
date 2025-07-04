use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const N: usize, T: RectScalar + Debug, A: VecAlignment, R: RectRepr> Debug
    for Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.resolve_ref() {
            ResolvedRectangleRef::Centered(rect) => write!(
                f,
                "{{ center: {:?}, extents: {:?} }}",
                rect.center(),
                rect.extents()
            ),
            ResolvedRectangleRef::Cornered(rect) => {
                write!(f, "{{ min: {:?}, size: {:?} }}", rect.min(), rect.size())
            }
            ResolvedRectangleRef::MinMaxed(rect) => {
                write!(f, "{{ min: {:?}, max: {:?} }}", rect.min(), rect.max())
            }
        }
    }
}

impl<const N: usize, T: RectScalar + Display, A: VecAlignment, R: RectRepr> Display
    for Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.resolve_ref() {
            ResolvedRectangleRef::Centered(rect) => write!(
                f,
                "{{ center: {}, extents: {} }}",
                rect.center(),
                rect.extents()
            ),
            ResolvedRectangleRef::Cornered(rect) => {
                write!(f, "{{ min: {}, size: {} }}", rect.min(), rect.size())
            }
            ResolvedRectangleRef::MinMaxed(rect) => {
                write!(f, "{{ min: {}, max: {} }}", rect.min(), rect.max())
            }
        }
    }
}
