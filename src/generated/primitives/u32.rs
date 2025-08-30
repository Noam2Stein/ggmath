/// A module with `u32` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod u32_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub U => u32);
}
