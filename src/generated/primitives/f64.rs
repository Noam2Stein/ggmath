/// A module with `f64` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod f64_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub D => f64);
}
