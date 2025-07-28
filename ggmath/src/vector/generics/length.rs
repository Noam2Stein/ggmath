use super::*;

/// Trait that marks a [`Usize`] value as a valid length for a vector.
///
/// This trait is implemented for `2`, `3` and `4`.
/// If this trait were to be implemented for an additional value, it would be a fully supported vector/matrix length/dimension.
///
/// In the future if rust has more powerful const-generics,
/// this trait could be deleted and all `usize` values will be valid vector lengths.
pub unsafe trait VecLen {
    /// Is used to "redirect" the vector to its alignment marker-type through this pattern:
    ///
    /// trait VecAlignment {
    ///     type Alignment<const N: usize, T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait VecLen {
    ///     type Alignment<T: Scalar>: AlignTrait;
    /// }
    ///
    /// trait Scalar {
    ///     type Vec2Alignment: AlignTrait;
    ///     type Vec3Alignment: AlignTrait;
    ///     type Vec4Alignment: AlignTrait;
    /// }
    type Alignment<T: Scalar>: AlignTrait;
}

unsafe impl VecLen for Usize<2> {
    type Alignment<T: Scalar> = T::Vec2Alignment;
}
unsafe impl VecLen for Usize<3> {
    type Alignment<T: Scalar> = T::Vec3Alignment;
}
unsafe impl VecLen for Usize<4> {
    type Alignment<T: Scalar> = T::Vec4Alignment;
}
