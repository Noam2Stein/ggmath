/// A module with `u16` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod u16_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub U16 => u16);
}
