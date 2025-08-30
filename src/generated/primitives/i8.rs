/// A module with `i8` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod i8_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub I8 => i8);
}
