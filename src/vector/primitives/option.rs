use crate::{Scalar, Usize, VecAlignment, VecLen, Vector};

impl<T: Scalar> Scalar for Option<T> {
    type InnerAlignedVec2 = [Option<T>; 2];
    type InnerAlignedVec3 = [Option<T>; 3];
    type InnerAlignedVec4 = [Option<T>; 4];

    const GARBAGE: Self = None;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = [None; _];
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = [None; _];
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = [None; _];
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, Option<T>, A>
where
    Usize<N>: VecLen,
{
    pub const fn flatten(self) -> Option<Vector<N, T, A>> {
        let mut output = Vector::GARBAGE;

        let mut i = 0;
        while i < N {
            if self.as_array()[i].is_none() {
                return None;
            }

            output.as_array_mut()[i] = self.as_array()[i].unwrap();

            i += 1;
        }

        Some(output)
    }
}
