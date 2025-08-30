/// A module with `i32` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod i32_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub I => i32);
}
