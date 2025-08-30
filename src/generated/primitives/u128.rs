/// A module with `u128` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod u128_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub U128 => u128);
}
