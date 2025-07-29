use super::*;

impl<T: Scalar, A: VecAlignment> Quaternion<T, A> {
    #[inline(always)]
    pub const fn to_layout<A2: VecAlignment>(self) -> Quaternion<T, A2> {
        Quaternion {
            inner: self.inner.to_layout(),
        }
    }

    // Alignment

    #[inline(always)]
    pub const fn align(self) -> Quaternion<T, VecAligned> {
        Quaternion {
            inner: self.inner.align(),
        }
    }

    #[inline(always)]
    pub const fn unalign(self) -> Quaternion<T, VecPacked> {
        Quaternion {
            inner: self.inner.unalign(),
        }
    }
}
