use crate::{Scalar, SimdBehaviour};

impl Scalar for usize {}

// TODO: use other ints as backend

impl SimdBehaviour<2> for usize {
    type VectorRepr = [usize; 2];
}

// TODO: use other ints as backend

impl SimdBehaviour<3> for usize {
    type VectorRepr = [usize; 3];
}

// TODO: use other ints as backend

impl SimdBehaviour<4> for usize {
    type VectorRepr = [usize; 4];
}
