use crate::{vector::{VecAlignment, Vector, VecLen}, Usize};

/// A module with `bool` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod bool_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub B => bool);
}

impl<const N: usize, A: VecAlignment> Vector<N, bool, A>
where
    Usize<N>: VecLen,
{
    
}


