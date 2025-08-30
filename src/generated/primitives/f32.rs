/// A module with `f32` type aliases.
#[cfg(feature = "primitive_aliases")]
pub mod f32_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub F => f32);
}
