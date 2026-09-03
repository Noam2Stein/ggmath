use crate::{Aligned, Alignment, Scalar, Unaligned, Vector};

/// TODO
#[repr(transparent)]
pub struct Rotation2<T, A: Alignment>(pub(crate) Vector<2, T, A>)
where
    T: Scalar;

/// TODO
pub type Rot2<T> = Rotation2<T, Unaligned>;

/// TODO
pub type Rot2A<T> = Rotation2<T, Aligned>;
