use crate::{Aligned, Alignment, Length, Scalar, Unaligned, Vector, length::Three};

/// TODO
#[repr(transparent)]
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    /// Currently, since only 3D rotors are supported, the internal field is
    /// always a vector4.
    ///
    /// If additional dimensions are ever supported, this will need to be
    /// changed.
    pub(crate) Vector<4, T, A>,
)
where
    Length<N>: Three,
    T: Scalar;

/// TODO
pub type Rotor3<T> = Rotor<3, T, Unaligned>;

/// TODO
pub type Rotor3A<T> = Rotor<3, T, Aligned>;
