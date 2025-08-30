/// A module with `i128` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod i128_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub I128 => i128);
}
