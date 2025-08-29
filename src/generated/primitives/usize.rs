#[cfg(feature = "primitive_aliases")]
pub mod usize_aliases {
    use crate::vector_aliases;

    vector_aliases!(pub Usize => usize);
}
