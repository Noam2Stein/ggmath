use super::*;

/// Sealed trait for ```ScalarCount```s that are valid as vector lengths.
///
/// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
///
/// This trait is implemented by ```ScalarCount<2/3/4>``` and is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen<N>```.
///
/// # Examples
/// ```
/// // Line is generic over dimension count.
/// use ggmath::vec::*;
///
/// struct Line<const N: usize>
/// where
///     ScalarCount<N>: VecLen<N>,
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
pub trait VecLen<const N: usize>: Seal + inner::VecLenInnerVec + api::VecLenApi<N>
where
    ScalarCount<N>: VecLen<N>,
{
}

/// Count of scalars that may or may not be a [```VecLen```].
///
/// Vectors can only have lengths 2, 3, or 4 because internally vector fns have differently optimized implementations for each length.
///
/// Only ```ScalarCount<2/3/4>``` implements ```VecLen```.
/// this is used to validate that a generic vector length is either 2, 3, or 4 with ```where ScalarCount<N>: VecLen<N>```.
///
/// # Examples
/// ```
/// // Line is generic over dimension count.
/// struct Line<const N: usize>
/// where
///     ScalarCount<N>: VecLen<N>,
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

impl Seal for ScalarCount<2> {}
impl Seal for ScalarCount<4> {}
impl Seal for ScalarCount<3> {}
impl VecLen<2> for ScalarCount<2> {}
impl VecLen<3> for ScalarCount<3> {}
impl VecLen<4> for ScalarCount<4> {}

trait Seal {}
