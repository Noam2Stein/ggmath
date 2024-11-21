use super::*;

/// Sealed trait for ```ScalarCount```s that are valid as vector lengths.
///
/// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
///
/// This trait is implemented by ```ScalarCount<2/3/4>``` and is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen```.
///
/// # Examples
/// ```
/// // Line is generic over dimension count.
/// use ggmath::vec::*;
///
/// struct Line<const N: usize>
/// where
///     ScalarCount<N>: VecLen,
/// {
///     start: VecN<N, f32>,
///     end: VecN<N, f32>,
/// }
///
/// type Line2D = Line<2>;
/// type Line3D = Line<3>;
/// type Line4D = Line<4>;
///
/// // Using an N that isn't 2, 3, or 4. Wont work.
/// struct InvalidStruct {
///     line: Line<6>, // ERROR: the trait bound `ScalarCount<6>: VecLen<6>` is not satisfied
/// }
/// ```
#[allow(private_bounds)]
pub trait VecLen: Seal + Sized + 'static + Send + Sync {
    type InnerAlignedVector<T: Scalar>: Construct;
}

/// Count of scalars that may or may not be a [```VecLen```].
///
/// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
///
/// Only ```ScalarCount<2/3/4>``` implements ```VecLen```.
/// this is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen```.
///
/// # Examples
/// ```
/// // Line is generic over dimension count.
/// struct Line<const N: usize>
/// where
///     ScalarCount<N>: VecLen,
/// {
///     start: FVecN<N>,
///     end: FVecN<N>,
/// }
///
/// type Line2D = Line<2>;
/// type Line3D = Line<3>;
/// type Line4D = Line<4>;
///
/// // Using an N that isn't 2, 3, or 4. Wont work.
/// struct InvalidStruct {
///     line: Line<6>, // ERROR: the trait bound `ScalarCount<6>: VecLen<6>` is not satisfied
/// }
/// ```
pub struct ScalarCount<const VALUE: usize>;

impl VecLen for ScalarCount<2> {
    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec2;
}
impl VecLen for ScalarCount<3> {
    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec4;
}
impl VecLen for ScalarCount<4> {
    type InnerAlignedVector<T: Scalar> = T::InnerAlignedVec4;
}

trait Seal {}
impl<const N: usize> Seal for ScalarCount<N> {}
