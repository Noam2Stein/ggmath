use crate::{
    Usize,
    vector::{Scalar, VecAlignment, VecLen, Vector},
};

impl Scalar for bool {
    type InnerAlignedVec2 = glam::BVec2;
    type InnerAlignedVec3 = glam::BVec3A;
    type InnerAlignedVec4 = glam::BVec4A;

    const GARBAGE: Self = false;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = glam::BVec2::FALSE;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = glam::BVec3A::FALSE;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = glam::BVec4A::FALSE;
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    /// Returns true if all components of the vector are true.
    #[inline(always)]
    pub const fn all_true(self) -> bool {
        let mut i = 0;
        while i < N {
            if !self.as_array()[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Returns true if all components of the vector are false.
    #[inline(always)]
    pub const fn all_false(self) -> bool {
        let mut i = 0;
        while i < N {
            if self.as_array()[i] {
                return false;
            }
            i += 1;
        }

        true
    }

    /// Returns true if any component of the vector is true.
    #[inline(always)]
    pub const fn any_true(self) -> bool {
        let mut i = 0;
        while i < N {
            if self.as_array()[i] {
                return true;
            }
            i += 1;
        }

        false
    }

    /// Returns true if any component of the vector is false.
    #[inline(always)]
    pub const fn any_false(self) -> bool {
        let mut i = 0;
        while i < N {
            if !self.as_array()[i] {
                return true;
            }
            i += 1;
        }

        false
    }

    /// Returns the number of true components in the vector.
    #[inline(always)]
    pub const fn count_true(self) -> usize {
        let mut i = 0;
        let mut count = 0;
        while i < N {
            if self.as_array()[i] {
                count += 1;
            }
            i += 1;
        }

        count
    }

    /// Returns the number of false components in the vector.
    #[inline(always)]
    pub const fn count_false(self) -> usize {
        let mut i = 0;
        let mut count = 0;
        while i < N {
            if !self.as_array()[i] {
                count += 1;
            }
            i += 1;
        }

        count
    }
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    /// Returns `self == other` and supports const contexts.
    #[inline(always)]
    pub const fn const_eq(self, other: Vector<N, bool, impl VecAlignment>) -> bool {
        let mut i = 0;
        while i < N {
            if self.as_array()[i] != other.as_array()[i] {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Returns `self != other` and supports const contexts.
    #[inline(always)]
    pub const fn const_ne(self, other: Vector<N, bool, impl VecAlignment>) -> bool {
        let mut i = 0;
        while i < N {
            if self.as_array()[i] != other.as_array()[i] {
                return true;
            }
            i += 1;
        }
        false
    }

    /// Returns `self.eq_mask(other)` and supports const contexts.
    pub const fn const_eq_mask(
        self,
        other: Vector<N, bool, impl VecAlignment>,
    ) -> Vector<N, bool, A> {
        let mut output = Vector::<N, bool, A>::splat(false);
        let mut i = 0;
        while i < N {
            output.as_array_mut()[i] = self.as_array()[i] == other.as_array()[i];
            i += 1;
        }
        output
    }

    /// Returns `self.ne_mask(other)` and supports const contexts.
    pub const fn const_ne_mask(
        self,
        other: Vector<N, bool, impl VecAlignment>,
    ) -> Vector<N, bool, A> {
        let mut output = Vector::<N, bool, A>::splat(false);
        let mut i = 0;
        while i < N {
            output.as_array_mut()[i] = self.as_array()[i] != other.as_array()[i];
            i += 1;
        }
        output
    }

    /// Returns `self.lt_mask(other)` and supports const contexts.
    pub const fn const_lt_mask(
        self,
        other: Vector<N, bool, impl VecAlignment>,
    ) -> Vector<N, bool, A> {
        let mut output = Vector::<N, bool, A>::splat(false);
        let mut i = 0;
        while i < N {
            output.as_array_mut()[i] = self.as_array()[i] < other.as_array()[i];
            i += 1;
        }
        output
    }

    /// Returns `self.gt_mask(other)` and supports const contexts.
    pub const fn const_gt_mask(
        self,
        other: Vector<N, bool, impl VecAlignment>,
    ) -> Vector<N, bool, A> {
        let mut output = Vector::<N, bool, A>::splat(false);
        let mut i = 0;
        while i < N {
            output.as_array_mut()[i] = self.as_array()[i] > other.as_array()[i];
            i += 1;
        }
        output
    }

    /// Returns `self.le_mask(other)` and supports const contexts.
    pub const fn const_le_mask(
        self,
        other: Vector<N, bool, impl VecAlignment>,
    ) -> Vector<N, bool, A> {
        let mut output = Vector::<N, bool, A>::splat(false);
        let mut i = 0;
        while i < N {
            output.as_array_mut()[i] = self.as_array()[i] <= other.as_array()[i];
            i += 1;
        }
        output
    }

    /// Returns `self.ge_mask(other)` and supports const contexts.
    pub const fn const_ge_mask(
        self,
        other: Vector<N, bool, impl VecAlignment>,
    ) -> Vector<N, bool, A> {
        let mut output = Vector::<N, bool, A>::splat(false);
        let mut i = 0;
        while i < N {
            output.as_array_mut()[i] = self.as_array()[i] >= other.as_array()[i];
            i += 1;
        }
        output
    }
}
