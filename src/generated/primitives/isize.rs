/// A module with `isize` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod isize_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub Isize => isize);
}
