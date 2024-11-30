//! The base trait for mathamatical types.

/// The base trait for mathamatical types.
///
/// Makes sure a type is [Sized], [Send], [Sync], [Copy], and 'static.
/// Is automatically implemented for types that implement its supertraits.
pub trait Construct: Sized + Send + Sync + Copy + 'static {}

impl<T: Sized + Send + Sync + Copy + 'static> Construct for T {}
