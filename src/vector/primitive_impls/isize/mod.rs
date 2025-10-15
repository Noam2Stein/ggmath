use crate::{Scalar, SimdBehaviour};

impl Scalar for isize {}

// TODO: use other ints as backend

impl SimdBehaviour<2> for isize {
    type VectorRepr = [isize; 2];
}

// TODO: use other ints as backend

impl SimdBehaviour<3> for isize {
    type VectorRepr = [isize; 3];
}

// TODO: use other ints as backend

impl SimdBehaviour<4> for isize {
    type VectorRepr = [isize; 4];
}
