/// A module with `u64` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod u64_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub U64 => u64);
}
