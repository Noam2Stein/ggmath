/// A module with `u8` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod u8_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub U8 => u8);
}
