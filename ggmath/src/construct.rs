//! base trait for mathamatical types.

/// The base trait for mathamatical types.
///
/// makes sure a type is [Sized], [Send], [Sync], and [Copy].
/// Is automatically implemented for types that implement its supertraits.
pub trait Construct: Sized + Send + Sync + Copy {}

impl<T: Sized + Send + Sync + Copy> Construct for T {}
