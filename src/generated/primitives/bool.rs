#[cfg(feature = "primitive_aliases")]
pub mod bool_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub B => bool);
}
