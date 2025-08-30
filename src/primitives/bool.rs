use crate::{
    Usize,
    vector::{Scalar, VecAlignment, VecLen, VecLenEnum, Vector},
};

impl Scalar for bool {
    type InnerAlignedVec2 = [Self; 2];
    type InnerAlignedVec3 = [Self; 3];
    type InnerAlignedVec4 = [Self; 4];

    const GARBAGE: Self = false;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = [false, false];
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = [false, false, false];
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = [false, false, false, false];
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    /// Returns true if all components of the vector are true.
    #[inline(always)]
    pub const fn all_true(self) -> bool {
        match Usize::<N>::ENUM {
            VecLenEnum::Two => self.as_array()[0] && self.as_array()[1],
            VecLenEnum::Three => self.as_array()[0] && self.as_array()[1] && self.as_array()[2],
            VecLenEnum::Four => {
                self.as_array()[0] && self.as_array()[1] && self.as_array()[2] && self.as_array()[3]
            }
        }
    }

    /// Returns true if all components of the vector are false.
    #[inline(always)]
    pub const fn all_false(self) -> bool {
        match Usize::<N>::ENUM {
            VecLenEnum::Two => !self.as_array()[0] && !self.as_array()[1],
            VecLenEnum::Three => !self.as_array()[0] && !self.as_array()[1] && !self.as_array()[2],
            VecLenEnum::Four => {
                !self.as_array()[0]
                    && !self.as_array()[1]
                    && !self.as_array()[2]
                    && !self.as_array()[3]
            }
        }
    }

    /// Returns true if any component of the vector is true.
    #[inline(always)]
    pub const fn any_true(self) -> bool {
        match Usize::<N>::ENUM {
            VecLenEnum::Two => self.as_array()[0] || self.as_array()[1],
            VecLenEnum::Three => self.as_array()[0] || self.as_array()[1] || self.as_array()[2],
            VecLenEnum::Four => {
                self.as_array()[0] || self.as_array()[1] || self.as_array()[2] || self.as_array()[3]
            }
        }
    }

    /// Returns true if any component of the vector is false.
    #[inline(always)]
    pub const fn any_false(self) -> bool {
        match Usize::<N>::ENUM {
            VecLenEnum::Two => !self.as_array()[0] || !self.as_array()[1],
            VecLenEnum::Three => !self.as_array()[0] || !self.as_array()[1] || !self.as_array()[2],
            VecLenEnum::Four => {
                !self.as_array()[0]
                    || !self.as_array()[1]
                    || !self.as_array()[2]
                    || !self.as_array()[3]
            }
        }
    }

    /// Returns the number of true components in the vector.
    #[inline(always)]
    pub const fn count_true(self) -> usize {
        match Usize::<N>::ENUM {
            VecLenEnum::Two => self.as_array()[0] as usize + self.as_array()[1] as usize,
            VecLenEnum::Three => {
                self.as_array()[0] as usize
                    + self.as_array()[1] as usize
                    + self.as_array()[2] as usize
            }
            VecLenEnum::Four => {
                self.as_array()[0] as usize
                    + self.as_array()[1] as usize
                    + self.as_array()[2] as usize
                    + self.as_array()[3] as usize
            }
        }
    }

    /// Returns the number of false components in the vector.
    #[inline(always)]
    pub const fn count_false(self) -> usize {
        match Usize::<N>::ENUM {
            VecLenEnum::Two => !self.as_array()[0] as usize + !self.as_array()[1] as usize,
            VecLenEnum::Three => {
                !self.as_array()[0] as usize
                    + !self.as_array()[1] as usize
                    + !self.as_array()[2] as usize
            }
            VecLenEnum::Four => {
                !self.as_array()[0] as usize
                    + !self.as_array()[1] as usize
                    + !self.as_array()[2] as usize
                    + !self.as_array()[3] as usize
            }
        }
    }
}
