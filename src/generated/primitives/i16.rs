/// A module with `i16` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod i16_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub I16 => i16);
}
