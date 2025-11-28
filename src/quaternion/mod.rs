#![expect(missing_docs)]

use crate::{Alignment, Scalar, Vector};

#[repr(transparent)]
pub struct Quaternion<T: Scalar, A: Alignment>(Vector<4, T, A>);

impl<T: Scalar, A: Alignment> Clone for Quaternion<T, A> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Scalar, A: Alignment> Copy for Quaternion<T, A> {}
